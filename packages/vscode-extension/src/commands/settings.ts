import * as vscode from 'vscode';
import {
  deleteCustomConfig,
  deleteGlobalConfig,
  deleteLocalConfig,
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
import {
  Command,
  ScanMode,
  ToastKind,
  WorkspaceStateKey,
  executeCommand,
  getCurrentWorkspaceFolder,
  openTextDocument,
  registerCommand,
  showToastMessage,
  updateState,
} from '../common/lib/vscode-utils';
import { getAllBranches, getCurrentBranch, invalidateCache } from '../common/utils/git-helper';
import { logger } from '../common/utils/logger';
import type { SearchResultProvider } from '../sidebar/search-provider';
import { setCopyScanContext } from './copy-issues';

enum SettingsMenuOption {
  ManageRules = 'manage-rules',
  ManageScanMode = 'manage-scan-mode',
  ManageConfigLocation = 'manage-config-location',
  OpenConfigFile = 'open-config-file',
}

enum ConfigLocation {
  ExtensionStorage = 'extension-storage',
  ProjectFolder = 'project-folder',
  CustomPath = 'custom-path',
}

enum BranchMenuOption {
  KeepCurrent = 'keep-current',
  ChooseAnother = 'choose-another',
}

type QuickPickItemWithId = {
  id: string;
} & vscode.QuickPickItem;

export function createOpenSettingsMenuCommand(
  updateStatusBar: () => Promise<void>,
  updateBadge: () => void,
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
  searchProvider: SearchResultProvider,
) {
  return registerCommand(Command.OpenSettingsMenu, async () => {
    logger.info('openSettingsMenu command called');

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      showToastMessage(ToastKind.Error, 'No workspace folder open');
      return;
    }

    const workspacePath = workspaceFolder.uri.fsPath;
    const customConfigDir = currentCustomConfigDirRef.current;

    const hasCustom = customConfigDir ? await hasCustomConfig(workspacePath, customConfigDir) : false;
    const hasLocal = await hasLocalConfig(workspacePath);
    const hasGlobal = await hasGlobalConfig(context, workspacePath);
    const hasAnyConfig = hasCustom || hasLocal || hasGlobal;

    const currentLocationLabel = getCurrentLocationLabel(hasCustom, hasLocal, hasGlobal, customConfigDir);

    const mainMenuItems: QuickPickItemWithId[] = [
      {
        id: SettingsMenuOption.ManageRules,
        label: '$(checklist) Manage Rules',
        detail: 'Enable/disable rules',
      },
    ];

    if (hasAnyConfig) {
      mainMenuItems.push({
        id: SettingsMenuOption.ManageScanMode,
        label: '$(gear) Manage Scan Mode',
        detail: 'Choose between Codebase or Branch scan mode',
      });
    }

    mainMenuItems.push({
      id: SettingsMenuOption.ManageConfigLocation,
      label: '$(folder) Manage Config Location',
      detail: currentLocationLabel,
    });

    if (hasAnyConfig) {
      mainMenuItems.push({
        id: SettingsMenuOption.OpenConfigFile,
        label: '$(edit) Open Config File',
        detail: 'Edit current config file',
      });
    }

    const selected = await vscode.window.showQuickPick(mainMenuItems, {
      placeHolder: 'TScanner Settings',
      ignoreFocusOut: false,
    });

    if (!selected) {
      return;
    }

    switch (selected.id) {
      case SettingsMenuOption.ManageRules:
        logger.info('User selected: Manage Rules');
        await executeCommand(Command.ManageRules);
        break;
      case SettingsMenuOption.ManageScanMode:
        logger.info('User selected: Manage Scan Mode');
        await showScanModeMenu(updateStatusBar, currentScanModeRef, currentCompareBranchRef, context, searchProvider);
        break;
      case SettingsMenuOption.ManageConfigLocation:
        logger.info('User selected: Manage Config Location');
        await showConfigLocationMenu(updateStatusBar, updateBadge, currentCustomConfigDirRef, context, searchProvider);
        break;
      case SettingsMenuOption.OpenConfigFile:
        logger.info('User selected: Open Config File');
        await openConfigFile(context, currentCustomConfigDirRef.current);
        break;
    }
  });
}

function getCurrentLocationLabel(
  hasCustom: boolean,
  hasLocal: boolean,
  hasGlobal: boolean,
  customConfigDir: string | null,
): string {
  if (hasCustom && customConfigDir) {
    return `Current: ${customConfigDir}`;
  }
  if (hasLocal) {
    return 'Current: .tscanner (Project Folder)';
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

async function openConfigFile(context: vscode.ExtensionContext, customConfigDir: string | null) {
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

  showToastMessage(ToastKind.Error, 'No TScanner configuration found. Create one via "Manage Rules" first.');
}

async function showScanModeMenu(
  updateStatusBar: () => Promise<void>,
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  context: vscode.ExtensionContext,
  searchProvider: SearchResultProvider,
) {
  logger.info('showScanModeMenu called');
  const scanModeItems: QuickPickItemWithId[] = [
    {
      id: ScanMode.Codebase,
      label: '$(file-directory) Codebase',
      description: currentScanModeRef.current === ScanMode.Codebase ? '✓ Active' : '',
      detail: 'Scan all files in workspace',
    },
    {
      id: ScanMode.Branch,
      label: '$(git-branch) Branch',
      description: currentScanModeRef.current === ScanMode.Branch ? '✓ Active' : '',
      detail: 'Scan only changed files in current branch',
    },
  ];

  const selected = await vscode.window.showQuickPick(scanModeItems, {
    placeHolder: 'Change checking mode',
    ignoreFocusOut: false,
  });

  if (!selected) {
    return;
  }

  if (selected.id === ScanMode.Codebase) {
    await handleCodebaseScan(updateStatusBar, currentScanModeRef, currentCompareBranchRef, context, searchProvider);
  }

  if (selected.id === ScanMode.Branch) {
    await handleBranchScan(updateStatusBar, currentScanModeRef, currentCompareBranchRef, context, searchProvider);
  }
}

async function handleCodebaseScan(
  updateStatusBar: () => Promise<void>,
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  context: vscode.ExtensionContext,
  searchProvider: SearchResultProvider,
) {
  logger.info('Switching to Codebase mode');
  searchProvider.setResults([]);
  currentScanModeRef.current = ScanMode.Codebase;
  updateState(context, WorkspaceStateKey.ScanMode, ScanMode.Codebase);
  setCopyScanContext(ScanMode.Codebase, currentCompareBranchRef.current);
  invalidateCache();
  await updateStatusBar();
  executeCommand(Command.FindIssue);
}

async function handleBranchScan(
  updateStatusBar: () => Promise<void>,
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  context: vscode.ExtensionContext,
  searchProvider: SearchResultProvider,
) {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    return;
  }

  const currentBranch = await getCurrentBranch(workspaceFolder.uri.fsPath);
  if (!currentBranch) {
    showToastMessage(ToastKind.Error, 'Not in a git repository');
    return;
  }

  const branchOptions: QuickPickItemWithId[] = [
    {
      id: BranchMenuOption.KeepCurrent,
      label: `Current value: ${currentCompareBranchRef.current}`,
      description: '✓',
      detail: 'Currently comparing against this branch',
    },
    {
      id: BranchMenuOption.ChooseAnother,
      label: '$(list-selection) Choose another branch',
      detail: 'Select a different branch to compare against',
    },
  ];

  const branchSelected = await vscode.window.showQuickPick(branchOptions, {
    placeHolder: 'Branch settings',
    ignoreFocusOut: false,
  });

  if (!branchSelected) return;

  if (branchSelected.id === BranchMenuOption.ChooseAnother) {
    const branches = await getAllBranches(workspaceFolder.uri.fsPath);

    if (branches.length === 0) {
      showToastMessage(ToastKind.Error, 'No branches found');
      return;
    }

    const otherBranches = branches.filter((b) => b !== currentBranch);

    const localBranches = otherBranches.filter((b) => !b.startsWith('origin/'));
    const remoteBranches = otherBranches.filter((b) => b.startsWith('origin/'));

    const branchItems: vscode.QuickPickItem[] = [];

    if (localBranches.length > 0) {
      branchItems.push(
        { label: 'Branches', kind: vscode.QuickPickItemKind.Separator },
        ...localBranches.map((branch) => ({
          label: `$(git-branch) ${branch}`,
          description: branch === currentCompareBranchRef.current ? '$(check) Current compare target' : '',
          detail: branch,
        })),
      );
    }

    if (remoteBranches.length > 0) {
      branchItems.push(
        { label: 'Remote branches', kind: vscode.QuickPickItemKind.Separator },
        ...remoteBranches.map((branch) => ({
          label: `$(cloud) ${branch}`,
          description: branch === currentCompareBranchRef.current ? '$(check) Current compare target' : '',
          detail: branch,
        })),
      );
    }

    const selectedBranch = await vscode.window.showQuickPick(branchItems, {
      placeHolder: `Select branch to compare against (current: ${currentBranch})`,
      matchOnDescription: true,
      matchOnDetail: true,
      ignoreFocusOut: true,
    });

    if (!selectedBranch || !selectedBranch.detail) return;

    currentCompareBranchRef.current = selectedBranch.detail;
    updateState(context, WorkspaceStateKey.CompareBranch, currentCompareBranchRef.current);
  }

  logger.info(`Switching to Branch mode (comparing against: ${currentCompareBranchRef.current})`);
  searchProvider.setResults([]);
  currentScanModeRef.current = ScanMode.Branch;
  updateState(context, WorkspaceStateKey.ScanMode, ScanMode.Branch);
  setCopyScanContext(ScanMode.Branch, currentCompareBranchRef.current);
  invalidateCache();
  await updateStatusBar();
  executeCommand(Command.FindIssue);
}

async function showConfigLocationMenu(
  updateStatusBar: () => Promise<void>,
  updateBadge: () => void,
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
  searchProvider: SearchResultProvider,
) {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    return;
  }

  const workspacePath = workspaceFolder.uri.fsPath;
  const customConfigDir = currentCustomConfigDirRef.current;

  const hasCustom = customConfigDir ? await hasCustomConfig(workspacePath, customConfigDir) : false;
  const hasLocal = await hasLocalConfig(workspacePath);
  const hasGlobal = await hasGlobalConfig(context, workspacePath);

  const currentLocation = getCurrentConfigLocation(hasCustom, hasLocal, hasGlobal);

  const menuItems: QuickPickItemWithId[] = [
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
      detail: '.tscanner/config.jsonc (can commit to git)',
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
        searchProvider,
        updateStatusBar,
        updateBadge,
        currentLocation,
        hasCustom,
        hasLocal,
        hasGlobal,
      );
    }
    return;
  }

  if (targetLocation === ConfigLocation.CustomPath) {
    await handleCustomPathSelection(
      workspacePath,
      currentCustomConfigDirRef,
      context,
      searchProvider,
      updateStatusBar,
      updateBadge,
      currentLocation,
      hasCustom,
      hasLocal,
      hasGlobal,
    );
    return;
  }

  if (currentLocation && (hasCustom || hasLocal || hasGlobal)) {
    await moveConfigToLocation(
      workspacePath,
      currentCustomConfigDirRef,
      context,
      searchProvider,
      updateStatusBar,
      updateBadge,
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
    showToastMessage(ToastKind.Info, 'Config location set. Use "Manage Rules" to create config.');
  }
}

async function handleCustomPathSelection(
  workspacePath: string,
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
  searchProvider: SearchResultProvider,
  updateStatusBar: () => Promise<void>,
  updateBadge: () => void,
  currentLocation: ConfigLocation | null,
  hasCustom: boolean,
  hasLocal: boolean,
  hasGlobal: boolean,
) {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) return;

  const startPath = currentCustomConfigDirRef.current || '.';
  const result = await showFolderPickerQuickPick(workspaceFolder.uri, startPath);

  if (result === 'cancelled' || result === null) return;

  if (currentLocation && (hasCustom || hasLocal || hasGlobal)) {
    await moveConfigToLocation(
      workspacePath,
      currentCustomConfigDirRef,
      context,
      searchProvider,
      updateStatusBar,
      updateBadge,
      currentLocation,
      ConfigLocation.CustomPath,
      result,
    );
  } else {
    currentCustomConfigDirRef.current = result;
    updateState(context, WorkspaceStateKey.CustomConfigDir, result);
    await updateStatusBar();
    logger.info(`Custom config folder set to: ${result}`);
    showToastMessage(ToastKind.Info, `Config location set to: ${result}. Use "Manage Rules" to create config.`);
  }
}

async function moveConfigToLocation(
  workspacePath: string,
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
  searchProvider: SearchResultProvider,
  updateStatusBar: () => Promise<void>,
  updateBadge: () => void,
  fromLocation: ConfigLocation,
  toLocation: ConfigLocation,
  customPath: string | null,
) {
  const fromLabel = getLocationLabel(fromLocation, currentCustomConfigDirRef.current);
  const toLabel = getLocationLabel(toLocation, customPath);

  const confirm = await vscode.window.showWarningMessage(
    `Move config from "${fromLabel}" to "${toLabel}"?`,
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

  searchProvider.setResults([]);
  updateBadge();
  invalidateCache();
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
      return '.tscanner';
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
  startPath: string,
): Promise<string | null | 'cancelled'> {
  let currentRelativePath = startPath;

  while (true) {
    const currentUri =
      currentRelativePath === '.' ? workspaceRoot : vscode.Uri.joinPath(workspaceRoot, currentRelativePath);

    const subfolders = await getSubfolders(currentUri);
    const displayPath = currentRelativePath === '.' ? '/' : `/${currentRelativePath}`;

    const items: QuickPickItemWithId[] = [];

    items.push({
      id: '__select__',
      label: '$(check) Select This Folder',
      detail: `Use: ${displayPath}`,
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
        detail: currentRelativePath === '.' ? `/${folder}` : `/${currentRelativePath}/${folder}`,
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
      const parts = currentRelativePath.split('/');
      parts.pop();
      currentRelativePath = parts.length === 0 ? '.' : parts.join('/');
      continue;
    }

    currentRelativePath = currentRelativePath === '.' ? selected.id : `${currentRelativePath}/${selected.id}`;
  }
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

  const menuItems: QuickPickItemWithId[] = [
    {
      id: ConfigLocation.ExtensionStorage,
      label: '$(cloud) Extension Storage',
      detail: 'Lost on extension uninstall',
    },
    {
      id: ConfigLocation.ProjectFolder,
      label: '$(file) Project Folder',
      detail: '.tscanner/config.jsonc (can commit to git)',
    },
    {
      id: ConfigLocation.CustomPath,
      label: '$(folder-opened) Custom Path',
      detail: 'Select a custom folder within workspace',
    },
  ];

  const selected = await vscode.window.showQuickPick(menuItems, {
    placeHolder: 'Where do you want to save the rules configuration?',
    ignoreFocusOut: true,
  });

  if (!selected) return null;

  const targetLocation = selected.id as ConfigLocation;

  if (targetLocation === ConfigLocation.CustomPath) {
    const result = await showFolderPickerQuickPick(workspaceFolder.uri, '.');
    if (result === 'cancelled' || result === null) return null;

    currentCustomConfigDirRef.current = result;
    updateState(context, WorkspaceStateKey.CustomConfigDir, result);

    return { location: targetLocation, customPath: result };
  }

  currentCustomConfigDirRef.current = null;
  updateState(context, WorkspaceStateKey.CustomConfigDir, null);

  return { location: targetLocation, customPath: null };
}

export { ConfigLocation };
