import { GitHelper, ScanMode } from 'tscanner-common';
import * as vscode from 'vscode';
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
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { ScanTrigger } from '../common/types/scan-trigger';
import type { RegularIssuesView } from '../issues-panel';

enum BranchMenuOption {
  KeepCurrent = 'keep-current',
  ChooseAnother = 'choose-another',
}

type ScanModeContext = {
  updateStatusBar: () => Promise<void>;
  regularView: RegularIssuesView;
};

export async function showScanModeMenu(ctx: ScanModeContext) {
  const { updateStatusBar, regularView } = ctx;

  logger.info('showScanModeMenu called');

  const currentScanMode = extensionStore.get(StoreKey.ScanMode);

  const scanModeItems: QuickPickItemWithId<ScanMode>[] = [
    {
      id: ScanMode.Codebase,
      label: '$(file-directory) Codebase',
      description: currentScanMode === ScanMode.Codebase ? '✓ Active' : '',
      detail: 'Scan all files in workspace',
    },
    {
      id: ScanMode.Branch,
      label: '$(git-branch) Branch',
      description: currentScanMode === ScanMode.Branch ? '✓ Active' : '',
      detail: 'Scan changes compared to target branch (git diff <branch>)',
    },
    {
      id: ScanMode.Uncommitted,
      label: '$(git-commit) Uncommitted',
      description: currentScanMode === ScanMode.Uncommitted ? '✓ Active' : '',
      detail: 'Scan staged and unstaged changes (git diff HEAD)',
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

  if (selected.id === ScanMode.Uncommitted) {
    await handleUncommittedScan(ctx);
  }
}

async function handleCodebaseScan(ctx: ScanModeContext) {
  const { updateStatusBar, regularView } = ctx;

  logger.info('Switching to Codebase mode');
  regularView.setResults([]);
  extensionStore.set(StoreKey.ScanMode, ScanMode.Codebase);
  await updateStatusBar();
  executeCommand(Command.RefreshIssues, { trigger: ScanTrigger.ScanModeChange });
}

async function handleBranchScan(ctx: ScanModeContext) {
  const { updateStatusBar, regularView } = ctx;

  const workspaceFolder = requireWorkspaceOrNull();
  if (!workspaceFolder) return;

  const currentBranch = VscodeGit.getCurrentBranch(workspaceFolder.uri.fsPath);
  if (!currentBranch) {
    showToastMessage(ToastKind.Error, 'Not in a git repository');
    return;
  }

  const currentCompareBranch = extensionStore.get(StoreKey.CompareBranch);

  const branchOptions: QuickPickItemWithId<BranchMenuOption>[] = [
    {
      id: BranchMenuOption.KeepCurrent,
      label: `Current value: ${currentCompareBranch}`,
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
          description: branch === currentCompareBranch ? '$(check) Current compare target' : '',
          detail: branch,
        })),
      );
    }

    if (remoteBranches.length > 0) {
      branchItems.push(
        { label: 'Remote branches', kind: vscode.QuickPickItemKind.Separator },
        ...remoteBranches.map((branch) => ({
          label: `$(cloud) ${branch}`,
          description: branch === currentCompareBranch ? '$(check) Current compare target' : '',
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

    extensionStore.set(StoreKey.CompareBranch, selectedBranch.detail);
  }

  const compareBranch = extensionStore.get(StoreKey.CompareBranch);
  logger.info(`Switching to Branch mode (comparing against: ${compareBranch})`);
  regularView.setResults([]);
  extensionStore.set(StoreKey.ScanMode, ScanMode.Branch);
  await updateStatusBar();
  executeCommand(Command.RefreshIssues, { trigger: ScanTrigger.ScanModeChange });
}

async function handleUncommittedScan(ctx: ScanModeContext) {
  const { updateStatusBar, regularView } = ctx;

  const workspaceFolder = requireWorkspaceOrNull();
  if (!workspaceFolder) return;

  const currentBranch = VscodeGit.getCurrentBranch(workspaceFolder.uri.fsPath);
  if (!currentBranch) {
    showToastMessage(ToastKind.Error, 'Not in a git repository');
    return;
  }

  logger.info('Switching to Uncommitted mode');
  regularView.setResults([]);
  extensionStore.set(StoreKey.ScanMode, ScanMode.Uncommitted);
  await updateStatusBar();
  executeCommand(Command.RefreshIssues, { trigger: ScanTrigger.ScanModeChange });
}
