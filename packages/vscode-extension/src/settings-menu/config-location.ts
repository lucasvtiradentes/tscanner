import * as path from 'node:path';
import { CONFIG_DIR_NAME, PACKAGE_DISPLAY_NAME } from 'tscanner-common';
import * as vscode from 'vscode';
import {
  type ConfigState,
  deleteCustomConfig,
  deleteGlobalConfig,
  deleteLocalConfig,
  getConfigState,
  getCustomConfigPath,
  getGlobalConfigPath,
  getLocalConfigPath,
  hasCustomConfig,
  hasGlobalConfig,
  hasLocalConfig,
  loadEffectiveConfig,
  saveCustomConfig,
  saveGlobalConfig,
  saveLocalConfig,
} from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import {
  Command,
  type QuickPickItemWithId,
  ToastKind,
  executeCommand,
  getCurrentWorkspaceFolder,
  openTextDocument,
  showToastMessage,
} from '../common/lib/vscode-utils';
import { WorkspaceStateKey, updateState } from '../common/state/workspace-state';
import type { RegularIssuesView } from '../issues-panel';

export enum ConfigLocation {
  ExtensionStorage = 'extension-storage',
  ProjectFolder = 'project-folder',
  CustomPath = 'custom-path',
}

export function getCurrentLocationLabel(
  hasCustom: boolean,
  hasLocal: boolean,
  hasGlobal: boolean,
  customConfigDir: string | null,
): string {
  if (hasCustom && customConfigDir) {
    return `Current: ${customConfigDir}/${CONFIG_DIR_NAME}`;
  }
  if (hasLocal) {
    return `Current: ${CONFIG_DIR_NAME} (Project Folder)`;
  }
  if (hasGlobal) {
    return 'Current: Extension Storage';
  }
  return 'No config set';
}

function getCurrentConfigLocation(hasCustom: boolean, hasLocal: boolean, hasGlobal: boolean): ConfigLocation | null {
  if (hasCustom) return ConfigLocation.CustomPath;
  if (hasLocal) return ConfigLocation.ProjectFolder;
  if (hasGlobal) return ConfigLocation.ExtensionStorage;
  return null;
}

export async function openConfigFile(context: vscode.ExtensionContext, customConfigDir: string | null) {
  const workspaceFolder = getCurrentWorkspaceFolder();

  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    return;
  }

  const workspacePath = workspaceFolder.uri.fsPath;

  if (customConfigDir) {
    const hasCustom = await hasCustomConfig(workspacePath, customConfigDir);
    if (hasCustom) {
      const customConfigPath = getCustomConfigPath(workspacePath, customConfigDir);
      const doc = await openTextDocument(customConfigPath);
      await vscode.window.showTextDocument(doc);
      return;
    }
    logger.info('Custom config not found, falling back to local/global');
  }

  const hasLocal = await hasLocalConfig(workspacePath);
  if (hasLocal) {
    const localConfigPath = getLocalConfigPath(workspacePath);
    const doc = await openTextDocument(localConfigPath);
    await vscode.window.showTextDocument(doc);
    return;
  }

  logger.info('Local config not found, trying global config');

  const hasGlobal = await hasGlobalConfig(context, workspacePath);
  if (hasGlobal) {
    const globalConfigPath = getGlobalConfigPath(context, workspacePath);
    const doc = await openTextDocument(globalConfigPath);
    await vscode.window.showTextDocument(doc);
    return;
  }

  showToastMessage(
    ToastKind.Error,
    `No ${PACKAGE_DISPLAY_NAME} configuration found. Run "tscanner init" to create one.`,
  );
}

export async function showConfigLocationMenu(
  updateStatusBar: () => Promise<void>,
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
  regularView: RegularIssuesView,
) {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    return;
  }

  const workspacePath = workspaceFolder.uri.fsPath;
  const customConfigDir = currentCustomConfigDirRef.current;
  const configState = await getConfigState(context, workspacePath, customConfigDir);
  const currentLocation = getCurrentConfigLocation(configState.hasCustom, configState.hasLocal, configState.hasGlobal);

  const menuItems: QuickPickItemWithId<ConfigLocation>[] = [
    {
      id: ConfigLocation.ExtensionStorage,
      label: '$(cloud) Extension Storage',
      description: currentLocation === ConfigLocation.ExtensionStorage ? '✓ Active' : '',
      detail: 'Lost on extension uninstall',
    },
    {
      id: ConfigLocation.ProjectFolder,
      label: '$(file) Project Folder',
      description: currentLocation === ConfigLocation.ProjectFolder ? '✓ Active' : '',
      detail: `${CONFIG_DIR_NAME} (can commit to git)`,
    },
    {
      id: ConfigLocation.CustomPath,
      label: '$(folder-opened) Custom Path',
      description: currentLocation === ConfigLocation.CustomPath ? `✓ Active (${customConfigDir})` : '',
      detail: 'Select a custom folder within workspace',
    },
  ];

  const selected = await vscode.window.showQuickPick(menuItems, {
    placeHolder: 'Select config location',
    ignoreFocusOut: false,
  });

  if (!selected) return;

  const targetLocation = selected.id as ConfigLocation;

  if (targetLocation === currentLocation) {
    if (targetLocation === ConfigLocation.CustomPath) {
      await handleCustomPathSelection(
        workspacePath,
        currentCustomConfigDirRef,
        context,
        regularView,
        updateStatusBar,
        currentLocation,
        configState,
      );
    }
    return;
  }

  if (targetLocation === ConfigLocation.CustomPath) {
    await handleCustomPathSelection(
      workspacePath,
      currentCustomConfigDirRef,
      context,
      regularView,
      updateStatusBar,
      currentLocation,
      configState,
    );
    return;
  }

  if (currentLocation && configState.hasAny) {
    await moveConfigToLocation(
      workspacePath,
      currentCustomConfigDirRef,
      context,
      regularView,
      updateStatusBar,
      currentLocation,
      targetLocation,
      null,
    );
  } else {
    if (targetLocation === ConfigLocation.ProjectFolder) {
      currentCustomConfigDirRef.current = null;
      updateState(context, WorkspaceStateKey.CustomConfigDir, null);
    }
    await updateStatusBar();
    showToastMessage(ToastKind.Info, 'Config location set. Run "tscanner init" to create config.');
  }
}

async function handleCustomPathSelection(
  workspacePath: string,
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
  regularView: RegularIssuesView,
  updateStatusBar: () => Promise<void>,
  currentLocation: ConfigLocation | null,
  configState: ConfigState,
) {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) return;

  const startPath = currentCustomConfigDirRef.current || '.';
  const result = await showFolderPickerQuickPick(workspaceFolder.uri, startPath);

  if (result === 'cancelled' || result === null) return;

  if (currentLocation && configState.hasAny) {
    await moveConfigToLocation(
      workspacePath,
      currentCustomConfigDirRef,
      context,
      regularView,
      updateStatusBar,
      currentLocation,
      ConfigLocation.CustomPath,
      result,
    );
  } else {
    currentCustomConfigDirRef.current = result;
    updateState(context, WorkspaceStateKey.CustomConfigDir, result);
    await updateStatusBar();
    logger.info(`Custom config folder set to: ${result}`);
    showToastMessage(ToastKind.Info, `Config location set to: ${result}. Run "tscanner init" to create config.`);
  }
}

async function moveConfigToLocation(
  workspacePath: string,
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
  regularView: RegularIssuesView,
  updateStatusBar: () => Promise<void>,
  fromLocation: ConfigLocation,
  toLocation: ConfigLocation,
  customPath: string | null,
) {
  const fromLabel = getLocationLabel(fromLocation, currentCustomConfigDirRef.current);
  const toLabel = getLocationLabel(toLocation, customPath);

  const confirm = await vscode.window.showWarningMessage(
    `Move config from "${fromLabel}" to "${toLabel}/${CONFIG_DIR_NAME}"?`,
    { modal: true },
    'Move',
  );

  if (confirm !== 'Move') return;

  const config = await loadEffectiveConfig(context, workspacePath, currentCustomConfigDirRef.current);
  if (!config) {
    showToastMessage(ToastKind.Error, 'Failed to load current config');
    return;
  }

  switch (fromLocation) {
    case ConfigLocation.ExtensionStorage:
      await deleteGlobalConfig(context, workspacePath);
      break;
    case ConfigLocation.ProjectFolder:
      await deleteLocalConfig(workspacePath);
      break;
    case ConfigLocation.CustomPath:
      if (currentCustomConfigDirRef.current) {
        await deleteCustomConfig(workspacePath, currentCustomConfigDirRef.current);
      }
      break;
  }

  switch (toLocation) {
    case ConfigLocation.ExtensionStorage:
      await saveGlobalConfig(context, workspacePath, config);
      currentCustomConfigDirRef.current = null;
      updateState(context, WorkspaceStateKey.CustomConfigDir, null);
      break;
    case ConfigLocation.ProjectFolder:
      await saveLocalConfig(workspacePath, config);
      currentCustomConfigDirRef.current = null;
      updateState(context, WorkspaceStateKey.CustomConfigDir, null);
      break;
    case ConfigLocation.CustomPath:
      if (customPath) {
        await saveCustomConfig(workspacePath, customPath, config);
        currentCustomConfigDirRef.current = customPath;
        updateState(context, WorkspaceStateKey.CustomConfigDir, customPath);
      }
      break;
  }

  regularView.setResults([]);
  await updateStatusBar();

  logger.info(`Moved config from ${fromLabel} to ${toLabel}`);
  showToastMessage(ToastKind.Info, `Config moved to ${toLabel}`);
  executeCommand(Command.FindIssue);
}

function getLocationLabel(location: ConfigLocation, customPath: string | null): string {
  switch (location) {
    case ConfigLocation.ExtensionStorage:
      return 'Extension Storage';
    case ConfigLocation.ProjectFolder:
      return CONFIG_DIR_NAME;
    case ConfigLocation.CustomPath:
      return customPath || 'Custom Path';
  }
}

async function getSubfolders(dirUri: vscode.Uri): Promise<string[]> {
  try {
    const entries = await vscode.workspace.fs.readDirectory(dirUri);
    return entries.filter(([_, type]) => type === vscode.FileType.Directory).map(([name]) => name);
  } catch {
    return [];
  }
}

async function showFolderPickerQuickPick(
  workspaceRoot: vscode.Uri,
  currentRelativePath: string,
): Promise<string | null | 'cancelled'> {
  const currentUri =
    currentRelativePath === '.' ? workspaceRoot : vscode.Uri.joinPath(workspaceRoot, currentRelativePath);

  const subfolders = await getSubfolders(currentUri);
  const displayPath = currentRelativePath;

  const items: QuickPickItemWithId<string>[] = [];

  items.push({
    id: '__select__',
    label: '$(check) Select this Folder',
    detail: `Use: ${path.posix.join(displayPath, CONFIG_DIR_NAME)}`,
  });

  if (currentRelativePath !== '.') {
    items.push({
      id: '__parent__',
      label: '$(arrow-up) ..',
      detail: 'Go to parent folder',
    });
  }

  for (const folder of subfolders.sort()) {
    items.push({
      id: folder,
      label: `$(folder) ${folder}`,
      detail: currentRelativePath === '.' ? folder : path.posix.join(currentRelativePath, folder),
    });
  }

  const selected = await vscode.window.showQuickPick(items, {
    placeHolder: `Current: ${displayPath}`,
    ignoreFocusOut: true,
  });

  if (!selected) return 'cancelled';

  if (selected.id === '__select__') {
    return currentRelativePath;
  }

  if (selected.id === '__parent__') {
    const parent = path.posix.dirname(currentRelativePath);
    const parentPath = parent === '.' || parent === '' ? '.' : parent;
    return showFolderPickerQuickPick(workspaceRoot, parentPath);
  }

  const nextPath = currentRelativePath === '.' ? selected.id : path.posix.join(currentRelativePath, selected.id);
  return showFolderPickerQuickPick(workspaceRoot, nextPath);
}

async function showConfigLocationMenuLoop(
  workspaceFolder: vscode.WorkspaceFolder,
  workspacePath: string,
  menuItems: QuickPickItemWithId<ConfigLocation>[],
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
): Promise<{ location: ConfigLocation; customPath: string | null } | null> {
  const selected = await vscode.window.showQuickPick(menuItems, {
    placeHolder: 'Where do you want to save the rules configuration?',
    ignoreFocusOut: true,
  });

  if (!selected) return null;

  const targetLocation = selected.id as ConfigLocation;

  if (targetLocation === ConfigLocation.CustomPath) {
    const result = await showFolderPickerQuickPick(workspaceFolder.uri, '.');
    if (result === 'cancelled' || result === null) {
      return showConfigLocationMenuLoop(workspaceFolder, workspacePath, menuItems, currentCustomConfigDirRef, context);
    }

    if (result === '.') {
      const existingLocal = await hasLocalConfig(workspacePath);
      if (existingLocal) {
        const confirm = await vscode.window.showWarningMessage(
          `A config already exists at "${CONFIG_DIR_NAME}". This will overwrite it.`,
          { modal: true },
          'Overwrite',
        );
        if (confirm !== 'Overwrite') {
          return showConfigLocationMenuLoop(
            workspaceFolder,
            workspacePath,
            menuItems,
            currentCustomConfigDirRef,
            context,
          );
        }
      }

      currentCustomConfigDirRef.current = null;
      updateState(context, WorkspaceStateKey.CustomConfigDir, null);

      return { location: ConfigLocation.ProjectFolder, customPath: null };
    }

    const existingConfig = await hasCustomConfig(workspacePath, result);
    if (existingConfig) {
      const confirm = await vscode.window.showWarningMessage(
        `A config already exists at "${result}/${CONFIG_DIR_NAME}". This will overwrite it.`,
        { modal: true },
        'Overwrite',
      );
      if (confirm !== 'Overwrite') {
        return showConfigLocationMenuLoop(
          workspaceFolder,
          workspacePath,
          menuItems,
          currentCustomConfigDirRef,
          context,
        );
      }
    }

    currentCustomConfigDirRef.current = result;
    updateState(context, WorkspaceStateKey.CustomConfigDir, result);

    return { location: targetLocation, customPath: result };
  }

  if (targetLocation === ConfigLocation.ProjectFolder) {
    const existingLocal = await hasLocalConfig(workspacePath);
    if (existingLocal) {
      const confirm = await vscode.window.showWarningMessage(
        `A config already exists at "${CONFIG_DIR_NAME}". This will overwrite it.`,
        { modal: true },
        'Overwrite',
      );
      if (confirm !== 'Overwrite') {
        return showConfigLocationMenuLoop(
          workspaceFolder,
          workspacePath,
          menuItems,
          currentCustomConfigDirRef,
          context,
        );
      }
    }
  }

  if (targetLocation === ConfigLocation.ExtensionStorage) {
    const existingGlobal = await hasGlobalConfig(context, workspacePath);
    if (existingGlobal) {
      const confirm = await vscode.window.showWarningMessage(
        'A config already exists in Extension Storage. This will overwrite it.',
        { modal: true },
        'Overwrite',
      );
      if (confirm !== 'Overwrite') {
        return showConfigLocationMenuLoop(
          workspaceFolder,
          workspacePath,
          menuItems,
          currentCustomConfigDirRef,
          context,
        );
      }
    }
  }

  currentCustomConfigDirRef.current = null;
  updateState(context, WorkspaceStateKey.CustomConfigDir, null);

  return { location: targetLocation, customPath: null };
}

export async function showConfigLocationMenuForFirstSetup(
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
): Promise<{ location: ConfigLocation; customPath: string | null } | null> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    return null;
  }

  const workspacePath = workspaceFolder.uri.fsPath;

  const menuItems: QuickPickItemWithId<ConfigLocation>[] = [
    {
      id: ConfigLocation.ExtensionStorage,
      label: '$(cloud) Extension Storage',
      detail: 'Lost on extension uninstall',
    },
    {
      id: ConfigLocation.ProjectFolder,
      label: '$(file) Project Folder',
      detail: `${CONFIG_DIR_NAME} (can commit to git)`,
    },
    {
      id: ConfigLocation.CustomPath,
      label: '$(folder-opened) Custom Path',
      detail: 'Select a custom folder within workspace',
    },
  ];

  return showConfigLocationMenuLoop(workspaceFolder, workspacePath, menuItems, currentCustomConfigDirRef, context);
}
