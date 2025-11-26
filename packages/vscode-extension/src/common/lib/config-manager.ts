import { createHash } from 'node:crypto';
import { isAbsolute } from 'node:path';
import * as jsonc from 'jsonc-parser';
import * as vscode from 'vscode';
import defaultConfig from '../../../../../assets/default-config.json';
import { CONFIG_DIR_NAME, CONFIG_FILE_NAME } from '../constants';
import { TscannerConfig } from '../types';
import { logger } from '../utils/logger';

export { TscannerConfig };

function getWorkspaceHash(workspacePath: string): string {
  return createHash('md5').update(workspacePath).digest('hex').substring(0, 16);
}

export function getGlobalConfigDir(context: vscode.ExtensionContext): vscode.Uri {
  return vscode.Uri.joinPath(context.globalStorageUri, 'configs');
}

export function getGlobalConfigPath(context: vscode.ExtensionContext, workspacePath: string): vscode.Uri {
  const workspaceHash = getWorkspaceHash(workspacePath);
  return vscode.Uri.joinPath(getGlobalConfigDir(context), workspaceHash, CONFIG_FILE_NAME);
}

export function getLocalConfigPath(workspacePath: string): vscode.Uri {
  return vscode.Uri.joinPath(vscode.Uri.file(workspacePath), CONFIG_DIR_NAME, CONFIG_FILE_NAME);
}

export function getCustomConfigPath(workspacePath: string, customConfigDir: string): vscode.Uri {
  const customDir = isAbsolute(customConfigDir)
    ? vscode.Uri.file(customConfigDir)
    : vscode.Uri.joinPath(vscode.Uri.file(workspacePath), customConfigDir);
  return vscode.Uri.joinPath(customDir, CONFIG_DIR_NAME, CONFIG_FILE_NAME);
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

export async function loadConfig(configPath: vscode.Uri): Promise<TscannerConfig | null> {
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

export async function getEffectiveConfigPath(
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
    logger.info(`Using local config for workspace: ${workspacePath}`);
    return getLocalConfigPath(workspacePath);
  }

  logger.info(`Using global config for workspace: ${workspacePath}`);
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

const AUTO_MANAGED_MARKER = '// AUTO-MANAGED BY TSCANNER EXTENSION - DO NOT EDIT THIS LINE';

export function isAutoManagedConfig(configContent: string): boolean {
  return configContent.includes(AUTO_MANAGED_MARKER);
}

export function addAutoManagedMarker(config: TscannerConfig): string {
  const lines = JSON.stringify(config, null, 2).split('\n');
  lines.splice(1, 0, `  "${AUTO_MANAGED_MARKER}": true,`);
  return lines.join('\n');
}

export async function syncGlobalToLocal(context: vscode.ExtensionContext, workspacePath: string): Promise<void> {
  const localPath = getLocalConfigPath(workspacePath);

  try {
    const existingContent = await vscode.workspace.fs.readFile(localPath);
    const contentStr = Buffer.from(existingContent).toString('utf8');

    if (!isAutoManagedConfig(contentStr)) {
      logger.info(`Local config exists and is user-managed, skipping sync for ${workspacePath}`);
      return;
    }
  } catch {}

  const globalConfig = await loadConfig(getGlobalConfigPath(context, workspacePath));
  if (!globalConfig) {
    logger.debug(`No global config found for ${workspacePath}, skipping sync`);
    return;
  }

  const localConfigDir = vscode.Uri.joinPath(vscode.Uri.file(workspacePath), CONFIG_DIR_NAME);
  await vscode.workspace.fs.createDirectory(localConfigDir);

  const configWithMarker = addAutoManagedMarker(globalConfig);
  await vscode.workspace.fs.writeFile(localPath, Buffer.from(configWithMarker));

  logger.info(`Synced global config to local config for ${workspacePath}`);
}

export async function shouldSyncToLocal(workspacePath: string): Promise<boolean> {
  const localPath = getLocalConfigPath(workspacePath);

  try {
    const existingContent = await vscode.workspace.fs.readFile(localPath);
    const contentStr = Buffer.from(existingContent).toString('utf8');
    return isAutoManagedConfig(contentStr);
  } catch {
    return true;
  }
}

export async function ensureLocalConfigForScan(
  context: vscode.ExtensionContext,
  workspacePath: string,
): Promise<boolean> {
  const hasLocal = await hasLocalConfig(workspacePath);
  if (hasLocal) {
    logger.debug('Local config already exists, using it for scan');
    return true;
  }

  const globalConfig = await loadConfig(getGlobalConfigPath(context, workspacePath));
  if (!globalConfig) {
    logger.info('No config found (neither local nor global)');
    return false;
  }

  await syncGlobalToLocal(context, workspacePath);
  logger.info('Synced global config to local config for Rust scanner');
  return true;
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
  const customPath = getCustomConfigPath(workspacePath, customConfigDir);
  const configDir = vscode.Uri.joinPath(customPath, '..');
  logger.info(`Attempting to delete custom config dir at ${configDir.fsPath}`);
  try {
    await vscode.workspace.fs.delete(configDir, { recursive: true });
    logger.info(`Deleted custom config dir at ${configDir.fsPath}`);
  } catch (err) {
    logger.debug(`No custom config to delete: ${err}`);
  }
}
