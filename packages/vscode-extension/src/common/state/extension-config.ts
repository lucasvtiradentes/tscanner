import { type StartupScanMode, VSCODE_SETTINGS_DEFAULTS } from 'tscanner-common';
import * as vscode from 'vscode';
import { IS_DEV } from '../constants';
import { createLogger } from '../lib/logger';
import { buildConfigSection, buildFullConfigKey } from '../scripts-constants';

const configLogger = createLogger('extension-config');

export enum ExtensionConfigKey {
  LspBin = 'lsp.bin',
  LogsEnabled = 'logs.enabled',
  AutoScanInterval = 'scan.autoInterval',
  AutoAiScanInterval = 'aiScan.autoInterval',
  StartupScan = 'scan.startup',
  StartupAiScan = 'aiScan.startup',
}

export function getFullConfigKeyPath(key: ExtensionConfigKey): string {
  return buildFullConfigKey(IS_DEV, key);
}

type ExtensionConfigSchema = {
  [ExtensionConfigKey.LspBin]: string;
  [ExtensionConfigKey.LogsEnabled]: boolean;
  [ExtensionConfigKey.AutoScanInterval]: number;
  [ExtensionConfigKey.AutoAiScanInterval]: number;
  [ExtensionConfigKey.StartupScan]: StartupScanMode;
  [ExtensionConfigKey.StartupAiScan]: StartupScanMode;
};

const defaultValues: ExtensionConfigSchema = {
  [ExtensionConfigKey.LspBin]: '',
  [ExtensionConfigKey.LogsEnabled]: false,
  [ExtensionConfigKey.AutoScanInterval]: VSCODE_SETTINGS_DEFAULTS.autoScanInterval,
  [ExtensionConfigKey.AutoAiScanInterval]: VSCODE_SETTINGS_DEFAULTS.autoAiScanInterval,
  [ExtensionConfigKey.StartupScan]: VSCODE_SETTINGS_DEFAULTS.startupScan as StartupScanMode,
  [ExtensionConfigKey.StartupAiScan]: VSCODE_SETTINGS_DEFAULTS.startupAiScan as StartupScanMode,
};

function getConfigSection(): string {
  return buildConfigSection(IS_DEV);
}

export function getExtensionConfig<K extends ExtensionConfigKey>(key: K): ExtensionConfigSchema[K] {
  const config = vscode.workspace.getConfiguration(getConfigSection());
  const value = config.get<ExtensionConfigSchema[K]>(key) ?? defaultValues[key];
  configLogger.debug(`GET ${key} = ${JSON.stringify(value)}`);
  return value;
}
