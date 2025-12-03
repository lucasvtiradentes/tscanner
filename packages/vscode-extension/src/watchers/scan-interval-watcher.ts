import type * as vscode from 'vscode';
import { loadEffectiveConfig } from '../common/lib/config-manager';
import { Command, executeCommand, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import type { ExtensionStateRefs } from '../common/state/extension-state';
import { logger } from '../common/utils/logger';

let scanIntervalTimer: NodeJS.Timeout | null = null;

export async function setupScanInterval(
  context: vscode.ExtensionContext,
  stateRefs: ExtensionStateRefs,
): Promise<void> {
  if (scanIntervalTimer) {
    clearInterval(scanIntervalTimer);
    scanIntervalTimer = null;
  }

  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) return;

  const config = await loadEffectiveConfig(
    context,
    workspaceFolder.uri.fsPath,
    stateRefs.currentCustomConfigDirRef.current,
  );

  const scanIntervalSeconds = config?.codeEditor?.scanIntervalSeconds ?? 0;

  if (scanIntervalSeconds <= 0) {
    logger.info('Auto-scan interval disabled (scanIntervalSeconds = 0)');
    return;
  }

  const intervalMs = scanIntervalSeconds * 1000;
  logger.info(`Setting up auto-scan interval: ${scanIntervalSeconds} seconds`);

  scanIntervalTimer = setInterval(() => {
    if (stateRefs.isSearchingRef.current) {
      logger.debug('Auto-scan skipped: search in progress');
      return;
    }
    logger.debug('Running auto-scan...');
    executeCommand(Command.FindIssue, { silent: true });
  }, intervalMs);
}

export function disposeScanInterval(): void {
  if (scanIntervalTimer) {
    clearInterval(scanIntervalTimer);
    scanIntervalTimer = null;
  }
}
