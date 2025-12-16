import { CONFIG_DIR_NAME, CONFIG_FILE_NAME } from 'tscanner-common';
import * as vscode from 'vscode';
import { getConfigPath } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { ScanTrigger } from '../common/types/scan-trigger';
import { runScanSequence } from '../startup/runner';

export function createConfigWatcher(onConfigChange: () => Promise<void>): vscode.FileSystemWatcher {
  const configWatcher = vscode.workspace.createFileSystemWatcher(`**/${CONFIG_DIR_NAME}/${CONFIG_FILE_NAME}`);

  const handleConfigChange = async (uri: vscode.Uri) => {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) return;

    const configDir = extensionStore.get(StoreKey.ConfigDir);
    const expectedPath = getConfigPath(workspaceFolder.uri.fsPath, configDir);

    if (uri.fsPath !== expectedPath) {
      logger.debug(`Ignoring config change in non-active config: ${uri.fsPath}`);
      return;
    }

    const relativePath = vscode.workspace.asRelativePath(uri);
    logger.info(`Config file changed: ${relativePath}`);

    await onConfigChange();
    await runScanSequence(ScanTrigger.ConfigChange);
  };

  configWatcher.onDidChange(handleConfigChange);
  configWatcher.onDidCreate(handleConfigChange);

  return configWatcher;
}
