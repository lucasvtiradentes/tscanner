import * as vscode from 'vscode';
import {
  type TscannerConfig,
  getDefaultConfig,
  loadEffectiveConfig,
  saveGlobalConfig,
  saveLocalConfig,
  shouldSyncToLocal,
} from '../common/lib/config-manager';
import { RustClient } from '../common/lib/rust-client';
import { getRustBinaryPath } from '../common/lib/scanner';
import {
  Command,
  ToastKind,
  executeCommand,
  getCurrentWorkspaceFolder,
  registerCommand,
  showToastMessage,
} from '../common/lib/vscode-utils';
import { logger } from '../common/utils/logger';

interface RuleQuickPickItem extends vscode.QuickPickItem {
  ruleName: string;
  picked: boolean;
  isCustom: boolean;
}

function getCategoryIcon(category: string): string {
  const icons: Record<string, string> = {
    typesafety: 'shield',
    variables: 'symbol-variable',
    imports: 'package',
    codequality: 'beaker',
    bugprevention: 'bug',
    style: 'symbol-color',
    performance: 'dashboard',
  };
  return icons[category] || 'circle-outline';
}

export function createManageRulesCommand(updateStatusBar: () => Promise<void>, context: vscode.ExtensionContext) {
  return registerCommand(Command.ManageRules, async () => {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      showToastMessage(ToastKind.Error, 'No workspace folder open');
      return;
    }

    const workspacePath = workspaceFolder.uri.fsPath;

    const binaryPath = getRustBinaryPath();
    if (!binaryPath) {
      showToastMessage(ToastKind.Error, 'Tscanner: Rust binary not found. Please build the Rust core first.');
      return;
    }

    const client = new RustClient(binaryPath);
    await client.start();

    try {
      const rules = await client.getRulesMetadata();

      const existingConfig = (await loadEffectiveConfig(context, workspacePath)) || getDefaultConfig();

      const customRuleTypeMap = {
        regex: { icon: '$(regex)', detailKey: 'pattern' as const },
        script: { icon: '$(file-code)', detailKey: 'script' as const },
        ai: { icon: '$(sparkle)', detailKey: 'prompt' as const },
      };

      const customRules: RuleQuickPickItem[] = [];

      if (existingConfig?.customRules) {
        for (const [ruleName, ruleConfig] of Object.entries(existingConfig.customRules)) {
          const typeInfo = customRuleTypeMap[ruleConfig.type];
          customRules.push({
            label: `${typeInfo.icon} ${ruleName}`,
            description: `[${ruleConfig.type.toUpperCase()}] custom`,
            detail: ruleConfig.message || ruleConfig[typeInfo.detailKey] || '',
            ruleName,
            picked: ruleConfig.enabled ?? true,
            isCustom: true,
          });
        }
      }

      const rulesByCategory = new Map<string, RuleQuickPickItem[]>();

      for (const rule of rules) {
        const existingRule = existingConfig?.builtinRules?.[rule.name];
        const isEnabled = existingRule?.enabled ?? existingRule !== undefined;

        const ruleItem: RuleQuickPickItem = {
          label: `$(${getCategoryIcon(rule.category)}) ${rule.displayName}`,
          description: `[${rule.ruleType.toUpperCase()}] ${rule.defaultSeverity}`,
          detail: rule.description,
          ruleName: rule.name,
          picked: isEnabled,
          isCustom: false,
        };

        const category = rule.category;
        if (!rulesByCategory.has(category)) {
          rulesByCategory.set(category, []);
        }
        rulesByCategory.get(category)?.push(ruleItem);
      }

      const categoryOrder = [
        'typesafety',
        'variables',
        'imports',
        'codequality',
        'bugprevention',
        'style',
        'performance',
      ];
      const categoryLabels: Record<string, string> = {
        typesafety: 'Type Safety',
        variables: 'Variables',
        imports: 'Imports',
        codequality: 'Code Quality',
        bugprevention: 'Bug Prevention',
        style: 'Style',
        performance: 'Performance',
      };

      const items: RuleQuickPickItem[] = [
        ...(customRules.length > 0
          ? [{ label: 'Custom Rules', kind: vscode.QuickPickItemKind.Separator } as any, ...customRules]
          : []),
      ];

      for (const category of categoryOrder) {
        const categoryRules = rulesByCategory.get(category);
        if (categoryRules && categoryRules.length > 0) {
          items.push(
            { label: categoryLabels[category], kind: vscode.QuickPickItemKind.Separator } as any,
            ...categoryRules,
          );
        }
      }

      const selected = await vscode.window.showQuickPick(items, {
        placeHolder: 'Select rules to enable (Space to toggle, Enter to confirm)',
        canPickMany: true,
        ignoreFocusOut: true,
      });

      if (!selected) {
        await client.stop();
        return;
      }

      const enabledRules = new Set(
        selected.filter((item): item is RuleQuickPickItem => 'ruleName' in item).map((item) => item.ruleName),
      );

      logger.info(`User selected ${enabledRules.size} rules: ${Array.from(enabledRules).join(', ')}`);

      const config: TscannerConfig = existingConfig;

      if (!config.builtinRules) {
        config.builtinRules = {};
      }
      if (!config.customRules) {
        config.customRules = {};
      }

      for (const rule of rules) {
        if (enabledRules.has(rule.name)) {
          const existingRuleConfig = config.builtinRules[rule.name];
          if (!existingRuleConfig) {
            config.builtinRules[rule.name] = {};
          } else {
            existingRuleConfig.enabled = undefined;
          }
        } else {
          const existingRuleConfig = config.builtinRules[rule.name];
          if (existingRuleConfig && Object.keys(existingRuleConfig).length > 0) {
            existingRuleConfig.enabled = false;
          } else {
            delete config.builtinRules[rule.name];
          }
        }
      }

      for (const customRule of customRules) {
        const existingCustom = existingConfig?.customRules?.[customRule.ruleName];
        if (existingCustom) {
          if (enabledRules.has(customRule.ruleName)) {
            existingCustom.enabled = undefined;
          } else {
            existingCustom.enabled = false;
          }
          config.customRules[customRule.ruleName] = existingCustom;
        }
      }

      const isUserManaged = !(await shouldSyncToLocal(workspacePath));

      let saveLocation: 'global' | 'local' | undefined;

      if (isUserManaged) {
        await saveLocalConfig(workspacePath, config);
        logger.info('Updated user-managed local .tscanner/rules.json');
      } else {
        const locationChoice = await vscode.window.showQuickPick(
          [
            {
              label: '$(cloud) Extension Storage (Recommended)',
              description: 'Managed by extension, synced across projects',
              detail: 'Config saved in extension folder',
              value: 'global',
            },
            {
              label: '$(file) Project Folder',
              description: 'Local to this project only',
              detail: 'Creates .tscanner/rules.json in project (can be committed to git)',
              value: 'local',
            },
          ],
          {
            placeHolder: 'Where do you want to save the rules configuration?',
            ignoreFocusOut: true,
          },
        );

        if (!locationChoice) {
          await client.stop();
          return;
        }

        saveLocation = locationChoice.value as 'global' | 'local';

        if (saveLocation === 'global') {
          await saveGlobalConfig(context, workspacePath, config);
          logger.info('Saved to global config (extension storage)');
          showToastMessage(ToastKind.Info, 'Rules saved to extension storage');
        } else {
          await saveLocalConfig(workspacePath, config);
          logger.info('Saved to local .tscanner/rules.json (user-managed)');
          showToastMessage(ToastKind.Info, 'Rules saved to .tscanner/rules.json');
        }
      }

      await client.stop();

      await updateStatusBar();

      await executeCommand(Command.FindIssue);
    } catch (error) {
      logger.error(`Failed to manage rules: ${error}`);
      await client.stop();
      showToastMessage(ToastKind.Error, `Failed to load rules: ${error}`);
    }
  });
}
