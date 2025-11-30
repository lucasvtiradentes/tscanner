import type * as vscode from 'vscode';
import type { CommandContext } from '../common/lib/extension-state';
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
import { createCycleViewModeCommand } from './internal/view-mode';
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
    createCycleViewModeCommand(panelContent, context),
    createOpenFileCommand(),
    createCopyRuleIssuesCommand(),
    createCopyFileIssuesCommand(),
    createCopyFolderIssuesCommand(),
  ];
}

export { resetIssueIndex };
