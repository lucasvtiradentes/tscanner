import * as vscode from 'vscode';
import { CONFIG_FILE_NAME } from '../common/constants';
import { Command, executeCommand } from '../common/lib/vscode-utils';
import { logger } from '../common/utils/logger';

export function createConfigWatcher(onConfigChange: () => Promise<void>): vscode.FileSystemWatcher {
  const configWatcher = vscode.workspace.createFileSystemWatcher(`**/${CONFIG_FILE_NAME}`);

  const handleConfigChange = async (uri: vscode.Uri) => {
    const relativePath = vscode.workspace.asRelativePath(uri);
    logger.info(`Config file changed: ${relativePath}, triggering hard scan...`);

    await onConfigChange();

    executeCommand(Command.HardScan);
  };

  configWatcher.onDidChange(handleConfigChange);
  configWatcher.onDidCreate(handleConfigChange);

  return configWatcher;
}
