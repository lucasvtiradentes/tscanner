import * as jsonc from 'jsonc-parser';
import { CONFIG_DIR_NAME, CONFIG_FILE_NAME, type TscannerConfig } from 'tscanner-common';
import * as vscode from 'vscode';
import { StoreKey, extensionStore } from '../state/extension-store';
import { logger } from './logger';

export function getConfigBaseDir(workspacePath: string): string {
  return workspacePath;
}

export function getConfigDirPath(workspacePath: string): string {
  return vscode.Uri.joinPath(vscode.Uri.file(workspacePath), CONFIG_DIR_NAME).fsPath;
}

export function getConfigPath(workspacePath: string): string {
  return vscode.Uri.joinPath(vscode.Uri.file(getConfigDirPath(workspacePath)), CONFIG_FILE_NAME).fsPath;
}

export async function hasConfig(workspacePath: string): Promise<boolean> {
  const configPath = vscode.Uri.file(getConfigPath(workspacePath));
  try {
    await vscode.workspace.fs.stat(configPath);
    return true;
  } catch {
    return false;
  }
}

async function loadConfig(workspacePath: string): Promise<TscannerConfig | null> {
  const configPath = getConfigPath(workspacePath);
  const configUri = vscode.Uri.file(configPath);

  try {
    const data = await vscode.workspace.fs.readFile(configUri);
    const content = Buffer.from(data).toString('utf8');
    const errors: jsonc.ParseError[] = [];
    const config = jsonc.parse(content, errors);

    if (errors.length > 0) {
      logger.error(`JSONC parse errors in ${configPath}: ${JSON.stringify(errors)}`);
      return null;
    }

    logger.debug(`loadConfig: path=${configPath}, loaded=true`);
    return config as TscannerConfig;
  } catch (error) {
    logger.debug(`loadConfig: path=${configPath}, loaded=false, error=${error}`);
    return null;
  }
}

export async function loadAndCacheConfig(workspacePath: string): Promise<TscannerConfig | null> {
  const config = await loadConfig(workspacePath);
  extensionStore.set(StoreKey.CachedConfig, config);
  return config;
}

export function getCachedConfig(): TscannerConfig | null {
  return extensionStore.get(StoreKey.CachedConfig);
}

export async function getOrLoadConfig(workspacePath: string): Promise<TscannerConfig | null> {
  const cached = extensionStore.get(StoreKey.CachedConfig);
  if (cached) return cached;
  return loadAndCacheConfig(workspacePath);
}
