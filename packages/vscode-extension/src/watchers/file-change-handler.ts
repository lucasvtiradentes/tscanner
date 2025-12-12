import { GitHelper, ScanMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCachedConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { VscodeGit } from '../common/lib/vscode-git';
import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { WorkspaceStateKey, setWorkspaceState } from '../common/state/workspace-state';
import { serializeResults } from '../common/types';
import type { RegularIssuesView } from '../issues-panel';
import { scanContent } from '../scanner/content-scan';
import { filterIssuesToModifiedLines } from './modified-lines-filter';

type FileChangeHandlerDeps = {
  context: vscode.ExtensionContext;
  regularView: RegularIssuesView;
};

function clearStaleIssuesForFile(deps: FileChangeHandlerDeps, relativePath: string, reason: string): void {
  const { context, regularView } = deps;
  const currentResults = regularView.getResults();
  const filteredResults = currentResults.filter((r) => {
    return vscode.workspace.asRelativePath(r.uri) !== relativePath;
  });

  if (filteredResults.length !== currentResults.length) {
    const removedCount = currentResults.length - filteredResults.length;
    logger.debug(`Cleared ${removedCount} stale issues for ${reason}: ${relativePath}`);
    regularView.setResults(filteredResults);
    setWorkspaceState(context, WorkspaceStateKey.CachedResults, serializeResults(filteredResults));
  }
}

export function createFileChangeHandler(deps: FileChangeHandlerDeps) {
  const { context, regularView } = deps;

  return async (uri: vscode.Uri) => {
    if (extensionStore.get(StoreKey.IsSearching)) return;

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) return;

    const relativePath = vscode.workspace.asRelativePath(uri);
    logger.debug(`File changed: ${relativePath}`);

    const scanMode = extensionStore.get(StoreKey.ScanMode);
    const compareBranch = extensionStore.get(StoreKey.CompareBranch);

    if (scanMode === ScanMode.Branch) {
      const changedFiles = await VscodeGit.getChangedFiles(workspaceFolder.uri.fsPath, compareBranch);
      if (!changedFiles.has(relativePath)) {
        logger.debug(`File not in changed files set, skipping: ${relativePath}`);
        clearStaleIssuesForFile(deps, relativePath, 'reverted file');
        return;
      }
    }

    try {
      const scanModeLabel = scanMode === ScanMode.Branch ? 'branch' : 'codebase';
      logger.debug(`Scanning single file (${scanModeLabel} mode): ${relativePath}`);

      const document = await vscode.workspace.openTextDocument(uri);
      const content = document.getText();
      const configDir = extensionStore.get(StoreKey.ConfigDir);
      const config = getCachedConfig();
      const scanResult = await scanContent(uri.fsPath, content, config ?? undefined, configDir ?? undefined);
      let newResults = scanResult.issues;

      if (scanMode === ScanMode.Branch) {
        const ranges = await GitHelper.getModifiedLineRanges(workspaceFolder.uri.fsPath, relativePath, compareBranch);
        const modifiedRanges = new Map([[uri.fsPath, ranges]]);
        newResults = filterIssuesToModifiedLines(newResults, modifiedRanges);
        logger.debug(`Filtered ${newResults.length} issues to modified lines only`);
      }

      const currentResults = regularView.getResults();
      const relatedFilesSet = new Set<string>(scanResult.relatedFiles.map((f) => vscode.workspace.asRelativePath(f)));

      const affectedRules = new Set<string>();
      for (const r of newResults) {
        const rPath = vscode.workspace.asRelativePath(r.uri);
        if (rPath !== relativePath) {
          affectedRules.add(r.rule);
        }
      }

      const filteredResults = currentResults.filter((result) => {
        const resultPath = vscode.workspace.asRelativePath(result.uri);
        if (resultPath === relativePath) return false;
        if (relatedFilesSet.has(resultPath)) return false;
        if (affectedRules.has(result.rule)) return false;
        return true;
      });

      const mergedResults = [...filteredResults, ...newResults];
      logger.debug(
        `Updated results: removed ${currentResults.length - filteredResults.length}, added ${newResults.length}, related files: ${scanResult.relatedFiles.length}, total ${mergedResults.length}`,
      );

      regularView.setResults(mergedResults);
      setWorkspaceState(context, WorkspaceStateKey.CachedResults, serializeResults(mergedResults));
    } catch (error) {
      logger.error(`Failed to update single file: ${error}`);
    }
  };
}

export function createFileDeleteHandler(deps: FileChangeHandlerDeps) {
  const { context, regularView } = deps;

  return (uri: vscode.Uri) => {
    const relativePath = vscode.workspace.asRelativePath(uri);
    logger.debug(`File deleted: ${relativePath}`);

    const currentResults = regularView.getResults();
    const filteredResults = currentResults.filter((r) => {
      const resultPath = vscode.workspace.asRelativePath(r.uri);
      return resultPath !== relativePath;
    });

    if (filteredResults.length !== currentResults.length) {
      logger.debug(`Removed ${currentResults.length - filteredResults.length} issues from deleted file`);
      regularView.setResults(filteredResults);
      setWorkspaceState(context, WorkspaceStateKey.CachedResults, serializeResults(filteredResults));
    }
  };
}
