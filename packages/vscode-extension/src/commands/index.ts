import type * as vscode from 'vscode';
import { setCopyLspClient, setCopyScanContext } from '../common/lib/copy-utils';
import { Command, executeCommand, registerCommand } from '../common/lib/vscode-utils';
import type { CommandContext } from '../common/state/extension-state';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { ScanTrigger } from '../common/types/scan-trigger';
import type { AiIssuesView, RegularIssuesView } from '../issues-panel';
import type { SettingsView } from '../settings-view';
import { CopyMode, CopyScope, createCopyCommand } from './internal/copy-items';
import { createOpenFileCommand } from './internal/navigation';
import {
  createCycleViewModeFileFlatViewCommand,
  createCycleViewModeFileTreeViewCommand,
  createCycleViewModeRuleFlatViewCommand,
  createCycleViewModeRuleTreeViewCommand,
} from './internal/view-mode';
import { createClearScanCachesCommand } from './public/clear-scan-caches';
import { createGoToNextIssueCommand, createGoToPreviousIssueCommand } from './public/issue-navigation';
import { createOpenLocalConfigCommand, createOpenProjectConfigCommand } from './public/open-config-files';
import { createOpenSettingsViewCommand } from './public/open-settings-view';
import { createRefreshAiIssuesCommand } from './public/refresh-ai-issues';
import { createRefreshIssuesCommand } from './public/refresh-issues';
import {
  createManageAiModelCommand,
  createManageAiProviderCommand,
  createManageScanModeCommand,
  createManageStartupAiScanCommand,
  createManageStartupScanCommand,
  createToggleAutoAiScanCommand,
  createToggleAutoScanCommand,
} from './public/settings-view-actions';
import { createShowLogsCommand } from './public/show-logs';

export function registerAllCommands(
  ctx: CommandContext,
  regularView: RegularIssuesView,
  aiView: AiIssuesView,
  settingsView: SettingsView,
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
    createRefreshIssuesCommand(ctx, regularView),
    registerCommand(Command.RefreshIssuesCached, () => {
      return executeCommand(Command.RefreshIssues, { trigger: ScanTrigger.IssuesPanelRefresh });
    }),
    createRefreshAiIssuesCommand(ctx, aiView),
    registerCommand(Command.RefreshAiIssuesCached, () => {
      return executeCommand(Command.RefreshAiIssues, { trigger: ScanTrigger.IssuesPanelRefresh });
    }),
    createGoToNextIssueCommand(regularView),
    createGoToPreviousIssueCommand(regularView),
    createShowLogsCommand(),
    createClearScanCachesCommand(),
    createOpenSettingsViewCommand(),
    createOpenProjectConfigCommand(),
    createOpenLocalConfigCommand(),
    createManageAiProviderCommand({ commandContext: ctx, regularView, settingsView }),
    createManageAiModelCommand({ commandContext: ctx, regularView, settingsView }),
    createManageScanModeCommand({ commandContext: ctx, regularView, settingsView }),
    createManageStartupScanCommand({ commandContext: ctx, regularView, settingsView }),
    createManageStartupAiScanCommand({ commandContext: ctx, regularView, settingsView }),
    createToggleAutoScanCommand({ commandContext: ctx, regularView, settingsView }),
    createToggleAutoAiScanCommand({ commandContext: ctx, regularView, settingsView }),
    createCycleViewModeFileFlatViewCommand(regularView, aiView, context),
    createCycleViewModeFileTreeViewCommand(regularView, aiView, context),
    createCycleViewModeRuleFlatViewCommand(regularView, aiView, context),
    createCycleViewModeRuleTreeViewCommand(regularView, aiView, context),
    createOpenFileCommand(),
    createCopyCommand(CopyMode.Regular, CopyScope.Rule),
    createCopyCommand(CopyMode.Regular, CopyScope.File),
    createCopyCommand(CopyMode.Regular, CopyScope.Folder),
    createCopyCommand(CopyMode.Regular, CopyScope.All, {
      getResults: () => regularView.getResults(),
      getGroupMode: () => regularView.groupMode,
    }),
    createCopyCommand(CopyMode.Ai, CopyScope.Rule),
    createCopyCommand(CopyMode.Ai, CopyScope.File),
    createCopyCommand(CopyMode.Ai, CopyScope.Folder),
    createCopyCommand(CopyMode.Ai, CopyScope.All, {
      getResults: () => aiView.getResults(),
      getGroupMode: () => aiView.groupMode,
    }),
  ];
}
