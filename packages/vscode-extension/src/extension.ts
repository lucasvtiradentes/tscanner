import { CONFIG_DIR_NAME, LOCAL_CONFIG_FILE_NAME, VSCODE_EXTENSION } from 'tscanner-common';
import * as vscode from 'vscode';
import { registerAllCommands } from './commands';
import { IS_DEV, getAiViewId, getSettingsViewId, getViewId } from './common/constants';
import { getLocalConfigPath } from './common/lib/local-config';
import { initializeLogger, logger } from './common/lib/logger';
import { Command, executeCommand, getCurrentWorkspaceFolder } from './common/lib/vscode-utils';
import { EXTENSION_DISPLAY_NAME, buildConfigSection } from './common/scripts-constants';
import { ExtensionConfigKey, getExtensionConfig, getFullConfigKeyPath } from './common/state/extension-config';
import type { CommandContext } from './common/state/extension-state';
import { StoreKey, extensionStore } from './common/state/extension-store';
import { ContextKey, WorkspaceStateKey, getWorkspaceState, setContextKey } from './common/state/workspace-state';
import { ScanTrigger } from './common/types/scan-trigger';
import { AiIssuesView, IssuesViewIcon, RegularIssuesView } from './issues-panel';
import { dispose as disposeScanner, getLspClient, startLspClient } from './scanner/client';
import { SettingsView } from './settings-view';
import { disposeRunner, initializeRunner, runStartupSequence } from './startup/runner';
import { StatusBarManager } from './status-bar/status-bar-manager';
import {
  aiScanIntervalWatcher,
  createConfigWatcher,
  createFileWatcher,
  createGitWatcher,
  scanIntervalWatcher,
} from './watchers';

let activationKey: string | undefined;

type Views = {
  regularView: RegularIssuesView;
  aiView: AiIssuesView;
  settingsView: SettingsView;
  treeView: vscode.TreeView<vscode.TreeItem>;
  aiTreeView: vscode.TreeView<vscode.TreeItem>;
  settingsTreeView: vscode.TreeView<vscode.TreeItem>;
  regularViewIcon: IssuesViewIcon;
  aiViewIcon: IssuesViewIcon;
};

type WatchersSetupOptions = {
  context: vscode.ExtensionContext;
  regularView: RegularIssuesView;
  updateStatusBar: () => Promise<void>;
  updateSettingsView: () => void;
};

function setupViews(context: vscode.ExtensionContext): Views {
  const viewModeKey = getWorkspaceState(context, WorkspaceStateKey.ViewMode);
  const groupModeKey = getWorkspaceState(context, WorkspaceStateKey.GroupMode);

  const regularView = new RegularIssuesView();
  regularView.viewMode = viewModeKey;
  regularView.groupMode = groupModeKey;
  regularView.setResults([]);

  const aiView = new AiIssuesView();
  aiView.viewMode = viewModeKey;
  aiView.groupMode = groupModeKey;
  aiView.setResults([], true);

  const settingsView = new SettingsView();

  const treeView = vscode.window.createTreeView(getViewId(), { treeDataProvider: regularView });
  const aiTreeView = vscode.window.createTreeView(getAiViewId(), { treeDataProvider: aiView });
  const settingsTreeView = vscode.window.createTreeView(getSettingsViewId(), { treeDataProvider: settingsView });

  logger.info(`Registered tree views: ${getViewId()}, ${getAiViewId()}, ${getSettingsViewId()}`);

  return {
    regularView,
    aiView,
    settingsView,
    treeView,
    aiTreeView,
    settingsTreeView,
    regularViewIcon: new IssuesViewIcon(treeView, regularView),
    aiViewIcon: new IssuesViewIcon(aiTreeView, aiView, 'AI'),
  };
}

function setupContextKeys(context: vscode.ExtensionContext): void {
  const viewMode = getWorkspaceState(context, WorkspaceStateKey.ViewMode);
  const groupMode = getWorkspaceState(context, WorkspaceStateKey.GroupMode);
  const scanMode = getWorkspaceState(context, WorkspaceStateKey.ScanMode);

  logger.info(`Setting context keys: viewMode=${viewMode}, groupMode=${groupMode}, scanMode=${scanMode}`);

  setContextKey(ContextKey.ViewMode, viewMode);
  setContextKey(ContextKey.GroupMode, groupMode);
  setContextKey(ContextKey.ScanMode, scanMode);
  setContextKey(ContextKey.Searching, false);
  setContextKey(ContextKey.HasScanned, false);
  setContextKey(ContextKey.HasAiScanned, false);
}

async function setupWatchers({
  context,
  regularView,
  updateStatusBar,
  updateSettingsView,
}: WatchersSetupOptions): Promise<vscode.Disposable> {
  let currentFileWatcher: vscode.FileSystemWatcher | null = null;

  const recreateFileWatcher = async () => {
    if (currentFileWatcher) {
      currentFileWatcher.dispose();
    }
    currentFileWatcher = await createFileWatcher(context, regularView);
  };

  const configWatcher = createConfigWatcher(async () => {
    await scanIntervalWatcher.setup();
    await aiScanIntervalWatcher.setup();
    await recreateFileWatcher();
    await updateStatusBar();
    updateSettingsView();
  });

  const localConfigWatcher = vscode.workspace.createFileSystemWatcher(
    `**/${CONFIG_DIR_NAME}/${LOCAL_CONFIG_FILE_NAME}`,
  );
  const handleLocalConfigChange = async (uri: vscode.Uri) => {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) return;

    const expectedPath = getLocalConfigPath(workspaceFolder.uri.fsPath);
    if (uri.fsPath !== expectedPath) return;

    logger.info(`Local config file changed: ${vscode.workspace.asRelativePath(uri)}`);
    await updateStatusBar();
    updateSettingsView();
  };
  localConfigWatcher.onDidChange(handleLocalConfigChange);
  localConfigWatcher.onDidCreate(handleLocalConfigChange);
  localConfigWatcher.onDidDelete(handleLocalConfigChange);

  const gitWatcher = await createGitWatcher();

  void recreateFileWatcher();

  const disposables: vscode.Disposable[] = [configWatcher, localConfigWatcher];
  if (gitWatcher) {
    disposables.push(gitWatcher);
    logger.info('Git watcher enabled - will refresh on commits/checkouts');
  } else {
    logger.warn('Git extension not available - commit/checkout detection disabled');
  }

  return vscode.Disposable.from(...disposables);
}

function setupSettingsListener(
  updateStatusBar: () => Promise<void>,
  updateSettingsView: () => void,
): vscode.Disposable {
  return vscode.workspace.onDidChangeConfiguration(async (e) => {
    if (e.affectsConfiguration(buildConfigSection(IS_DEV))) {
      updateSettingsView();
    }

    if (e.affectsConfiguration(getFullConfigKeyPath(ExtensionConfigKey.AutoScanInterval))) {
      scanIntervalWatcher.setup(true);
      await updateStatusBar();
    }

    if (e.affectsConfiguration(getFullConfigKeyPath(ExtensionConfigKey.AutoAiScanInterval))) {
      aiScanIntervalWatcher.setup(true);
      await updateStatusBar();
    }

    if (e.affectsConfiguration(getFullConfigKeyPath(ExtensionConfigKey.LspBin))) {
      const restart = await vscode.window.showInformationMessage(
        `${EXTENSION_DISPLAY_NAME} binary path changed. Restart LSP server?`,
        'Restart',
        'Later',
      );

      if (restart === 'Restart') {
        disposeScanner();
        await startLspClient();
        executeCommand(Command.RefreshIssues, { trigger: ScanTrigger.Startup });
      }
    }
  });
}

export function activate(context: vscode.ExtensionContext) {
  const logsEnabled = getExtensionConfig(ExtensionConfigKey.LogsEnabled);
  initializeLogger(logsEnabled);
  logger.clear();

  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    logger.info(`${EXTENSION_DISPLAY_NAME} extension activated (no workspace open)`);
    return;
  }

  if (activationKey === workspaceFolder.uri.fsPath) {
    logger.warn('Extension already activated for this workspace, skipping');
    return;
  }

  activationKey = workspaceFolder.uri.fsPath;

  logger.info(`${EXTENSION_DISPLAY_NAME} extension activated`);

  extensionStore.initialize(context);
  setupContextKeys(context);

  const { regularView, aiView, settingsView, treeView, aiTreeView, settingsTreeView, regularViewIcon, aiViewIcon } =
    setupViews(context);

  const statusBarManager = new StatusBarManager();
  const updateStatusBar = async () => statusBarManager.update();
  updateStatusBar().then(() => logger.info('Status bar setup complete'));

  extensionStore.subscribe(StoreKey.ScanMode, () => updateStatusBar());
  extensionStore.subscribe(StoreKey.CompareBranch, () => updateStatusBar());
  extensionStore.subscribe(StoreKey.ScanMode, () => settingsView.refresh());
  extensionStore.subscribe(StoreKey.CompareBranch, () => settingsView.refresh());

  const commandContext: CommandContext = {
    context,
    treeView,
    updateStatusBar,
    getLspClient,
  };

  const commands = registerAllCommands(commandContext, regularView, aiView, settingsView);
  const settingsWatcher = setupSettingsListener(updateStatusBar, () => settingsView.refresh());

  setupWatchers({
    context,
    regularView,
    updateStatusBar,
    updateSettingsView: () => settingsView.refresh(),
  }).then((watchers) => {
    context.subscriptions.push(watchers);
  });

  context.subscriptions.push(
    ...commands,
    settingsWatcher,
    statusBarManager.getDisposable(),
    treeView,
    aiTreeView,
    settingsTreeView,
    regularViewIcon,
    aiViewIcon,
  );

  initializeRunner({ context, regularView, aiView, updateStatusBar });

  setTimeout(() => runStartupSequence(), VSCODE_EXTENSION.delays.extensionStartupSeconds * 1000);
}

export function deactivate() {
  scanIntervalWatcher.dispose();
  aiScanIntervalWatcher.dispose();
  disposeRunner();
  disposeScanner();
}
