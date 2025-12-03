import type { ScanMode } from 'tscanner-common';
import type * as vscode from 'vscode';
import type { TscannerLspClient } from '../../lsp';
import { WorkspaceStateKey, getWorkspaceState } from './vscode-utils';

export type ExtensionState = {
  isSearching: boolean;
  scanMode: ScanMode;
  compareBranch: string;
  customConfigDir: string | null;
};

export type ExtensionStateRefs = {
  isSearchingRef: { current: boolean };
  currentScanModeRef: { current: ScanMode };
  currentCompareBranchRef: { current: string };
  currentCustomConfigDirRef: { current: string | null };
};

export function createExtensionStateRefs(context: vscode.ExtensionContext): ExtensionStateRefs {
  return {
    isSearchingRef: { current: false },
    currentScanModeRef: { current: getWorkspaceState(context, WorkspaceStateKey.ScanMode) },
    currentCompareBranchRef: { current: getWorkspaceState(context, WorkspaceStateKey.CompareBranch) },
    currentCustomConfigDirRef: { current: getWorkspaceState(context, WorkspaceStateKey.CustomConfigDir) },
  };
}

export type CommandContext = {
  context: vscode.ExtensionContext;
  treeView: vscode.TreeView<any>;
  stateRefs: ExtensionStateRefs;
  updateBadge: () => void;
  updateStatusBar: () => Promise<void>;
  getLspClient: () => TscannerLspClient | null;
};
