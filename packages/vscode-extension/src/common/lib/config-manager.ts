import { isAbsolute } from 'node:path';
import * as jsonc from 'jsonc-parser';
import { CONFIG_DIR_NAME, CONFIG_FILE_NAME, type TscannerConfig } from 'tscanner-common';
import * as vscode from 'vscode';
import { StoreKey, extensionStore } from '../state/extension-store';
import { logger } from './logger';

export function getConfigDir(workspacePath: string, configDir: string | null): vscode.Uri {
  const baseDir = vscode.Uri.file(workspacePath);

  if (!configDir) {
    return vscode.Uri.joinPath(baseDir, CONFIG_DIR_NAME);
  }

  const customDir = isAbsolute(configDir) ? vscode.Uri.file(configDir) : vscode.Uri.joinPath(baseDir, configDir);
  return vscode.Uri.joinPath(customDir, CONFIG_DIR_NAME);
}

function getConfigPath(workspacePath: string, configDir: string | null): vscode.Uri {
  return vscode.Uri.joinPath(getConfigDir(workspacePath, configDir), CONFIG_FILE_NAME);
}

export async function hasConfig(workspacePath: string, configDir: string | null): Promise<boolean> {
  const configPath = getConfigPath(workspacePath, configDir);
  try {
    await vscode.workspace.fs.stat(configPath);
    return true;
  } catch {
    return false;
  }
}

export async function loadConfig(workspacePath: string, configDir: string | null): Promise<TscannerConfig | null> {
  const configPath = getConfigPath(workspacePath, configDir);

  try {
    const data = await vscode.workspace.fs.readFile(configPath);
    const content = Buffer.from(data).toString('utf8');
    const errors: jsonc.ParseError[] = [];
    const config = jsonc.parse(content, errors);

    if (errors.length > 0) {
      logger.error(`JSONC parse errors in ${configPath.fsPath}: ${JSON.stringify(errors)}`);
      return null;
    }

    logger.debug(`loadConfig: path=${configPath.fsPath}, loaded=true`);
    return config as TscannerConfig;
  } catch (error) {
    logger.debug(`loadConfig: path=${configPath.fsPath}, loaded=false, error=${error}`);
    return null;
  }
}

async function copyDirectoryRecursive(source: vscode.Uri, target: vscode.Uri): Promise<void> {
  await vscode.workspace.fs.createDirectory(target);

  const entries = await vscode.workspace.fs.readDirectory(source);
  for (const [name, type] of entries) {
    const sourceEntry = vscode.Uri.joinPath(source, name);
    const targetEntry = vscode.Uri.joinPath(target, name);

    if (type === vscode.FileType.Directory) {
      await copyDirectoryRecursive(sourceEntry, targetEntry);
    } else {
      await vscode.workspace.fs.copy(sourceEntry, targetEntry, { overwrite: true });
    }
  }
}

export async function moveConfig(
  workspacePath: string,
  fromConfigDir: string | null,
  toConfigDir: string | null,
): Promise<void> {
  const sourceDir = getConfigDir(workspacePath, fromConfigDir);
  const targetDir = getConfigDir(workspacePath, toConfigDir);

  await copyDirectoryRecursive(sourceDir, targetDir);
  await vscode.workspace.fs.delete(sourceDir, { recursive: true });

  logger.info(`Moved config from ${sourceDir.fsPath} to ${targetDir.fsPath}`);
}

export function getConfigDirLabel(configDir: string | null): string {
  return configDir ? `${configDir}/${CONFIG_DIR_NAME}` : CONFIG_DIR_NAME;
}

export async function loadAndCacheConfig(workspacePath: string): Promise<TscannerConfig | null> {
  const configDir = extensionStore.get(StoreKey.ConfigDir);
  const config = await loadConfig(workspacePath, configDir);
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
