import type { AiExecutionMode } from 'tscanner-common';
import { logger } from '../common/lib/logger';
import { type Command, executeCommand, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { ExtensionConfigKey, getExtensionConfig } from '../common/state/extension-config';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { ScanTrigger } from '../common/types/scan-trigger';

export const IntervalConfigKey = {
  Scan: ExtensionConfigKey.AutoScanInterval,
  AiScan: ExtensionConfigKey.AutoAiScanInterval,
} as const;

export type IntervalConfigKey = (typeof IntervalConfigKey)[keyof typeof IntervalConfigKey];

type IntervalConfig = {
  name: string;
  configKey: IntervalConfigKey;
  command: Command;
  aiMode?: AiExecutionMode;
};

export function createIntervalWatcher(config: IntervalConfig) {
  let timer: NodeJS.Timeout | null = null;

  const setup = async (skipFirstRun = false): Promise<void> => {
    if (timer) {
      clearInterval(timer);
      timer = null;
    }

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) return;

    const intervalSeconds = getExtensionConfig(config.configKey);

    if (intervalSeconds <= 0) {
      logger.info(`${config.name} auto-scan disabled (${config.configKey} = 0)`);
      return;
    }

    logger.info(`Setting up ${config.name} auto-scan: ${intervalSeconds}s (skipFirstRun: ${skipFirstRun})`);

    if (!skipFirstRun) {
      if (extensionStore.get(StoreKey.IsSearching)) {
        logger.debug(`${config.name} initial auto-scan skipped: search in progress`);
      } else {
        logger.debug(`Running ${config.name} initial auto-scan...`);
        executeCommand(config.command, { aiMode: config.aiMode, trigger: ScanTrigger.Interval });
      }
    }

    timer = setInterval(() => {
      if (extensionStore.get(StoreKey.IsSearching)) {
        logger.debug(`${config.name} auto-scan skipped: search in progress`);
        return;
      }
      logger.debug(`Running ${config.name} auto-scan...`);
      executeCommand(config.command, { aiMode: config.aiMode, trigger: ScanTrigger.Interval });
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
