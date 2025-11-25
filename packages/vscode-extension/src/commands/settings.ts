import * as vscode from 'vscode';
import {
  getCustomConfigPath,
  getGlobalConfigPath,
  getLocalConfigPath,
  hasCustomConfig,
  hasLocalConfig,
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
  ManageScanSettings = 'manage-scan-settings',
  SelectConfigFolder = 'select-config-folder',
  OpenConfigs = 'open-configs',
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

    const customConfigDir = currentCustomConfigDirRef.current;
    const configFolderDetail = customConfigDir ? `Currently: ${customConfigDir}` : 'Use default config location';

    const mainMenuItems: QuickPickItemWithId[] = [
      {
        id: SettingsMenuOption.ManageRules,
        label: '$(checklist) Manage Rules',
        detail: 'Enable/disable built-in rules and generate configuration',
      },
      {
        id: SettingsMenuOption.ManageScanSettings,
        label: '$(gear) Manage Scan Settings',
        detail: 'Choose between Codebase or Branch scan mode',
      },
      {
        id: SettingsMenuOption.SelectConfigFolder,
        label: '$(folder) Select Config Folder',
        detail: configFolderDetail,
      },
      {
        id: SettingsMenuOption.OpenConfigs,
        label: '$(edit) Open Project TScanner Configs',
        detail: 'Edit .tscanner config or global extension config',
      },
    ];

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
      case SettingsMenuOption.ManageScanSettings:
        logger.info('User selected: Manage Scan Settings');
        await showScanSettingsMenu(
          updateStatusBar,
          currentScanModeRef,
          currentCompareBranchRef,
          context,
          searchProvider,
        );
        break;
      case SettingsMenuOption.SelectConfigFolder:
        logger.info('User selected: Select Config Folder');
        await showSelectConfigFolderMenu(
          updateStatusBar,
          updateBadge,
          currentCustomConfigDirRef,
          context,
          searchProvider,
        );
        break;
      case SettingsMenuOption.OpenConfigs:
        logger.info('User selected: Open Project TScanner Configs');
        await openProjectTscannerConfigs(context, currentCustomConfigDirRef.current);
        break;
    }
  });
}

async function openProjectTscannerConfigs(context: vscode.ExtensionContext, customConfigDir: string | null) {
  const workspaceFolder = getCurrentWorkspaceFolder();

  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    return;
  }

  if (customConfigDir) {
    const hasCustom = await hasCustomConfig(workspaceFolder.uri.fsPath, customConfigDir);
    if (hasCustom) {
      const customConfigPath = getCustomConfigPath(workspaceFolder.uri.fsPath, customConfigDir);
      const doc = await openTextDocument(customConfigPath);
      await vscode.window.showTextDocument(doc);
      return;
    }
    logger.info('Custom config not found, falling back to local/global');
  }

  const hasLocal = await hasLocalConfig(workspaceFolder.uri.fsPath);
  if (hasLocal) {
    const localConfigPath = getLocalConfigPath(workspaceFolder.uri.fsPath);
    const doc = await openTextDocument(localConfigPath);
    await vscode.window.showTextDocument(doc);
    return;
  }

  logger.info('Local config not found, trying global config');

  const globalConfigPath = getGlobalConfigPath(context, workspaceFolder.uri.fsPath);
  try {
    await vscode.workspace.fs.stat(globalConfigPath);
    const doc = await openTextDocument(globalConfigPath);
    await vscode.window.showTextDocument(doc);
    return;
  } catch {
    logger.debug('Global config not found');
  }

  showToastMessage(ToastKind.Error, 'No TScanner configuration found. Create one via "Manage Rules" first.');
}

async function showScanSettingsMenu(
  updateStatusBar: () => Promise<void>,
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  context: vscode.ExtensionContext,
  searchProvider: SearchResultProvider,
) {
  logger.info('showScanSettingsMenu called');
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

enum ConfigFolderOption {
  SelectFolder = 'select-folder',
  ClearCustom = 'clear-custom',
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

async function showSelectConfigFolderMenu(
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

  const currentCustomDir = currentCustomConfigDirRef.current;

  const menuItems: QuickPickItemWithId[] = [
    {
      id: ConfigFolderOption.SelectFolder,
      label: '$(folder-opened) Select Folder',
      detail: 'Browse and select a folder within workspace',
    },
  ];

  if (currentCustomDir) {
    menuItems.push({
      id: ConfigFolderOption.ClearCustom,
      label: '$(close) Clear Custom Config Folder',
      detail: `Currently using: ${currentCustomDir}`,
    });
  }

  const selected = await vscode.window.showQuickPick(menuItems, {
    placeHolder: currentCustomDir ? `Config folder: ${currentCustomDir}` : 'Select config folder location',
    ignoreFocusOut: false,
  });

  if (!selected) return;

  if (selected.id === ConfigFolderOption.ClearCustom) {
    currentCustomConfigDirRef.current = null;
    updateState(context, WorkspaceStateKey.CustomConfigDir, null);
    searchProvider.setResults([]);
    updateBadge();
    invalidateCache();
    await updateStatusBar();
    showToastMessage(ToastKind.Info, 'Custom config folder cleared. Using default location.');
    executeCommand(Command.FindIssue);
    return;
  }

  if (selected.id === ConfigFolderOption.SelectFolder) {
    const startPath = currentCustomDir || '.';
    const result = await showFolderPickerQuickPick(workspaceFolder.uri, startPath);

    if (result === 'cancelled' || result === null) return;

    currentCustomConfigDirRef.current = result;
    updateState(context, WorkspaceStateKey.CustomConfigDir, result);

    searchProvider.setResults([]);
    updateBadge();
    invalidateCache();
    await updateStatusBar();

    logger.info(`Custom config folder set to: ${result}`);
    showToastMessage(ToastKind.Info, `Config folder set to: ${result === '.' ? '/' : result}`);
    executeCommand(Command.FindIssue);
  }
}
