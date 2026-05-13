import { dirname } from 'node:path';
import * as jsonc from 'jsonc-parser';
import { type AiProvider, LOCAL_CONFIG_FILE_NAME } from 'tscanner-common';
import * as vscode from 'vscode';
import { StoreKey, extensionStore } from '../state/extension-store';
import { getConfigDirPath } from './config-manager';

type LocalAiConfig = {
  provider?: AiProvider;
  model?: string;
};

type LocalConfig = {
  ai?: LocalAiConfig;
  [key: string]: unknown;
};

export function getLocalConfigPath(workspacePath: string): string {
  const configDir = extensionStore.get(StoreKey.ConfigDir);
  return vscode.Uri.joinPath(vscode.Uri.file(getConfigDirPath(workspacePath, configDir)), LOCAL_CONFIG_FILE_NAME)
    .fsPath;
}

export async function readLocalConfig(workspacePath: string): Promise<LocalConfig> {
  const path = getLocalConfigPath(workspacePath);
  try {
    const data = await vscode.workspace.fs.readFile(vscode.Uri.file(path));
    const errors: jsonc.ParseError[] = [];
    const parsed = jsonc.parse(Buffer.from(data).toString('utf8'), errors);
    if (errors.length > 0 || !parsed || typeof parsed !== 'object') return {};
    return parsed as LocalConfig;
  } catch {
    return {};
  }
}

export async function writeLocalConfig(workspacePath: string, config: LocalConfig): Promise<void> {
  const path = getLocalConfigPath(workspacePath);
  const uri = vscode.Uri.file(path);
  if (isLocalConfigEmpty(config)) {
    try {
      await vscode.workspace.fs.delete(uri);
    } catch (error) {
      void error;
    }
    return;
  }

  await vscode.workspace.fs.createDirectory(vscode.Uri.file(dirname(path)));
  await ensureLocalConfigIgnored(workspacePath);
  await vscode.workspace.fs.writeFile(uri, Buffer.from(`${JSON.stringify(config, null, 2)}\n`));
}

export async function setLocalAiConfig(workspacePath: string, provider: AiProvider, model?: string): Promise<void> {
  const config = await readLocalConfig(workspacePath);
  config.ai = model ? { provider, model } : { provider };
  await writeLocalConfig(workspacePath, config);
}

export async function unsetLocalAiConfig(workspacePath: string): Promise<void> {
  const config = await readLocalConfig(workspacePath);
  const { ai: _ai, ...rest } = config;
  await writeLocalConfig(workspacePath, rest);
}

function isLocalConfigEmpty(config: LocalConfig): boolean {
  return Object.keys(config).length === 0;
}

async function ensureLocalConfigIgnored(workspacePath: string): Promise<void> {
  const configDir = extensionStore.get(StoreKey.ConfigDir);
  const ignoreUri = vscode.Uri.joinPath(vscode.Uri.file(getConfigDirPath(workspacePath, configDir)), '.gitignore');
  const entry = LOCAL_CONFIG_FILE_NAME;

  try {
    const data = await vscode.workspace.fs.readFile(ignoreUri);
    const content = Buffer.from(data).toString('utf8');
    if (content.split(/\r?\n/).some((line) => line.trim() === entry)) return;

    const prefix = content.endsWith('\n') || content.length === 0 ? '' : '\n';
    await vscode.workspace.fs.writeFile(ignoreUri, Buffer.from(`${content}${prefix}${entry}\n`));
  } catch {
    await vscode.workspace.fs.writeFile(ignoreUri, Buffer.from(`${entry}\n`));
  }
}
