import * as vscode from 'vscode';
import { setCopyScanContext } from '../commands/internal/copy';
import {
  Command,
  ScanMode,
  ToastKind,
  WorkspaceStateKey,
  executeCommand,
  getCurrentWorkspaceFolder,
  showToastMessage,
  updateState,
} from '../common/lib/vscode-utils';
import { getAllBranches, getCurrentBranch, invalidateCache } from '../common/utils/git-helper';
import { logger } from '../common/utils/logger';
import type { IssuesPanelContent } from '../issues-panel/panel-content';

type QuickPickItemWithId = {
  id: string;
} & vscode.QuickPickItem;

enum BranchMenuOption {
  KeepCurrent = 'keep-current',
  ChooseAnother = 'choose-another',
}

export async function showScanModeMenu(
  updateStatusBar: () => Promise<void>,
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  context: vscode.ExtensionContext,
  panelContent: IssuesPanelContent,
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
    await handleCodebaseScan(updateStatusBar, currentScanModeRef, currentCompareBranchRef, context, panelContent);
  }

  if (selected.id === ScanMode.Branch) {
    await handleBranchScan(updateStatusBar, currentScanModeRef, currentCompareBranchRef, context, panelContent);
  }
}

async function handleCodebaseScan(
  updateStatusBar: () => Promise<void>,
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  context: vscode.ExtensionContext,
  panelContent: IssuesPanelContent,
) {
  logger.info('Switching to Codebase mode');
  panelContent.setResults([]);
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
  panelContent: IssuesPanelContent,
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
  panelContent.setResults([]);
  currentScanModeRef.current = ScanMode.Branch;
  updateState(context, WorkspaceStateKey.ScanMode, ScanMode.Branch);
  setCopyScanContext(ScanMode.Branch, currentCompareBranchRef.current);
  invalidateCache();
  await updateStatusBar();
  executeCommand(Command.FindIssue);
}
