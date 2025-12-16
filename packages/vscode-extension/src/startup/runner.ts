import { CODE_EDITOR_DEFAULTS, StartupScanMode } from 'tscanner-common';
import type * as vscode from 'vscode';
import { getConfigBaseDir, getOrLoadConfig, hasConfig } from '../common/lib/config-manager';
import { validateConfigAndNotify } from '../common/lib/config-validator';
import { logger } from '../common/lib/logger';
import { checkVersionCompatibility } from '../common/lib/version-checker';
import { Command, executeCommand, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { ScanTrigger } from '../common/types/scan-trigger';
import type { AiIssuesView, RegularIssuesView } from '../issues-panel';
import { getLspClient, startLspClient } from '../scanner/client';
import { aiScanIntervalWatcher, scanIntervalWatcher } from '../watchers';

export type StartupRunnerContext = {
  context: vscode.ExtensionContext;
  regularView: RegularIssuesView;
  aiView: AiIssuesView;
  updateStatusBar: () => Promise<void>;
};

let runnerContext: StartupRunnerContext | null = null;
let configPollInterval: ReturnType<typeof setInterval> | null = null;

export function initializeRunner(ctx: StartupRunnerContext): void {
  runnerContext = ctx;
}

export async function runStartupSequence(): Promise<void> {
  if (!runnerContext) {
    logger.error('Runner context not initialized');
    return;
  }

  const { context } = runnerContext;

  logger.info('Starting LSP client...');
  try {
    await startLspClient();
  } catch (err) {
    logger.error(`Failed to start LSP client: ${err}`);
    return;
  }

  const lspClient = getLspClient();
  logger.info(`LSP client available: ${!!lspClient}`);
  if (lspClient) {
    logger.info('Getting server version...');
    const binaryVersion = await lspClient.getServerVersion();
    logger.info(`Server version retrieved: ${binaryVersion}`);
    checkVersionCompatibility(binaryVersion, context);
  }

  await runScanSequence(ScanTrigger.Startup);
}

export async function runScanSequence(trigger: ScanTrigger): Promise<void> {
  if (!runnerContext) {
    logger.error('Runner context not initialized');
    return;
  }

  const { regularView, aiView, updateStatusBar } = runnerContext;

  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    logger.warn('No workspace folder, skipping scans');
    return;
  }

  const configDir = extensionStore.get(StoreKey.ConfigDir);
  const configBasePath = getConfigBaseDir(workspaceFolder.uri.fsPath, configDir);

  const configExists = await hasConfig(workspaceFolder.uri.fsPath, configDir);
  if (!configExists) {
    logger.info('No config found, starting config poll...');
    startConfigPoll();
    return;
  }

  stopConfigPoll();

  logger.info(`Validating config at base path: ${configBasePath} (trigger: ${trigger})`);
  const isValid = await validateConfigAndNotify(configBasePath);

  if (!isValid) {
    logger.warn('Config invalid, showing error in views');
    regularView.setError('Fix config errors to scan again');
    aiView.setError('Fix config errors to scan again');
    return;
  }

  logger.info('Config valid, clearing errors');
  regularView.clearError();
  aiView.clearError();

  await updateStatusBar();

  const config = await getOrLoadConfig(workspaceFolder.uri.fsPath);
  const startupScan = config?.codeEditor?.startupScan ?? (CODE_EDITOR_DEFAULTS.startupScan as StartupScanMode);
  const startupAiScan = config?.codeEditor?.startupAiScan ?? (CODE_EDITOR_DEFAULTS.startupAiScan as StartupScanMode);

  const hasStartupScan = startupScan !== StartupScanMode.Off;
  const hasStartupAiScan = startupAiScan !== StartupScanMode.Off;

  if (hasStartupScan) {
    const useCache = trigger === ScanTrigger.Startup && startupScan === StartupScanMode.Cached;
    logger.info(`Running scan (trigger: ${trigger}, cache: ${useCache})...`);
    executeCommand(Command.RefreshIssues, { trigger, useCache });
    await waitForScan(StoreKey.IsSearching);
    logger.info('Regular scan completed');
  }

  if (hasStartupAiScan) {
    const useCache = trigger === ScanTrigger.Startup && startupAiScan === StartupScanMode.Cached;
    logger.info(`Running AI scan (trigger: ${trigger}, cache: ${useCache})...`);
    executeCommand(Command.RefreshAiIssues, { trigger, useCache });
    await waitForScan(StoreKey.IsAiSearching);
    logger.info('AI scan completed');
  }

  logger.info('Setting up interval watchers');
  await scanIntervalWatcher.setup(hasStartupScan);
  await aiScanIntervalWatcher.setup(hasStartupAiScan);
}

function startConfigPoll(): void {
  if (configPollInterval) {
    return;
  }

  const POLL_INTERVAL_MS = 2000;

  logger.info(`Starting config poll (interval: ${POLL_INTERVAL_MS}ms)`);

  configPollInterval = setInterval(async () => {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      return;
    }

    const configDir = extensionStore.get(StoreKey.ConfigDir);
    const configExists = await hasConfig(workspaceFolder.uri.fsPath, configDir);

    if (configExists) {
      logger.info('Config detected by poll, running scan sequence...');
      stopConfigPoll();

      if (!getLspClient()) {
        try {
          await startLspClient();
        } catch (err) {
          logger.error(`Failed to start LSP client: ${err}`);
          return;
        }
      }

      await runScanSequence(ScanTrigger.ConfigChange);
    }
  }, POLL_INTERVAL_MS);
}

function stopConfigPoll(): void {
  if (configPollInterval) {
    logger.info('Stopping config poll');
    clearInterval(configPollInterval);
    configPollInterval = null;
  }
}

function waitForScan(storeKey: StoreKey.IsSearching | StoreKey.IsAiSearching): Promise<void> {
  return new Promise((resolve) => {
    const check = () => {
      const isScanning = extensionStore.get(storeKey);
      if (!isScanning) {
        resolve();
      } else {
        setTimeout(check, 100);
      }
    };
    setTimeout(check, 100);
  });
}

export function disposeRunner(): void {
  stopConfigPoll();
  runnerContext = null;
}
