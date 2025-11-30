import { ScanMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { loadEffectiveConfig } from '../common/lib/config-manager';
import type { ExtensionStateRefs } from '../common/lib/extension-state';
import { scanContent } from '../common/lib/scanner';
import { WorkspaceStateKey, getCurrentWorkspaceFolder, setWorkspaceState } from '../common/lib/vscode-utils';
import { serializeResults } from '../common/types';
import { getChangedFiles, getModifiedLineRanges, invalidateCache } from '../common/utils/git-helper';
import { getNewIssues } from '../common/utils/issue-comparator';
import { logger } from '../common/utils/logger';
import type { IssuesPanelContent } from '../issues-panel/panel-content';

export function createFileWatcher(
  context: vscode.ExtensionContext,
  panelContent: IssuesPanelContent,
  stateRefs: ExtensionStateRefs,
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
