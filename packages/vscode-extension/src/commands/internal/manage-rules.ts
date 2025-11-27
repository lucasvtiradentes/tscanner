import * as vscode from 'vscode';
import {
  type TscannerConfig,
  getDefaultConfig,
  hasCustomConfig,
  hasGlobalConfig,
  hasLocalConfig,
  loadEffectiveConfig,
  saveCustomConfig,
  saveGlobalConfig,
  saveLocalConfig,
} from '../../common/lib/config-manager';
import { RustClient } from '../../common/lib/rust-client';
import { getRustBinaryPath } from '../../common/lib/scanner';
import {
  Command,
  ToastKind,
  executeCommand,
  getCurrentWorkspaceFolder,
  registerCommand,
  showToastMessage,
} from '../../common/lib/vscode-utils';
import { logger } from '../../common/utils/logger';
import { ConfigLocation, showConfigLocationMenuForFirstSetup } from './settings-menu';

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

export function createManageRulesCommand(
  updateStatusBar: () => Promise<void>,
  context: vscode.ExtensionContext,
  currentCustomConfigDirRef: { current: string | null },
) {
  return registerCommand(Command.ManageRules, async () => {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      showToastMessage(ToastKind.Error, 'No workspace folder open');
      return;
    }

    const workspacePath = workspaceFolder.uri.fsPath;
    const customConfigDir = currentCustomConfigDirRef.current;

    const binaryPath = getRustBinaryPath();
    if (!binaryPath) {
      showToastMessage(ToastKind.Error, 'TScanner: Rust binary not found. Please build the Rust core first.');
      return;
    }

    const client = new RustClient(binaryPath);
    await client.start();

    try {
      const rules = await client.getRulesMetadata();

      logger.info(`Loading config for manage-rules, customConfigDir: ${customConfigDir ?? 'null'}`);
      const existingConfig = (await loadEffectiveConfig(context, workspacePath, customConfigDir)) || getDefaultConfig();
      logger.info(`Loaded config with ${Object.keys(existingConfig.builtinRules || {}).length} builtin rules`);

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

      const hasCustom = customConfigDir ? await hasCustomConfig(workspacePath, customConfigDir) : false;
      const hasLocal = await hasLocalConfig(workspacePath);
      const hasGlobal = await hasGlobalConfig(context, workspacePath);
      const hasAnyConfig = hasCustom || hasLocal || hasGlobal;

      if (hasAnyConfig) {
        if (hasCustom && customConfigDir) {
          await saveCustomConfig(workspacePath, customConfigDir, config);
          logger.info(`Updated custom config at ${customConfigDir}`);
          showToastMessage(ToastKind.Info, `Rules saved to ${customConfigDir}`);
        } else if (hasLocal) {
          await saveLocalConfig(workspacePath, config);
          logger.info('Updated local .tscanner config');
          showToastMessage(ToastKind.Info, 'Rules saved to .tscanner');
        } else if (hasGlobal) {
          await saveGlobalConfig(context, workspacePath, config);
          logger.info('Updated global config (extension storage)');
          showToastMessage(ToastKind.Info, 'Rules saved to extension storage');
        }
      } else {
        const locationResult = await showConfigLocationMenuForFirstSetup(currentCustomConfigDirRef, context);

        if (!locationResult) {
          await client.stop();
          return;
        }

        switch (locationResult.location) {
          case ConfigLocation.ExtensionStorage:
            await saveGlobalConfig(context, workspacePath, config);
            logger.info('Saved to global config (extension storage)');
            showToastMessage(ToastKind.Info, 'Rules saved to extension storage');
            break;
          case ConfigLocation.ProjectFolder:
            await saveLocalConfig(workspacePath, config);
            logger.info('Saved to local .tscanner config');
            showToastMessage(ToastKind.Info, 'Rules saved to .tscanner');
            break;
          case ConfigLocation.CustomPath:
            if (locationResult.customPath) {
              await saveCustomConfig(workspacePath, locationResult.customPath, config);
              logger.info(`Saved to custom config at ${locationResult.customPath}`);
              showToastMessage(ToastKind.Info, `Rules saved to ${locationResult.customPath}`);
            }
            break;
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
