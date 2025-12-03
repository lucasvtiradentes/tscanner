import { CustomRuleType, RuleCategory } from 'tscanner-common';
import * as vscode from 'vscode';
import { CONFIG_DIR_NAME } from '../common/constants';
import {
  type TscannerConfig,
  getConfigState,
  getDefaultConfig,
  loadEffectiveConfig,
  saveCustomConfig,
  saveGlobalConfig,
  saveLocalConfig,
} from '../common/lib/config-manager';
import { getRustBinaryPath } from '../common/lib/scanner';
import {
  Command,
  ToastKind,
  executeCommand,
  registerCommand,
  requireWorkspaceOrNull,
  showToastMessage,
} from '../common/lib/vscode-utils';
import { logger } from '../common/utils/logger';
import { TscannerLspClient } from '../lsp';
import { ConfigLocation, showConfigLocationMenuForFirstSetup } from './config-location';

type RuleQuickPickItem = vscode.QuickPickItem & {
  ruleName: string;
  picked: boolean;
  isCustom: boolean;
};

const CUSTOM_RULE_TYPE_CONFIG: Record<CustomRuleType, { icon: string; detailKey: 'pattern' | 'script' | 'prompt' }> = {
  [CustomRuleType.Regex]: { icon: '$(regex)', detailKey: 'pattern' },
  [CustomRuleType.Script]: { icon: '$(file-code)', detailKey: 'script' },
  [CustomRuleType.Ai]: { icon: '$(sparkle)', detailKey: 'prompt' },
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

function getCustomRuleDetail(ruleConfig: NonNullable<TscannerConfig['customRules']>[string]): string {
  if (ruleConfig.message) return ruleConfig.message;
  switch (ruleConfig.type) {
    case CustomRuleType.Regex:
      return ruleConfig.pattern;
    case CustomRuleType.Script:
      return ruleConfig.script;
    case CustomRuleType.Ai:
      return ruleConfig.prompt;
    default:
      return '';
  }
}

function buildCustomRuleItems(existingConfig: TscannerConfig): RuleQuickPickItem[] {
  const customRules: RuleQuickPickItem[] = [];

  if (existingConfig?.customRules) {
    for (const [ruleName, ruleConfig] of Object.entries(existingConfig.customRules)) {
      const typeInfo = CUSTOM_RULE_TYPE_CONFIG[ruleConfig.type];
      customRules.push({
        label: `${typeInfo.icon} ${ruleName}`,
        description: `[${ruleConfig.type.toUpperCase()}] custom`,
        detail: getCustomRuleDetail(ruleConfig),
        ruleName,
        picked: ruleConfig.enabled ?? true,
        isCustom: true,
      });
    }
  }

  return customRules;
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

    const binaryPath = getRustBinaryPath();
    if (!binaryPath) {
      showToastMessage(ToastKind.Error, 'TScanner: Rust binary not found. Please build the Rust core first.');
      return;
    }

    const client = new TscannerLspClient(binaryPath);
    await client.start(workspacePath);

    try {
      const rules = await client.getRulesMetadata();

      logger.info(`Loading config for manage-rules, customConfigDir: ${customConfigDir ?? 'null'}`);
      const existingConfig = (await loadEffectiveConfig(context, workspacePath, customConfigDir)) || getDefaultConfig();
      logger.info(`Loaded config with ${Object.keys(existingConfig.builtinRules || {}).length} builtin rules`);

      const customRules = buildCustomRuleItems(existingConfig);
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
