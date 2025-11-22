import type * as vscode from 'vscode';
import type { ScanMode } from '../common/lib/vscode-utils';
import type { SearchResultProvider } from '../sidebar/search-provider';
import {
  createCopyFileIssuesCommand,
  createCopyFolderIssuesCommand,
  createCopyRuleIssuesCommand,
  setCopyScanContext,
} from './copy-issues';
import { createFindIssueCommand } from './find-issue';
import { createGoToNextIssueCommand, createGoToPreviousIssueCommand, resetIssueIndex } from './issue-navigation';
import { createManageRulesCommand } from './manage-rules';
import { createOpenFileCommand } from './navigation';
import { createHardScanCommand, createRefreshCommand } from './scan';
import { createOpenSettingsMenuCommand } from './settings';
import { createShowLogsCommand } from './show-logs';
import {
  createCycleViewModeFileFlatViewCommand,
  createCycleViewModeFileTreeViewCommand,
  createCycleViewModeRuleFlatViewCommand,
  createCycleViewModeRuleTreeViewCommand,
} from './view-mode';

export interface CommandContext {
  searchProvider: SearchResultProvider;
  context: vscode.ExtensionContext;
  treeView: vscode.TreeView<any>;
  updateBadge: () => void;
  updateStatusBar: () => Promise<void>;
  isSearchingRef: { current: boolean };
  currentScanModeRef: { current: ScanMode };
  currentCompareBranchRef: { current: string };
}

export function registerAllCommands(ctx: CommandContext): vscode.Disposable[] {
  setCopyScanContext(ctx.currentScanModeRef.current, ctx.currentCompareBranchRef.current);

  return [
    createFindIssueCommand(
      ctx.searchProvider,
      ctx.context,
      ctx.treeView,
      ctx.updateBadge,
      ctx.updateStatusBar,
      ctx.isSearchingRef,
      ctx.currentScanModeRef,
      ctx.currentCompareBranchRef,
    ),
    createManageRulesCommand(ctx.updateStatusBar, ctx.context),
    createOpenSettingsMenuCommand(
      ctx.updateStatusBar,
      ctx.currentScanModeRef,
      ctx.currentCompareBranchRef,
      ctx.context,
      ctx.searchProvider,
    ),
    createCycleViewModeFileFlatViewCommand(ctx.searchProvider, ctx.context),
    createCycleViewModeFileTreeViewCommand(ctx.searchProvider, ctx.context),
    createCycleViewModeRuleFlatViewCommand(ctx.searchProvider, ctx.context),
    createCycleViewModeRuleTreeViewCommand(ctx.searchProvider, ctx.context),
    createOpenFileCommand(),
    createRefreshCommand(),
    createHardScanCommand(ctx.isSearchingRef),
    createGoToNextIssueCommand(ctx.searchProvider),
    createGoToPreviousIssueCommand(ctx.searchProvider),
    createShowLogsCommand(),
    createCopyRuleIssuesCommand(),
    createCopyFileIssuesCommand(),
    createCopyFolderIssuesCommand(),
  ];
}

export { resetIssueIndex };
