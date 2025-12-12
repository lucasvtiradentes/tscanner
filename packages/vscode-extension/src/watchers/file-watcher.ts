import * as vscode from 'vscode';
import { getOrLoadConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import type { RegularIssuesView } from '../issues-panel';
import { createFileChangeHandler, createFileDeleteHandler } from './file-change-handler';
import { buildWatchPattern } from './watch-pattern-builder';

export async function createFileWatcher(
  context: vscode.ExtensionContext,
  regularView: RegularIssuesView,
): Promise<vscode.FileSystemWatcher | null> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    return null;
  }

  let watchPattern: string | null = null;

  try {
    const config = await getOrLoadConfig(workspaceFolder.uri.fsPath);
    watchPattern = buildWatchPattern(config);
  } catch {
    logger.debug('Failed to load config for file watcher');
  }

  if (!watchPattern) {
    logger.debug('No watch pattern configured, file watcher disabled');
    return null;
  }

  logger.debug(`File watcher pattern: ${watchPattern}`);

  const deps = { context, regularView };
  const handleFileChange = createFileChangeHandler(deps);
  const handleFileDelete = createFileDeleteHandler(deps);

  const watcher = vscode.workspace.createFileSystemWatcher(watchPattern);
  watcher.onDidChange(handleFileChange);
  watcher.onDidCreate(handleFileChange);
  watcher.onDidDelete(handleFileDelete);

  return watcher;
}
