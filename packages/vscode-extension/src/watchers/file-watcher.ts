import { GitHelper, ScanMode, type TscannerConfig } from 'tscanner-common';
import * as vscode from 'vscode';
import { loadEffectiveConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { VscodeGit } from '../common/lib/vscode-git';
import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import type { ExtensionStateRefs } from '../common/state/extension-state';
import { WorkspaceStateKey, setWorkspaceState } from '../common/state/workspace-state';
import { type IssueResult, type ModifiedLineRange, serializeResults } from '../common/types';
import type { IssuesPanelContent } from '../issues-panel/panel-content';
import { scanContent } from '../scanner/content-scan';

function normalizePattern(pattern: string): string {
  if (pattern.startsWith('**/') || pattern.startsWith('{')) {
    return pattern;
  }
  return `**/${pattern}`;
}

function isLineInRanges(line: number, ranges: ModifiedLineRange[]): boolean {
  return ranges.some((range) => {
    const endLine = range.startLine + range.lineCount - 1;
    return line >= range.startLine && line <= endLine;
  });
}

function getNewIssues(
  currentIssues: IssueResult[],
  modifiedRangesByFile: Map<string, ModifiedLineRange[]>,
): IssueResult[] {
  return currentIssues.filter((issue) => {
    const ranges = modifiedRangesByFile.get(issue.uri.fsPath);
    if (!ranges || ranges.length === 0) {
      return true;
    }
    return isLineInRanges(issue.line + 1, ranges);
  });
}

function buildWatchPattern(config: TscannerConfig | null): string {
  const patterns = new Set<string>();

  if (config?.files?.include) {
    for (const pattern of config.files.include) {
      patterns.add(normalizePattern(pattern));
    }
  }

  if (config?.customRules) {
    for (const rule of Object.values(config.customRules)) {
      if (rule.include) {
        for (const pattern of rule.include) {
          patterns.add(normalizePattern(pattern));
        }
      }
    }
  }

  if (patterns.size === 0) {
    return '**/*.{ts,tsx,js,jsx,mjs,cjs}';
  }

  const uniquePatterns = [...patterns];
  if (uniquePatterns.length === 1) {
    return uniquePatterns[0];
  }

  return `{${uniquePatterns.join(',')}}`;
}

export async function createFileWatcher(
  context: vscode.ExtensionContext,
  panelContent: IssuesPanelContent,
  stateRefs: ExtensionStateRefs,
  updateBadge: () => void,
): Promise<vscode.FileSystemWatcher> {
  const updateSingleFile = async (uri: vscode.Uri) => {
    if (stateRefs.isSearchingRef.current) return;

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) return;

    const relativePath = vscode.workspace.asRelativePath(uri);
    logger.debug(`File changed: ${relativePath}`);

    if (stateRefs.currentScanModeRef.current === ScanMode.Branch) {
      const changedFiles = await VscodeGit.getChangedFiles(
        workspaceFolder.uri.fsPath,
        stateRefs.currentCompareBranchRef.current,
      );
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
      const scanResult = await scanContent(uri.fsPath, content, config ?? undefined);
      let newResults = scanResult.issues;

      if (stateRefs.currentScanModeRef.current === ScanMode.Branch) {
        const ranges = await GitHelper.getModifiedLineRanges(
          workspaceFolder.uri.fsPath,
          relativePath,
          stateRefs.currentCompareBranchRef.current,
        );
        const modifiedRanges = new Map([[uri.fsPath, ranges]]);
        newResults = getNewIssues(newResults, modifiedRanges);
        logger.debug(`Filtered ${newResults.length} issues to modified lines only`);
      }

      const currentResults = panelContent.getResults();

      const relatedFilesSet = new Set<string>(scanResult.relatedFiles.map((f) => vscode.workspace.asRelativePath(f)));

      const affectedRules = new Set<string>();
      for (const r of newResults) {
        const rPath = vscode.workspace.asRelativePath(r.uri);
        if (rPath !== relativePath) {
          affectedRules.add(r.rule);
        }
      }

      const filteredResults = currentResults.filter((r) => {
        const resultPath = vscode.workspace.asRelativePath(r.uri);
        if (resultPath === relativePath) {
          return false;
        }
        if (relatedFilesSet.has(resultPath)) {
          return false;
        }
        if (affectedRules.has(r.rule)) {
          return false;
        }
        return true;
      });

      const mergedResults = [...filteredResults, ...newResults];
      logger.debug(
        `Updated results: removed ${currentResults.length - filteredResults.length}, added ${newResults.length}, related files: ${scanResult.relatedFiles.length}, total ${mergedResults.length}`,
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

  const workspaceFolder = getCurrentWorkspaceFolder();
  let watchPattern = '**/*.{ts,tsx,js,jsx,mjs,cjs}';

  if (workspaceFolder) {
    try {
      const config = await loadEffectiveConfig(
        context,
        workspaceFolder.uri.fsPath,
        stateRefs.currentCustomConfigDirRef.current,
      );
      watchPattern = buildWatchPattern(config);
      logger.debug(`File watcher pattern: ${watchPattern}`);
    } catch {}
  }

  const watcher = vscode.workspace.createFileSystemWatcher(watchPattern);
  watcher.onDidChange(updateSingleFile);
  watcher.onDidCreate(updateSingleFile);
  watcher.onDidDelete(handleFileDelete);

  return watcher;
}
