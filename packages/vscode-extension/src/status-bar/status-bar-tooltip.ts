import { DEV_SUFFIX } from 'src/common/scripts-constants';
import { CODE_EDITOR_DEFAULTS, DISPLAY_ICONS, type TscannerConfig } from 'tscanner-common';
import * as vscode from 'vscode';
import { getConfigDirLabel } from '../common/lib/config-manager';
import { getBinaryVersionLabel, getExtensionVersionLabel } from '../common/lib/version-checker';
import { type BinaryInfo, LOCATOR_SOURCE_LABELS } from '../locator';

function getAiProviderLabel(config: TscannerConfig | null): string {
  if (!config?.ai?.provider) {
    return 'None';
  }
  const provider = config.ai.provider;
  return provider.charAt(0).toUpperCase() + provider.slice(1);
}

function getActiveRulesLabel(config: TscannerConfig | null): string {
  if (!config) return 'None';

  const parts: string[] = [];

  const builtin = config.rules.builtin ? Object.keys(config.rules.builtin).length : 0;
  const regex = config.rules.regex ? Object.keys(config.rules.regex).length : 0;
  const script = config.rules.script ? Object.keys(config.rules.script).length : 0;
  const ai = config.aiRules ? Object.keys(config.aiRules).length : 0;

  if (builtin > 0) parts.push(`${DISPLAY_ICONS.builtin} ${builtin}`);
  if (regex > 0) parts.push(`${DISPLAY_ICONS.regex} ${regex}`);
  if (script > 0) parts.push(`${DISPLAY_ICONS.script} ${script}`);
  if (ai > 0) parts.push(`${DISPLAY_ICONS.ai} ${ai}`);

  return parts.length > 0 ? parts.join('  ') : 'None';
}

function formatAutoInterval(seconds: number): string {
  if (seconds === 0) return 'off';
  if (seconds < 60) return `${seconds}s`;
  const minutes = Math.floor(seconds / 60);
  return `${minutes}m`;
}

function getScanSettingsLabel(config: TscannerConfig | null): string {
  const startup = config?.codeEditor?.startupScan ?? CODE_EDITOR_DEFAULTS.startupScan;
  const autoScanInterval = config?.codeEditor?.autoScanInterval ?? CODE_EDITOR_DEFAULTS.autoScanInterval;
  const autoLabel = formatAutoInterval(autoScanInterval);
  return `startup ${startup}, auto ${autoLabel}`;
}

function getAiScanSettingsLabel(config: TscannerConfig | null): string {
  const startup = config?.codeEditor?.startupAiScan ?? CODE_EDITOR_DEFAULTS.startupAiScan;
  const autoAiScanInterval = config?.codeEditor?.autoAiScanInterval ?? CODE_EDITOR_DEFAULTS.autoAiScanInterval;
  const autoLabel = formatAutoInterval(autoAiScanInterval);
  return `startup ${startup}, auto ${autoLabel}`;
}

export function buildConfiguredTooltip(
  configDir: string | null,
  config: TscannerConfig | null,
  binaryInfo: BinaryInfo,
  versionWarning: string | null = null,
  invalidConfigFields: string[] = [],
): vscode.MarkdownString {
  const configLabel = getConfigDirLabel(configDir);
  const configSource = LOCATOR_SOURCE_LABELS[binaryInfo.source];

  const extensionLabel = getExtensionVersionLabel();
  const binaryVersionLabel = getBinaryVersionLabel();
  const binaryLabel = binaryVersionLabel === DEV_SUFFIX ? DEV_SUFFIX : `${configSource} (${binaryVersionLabel})`;
  const versionLabel = `ext ${extensionLabel}, cli ${binaryLabel}`;

  const aiProviderLabel = getAiProviderLabel(config);
  const activeRulesLabel = getActiveRulesLabel(config);
  const scanSettingsLabel = getScanSettingsLabel(config);
  const aiScanSettingsLabel = getAiScanSettingsLabel(config);

  const rows = [
    ['Version', versionLabel],
    ['Config', configLabel],
    ['Active Rules', activeRulesLabel],
    ['Scan', scanSettingsLabel],
    ['AI Scan', aiScanSettingsLabel],
    config?.ai?.provider ? ['AI Provider', aiProviderLabel] : null,
  ].filter(Boolean) as string[][];

  let content = ['| | |', '|---|---|', ...rows.map(([label, value]) => `| **${label}** | ${value} |`)].join('\n');

  if (versionWarning) {
    content += `\n\n⚠️ **Version Warning**: ${versionWarning}`;
  }

  if (invalidConfigFields.length > 0) {
    content += `\n\n⚠️ **Invalid Config Fields**: ${invalidConfigFields.join(', ')}`;
  }

  const md = new vscode.MarkdownString(content);
  md.supportHtml = true;
  return md;
}
