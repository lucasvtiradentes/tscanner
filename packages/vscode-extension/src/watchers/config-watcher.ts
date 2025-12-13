import type { AiIssuesView, RegularIssuesView } from 'src/issues-panel';
import { CONFIG_FILE_NAME } from 'tscanner-common';
import * as vscode from 'vscode';
import { loadAndCacheConfig } from '../common/lib/config-manager';
import { validateConfigAndNotify } from '../common/lib/config-validator';
import { logger } from '../common/lib/logger';
import { Command, executeCommand, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { ScanTrigger } from '../common/types/scan-trigger';

export function createConfigWatcher(
  onConfigChange: () => Promise<void>,
  regularView: RegularIssuesView,
  aiView: AiIssuesView,
): vscode.FileSystemWatcher {
  const configWatcher = vscode.workspace.createFileSystemWatcher(`**/${CONFIG_FILE_NAME}`);

  const handleConfigChange = async (uri: vscode.Uri) => {
    const relativePath = vscode.workspace.asRelativePath(uri);
    logger.info(`Config file changed: ${relativePath}, validating and reloading...`);

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (workspaceFolder) {
      const configPath = uri.fsPath.replace(`/${CONFIG_FILE_NAME}`, '');
      const isValid = await validateConfigAndNotify(configPath);

      if (!isValid) {
        regularView.setResults([]);
        aiView.setResults([], true);
        logger.error('Config validation failed, cleared all issues');
        return;
      }

      await loadAndCacheConfig(workspaceFolder.uri.fsPath);
    }

    await onConfigChange();

    executeCommand(Command.RefreshIssues, { silent: true, trigger: ScanTrigger.ConfigChange });
  };

  configWatcher.onDidChange(handleConfigChange);
  configWatcher.onDidCreate(handleConfigChange);

  return configWatcher;
}
