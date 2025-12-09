import { AiExecutionMode, CONFIG_DIR_NAME, ScanMode, hasConfiguredRules } from 'tscanner-common';
import { getConfigDirLabel, loadConfig } from '../../common/lib/config-manager';
import { logger } from '../../common/lib/logger';
import {
  Command,
  ToastKind,
  executeCommand,
  getCurrentWorkspaceFolder,
  registerCommand,
  showToastMessage,
} from '../../common/lib/vscode-utils';
import type { CommandContext } from '../../common/state/extension-state';
import { ContextKey, setContextKey } from '../../common/state/workspace-state';
import type { AiIssuesView } from '../../issues-panel';
import { getLspClient } from '../../scanner/client';
import { scan } from '../../scanner/scan';

export function createRefreshCommand() {
  return registerCommand(Command.Refresh, async () => {
    await executeCommand(Command.HardScan);
  });
}

export function createRefreshAiIssuesCommand(ctx: CommandContext, aiView: AiIssuesView) {
  const { stateRefs } = ctx;
  const { currentScanModeRef, currentCompareBranchRef, currentConfigDirRef } = stateRefs;

  return registerCommand(Command.RefreshAiIssues, async () => {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      showToastMessage(ToastKind.Error, 'No workspace folder open');
      return;
    }

    setContextKey(ContextKey.AiSearching, true);
    aiView.setResults([], true);

    let progressDisposable: { dispose(): void } | null = null;

    try {
      const configDir = currentConfigDirRef.current;
      const config = await loadConfig(workspaceFolder.uri.fsPath, configDir);

      if (!hasConfiguredRules(config)) {
        aiView.setResults([], true);
        showToastMessage(ToastKind.Warning, 'No rules configured for this workspace');
        return;
      }

      const configToPass = configDir ? (config ?? undefined) : undefined;
      if (configDir) {
        logger.info(`[AI Scan] Using config from ${getConfigDirLabel(configDir)}`);
      } else {
        logger.info(`[AI Scan] Using local config from ${CONFIG_DIR_NAME}`);
      }

      logger.info('[AI Scan] Starting AI-only scan...');

      const client = getLspClient();
      if (client) {
        progressDisposable = client.onAiProgress((params) => {
          logger.debug(`[AI Progress] ${params.rule_name}: ${JSON.stringify(params.status)}`);
          aiView.updateProgress(params);
        });
      }

      const startTime = Date.now();
      const branch = currentScanModeRef.current === ScanMode.Branch ? currentCompareBranchRef.current : undefined;
      const results = await scan({ branch, config: configToPass, aiMode: AiExecutionMode.Only });

      const elapsed = Date.now() - startTime;
      logger.info(`[AI Scan] Completed in ${elapsed}ms, found ${results.length} AI issues`);

      aiView.setResults(results);
    } catch (error) {
      logger.error(`[AI Scan] Error: ${error}`);
      showToastMessage(ToastKind.Error, `AI scan failed: ${error}`);
      aiView.clearProgress();
    } finally {
      if (progressDisposable) {
        progressDisposable.dispose();
      }
      setContextKey(ContextKey.AiSearching, false);
      setContextKey(ContextKey.HasAiScanned, true);
    }
  });
}
