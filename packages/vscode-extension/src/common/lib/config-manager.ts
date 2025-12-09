import { createHash } from 'node:crypto';
import { isAbsolute } from 'node:path';
import * as jsonc from 'jsonc-parser';
import { CONFIG_DIR_NAME, CONFIG_FILE_NAME, type TscannerConfig } from 'tscanner-common';
import * as vscode from 'vscode';
import defaultConfig from '../../../../../assets/configs/default.json';
import { logger } from './logger';

function getWorkspaceHash(workspacePath: string): string {
  return createHash('md5').update(workspacePath).digest('hex').substring(0, 16);
}

function getGlobalConfigDir(context: vscode.ExtensionContext): vscode.Uri {
  return vscode.Uri.joinPath(context.globalStorageUri, 'configs');
}

export function getGlobalConfigPath(context: vscode.ExtensionContext, workspacePath: string): vscode.Uri {
  const workspaceHash = getWorkspaceHash(workspacePath);
  return vscode.Uri.joinPath(getGlobalConfigDir(context), workspaceHash, CONFIG_FILE_NAME);
}

export function getLocalConfigDir(workspacePath: string): vscode.Uri {
  return vscode.Uri.joinPath(vscode.Uri.file(workspacePath), CONFIG_DIR_NAME);
}

export function getLocalConfigPath(workspacePath: string): vscode.Uri {
  return vscode.Uri.joinPath(getLocalConfigDir(workspacePath), CONFIG_FILE_NAME);
}

export function getCustomConfigDir(workspacePath: string, customConfigDir: string): vscode.Uri {
  const customDir = isAbsolute(customConfigDir)
    ? vscode.Uri.file(customConfigDir)
    : vscode.Uri.joinPath(vscode.Uri.file(workspacePath), customConfigDir);
  return vscode.Uri.joinPath(customDir, CONFIG_DIR_NAME);
}

export function getCustomConfigPath(workspacePath: string, customConfigDir: string): vscode.Uri {
  return vscode.Uri.joinPath(getCustomConfigDir(workspacePath, customConfigDir), CONFIG_FILE_NAME);
}

export async function hasLocalConfig(workspacePath: string): Promise<boolean> {
  const localPath = getLocalConfigPath(workspacePath);
  try {
    await vscode.workspace.fs.stat(localPath);
    return true;
  } catch {
    return false;
  }
}

async function loadConfig(configPath: vscode.Uri): Promise<TscannerConfig | null> {
  try {
    const data = await vscode.workspace.fs.readFile(configPath);
    const content = Buffer.from(data).toString('utf8');
    const errors: jsonc.ParseError[] = [];
    const config = jsonc.parse(content, errors);

    if (errors.length > 0) {
      logger.error(`JSONC parse errors in ${configPath.fsPath}: ${JSON.stringify(errors)}`);
      return null;
    }

    return config as TscannerConfig;
  } catch (error) {
    logger.debug(`Failed to load config from ${configPath.fsPath}: ${error}`);
    return null;
  }
}

export async function hasCustomConfig(workspacePath: string, customConfigDir: string): Promise<boolean> {
  const customPath = getCustomConfigPath(workspacePath, customConfigDir);
  try {
    await vscode.workspace.fs.stat(customPath);
    return true;
  } catch {
    return false;
  }
}

async function getEffectiveConfigPath(
  context: vscode.ExtensionContext,
  workspacePath: string,
  customConfigDir?: string | null,
): Promise<vscode.Uri> {
  if (customConfigDir) {
    const hasCustom = await hasCustomConfig(workspacePath, customConfigDir);
    if (hasCustom) {
      logger.info(`Using custom config for workspace: ${workspacePath} from ${customConfigDir}`);
      return getCustomConfigPath(workspacePath, customConfigDir);
    }
    logger.info(`Custom config dir set but no config found: ${customConfigDir}`);
  }

  const hasLocal = await hasLocalConfig(workspacePath);
  if (hasLocal) {
    return getLocalConfigPath(workspacePath);
  }

  return getGlobalConfigPath(context, workspacePath);
}

export async function loadEffectiveConfig(
  context: vscode.ExtensionContext,
  workspacePath: string,
  customConfigDir?: string | null,
): Promise<TscannerConfig | null> {
  const configPath = await getEffectiveConfigPath(context, workspacePath, customConfigDir);
  return loadConfig(configPath);
}

export async function saveGlobalConfig(
  context: vscode.ExtensionContext,
  workspacePath: string,
  config: TscannerConfig,
): Promise<void> {
  const configPath = getGlobalConfigPath(context, workspacePath);
  const configDir = vscode.Uri.joinPath(configPath, '..');

  await vscode.workspace.fs.createDirectory(configDir);
  await vscode.workspace.fs.writeFile(configPath, Buffer.from(JSON.stringify(config, null, 2)));

  logger.info(`Saved global config for workspace: ${workspacePath} at ${configPath.fsPath}`);
}

export async function saveLocalConfig(workspacePath: string, config: TscannerConfig): Promise<void> {
  const localConfigDir = vscode.Uri.joinPath(vscode.Uri.file(workspacePath), CONFIG_DIR_NAME);
  const localConfigPath = getLocalConfigPath(workspacePath);

  await vscode.workspace.fs.createDirectory(localConfigDir);
  await vscode.workspace.fs.writeFile(localConfigPath, Buffer.from(JSON.stringify(config, null, 2)));

  logger.info(`Saved local config for workspace: ${workspacePath}`);
}

export async function saveCustomConfig(
  workspacePath: string,
  customConfigDir: string,
  config: TscannerConfig,
): Promise<void> {
  const customPath = getCustomConfigPath(workspacePath, customConfigDir);
  const customDir = vscode.Uri.joinPath(customPath, '..');

  await vscode.workspace.fs.createDirectory(customDir);
  await vscode.workspace.fs.writeFile(customPath, Buffer.from(JSON.stringify(config, null, 2)));

  logger.info(`Saved custom config at: ${customPath.fsPath}`);
}

export function getDefaultConfig(): TscannerConfig {
  return structuredClone(defaultConfig) as TscannerConfig;
}

export async function hasGlobalConfig(context: vscode.ExtensionContext, workspacePath: string): Promise<boolean> {
  const globalPath = getGlobalConfigPath(context, workspacePath);
  try {
    await vscode.workspace.fs.stat(globalPath);
    return true;
  } catch {
    return false;
  }
}

export async function deleteGlobalConfig(context: vscode.ExtensionContext, workspacePath: string): Promise<void> {
  const globalPath = getGlobalConfigPath(context, workspacePath);
  try {
    await vscode.workspace.fs.delete(globalPath);
    logger.info(`Deleted global config at ${globalPath.fsPath}`);
  } catch {
    logger.debug('No global config to delete');
  }
}

export async function deleteLocalConfig(workspacePath: string): Promise<void> {
  const localDir = vscode.Uri.joinPath(vscode.Uri.file(workspacePath), CONFIG_DIR_NAME);
  try {
    await vscode.workspace.fs.delete(localDir, { recursive: true });
    logger.info(`Deleted local config dir at ${localDir.fsPath}`);
  } catch {
    logger.debug('No local config to delete');
  }
}

export async function deleteCustomConfig(workspacePath: string, customConfigDir: string): Promise<void> {
  const configDir = getCustomConfigDir(workspacePath, customConfigDir);
  logger.info(`Attempting to delete custom config dir at ${configDir.fsPath}`);
  try {
    await vscode.workspace.fs.delete(configDir, { recursive: true });
    logger.info(`Deleted custom config dir at ${configDir.fsPath}`);
  } catch (err) {
    logger.debug(`No custom config to delete: ${err}`);
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

export async function moveLocalToCustom(workspacePath: string, customConfigDir: string): Promise<void> {
  const sourceDir = getLocalConfigDir(workspacePath);
  const targetDir = getCustomConfigDir(workspacePath, customConfigDir);

  await copyDirectoryRecursive(sourceDir, targetDir);
  await vscode.workspace.fs.delete(sourceDir, { recursive: true });

  logger.info(`Moved config from ${sourceDir.fsPath} to ${targetDir.fsPath}`);
}

export async function moveCustomToLocal(workspacePath: string, customConfigDir: string): Promise<void> {
  const sourceDir = getCustomConfigDir(workspacePath, customConfigDir);
  const targetDir = getLocalConfigDir(workspacePath);

  await copyDirectoryRecursive(sourceDir, targetDir);
  await vscode.workspace.fs.delete(sourceDir, { recursive: true });

  logger.info(`Moved config from ${sourceDir.fsPath} to ${targetDir.fsPath}`);
}

export async function moveCustomToCustom(
  workspacePath: string,
  fromCustomDir: string,
  toCustomDir: string,
): Promise<void> {
  const sourceDir = getCustomConfigDir(workspacePath, fromCustomDir);
  const targetDir = getCustomConfigDir(workspacePath, toCustomDir);

  await copyDirectoryRecursive(sourceDir, targetDir);
  await vscode.workspace.fs.delete(sourceDir, { recursive: true });

  logger.info(`Moved config from ${sourceDir.fsPath} to ${targetDir.fsPath}`);
}

export type ConfigState = {
  hasCustom: boolean;
  hasLocal: boolean;
  hasGlobal: boolean;
  hasAny: boolean;
};

export async function getConfigState(
  context: vscode.ExtensionContext,
  workspacePath: string,
  customConfigDir: string | null,
): Promise<ConfigState> {
  const hasCustom = customConfigDir ? await hasCustomConfig(workspacePath, customConfigDir) : false;
  const hasLocal = await hasLocalConfig(workspacePath);
  const hasGlobal = await hasGlobalConfig(context, workspacePath);
  return {
    hasCustom,
    hasLocal,
    hasGlobal,
    hasAny: hasCustom || hasLocal || hasGlobal,
  };
}
