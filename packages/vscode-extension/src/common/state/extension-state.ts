import type { ScanMode } from 'tscanner-common';
import type * as vscode from 'vscode';
import type { TscannerLspClient } from '../../lsp/client';
import { WorkspaceStateKey, getWorkspaceState } from './workspace-state';

export type ExtensionStateRefs = {
  isSearchingRef: { current: boolean };
  currentScanModeRef: { current: ScanMode };
  currentCompareBranchRef: { current: string };
  currentConfigDirRef: { current: string | null };
};

export function createExtensionStateRefs(context: vscode.ExtensionContext): ExtensionStateRefs {
  return {
    isSearchingRef: { current: false },
    currentScanModeRef: { current: getWorkspaceState(context, WorkspaceStateKey.ScanMode) },
    currentCompareBranchRef: { current: getWorkspaceState(context, WorkspaceStateKey.CompareBranch) },
    currentConfigDirRef: { current: getWorkspaceState(context, WorkspaceStateKey.CustomConfigDir) },
  };
}

export type CommandContext = {
  context: vscode.ExtensionContext;
  treeView: vscode.TreeView<any>;
  stateRefs: ExtensionStateRefs;
  updateStatusBar: () => Promise<void>;
  getLspClient: () => TscannerLspClient | null;
};
