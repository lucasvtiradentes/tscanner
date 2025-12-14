import { CODE_EDITOR_DEFAULTS, StartupScanMode, VSCODE_EXTENSION } from 'tscanner-common';
import * as vscode from 'vscode';
import { registerAllCommands } from './commands';
import { getAiViewId, getViewId } from './common/constants';
import { getConfigBaseDir, getOrLoadConfig } from './common/lib/config-manager';
import { validateConfigAndNotify } from './common/lib/config-validator';
import { initializeLogger, logger } from './common/lib/logger';
import { checkVersionCompatibility } from './common/lib/version-checker';
import { Command, executeCommand, getCurrentWorkspaceFolder } from './common/lib/vscode-utils';
import { EXTENSION_DISPLAY_NAME } from './common/scripts-constants';
import { ExtensionConfigKey, getExtensionConfig, getFullConfigKeyPath } from './common/state/extension-config';
import type { CommandContext } from './common/state/extension-state';
import { StoreKey, extensionStore } from './common/state/extension-store';
import { ContextKey, WorkspaceStateKey, getWorkspaceState, setContextKey } from './common/state/workspace-state';
import { ScanTrigger } from './common/types/scan-trigger';
import { AiIssuesView, IssuesViewIcon, RegularIssuesView } from './issues-panel';
import { dispose as disposeScanner, getLspClient, startLspClient } from './scanner/client';
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
  treeView: vscode.TreeView<vscode.TreeItem>;
  aiTreeView: vscode.TreeView<vscode.TreeItem>;
  regularViewIcon: IssuesViewIcon;
  aiViewIcon: IssuesViewIcon;
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

  const treeView = vscode.window.createTreeView(getViewId(), { treeDataProvider: regularView });
  const aiTreeView = vscode.window.createTreeView(getAiViewId(), { treeDataProvider: aiView });

  logger.info(`Registered tree views: ${getViewId()}, ${getAiViewId()}`);

  return {
    regularView,
    aiView,
    treeView,
    aiTreeView,
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

async function setupWatchers(
  context: vscode.ExtensionContext,
  regularView: RegularIssuesView,
  aiView: AiIssuesView,
  updateStatusBar: () => Promise<void>,
): Promise<vscode.Disposable> {
  let currentFileWatcher: vscode.FileSystemWatcher | null = null;

  const recreateFileWatcher = async () => {
    if (currentFileWatcher) {
      currentFileWatcher.dispose();
    }
    currentFileWatcher = await createFileWatcher(context, regularView);
  };

  const configWatcher = createConfigWatcher(
    async () => {
      await scanIntervalWatcher.setup();
      await aiScanIntervalWatcher.setup();
      await recreateFileWatcher();
      await updateStatusBar();
    },
    regularView,
    aiView,
  );

  const gitWatcher = await createGitWatcher();

  void recreateFileWatcher();

  const disposables: vscode.Disposable[] = [configWatcher];
  if (gitWatcher) {
    disposables.push(gitWatcher);
    logger.info('Git watcher enabled - will refresh on commits/checkouts');
  } else {
    logger.warn('Git extension not available - commit/checkout detection disabled');
  }

  return vscode.Disposable.from(...disposables);
}

function setupSettingsListener(): vscode.Disposable {
  return vscode.workspace.onDidChangeConfiguration(async (e) => {
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

async function startExtension(regularView: RegularIssuesView, aiView: AiIssuesView): Promise<void> {
  logger.info('Starting LSP client...');
  try {
    await startLspClient();
  } catch (err) {
    return;
  }

  const lspClient = getLspClient();
  if (lspClient) {
    const binaryVersion = lspClient.getServerVersion();
    checkVersionCompatibility(binaryVersion);
  }

  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    logger.warn('No workspace folder, skipping startup scans');
    return;
  }

  const configDir = extensionStore.get(StoreKey.ConfigDir);
  const configBasePath = getConfigBaseDir(workspaceFolder.uri.fsPath, configDir);
  logger.info(`Validating config at base path: ${configBasePath} (configDir store: ${configDir})`);
  const isValid = await validateConfigAndNotify(configBasePath);

  if (!isValid) {
    logger.warn('Config invalid on startup, showing error in views');
    regularView.setError('Fix config errors to scan again');
    aiView.setError('Fix config errors to scan again');
    return;
  }

  logger.info('Config valid on startup, clearing errors');
  regularView.clearError();
  aiView.clearError();

  const config = await getOrLoadConfig(workspaceFolder.uri.fsPath);
  const startupScan = config?.codeEditor?.startupScan ?? (CODE_EDITOR_DEFAULTS.startupScan as StartupScanMode);
  const startupAiScan = config?.codeEditor?.startupAiScan ?? (CODE_EDITOR_DEFAULTS.startupAiScan as StartupScanMode);

  const hasStartupScan = startupScan !== StartupScanMode.Off;
  const hasStartupAiScan = startupAiScan !== StartupScanMode.Off;

  if (hasStartupScan) {
    const useCache = startupScan === StartupScanMode.Cached;
    logger.info(`Running initial scan (mode: ${startupScan}, cache: ${useCache})...`);
    executeCommand(Command.RefreshIssues, {
      trigger: ScanTrigger.Startup,
      useCache,
    });
    logger.info('Waiting for regular scan to complete...');
    await waitForRegularScan();
    logger.info('Regular scan completed');
  } else {
    logger.info('Startup scan disabled by config');
  }

  if (hasStartupAiScan) {
    const useCache = startupAiScan === StartupScanMode.Cached;
    logger.info(`Running initial AI scan (mode: ${startupAiScan}, cache: ${useCache})...`);
    executeCommand(Command.RefreshAiIssues, {
      trigger: ScanTrigger.Startup,
      useCache,
    });
    logger.info('Waiting for AI scan to complete...');
    await waitForAiScan();
    logger.info('AI scan completed');
  } else {
    logger.info('Startup AI scan disabled by config');
  }

  logger.info('Setting up interval watchers');
  await scanIntervalWatcher.setup(hasStartupScan);
  await aiScanIntervalWatcher.setup(hasStartupAiScan);
}

function waitForScan(storeKey: StoreKey.IsSearching | StoreKey.IsAiSearching): Promise<void> {
  return new Promise((resolve) => {
    const check = () => {
      const isScanning = extensionStore.get(storeKey);
      if (!isScanning) {
        resolve();
      } else {
        setTimeout(check, 100);
      }
    };
    setTimeout(check, 100);
  });
}

function waitForRegularScan(): Promise<void> {
  return waitForScan(StoreKey.IsSearching);
}

function waitForAiScan(): Promise<void> {
  return waitForScan(StoreKey.IsAiSearching);
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

  const { regularView, aiView, treeView, regularViewIcon, aiViewIcon } = setupViews(context);

  const statusBarManager = new StatusBarManager();
  const updateStatusBar = async () => statusBarManager.update();
  updateStatusBar().then(() => logger.info('Status bar setup complete'));

  extensionStore.subscribe(StoreKey.ScanMode, () => updateStatusBar());
  extensionStore.subscribe(StoreKey.CompareBranch, () => updateStatusBar());
  extensionStore.subscribe(StoreKey.ConfigDir, () => updateStatusBar());

  const commandContext: CommandContext = {
    context,
    treeView,
    updateStatusBar,
    getLspClient,
  };

  const commands = registerAllCommands(commandContext, regularView, aiView);
  const settingsWatcher = setupSettingsListener();

  setupWatchers(context, regularView, aiView, updateStatusBar).then((watchers) => {
    context.subscriptions.push(watchers);
  });

  context.subscriptions.push(
    ...commands,
    settingsWatcher,
    statusBarManager.getDisposable(),
    regularViewIcon,
    aiViewIcon,
  );

  setTimeout(() => startExtension(regularView, aiView), VSCODE_EXTENSION.delays.extensionStartupSeconds * 1000);
}

export function deactivate() {
  scanIntervalWatcher.dispose();
  aiScanIntervalWatcher.dispose();
  disposeScanner();
}
