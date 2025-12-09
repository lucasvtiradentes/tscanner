import { GitHelper, ScanMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { setCopyScanContext } from '../common/lib/copy-utils';
import { logger } from '../common/lib/logger';
import { VscodeGit } from '../common/lib/vscode-git';
import {
  Command,
  type QuickPickItemWithId,
  ToastKind,
  executeCommand,
  requireWorkspaceOrNull,
  showToastMessage,
} from '../common/lib/vscode-utils';
import type { ExtensionStateRefs } from '../common/state/extension-state';
import { WorkspaceStateKey, updateState } from '../common/state/workspace-state';
import type { RegularIssuesView } from '../issues-panel';

enum BranchMenuOption {
  KeepCurrent = 'keep-current',
  ChooseAnother = 'choose-another',
}

type ScanModeContext = {
  updateStatusBar: () => Promise<void>;
  stateRefs: ExtensionStateRefs;
  context: vscode.ExtensionContext;
  regularView: RegularIssuesView;
};

export async function showScanModeMenu(ctx: ScanModeContext) {
  const { updateStatusBar, stateRefs, context, regularView } = ctx;
  const { currentScanModeRef, currentCompareBranchRef } = stateRefs;

  logger.info('showScanModeMenu called');

  const scanModeItems: QuickPickItemWithId<ScanMode>[] = [
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

  if (!selected) return;

  if (selected.id === ScanMode.Codebase) {
    await handleCodebaseScan(ctx);
  }

  if (selected.id === ScanMode.Branch) {
    await handleBranchScan(ctx);
  }
}

async function handleCodebaseScan(ctx: ScanModeContext) {
  const { updateStatusBar, stateRefs, context, regularView } = ctx;
  const { currentScanModeRef, currentCompareBranchRef } = stateRefs;

  logger.info('Switching to Codebase mode');
  regularView.setResults([]);
  currentScanModeRef.current = ScanMode.Codebase;
  updateState(context, WorkspaceStateKey.ScanMode, ScanMode.Codebase);
  setCopyScanContext(ScanMode.Codebase, currentCompareBranchRef.current);
  await updateStatusBar();
  executeCommand(Command.FindIssue);
}

async function handleBranchScan(ctx: ScanModeContext) {
  const { updateStatusBar, stateRefs, context, regularView } = ctx;
  const { currentScanModeRef, currentCompareBranchRef } = stateRefs;

  const workspaceFolder = requireWorkspaceOrNull();
  if (!workspaceFolder) return;

  const currentBranch = VscodeGit.getCurrentBranch(workspaceFolder.uri.fsPath);
  if (!currentBranch) {
    showToastMessage(ToastKind.Error, 'Not in a git repository');
    return;
  }

  const branchOptions: QuickPickItemWithId<BranchMenuOption>[] = [
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
    const branches = await GitHelper.getAllBranches(workspaceFolder.uri.fsPath);

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
  regularView.setResults([]);
  currentScanModeRef.current = ScanMode.Branch;
  updateState(context, WorkspaceStateKey.ScanMode, ScanMode.Branch);
  setCopyScanContext(ScanMode.Branch, currentCompareBranchRef.current);
  await updateStatusBar();
  executeCommand(Command.FindIssue);
}
