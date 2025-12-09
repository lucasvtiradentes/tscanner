import { AiExecutionMode } from 'tscanner-common';
import type * as vscode from 'vscode';
import { loadConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { Command, executeCommand, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import type { ExtensionStateRefs } from '../common/state/extension-state';

let aiScanIntervalTimer: NodeJS.Timeout | null = null;

export async function setupAiScanInterval(
  _context: vscode.ExtensionContext,
  stateRefs: ExtensionStateRefs,
): Promise<void> {
  if (aiScanIntervalTimer) {
    clearInterval(aiScanIntervalTimer);
    aiScanIntervalTimer = null;
  }

  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) return;

  const config = await loadConfig(workspaceFolder.uri.fsPath, stateRefs.currentConfigDirRef.current);

  const aiScanInterval = config?.codeEditor?.aiScanInterval ?? 0;

  if (aiScanInterval <= 0) {
    logger.info('AI auto-scan interval disabled (aiScanInterval = 0)');
    return;
  }

  const intervalMs = aiScanInterval * 1000;
  logger.info(`Setting up AI auto-scan interval: ${aiScanInterval} seconds`);

  aiScanIntervalTimer = setInterval(() => {
    if (stateRefs.isSearchingRef.current) {
      logger.debug('AI auto-scan skipped: search in progress');
      return;
    }
    logger.debug('Running AI auto-scan...');
    executeCommand(Command.FindIssue, { silent: true, aiMode: AiExecutionMode.Only });
  }, intervalMs);
}

export function disposeAiScanInterval(): void {
  if (aiScanIntervalTimer) {
    clearInterval(aiScanIntervalTimer);
    aiScanIntervalTimer = null;
  }
}
