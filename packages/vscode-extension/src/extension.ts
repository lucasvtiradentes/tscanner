import * as vscode from 'vscode';
import { registerAllCommands } from './commands';
import { getViewId } from './common/constants';
import { loadEffectiveConfig } from './common/lib/config-manager';
import { type CommandContext, type ExtensionStateRefs, createExtensionStateRefs } from './common/lib/extension-state';
import { dispose as disposeScanner, getRustClient, startLspClient } from './common/lib/scanner';
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

let activationKey: string | undefined;
let scanIntervalTimer: NodeJS.Timeout | null = null;

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
    getRustClient,
  };

  const commands = registerAllCommands(commandContext, panelContent);
  const fileWatcher = createFileWatcher(context, panelContent, stateRefs, updateBadge);
  const configWatcher = createConfigWatcher(async () => {
    await setupScanInterval(context, stateRefs);
  });

  context.subscriptions.push(...commands, fileWatcher, configWatcher, statusBarManager.getDisposable());

  setTimeout(async () => {
    logger.info('Starting LSP client...');
    await startLspClient();
    logger.info('Running initial scan after 2s delay...');
    executeCommand(Command.FindIssue, { silent: true });

    await setupScanInterval(context, stateRefs);
  }, 2000);
}

async function setupScanInterval(context: vscode.ExtensionContext, stateRefs: ExtensionStateRefs): Promise<void> {
  if (scanIntervalTimer) {
    clearInterval(scanIntervalTimer);
    scanIntervalTimer = null;
  }

  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) return;

  const config = await loadEffectiveConfig(
    context,
    workspaceFolder.uri.fsPath,
    stateRefs.currentCustomConfigDirRef.current,
  );

  const scanIntervalSeconds = config?.codeEditor?.scanIntervalSeconds ?? 0;

  if (scanIntervalSeconds <= 0) {
    logger.info('Auto-scan interval disabled (scanIntervalSeconds = 0)');
    return;
  }

  const intervalMs = scanIntervalSeconds * 1000;
  logger.info(`Setting up auto-scan interval: ${scanIntervalSeconds} seconds`);

  scanIntervalTimer = setInterval(() => {
    if (stateRefs.isSearchingRef.current) {
      logger.debug('Auto-scan skipped: search in progress');
      return;
    }
    logger.debug('Running auto-scan...');
    executeCommand(Command.FindIssue, { silent: true });
  }, intervalMs);
}

export function deactivate() {
  if (scanIntervalTimer) {
    clearInterval(scanIntervalTimer);
    scanIntervalTimer = null;
  }
  disposeScanner();
}
