import * as vscode from 'vscode';
import { registerAllCommands } from './commands';
import { getAiViewId, getViewId } from './common/constants';
import { initializeLogger, logger } from './common/lib/logger';
import { Command, executeCommand, getCurrentWorkspaceFolder } from './common/lib/vscode-utils';
import { EXTENSION_DISPLAY_NAME } from './common/scripts-constants';
import { ExtensionConfigKey, getExtensionConfig, getFullConfigKeyPath } from './common/state/extension-config';
import { type CommandContext, createExtensionStateRefs } from './common/state/extension-state';
import { ContextKey, WorkspaceStateKey, getWorkspaceState, setContextKey } from './common/state/workspace-state';
import { AiIssuesView, IssuesViewIcon, RegularIssuesView } from './issues-panel';
import { dispose as disposeScanner, getLspClient, startLspClient } from './scanner/client';
import { StatusBarManager } from './status-bar/status-bar-manager';
import { disposeAiScanInterval, setupAiScanInterval } from './watchers/ai-scan-interval-watcher';
import { createConfigWatcher } from './watchers/config-watcher';
import { createFileWatcher } from './watchers/file-watcher';
import { disposeScanInterval, setupScanInterval } from './watchers/scan-interval-watcher';

let activationKey: string | undefined;

function setupTreeView(view: RegularIssuesView): vscode.TreeView<vscode.TreeItem> {
  const viewId = getViewId();
  logger.info(`Registering tree view with ID: ${viewId}`);

  return vscode.window.createTreeView(viewId, {
    treeDataProvider: view,
  });
}

function setupAiTreeView(view: AiIssuesView): vscode.TreeView<vscode.TreeItem> {
  const viewId = getAiViewId();
  logger.info(`Registering AI tree view with ID: ${viewId}`);

  return vscode.window.createTreeView(viewId, {
    treeDataProvider: view,
  });
}

function setupContextKeys(viewModeKey: string, groupModeKey: string, scanModeKey: string): void {
  logger.info(`Setting context keys: viewMode=${viewModeKey}, groupMode=${groupModeKey}, scanMode=${scanModeKey}`);
  setContextKey(ContextKey.ViewMode, viewModeKey);
  setContextKey(ContextKey.GroupMode, groupModeKey);
  setContextKey(ContextKey.ScanMode, scanModeKey);
  setContextKey(ContextKey.Searching, false);
  setContextKey(ContextKey.HasScanned, false);
  setContextKey(ContextKey.HasAiScanned, false);
}

export function activate(context: vscode.ExtensionContext) {
  const workspaceFolder = getCurrentWorkspaceFolder();
  const currentKey = workspaceFolder?.uri.fsPath || 'no-workspace';

  if (activationKey === currentKey) {
    logger.warn('Extension already activated for this workspace, skipping duplicate activation');
    return;
  }

  activationKey = currentKey;

  const logsEnabled = getExtensionConfig(ExtensionConfigKey.LogsEnabled);
  initializeLogger(logsEnabled);
  logger.clear();
  logger.info(`${EXTENSION_DISPLAY_NAME} extension activated`);

  const regularView = new RegularIssuesView();
  const aiView = new AiIssuesView();
  const viewModeKey = getWorkspaceState(context, WorkspaceStateKey.ViewMode);
  const groupModeKey = getWorkspaceState(context, WorkspaceStateKey.GroupMode);
  const scanModeKey = getWorkspaceState(context, WorkspaceStateKey.ScanMode);

  regularView.viewMode = viewModeKey;
  regularView.groupMode = groupModeKey;
  regularView.setResults([]);
  aiView.viewMode = viewModeKey;
  aiView.groupMode = groupModeKey;
  aiView.setResults([], true);

  setupContextKeys(viewModeKey, groupModeKey, scanModeKey);

  const treeView = setupTreeView(regularView);
  const aiTreeView = setupAiTreeView(aiView);
  const regularViewIcon = new IssuesViewIcon(treeView, regularView);
  const aiViewIcon = new IssuesViewIcon(aiTreeView, aiView, 'AI');
  const stateRefs = createExtensionStateRefs(context);

  logger.info('Creating status bar manager...');
  const statusBarManager = new StatusBarManager(
    stateRefs.currentScanModeRef,
    stateRefs.currentCompareBranchRef,
    stateRefs.currentConfigDirRef,
  );

  const updateStatusBar = async () => {
    await statusBarManager.update();
  };

  updateStatusBar().then(() => logger.info('Status bar setup complete'));

  const commandContext: CommandContext = {
    context,
    treeView,
    stateRefs,
    updateStatusBar,
    getLspClient,
  };

  const commands = registerAllCommands(commandContext, regularView, aiView);

  let currentFileWatcher: vscode.FileSystemWatcher | null = null;

  const recreateFileWatcher = async () => {
    if (currentFileWatcher) {
      currentFileWatcher.dispose();
    }
    currentFileWatcher = await createFileWatcher(context, regularView, stateRefs);
  };

  const configWatcher = createConfigWatcher(async () => {
    await setupScanInterval(context, stateRefs);
    await setupAiScanInterval(context, stateRefs);
    await recreateFileWatcher();
  });

  context.subscriptions.push(...commands, configWatcher, statusBarManager.getDisposable(), regularViewIcon, aiViewIcon);

  void recreateFileWatcher();

  const settingsWatcher = vscode.workspace.onDidChangeConfiguration(async (e) => {
    if (e.affectsConfiguration(getFullConfigKeyPath(ExtensionConfigKey.LspBin))) {
      const restart = await vscode.window.showInformationMessage(
        `${EXTENSION_DISPLAY_NAME} binary path changed. Restart LSP server?`,
        'Restart',
        'Later',
      );

      if (restart === 'Restart') {
        disposeScanner();
        await startLspClient();
        executeCommand(Command.FindIssue, { silent: true });
      }
    }
  });

  context.subscriptions.push(settingsWatcher);

  setTimeout(async () => {
    logger.info('Starting LSP client...');
    await startLspClient();
    logger.info('Running initial scan after 2s delay...');
    executeCommand(Command.FindIssue, { silent: true });

    await setupScanInterval(context, stateRefs);
    await setupAiScanInterval(context, stateRefs);
  }, 2000);
}

export function deactivate() {
  disposeScanInterval();
  disposeAiScanInterval();
  disposeScanner();
}
