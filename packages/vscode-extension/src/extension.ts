import * as vscode from 'vscode';
import { registerAllCommands } from './commands';
import { getViewId } from './common/constants';
import { type CommandContext, createExtensionStateRefs } from './common/lib/extension-state';
import { dispose as disposeScanner, getLspClient, startLspClient } from './common/lib/scanner';
import {
  Command,
  ContextKey,
  WorkspaceStateKey,
  executeCommand,
  getCurrentWorkspaceFolder,
  getWorkspaceState,
  setContextKey,
} from './common/lib/vscode-utils';
import { logger } from './common/utils/logger';
import { IssuesPanelContent } from './issues-panel/panel-content';
import { IssuesPanelIcon } from './issues-panel/panel-icon';
import { StatusBarManager } from './status-bar/status-bar-manager';
import { createConfigWatcher } from './watchers/config-watcher';
import { createFileWatcher } from './watchers/file-watcher';
import { disposeScanInterval, setupScanInterval } from './watchers/scan-interval-watcher';

let activationKey: string | undefined;

function setupTreeView(panelContent: IssuesPanelContent): vscode.TreeView<any> {
  const viewId = getViewId();
  logger.info(`Registering tree view with ID: ${viewId}`);

  return vscode.window.createTreeView(viewId, {
    treeDataProvider: panelContent,
  });
}

function setupContextKeys(viewModeKey: string, groupModeKey: string, scanModeKey: string): void {
  setContextKey(ContextKey.ViewMode, viewModeKey);
  setContextKey(ContextKey.GroupMode, groupModeKey);
  setContextKey(ContextKey.ScanMode, scanModeKey);
  setContextKey(ContextKey.Searching, false);
}

export function activate(context: vscode.ExtensionContext) {
  const workspaceFolder = getCurrentWorkspaceFolder();
  const currentKey = workspaceFolder?.uri.fsPath || 'no-workspace';

  if (activationKey === currentKey) {
    logger.warn('Extension already activated for this workspace, skipping duplicate activation');
    return;
  }

  activationKey = currentKey;
  logger.clear();
  logger.info('TScanner extension activated');

  const panelContent = new IssuesPanelContent();
  const viewModeKey = getWorkspaceState(context, WorkspaceStateKey.ViewMode);
  const groupModeKey = getWorkspaceState(context, WorkspaceStateKey.GroupMode);
  const scanModeKey = getWorkspaceState(context, WorkspaceStateKey.ScanMode);

  panelContent.viewMode = viewModeKey;
  panelContent.groupMode = groupModeKey;
  panelContent.setResults([]);

  setupContextKeys(viewModeKey, groupModeKey, scanModeKey);

  const treeView = setupTreeView(panelContent);
  const panelIcon = new IssuesPanelIcon(treeView, panelContent);
  const stateRefs = createExtensionStateRefs(context);

  logger.info('Creating status bar manager...');
  const statusBarManager = new StatusBarManager(
    context,
    stateRefs.currentScanModeRef,
    stateRefs.currentCompareBranchRef,
    stateRefs.currentCustomConfigDirRef,
  );

  const updateStatusBar = async () => {
    await statusBarManager.update();
  };
  updateStatusBar();
  logger.info('Status bar setup complete');

  const updateBadge = () => {
    panelIcon.update();
  };
  updateBadge();

  const commandContext: CommandContext = {
    context,
    treeView,
    stateRefs,
    updateBadge,
    updateStatusBar,
    getLspClient,
  };

  const commands = registerAllCommands(commandContext, panelContent);

  let currentFileWatcher: vscode.FileSystemWatcher | null = null;

  const recreateFileWatcher = async () => {
    if (currentFileWatcher) {
      currentFileWatcher.dispose();
    }
    currentFileWatcher = await createFileWatcher(context, panelContent, stateRefs, updateBadge);
  };

  const configWatcher = createConfigWatcher(async () => {
    await setupScanInterval(context, stateRefs);
    await recreateFileWatcher();
  });

  context.subscriptions.push(...commands, configWatcher, statusBarManager.getDisposable());

  recreateFileWatcher();

  setTimeout(async () => {
    logger.info('Starting LSP client...');
    await startLspClient();
    logger.info('Running initial scan after 2s delay...');
    executeCommand(Command.FindIssue, { silent: true });

    await setupScanInterval(context, stateRefs);
  }, 2000);
}

export function deactivate() {
  disposeScanInterval();
  disposeScanner();
}
