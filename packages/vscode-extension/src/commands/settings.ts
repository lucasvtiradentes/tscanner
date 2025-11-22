import * as vscode from 'vscode';
import { getGlobalConfigPath, getLocalConfigPath } from '../common/lib/config-manager';
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
  OpenConfigs = 'open-configs',
}

enum BranchMenuOption {
  KeepCurrent = 'keep-current',
  ChooseAnother = 'choose-another',
}

interface QuickPickItemWithId extends vscode.QuickPickItem {
  id: string;
}

export function createOpenSettingsMenuCommand(
  updateStatusBar: () => Promise<void>,
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  context: vscode.ExtensionContext,
  searchProvider: SearchResultProvider,
) {
  return registerCommand(Command.OpenSettingsMenu, async () => {
    logger.info('openSettingsMenu command called');
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
        id: SettingsMenuOption.OpenConfigs,
        label: '$(edit) Open Project Tscanner Configs',
        detail: 'Edit .tscanner/rules.json or global extension config',
      },
    ];

    const selected = await vscode.window.showQuickPick(mainMenuItems, {
      placeHolder: 'Tscanner Settings',
      ignoreFocusOut: false,
    });

    logger.debug(`Main menu selection: ${selected ? selected.id : 'none (cancelled)'}`);

    if (!selected) {
      logger.debug('No selection made, returning');
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
      case SettingsMenuOption.OpenConfigs:
        logger.info('User selected: Open Project Tscanner Configs');
        await openProjectTscannerConfigs(context);
        break;
    }
  });
}

async function openProjectTscannerConfigs(context: vscode.ExtensionContext) {
  const workspaceFolder = getCurrentWorkspaceFolder();

  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    return;
  }

  const localConfigPath = getLocalConfigPath(workspaceFolder.uri.fsPath);
  try {
    await vscode.workspace.fs.stat(localConfigPath);
    const doc = await openTextDocument(localConfigPath);
    await vscode.window.showTextDocument(doc);
    return;
  } catch {
    logger.debug('Local config not found, trying global config');
  }

  const globalConfigPath = getGlobalConfigPath(context, workspaceFolder.uri.fsPath);
  try {
    await vscode.workspace.fs.stat(globalConfigPath);
    const doc = await openTextDocument(globalConfigPath);
    await vscode.window.showTextDocument(doc);
    return;
  } catch {
    logger.debug('Global config not found');
  }

  showToastMessage(ToastKind.Error, 'No Tscanner configuration found. Create one via "Manage Rules" first.');
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

  logger.debug(`Scan mode selection: ${selected ? selected.id : 'none (cancelled)'}`);

  if (!selected) {
    logger.debug('No scan mode selected, returning');
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
  logger.debug(`Current scan mode updated to: ${currentScanModeRef.current}`);
  updateState(context, WorkspaceStateKey.ScanMode, ScanMode.Codebase);
  setCopyScanContext(ScanMode.Codebase, currentCompareBranchRef.current);
  invalidateCache();
  logger.debug('Updating status bar');
  await updateStatusBar();
  logger.debug('Status bar updated, triggering scan');
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
    logger.debug(`Compare branch updated to: ${currentCompareBranchRef.current}`);
    updateState(context, WorkspaceStateKey.CompareBranch, currentCompareBranchRef.current);
  }

  logger.info(`Switching to Branch mode (comparing against: ${currentCompareBranchRef.current})`);
  searchProvider.setResults([]);
  currentScanModeRef.current = ScanMode.Branch;
  logger.debug(`Current scan mode updated to: ${currentScanModeRef.current}`);
  updateState(context, WorkspaceStateKey.ScanMode, ScanMode.Branch);
  setCopyScanContext(ScanMode.Branch, currentCompareBranchRef.current);
  invalidateCache();
  logger.debug('Updating status bar');
  await updateStatusBar();
  logger.debug('Status bar updated, triggering scan');
  executeCommand(Command.FindIssue);
}
