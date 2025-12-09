import * as vscode from 'vscode';
import { IS_DEV } from '../constants';
import { logger } from '../lib/logger';
import { buildConfigSection, buildFullConfigKey } from '../scripts-constants';

export enum ExtensionConfigKey {
  LspBin = 'lsp.bin',
  LogsEnabled = 'logs.enabled',
}

export function getFullConfigKeyPath(key: ExtensionConfigKey): string {
  return buildFullConfigKey(IS_DEV, key);
}

type ExtensionConfigSchema = {
  [ExtensionConfigKey.LspBin]: string;
  [ExtensionConfigKey.LogsEnabled]: boolean;
};

const defaultValues: ExtensionConfigSchema = {
  [ExtensionConfigKey.LspBin]: '',
  [ExtensionConfigKey.LogsEnabled]: false,
};

function getConfigSection(): string {
  return buildConfigSection(IS_DEV);
}

export function getExtensionConfig<K extends ExtensionConfigKey>(key: K): ExtensionConfigSchema[K] {
  const config = vscode.workspace.getConfiguration(getConfigSection());
  const value = config.get<ExtensionConfigSchema[K]>(key) ?? defaultValues[key];
  logger.debug(`[extension-config] GET ${key} = ${JSON.stringify(value)}`);
  return value;
}
