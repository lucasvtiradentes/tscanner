import { DEV_SUFFIX } from 'src/common/scripts-constants';
import { DISPLAY_ICONS, type TscannerConfig } from 'tscanner-common';
import * as vscode from 'vscode';
import { getConfigDirLabel } from '../common/lib/config-manager';
import { getBinaryVersionLabel, getExtensionVersionLabel } from '../common/lib/version-checker';
import { ExtensionConfigKey, getExtensionConfig } from '../common/state/extension-config';
import { type BinaryInfo, LOCATOR_SOURCE_LABELS } from '../locator';

function extractSchemaVersion(schemaUrl: string | undefined): string | null {
  if (!schemaUrl) return null;

  const match = schemaUrl.match(/tscanner@(\d+\.\d+\.\d+)/);
  if (!match) return null;

  return match[1];
}

export function getSchemaVersionWarning(config: TscannerConfig | null, binaryVersion: string): string | null {
  const schemaVersion = extractSchemaVersion(config?.$schema);
  if (!schemaVersion) return null;

  const cleanBinaryVersion = binaryVersion.replace(/^v/, '');
  if (schemaVersion === cleanBinaryVersion) return null;

  return `Config schema version (${schemaVersion}) does not match CLI version (${cleanBinaryVersion}). Update the $schema URL in your config file`;
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

function getScanSettingsLabel(): string {
  const startup = getExtensionConfig(ExtensionConfigKey.StartupScan);
  const autoScanInterval = getExtensionConfig(ExtensionConfigKey.AutoScanInterval);
  const autoLabel = formatAutoInterval(autoScanInterval);
  return `startup ${startup}, auto ${autoLabel}`;
}

function getAiScanSettingsLabel(): string {
  const startup = getExtensionConfig(ExtensionConfigKey.StartupAiScan);
  const autoAiScanInterval = getExtensionConfig(ExtensionConfigKey.AutoAiScanInterval);
  const autoLabel = formatAutoInterval(autoAiScanInterval);
  return `startup ${startup}, auto ${autoLabel}`;
}

type BuildConfiguredTooltipParams = {
  configDir: string | null;
  config: TscannerConfig | null;
  binaryInfo: BinaryInfo;
  versionWarning?: string | null;
  invalidConfigFields?: string[];
};

export function buildConfiguredTooltip(params: BuildConfiguredTooltipParams): vscode.MarkdownString {
  const { configDir, config, binaryInfo, versionWarning = null, invalidConfigFields = [] } = params;
  const configLabel = getConfigDirLabel(configDir);
  const configSource = LOCATOR_SOURCE_LABELS[binaryInfo.source];

  const extensionLabel = getExtensionVersionLabel();
  const binaryVersionLabel = getBinaryVersionLabel();
  const binaryLabel = binaryVersionLabel === DEV_SUFFIX ? DEV_SUFFIX : `${configSource} (${binaryVersionLabel})`;
  const versionLabel = `ext ${extensionLabel}, cli ${binaryLabel}`;

  const activeRulesLabel = getActiveRulesLabel(config);
  const scanSettingsLabel = getScanSettingsLabel();
  const aiScanSettingsLabel = getAiScanSettingsLabel();

  const rows = [
    ['Version', versionLabel],
    ['Config', configLabel],
    ['Active Rules', activeRulesLabel],
    ['Scan', scanSettingsLabel],
    ['AI Scan', aiScanSettingsLabel],
  ].filter(Boolean) as string[][];

  let content = ['| | |', '|---|---|', ...rows.map(([label, value]) => `| **${label}** | ${value} |`)].join('\n');

  if (versionWarning) {
    content += `\n\n⚠️ **Version Warning**: ${versionWarning}`;
  }

  const schemaWarning = getSchemaVersionWarning(config, binaryVersionLabel);
  if (schemaWarning) {
    content += `\n\n⚠️ **Schema Warning**: ${schemaWarning}`;
  }

  if (invalidConfigFields.length > 0) {
    content += `\n\n⚠️ **Invalid Config Fields**: ${invalidConfigFields.join(', ')}`;
  }

  const md = new vscode.MarkdownString(content);
  md.supportHtml = true;
  return md;
}
