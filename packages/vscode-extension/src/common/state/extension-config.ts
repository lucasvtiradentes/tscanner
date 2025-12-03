import * as vscode from 'vscode';

const CONFIG_SECTION = 'tscanner';

export enum ExtensionConfigKey {
  LspBin = 'lsp.bin',
  TraceServer = 'trace.server',
}

export enum TraceLevel {
  Off = 'off',
  Messages = 'messages',
  Verbose = 'verbose',
}

type ExtensionConfigSchema = {
  [ExtensionConfigKey.LspBin]: string;
  [ExtensionConfigKey.TraceServer]: TraceLevel;
};

const defaultValues: ExtensionConfigSchema = {
  [ExtensionConfigKey.LspBin]: '',
  [ExtensionConfigKey.TraceServer]: TraceLevel.Off,
};

export function getExtensionConfig<K extends ExtensionConfigKey>(key: K): ExtensionConfigSchema[K] {
  const config = vscode.workspace.getConfiguration(CONFIG_SECTION);
  return config.get<ExtensionConfigSchema[K]>(key) ?? defaultValues[key];
}
