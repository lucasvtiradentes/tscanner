import * as vscode from 'vscode';
import { registerAllCommands } from './commands';
import { getViewId } from './common/constants';
import { loadEffectiveConfig } from './common/lib/config-manager';
import { type CommandDependencies, createExtensionStateRefs } from './common/lib/extension-state';
import { dispose as disposeScanner, getRustClient, scanContent, startLspClient } from './common/lib/scanner';
import {
  Command,
  ContextKey,
  ScanMode,
  WorkspaceStateKey,
  executeCommand,
  getCurrentWorkspaceFolder,
  getWorkspaceState,
  setContextKey,
  setWorkspaceState,
} from './common/lib/vscode-utils';
import { serializeResults } from './common/types';
import { getChangedFiles, getModifiedLineRanges, invalidateCache } from './common/utils/git-helper';
import { getNewIssues } from './common/utils/issue-comparator';
import { logger } from './common/utils/logger';
import { IssuesPanelContent } from './issues-panel/panel-content';
import { IssuesPanelIcon } from './issues-panel/panel-icon';
import { StatusBarManager } from './status-bar/status-bar-manager';

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

function createFileWatcher(
  context: vscode.ExtensionContext,
  panelContent: IssuesPanelContent,
  stateRefs: CommandDependencies['stateRefs'],
  updateBadge: () => void,
): vscode.FileSystemWatcher {
  const updateSingleFile = async (uri: vscode.Uri) => {
    if (stateRefs.isSearchingRef.current) return;

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) return;

    const relativePath = vscode.workspace.asRelativePath(uri);
    logger.debug(`File changed: ${relativePath}`);

    if (stateRefs.currentScanModeRef.current === ScanMode.Branch) {
      invalidateCache();
      const changedFiles = await getChangedFiles(workspaceFolder.uri.fsPath, stateRefs.currentCompareBranchRef.current);
      if (!changedFiles.has(relativePath)) {
        logger.debug(`File not in changed files set, skipping: ${relativePath}`);
        return;
      }
    }

    try {
      logger.debug(`Scanning single file: ${relativePath}`);

      const document = await vscode.workspace.openTextDocument(uri);
      const content = document.getText();
      const config = await loadEffectiveConfig(
        context,
        workspaceFolder.uri.fsPath,
        stateRefs.currentCustomConfigDirRef.current,
      );
      let newResults = await scanContent(uri.fsPath, content, config ?? undefined);

      if (stateRefs.currentScanModeRef.current === ScanMode.Branch) {
        const ranges = await getModifiedLineRanges(
          workspaceFolder.uri.fsPath,
          relativePath,
          stateRefs.currentCompareBranchRef.current,
        );
        const modifiedRanges = new Map([[uri.fsPath, ranges]]);
        newResults = getNewIssues(newResults, modifiedRanges);
        logger.debug(`Filtered ${newResults.length} issues to modified lines only`);
      }

      const currentResults = panelContent.getResults();
      const filteredResults = currentResults.filter((r) => {
        const resultPath = vscode.workspace.asRelativePath(r.uri);
        return resultPath !== relativePath;
      });

      const mergedResults = [...filteredResults, ...newResults];
      logger.debug(
        `Updated results: removed ${currentResults.length - filteredResults.length}, added ${newResults.length}, total ${mergedResults.length}`,
      );

      panelContent.setResults(mergedResults);
      setWorkspaceState(context, WorkspaceStateKey.CachedResults, serializeResults(mergedResults));
      updateBadge();
    } catch (error) {
      logger.error(`Failed to update single file: ${error}`);
    }
  };

  const handleFileDelete = async (uri: vscode.Uri) => {
    const relativePath = vscode.workspace.asRelativePath(uri);
    logger.debug(`File deleted: ${relativePath}`);

    if (stateRefs.currentScanModeRef.current === ScanMode.Branch) {
      invalidateCache();
    }

    const currentResults = panelContent.getResults();
    const filteredResults = currentResults.filter((r) => {
      const resultPath = vscode.workspace.asRelativePath(r.uri);
      return resultPath !== relativePath;
    });

    if (filteredResults.length !== currentResults.length) {
      logger.debug(`Removed ${currentResults.length - filteredResults.length} issues from deleted file`);
      panelContent.setResults(filteredResults);
      setWorkspaceState(context, WorkspaceStateKey.CachedResults, serializeResults(filteredResults));
      updateBadge();
    }
  };

  const fileWatcher = vscode.workspace.createFileSystemWatcher('**/*.{ts,tsx,js,jsx}');
  fileWatcher.onDidChange(updateSingleFile);
  fileWatcher.onDidCreate(updateSingleFile);
  fileWatcher.onDidDelete(handleFileDelete);

  return fileWatcher;
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

  const deps: CommandDependencies = {
    context,
    treeView,
    stateRefs,
    updateBadge,
    updateStatusBar,
    getRustClient,
  };

  const commands = registerAllCommands(deps, panelContent);
  const fileWatcher = createFileWatcher(context, panelContent, stateRefs, updateBadge);

  context.subscriptions.push(...commands, fileWatcher, statusBarManager.getDisposable());

  setTimeout(async () => {
    logger.info('Starting LSP client...');
    await startLspClient();
    logger.info('Running initial scan after 2s delay...');
    executeCommand(Command.FindIssue, { silent: true });
  }, 2000);
}

export function deactivate() {
  disposeScanner();
}
