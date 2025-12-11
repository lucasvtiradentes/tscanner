import type * as vscode from 'vscode';
import { setCopyLspClient, setCopyScanContext } from '../common/lib/copy-utils';
import type { CommandContext } from '../common/state/extension-state';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import type { AiIssuesView, RegularIssuesView } from '../issues-panel';
import { createOpenSettingsMenuCommand } from '../settings-menu';
import { createCopyCommand } from './internal/copy-items';
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
  const { context, getLspClient } = ctx;

  setCopyLspClient(getLspClient);
  setCopyScanContext(extensionStore.get(StoreKey.ScanMode), extensionStore.get(StoreKey.CompareBranch));

  extensionStore.subscribe(StoreKey.ScanMode, (scanMode) => {
    setCopyScanContext(scanMode, extensionStore.get(StoreKey.CompareBranch));
  });
  extensionStore.subscribe(StoreKey.CompareBranch, (compareBranch) => {
    setCopyScanContext(extensionStore.get(StoreKey.ScanMode), compareBranch);
  });

  return [
    createScanWorkspaceCommand(ctx, regularView),
    createHardScanCommand(),
    createGoToNextIssueCommand(regularView),
    createGoToPreviousIssueCommand(regularView),
    createShowLogsCommand(),
    createRefreshCommand(),
    createOpenSettingsMenuCommand(ctx, regularView),
    createCycleViewModeFileFlatViewCommand(regularView, aiView, context),
    createCycleViewModeFileTreeViewCommand(regularView, aiView, context),
    createCycleViewModeRuleFlatViewCommand(regularView, aiView, context),
    createCycleViewModeRuleTreeViewCommand(regularView, aiView, context),
    createOpenFileCommand(),
    createCopyCommand('regular', 'rule'),
    createCopyCommand('regular', 'file'),
    createCopyCommand('regular', 'folder'),
    createCopyCommand('regular', 'all', {
      getResults: () => regularView.getResults(),
      getGroupMode: () => regularView.groupMode,
    }),
    createCopyCommand('ai', 'rule'),
    createCopyCommand('ai', 'file'),
    createCopyCommand('ai', 'folder'),
    createCopyCommand('ai', 'all', {
      getResults: () => aiView.getResults(),
      getGroupMode: () => aiView.groupMode,
    }),
    createRefreshAiIssuesCommand(ctx, aiView),
  ];
}
