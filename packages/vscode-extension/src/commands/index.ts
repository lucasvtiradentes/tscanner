import type * as vscode from 'vscode';
import type { CommandDependencies } from '../common/lib/extension-state';
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

export function registerAllCommands(deps: CommandDependencies, panelContent: IssuesPanelContent): vscode.Disposable[] {
  const { context, treeView, stateRefs, updateBadge, updateStatusBar, getRustClient } = deps;

  setCopyRustClient(getRustClient);
  setCopyScanContext(stateRefs.currentScanModeRef.current, stateRefs.currentCompareBranchRef.current);

  return [
    createScanWorkspaceCommand(
      panelContent,
      context,
      treeView,
      updateBadge,
      updateStatusBar,
      stateRefs.isSearchingRef,
      stateRefs.currentScanModeRef,
      stateRefs.currentCompareBranchRef,
      stateRefs.currentCustomConfigDirRef,
    ),
    createHardScanCommand(stateRefs.isSearchingRef),
    createGoToNextIssueCommand(panelContent),
    createGoToPreviousIssueCommand(panelContent),
    createShowLogsCommand(),
    createRefreshCommand(),
    createManageRulesCommand(updateStatusBar, context, stateRefs.currentCustomConfigDirRef),
    createOpenSettingsMenuCommand(
      updateStatusBar,
      updateBadge,
      stateRefs.currentScanModeRef,
      stateRefs.currentCompareBranchRef,
      stateRefs.currentCustomConfigDirRef,
      context,
      panelContent,
    ),
    createCycleViewModeFileFlatViewCommand(panelContent, context),
    createCycleViewModeFileTreeViewCommand(panelContent, context),
    createCycleViewModeRuleFlatViewCommand(panelContent, context),
    createCycleViewModeRuleTreeViewCommand(panelContent, context),
    createOpenFileCommand(),
    createCopyRuleIssuesCommand(),
    createCopyFileIssuesCommand(),
    createCopyFolderIssuesCommand(),
  ];
}

export { resetIssueIndex };
