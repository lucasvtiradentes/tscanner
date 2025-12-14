import type { AiIssuesView, RegularIssuesView } from 'src/issues-panel';
import { CONFIG_DIR_NAME, CONFIG_FILE_NAME } from 'tscanner-common';
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
      const configBasePath = uri.fsPath.replace(`/${CONFIG_DIR_NAME}/${CONFIG_FILE_NAME}`, '');
      logger.info(`Validating config at base path: ${configBasePath} (from file change: ${uri.fsPath})`);
      const isValid = await validateConfigAndNotify(configBasePath);

      if (!isValid) {
        logger.error('Config validation failed, showing error in views');
        regularView.setError('Fix config errors to scan again');
        aiView.setError('Fix config errors to scan again');
        return;
      }

      logger.info('Config validation passed, clearing errors');
      regularView.clearError();
      aiView.clearError();
      await loadAndCacheConfig(workspaceFolder.uri.fsPath);
    }

    await onConfigChange();

    executeCommand(Command.RefreshIssues, { trigger: ScanTrigger.ConfigChange });
  };

  configWatcher.onDidChange(handleConfigChange);
  configWatcher.onDidCreate(handleConfigChange);

  return configWatcher;
}
