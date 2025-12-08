import type * as vscode from 'vscode';
import { setCopyLspClient, setCopyScanContext } from '../common/lib/copy-utils';
import type { CommandContext } from '../common/state/extension-state';
import type { AiIssuesView, RegularIssuesView } from '../issues-panel';
import { createManageRulesCommand, createOpenSettingsMenuCommand } from '../settings-menu';
import { createCopyAllAiIssuesCommand, createCopyAllIssuesCommand } from './internal/copy-all';
import {
  createCopyAiFileIssuesCommand,
  createCopyAiFolderIssuesCommand,
  createCopyAiRuleIssuesCommand,
  createCopyFileIssuesCommand,
  createCopyFolderIssuesCommand,
  createCopyRuleIssuesCommand,
} from './internal/copy-items';
import { createOpenFileCommand } from './internal/navigation';
import { createRefreshAiIssuesCommand, createRefreshCommand } from './internal/refresh';
import {
  createCycleViewModeFileFlatViewCommand,
  createCycleViewModeFileTreeViewCommand,
  createCycleViewModeRuleFlatViewCommand,
  createCycleViewModeRuleTreeViewCommand,
} from './internal/view-mode';
import { createHardScanCommand } from './public/hard-scan';
import { createGoToNextIssueCommand, createGoToPreviousIssueCommand } from './public/issue-navigation';
import { createScanWorkspaceCommand } from './public/scan-workspace';
import { createShowLogsCommand } from './public/show-logs';

export function registerAllCommands(
  ctx: CommandContext,
  regularView: RegularIssuesView,
  aiView: AiIssuesView,
): vscode.Disposable[] {
  const { context, stateRefs, updateStatusBar, getLspClient } = ctx;

  setCopyLspClient(getLspClient);
  setCopyScanContext(stateRefs.currentScanModeRef.current, stateRefs.currentCompareBranchRef.current);

  void updateStatusBar;

  return [
    createScanWorkspaceCommand(ctx, regularView),
    createHardScanCommand(stateRefs.isSearchingRef),
    createGoToNextIssueCommand(regularView),
    createGoToPreviousIssueCommand(regularView),
    createShowLogsCommand(),
    createRefreshCommand(),
    createManageRulesCommand(updateStatusBar, context, stateRefs.currentCustomConfigDirRef),
    createOpenSettingsMenuCommand(ctx, regularView),
    createCycleViewModeFileFlatViewCommand(regularView, aiView, context),
    createCycleViewModeFileTreeViewCommand(regularView, aiView, context),
    createCycleViewModeRuleFlatViewCommand(regularView, aiView, context),
    createCycleViewModeRuleTreeViewCommand(regularView, aiView, context),
    createOpenFileCommand(),
    createCopyRuleIssuesCommand(),
    createCopyFileIssuesCommand(),
    createCopyFolderIssuesCommand(),
    createCopyAllIssuesCommand(
      () => regularView.getResults(),
      () => regularView.groupMode,
    ),
    createCopyAiRuleIssuesCommand(),
    createCopyAiFileIssuesCommand(),
    createCopyAiFolderIssuesCommand(),
    createCopyAllAiIssuesCommand(
      () => aiView.getResults(),
      () => aiView.groupMode,
    ),
    createRefreshAiIssuesCommand(ctx, aiView),
  ];
}
