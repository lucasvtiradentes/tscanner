import { AiExecutionMode } from 'tscanner-common';
import type * as vscode from 'vscode';
import { loadConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { Command, executeCommand, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import type { ExtensionStateRefs } from '../common/state/extension-state';

let scanIntervalTimer: NodeJS.Timeout | null = null;

export async function setupScanInterval(
  _context: vscode.ExtensionContext,
  stateRefs: ExtensionStateRefs,
): Promise<void> {
  if (scanIntervalTimer) {
    clearInterval(scanIntervalTimer);
    scanIntervalTimer = null;
  }

  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) return;

  const config = await loadConfig(workspaceFolder.uri.fsPath, stateRefs.currentConfigDirRef.current);

  const scanInterval = config?.codeEditor?.scanInterval ?? 0;

  if (scanInterval <= 0) {
    logger.info('Auto-scan interval disabled (scanInterval = 0)');
    return;
  }

  const intervalMs = scanInterval * 1000;
  logger.info(`Setting up auto-scan interval: ${scanInterval} seconds`);

  scanIntervalTimer = setInterval(() => {
    if (stateRefs.isSearchingRef.current) {
      logger.debug('Auto-scan skipped: search in progress');
      return;
    }
    logger.debug('Running auto-scan...');
    executeCommand(Command.FindIssue, { silent: true, aiMode: AiExecutionMode.Ignore });
  }, intervalMs);
}

export function disposeScanInterval(): void {
  if (scanIntervalTimer) {
    clearInterval(scanIntervalTimer);
    scanIntervalTimer = null;
  }
}
