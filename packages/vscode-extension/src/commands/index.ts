import type * as vscode from 'vscode';
import type { RustClient } from '../common/lib/rust-client';
import type { ScanMode } from '../common/lib/vscode-utils';
import type { IssuesPanelContent } from '../issues-panel/panel-content';
import { createManageRulesCommand, createOpenSettingsMenuCommand } from '../settings-menu';
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
  panelContent: IssuesPanelContent;
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
      ctx.panelContent,
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
    createGoToNextIssueCommand(ctx.panelContent),
    createGoToPreviousIssueCommand(ctx.panelContent),
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
      ctx.panelContent,
    ),
    createCycleViewModeFileFlatViewCommand(ctx.panelContent, ctx.context),
    createCycleViewModeFileTreeViewCommand(ctx.panelContent, ctx.context),
    createCycleViewModeRuleFlatViewCommand(ctx.panelContent, ctx.context),
    createCycleViewModeRuleTreeViewCommand(ctx.panelContent, ctx.context),
    createOpenFileCommand(),
    createCopyRuleIssuesCommand(),
    createCopyFileIssuesCommand(),
    createCopyFolderIssuesCommand(),
  ];
}

export { resetIssueIndex };
