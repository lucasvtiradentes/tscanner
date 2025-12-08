import { EXTENSION_DISPLAY_NAME } from 'src/common/scripts-constants';
import { CONFIG_DIR_NAME, PACKAGE_NAME, RuleCategory, type TscannerConfig } from 'tscanner-common';
import * as vscode from 'vscode';
import {
  getConfigState,
  getDefaultConfig,
  loadEffectiveConfig,
  saveCustomConfig,
  saveGlobalConfig,
  saveLocalConfig,
} from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import {
  Command,
  ToastKind,
  executeCommand,
  registerCommand,
  requireWorkspaceOrNull,
  showToastMessage,
} from '../common/lib/vscode-utils';
import { Locator } from '../locator';
import { TscannerLspClient } from '../lsp/client';
import { ConfigLocation, showConfigLocationMenuForFirstSetup } from './config-location';

type RuleQuickPickItem = vscode.QuickPickItem & {
  ruleName: string;
  picked: boolean;
  isCustom: boolean;
  ruleKind?: 'regex' | 'script' | 'ai';
};

const CATEGORY_ICONS: Record<RuleCategory, string> = {
  [RuleCategory.TypeSafety]: 'shield',
  [RuleCategory.CodeQuality]: 'beaker',
  [RuleCategory.Style]: 'symbol-color',
  [RuleCategory.Performance]: 'dashboard',
};

const CATEGORY_LABELS: Record<RuleCategory, string> = {
  [RuleCategory.TypeSafety]: 'Type Safety',
  [RuleCategory.CodeQuality]: 'Code Quality',
  [RuleCategory.Style]: 'Style',
  [RuleCategory.Performance]: 'Performance',
};

const CATEGORY_ORDER: RuleCategory[] = [
  RuleCategory.TypeSafety,
  RuleCategory.CodeQuality,
  RuleCategory.Style,
  RuleCategory.Performance,
];

function getCategoryIcon(category: string): string {
  return CATEGORY_ICONS[category as RuleCategory] || 'circle-outline';
}

function buildCustomRuleItems(existingConfig: TscannerConfig): RuleQuickPickItem[] {
  const items: RuleQuickPickItem[] = [];

  if (existingConfig?.rules?.regex) {
    for (const [ruleName, ruleConfig] of Object.entries(existingConfig.rules.regex)) {
      items.push({
        label: `$(regex) ${ruleName}`,
        description: '[REGEX] custom',
        detail: ruleConfig.message || ruleConfig.pattern,
        ruleName,
        picked: ruleConfig.enabled ?? true,
        isCustom: true,
        ruleKind: 'regex',
      });
    }
  }

  if (existingConfig?.rules?.script) {
    for (const [ruleName, ruleConfig] of Object.entries(existingConfig.rules.script)) {
      items.push({
        label: `$(file-code) ${ruleName}`,
        description: '[SCRIPT] custom',
        detail: ruleConfig.message || ruleConfig.command,
        ruleName,
        picked: ruleConfig.enabled ?? true,
        isCustom: true,
        ruleKind: 'script',
      });
    }
  }

  if (existingConfig?.aiRules) {
    for (const [ruleName, ruleConfig] of Object.entries(existingConfig.aiRules)) {
      items.push({
        label: `$(sparkle) ${ruleName}`,
        description: '[AI] custom',
        detail: ruleConfig.message || ruleConfig.prompt,
        ruleName,
        picked: ruleConfig.enabled ?? true,
        isCustom: true,
        ruleKind: 'ai',
      });
    }
  }

  return items;
}

export function createManageRulesCommand(
  updateStatusBar: () => Promise<void>,
  context: vscode.ExtensionContext,
  currentCustomConfigDirRef: { current: string | null },
) {
  return registerCommand(Command.ManageRules, async () => {
    const workspaceFolder = requireWorkspaceOrNull();
    if (!workspaceFolder) return;

    const workspacePath = workspaceFolder.uri.fsPath;
    const customConfigDir = currentCustomConfigDirRef.current;

    const locator = new Locator(workspacePath);
    const result = await locator.locate();

    if (!result) {
      showToastMessage(
        ToastKind.Error,
        `${EXTENSION_DISPLAY_NAME} binary not found. Install with: npm install -g ${PACKAGE_NAME}`,
      );
      return;
    }

    const client = new TscannerLspClient(result.path, ['lsp']);
    await client.start(workspacePath);

    try {
      const rules = await client.getRulesMetadata();

      logger.info(`Loading config for manage-rules, customConfigDir: ${customConfigDir ?? 'null'}`);
      const existingConfig = (await loadEffectiveConfig(context, workspacePath, customConfigDir)) || getDefaultConfig();
      logger.info(`Loaded config with ${Object.keys(existingConfig.rules?.builtin || {}).length} builtin rules`);

      const customRules = buildCustomRuleItems(existingConfig);
      const rulesByCategory = new Map<string, RuleQuickPickItem[]>();

      for (const rule of rules) {
        const existingRule = existingConfig?.rules?.builtin?.[rule.name];
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

      const items: RuleQuickPickItem[] = [
        ...(customRules.length > 0
          ? [{ label: 'Custom Rules', kind: vscode.QuickPickItemKind.Separator } as any, ...customRules]
          : []),
      ];

      for (const category of CATEGORY_ORDER) {
        const categoryRules = rulesByCategory.get(category);
        if (categoryRules && categoryRules.length > 0) {
          items.push(
            { label: CATEGORY_LABELS[category], kind: vscode.QuickPickItemKind.Separator } as any,
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

      if (!config.rules) {
        config.rules = {};
      }
      if (!config.rules.builtin) {
        config.rules.builtin = {};
      }
      if (!config.rules.regex) {
        config.rules.regex = {};
      }
      if (!config.rules.script) {
        config.rules.script = {};
      }
      if (!config.aiRules) {
        config.aiRules = {};
      }

      for (const rule of rules) {
        if (enabledRules.has(rule.name)) {
          const existingRuleConfig = config.rules.builtin[rule.name];
          if (!existingRuleConfig) {
            config.rules.builtin[rule.name] = {};
          } else {
            existingRuleConfig.enabled = undefined;
          }
        } else {
          const existingRuleConfig = config.rules.builtin[rule.name];
          if (existingRuleConfig && Object.keys(existingRuleConfig).length > 0) {
            existingRuleConfig.enabled = false;
          } else {
            delete config.rules.builtin[rule.name];
          }
        }
      }

      for (const customRule of customRules) {
        if (customRule.ruleKind === 'regex') {
          const existingRule = existingConfig?.rules?.regex?.[customRule.ruleName];
          if (existingRule) {
            existingRule.enabled = enabledRules.has(customRule.ruleName) ? undefined : false;
            config.rules.regex[customRule.ruleName] = existingRule;
          }
        } else if (customRule.ruleKind === 'script') {
          const existingRule = existingConfig?.rules?.script?.[customRule.ruleName];
          if (existingRule) {
            existingRule.enabled = enabledRules.has(customRule.ruleName) ? undefined : false;
            config.rules.script[customRule.ruleName] = existingRule;
          }
        } else if (customRule.ruleKind === 'ai') {
          const existingRule = existingConfig?.aiRules?.[customRule.ruleName];
          if (existingRule) {
            existingRule.enabled = enabledRules.has(customRule.ruleName) ? undefined : false;
            config.aiRules[customRule.ruleName] = existingRule;
          }
        }
      }

      const configState = await getConfigState(context, workspacePath, customConfigDir);

      if (configState.hasAny) {
        if (configState.hasCustom && customConfigDir) {
          await saveCustomConfig(workspacePath, customConfigDir, config);
          logger.info(`Updated custom config at ${customConfigDir}`);
          showToastMessage(ToastKind.Info, `Rules saved to ${customConfigDir}`);
        } else if (configState.hasLocal) {
          await saveLocalConfig(workspacePath, config);
          logger.info(`Updated local ${CONFIG_DIR_NAME} config`);
          showToastMessage(ToastKind.Info, `Rules saved to ${CONFIG_DIR_NAME}`);
        } else if (configState.hasGlobal) {
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
            logger.info(`Saved to local ${CONFIG_DIR_NAME} config`);
            showToastMessage(ToastKind.Info, `Rules saved to ${CONFIG_DIR_NAME}`);
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
