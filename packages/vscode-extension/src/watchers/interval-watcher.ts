import type { AiExecutionMode } from 'tscanner-common';
import { loadConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { Command, executeCommand, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../common/state/extension-store';

export enum IntervalConfigKey {
  Scan = 'scanInterval',
  AiScan = 'aiScanInterval',
}

type IntervalConfig = {
  name: string;
  configKey: IntervalConfigKey;
  aiMode: AiExecutionMode;
};

export function createIntervalWatcher(config: IntervalConfig) {
  let timer: NodeJS.Timeout | null = null;

  const setup = async (): Promise<void> => {
    if (timer) {
      clearInterval(timer);
      timer = null;
    }

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) return;

    const configDir = extensionStore.get(StoreKey.ConfigDir);
    const tscannerConfig = await loadConfig(workspaceFolder.uri.fsPath, configDir);
    const intervalSeconds = tscannerConfig?.codeEditor?.[config.configKey] ?? 0;

    if (intervalSeconds <= 0) {
      logger.info(`${config.name} auto-scan disabled (${config.configKey} = 0)`);
      return;
    }

    logger.info(`Setting up ${config.name} auto-scan: ${intervalSeconds}s`);

    timer = setInterval(() => {
      if (extensionStore.get(StoreKey.IsSearching)) {
        logger.debug(`${config.name} auto-scan skipped: search in progress`);
        return;
      }
      logger.debug(`Running ${config.name} auto-scan...`);
      executeCommand(Command.RefreshIssues, { silent: true, aiMode: config.aiMode });
    }, intervalSeconds * 1000);
  };

  const dispose = (): void => {
    if (timer) {
      clearInterval(timer);
      timer = null;
    }
  };

  return { setup, dispose };
}
