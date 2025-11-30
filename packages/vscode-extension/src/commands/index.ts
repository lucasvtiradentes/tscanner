import type * as vscode from 'vscode';
import { setCopyRustClient, setCopyScanContext } from '../common/lib/copy-utils';
import type { CommandContext } from '../common/lib/extension-state';
import type { IssuesPanelContent } from '../issues-panel/panel-content';
import { createManageRulesCommand, createOpenSettingsMenuCommand } from '../settings-menu';
import { createCopyAllIssuesCommand } from './internal/copy-all';
import {
  createCopyFileIssuesCommand,
  createCopyFolderIssuesCommand,
  createCopyRuleIssuesCommand,
} from './internal/copy-items';
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

export function registerAllCommands(ctx: CommandContext, panelContent: IssuesPanelContent): vscode.Disposable[] {
  const { context, stateRefs, updateBadge, updateStatusBar, getRustClient } = ctx;

  setCopyRustClient(getRustClient);
  setCopyScanContext(stateRefs.currentScanModeRef.current, stateRefs.currentCompareBranchRef.current);

  void updateBadge;

  return [
    createScanWorkspaceCommand(ctx, panelContent),
    createHardScanCommand(stateRefs.isSearchingRef),
    createGoToNextIssueCommand(panelContent),
    createGoToPreviousIssueCommand(panelContent),
    createShowLogsCommand(),
    createRefreshCommand(),
    createManageRulesCommand(updateStatusBar, context, stateRefs.currentCustomConfigDirRef),
    createOpenSettingsMenuCommand(ctx, panelContent),
    createCycleViewModeFileFlatViewCommand(panelContent, context),
    createCycleViewModeFileTreeViewCommand(panelContent, context),
    createCycleViewModeRuleFlatViewCommand(panelContent, context),
    createCycleViewModeRuleTreeViewCommand(panelContent, context),
    createOpenFileCommand(),
    createCopyRuleIssuesCommand(),
    createCopyFileIssuesCommand(),
    createCopyFolderIssuesCommand(),
    createCopyAllIssuesCommand(
      () => panelContent.getResults(),
      () => panelContent.groupMode,
    ),
  ];
}

export { resetIssueIndex };
