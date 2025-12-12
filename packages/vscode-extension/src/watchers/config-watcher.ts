import { CONFIG_FILE_NAME } from 'tscanner-common';
import * as vscode from 'vscode';
import { loadAndCacheConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { Command, executeCommand, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';

export function createConfigWatcher(onConfigChange: () => Promise<void>): vscode.FileSystemWatcher {
  const configWatcher = vscode.workspace.createFileSystemWatcher(`**/${CONFIG_FILE_NAME}`);

  const handleConfigChange = async (uri: vscode.Uri) => {
    const relativePath = vscode.workspace.asRelativePath(uri);
    logger.info(`Config file changed: ${relativePath}, triggering scan...`);

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (workspaceFolder) {
      await loadAndCacheConfig(workspaceFolder.uri.fsPath);
    }

    await onConfigChange();

    executeCommand(Command.RefreshIssues, { silent: true });
  };

  configWatcher.onDidChange(handleConfigChange);
  configWatcher.onDidCreate(handleConfigChange);

  return configWatcher;
}
