import type * as vscode from 'vscode';
import type { RustClient } from '../common/lib/rust-client';
import type { ScanMode } from '../common/lib/vscode-utils';
import { createManageRulesCommand, createOpenSettingsMenuCommand } from '../settings-menu';
import type { SearchResultProvider } from '../sidebar/search-provider';
import {
  createCopyFileIssuesCommand,
  createCopyFolderIssuesCommand,
  createCopyRuleIssuesCommand,
  setCopyRustClient,
  setCopyScanContext,
} from './internal/copy';
import { createOpenFileCommand } from './internal/navigation';
import { createRefreshCommand } from './internal/refresh';
import {
  createCycleViewModeFileFlatViewCommand,
  createCycleViewModeFileTreeViewCommand,
  createCycleViewModeRuleFlatViewCommand,
  createCycleViewModeRuleTreeViewCommand,
} from './internal/view-mode';
import { createHardScanCommand } from './public/hard-scan';
import { createGoToNextIssueCommand, createGoToPreviousIssueCommand, resetIssueIndex } from './public/issue-navigation';
import { createScanWorkspaceCommand } from './public/scan-workspace';
import { createShowLogsCommand } from './public/show-logs';

export interface CommandContext {
  searchProvider: SearchResultProvider;
  context: vscode.ExtensionContext;
  treeView: vscode.TreeView<any>;
  updateBadge: () => void;
  updateStatusBar: () => Promise<void>;
  isSearchingRef: { current: boolean };
  currentScanModeRef: { current: ScanMode };
  currentCompareBranchRef: { current: string };
  currentCustomConfigDirRef: { current: string | null };
  getRustClient: () => RustClient | null;
}

export function registerAllCommands(ctx: CommandContext): vscode.Disposable[] {
  setCopyRustClient(ctx.getRustClient);
  setCopyScanContext(ctx.currentScanModeRef.current, ctx.currentCompareBranchRef.current);

  return [
    createScanWorkspaceCommand(
      ctx.searchProvider,
      ctx.context,
      ctx.treeView,
      ctx.updateBadge,
      ctx.updateStatusBar,
      ctx.isSearchingRef,
      ctx.currentScanModeRef,
      ctx.currentCompareBranchRef,
      ctx.currentCustomConfigDirRef,
    ),
    createHardScanCommand(ctx.isSearchingRef),
    createGoToNextIssueCommand(ctx.searchProvider),
    createGoToPreviousIssueCommand(ctx.searchProvider),
    createShowLogsCommand(),
    createRefreshCommand(),
    createManageRulesCommand(ctx.updateStatusBar, ctx.context, ctx.currentCustomConfigDirRef),
    createOpenSettingsMenuCommand(
      ctx.updateStatusBar,
      ctx.updateBadge,
      ctx.currentScanModeRef,
      ctx.currentCompareBranchRef,
      ctx.currentCustomConfigDirRef,
      ctx.context,
      ctx.searchProvider,
    ),
    createCycleViewModeFileFlatViewCommand(ctx.searchProvider, ctx.context),
    createCycleViewModeFileTreeViewCommand(ctx.searchProvider, ctx.context),
    createCycleViewModeRuleFlatViewCommand(ctx.searchProvider, ctx.context),
    createCycleViewModeRuleTreeViewCommand(ctx.searchProvider, ctx.context),
    createOpenFileCommand(),
    createCopyRuleIssuesCommand(),
    createCopyFileIssuesCommand(),
    createCopyFolderIssuesCommand(),
  ];
}

export { resetIssueIndex };
