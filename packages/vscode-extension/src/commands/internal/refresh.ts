import { AiExecutionMode, CONFIG_DIR_NAME, ScanMode, hasConfiguredRules } from 'tscanner-common';
import { getConfigState, loadEffectiveConfig } from '../../common/lib/config-manager';
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
import { scanBranch } from '../../scanner/branch-scan';
import { getLspClient } from '../../scanner/client';
import { scanCodebase } from '../../scanner/codebase-scan';

export function createRefreshCommand() {
  return registerCommand(Command.Refresh, async () => {
    await executeCommand(Command.HardScan);
  });
}

export function createRefreshAiIssuesCommand(ctx: CommandContext, aiView: AiIssuesView) {
  const { context, stateRefs } = ctx;
  const { currentScanModeRef, currentCompareBranchRef, currentCustomConfigDirRef } = stateRefs;

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
      const customConfigDir = currentCustomConfigDirRef.current;
      const effectiveConfig = await loadEffectiveConfig(context, workspaceFolder.uri.fsPath, customConfigDir);
      const configState = await getConfigState(context, workspaceFolder.uri.fsPath, customConfigDir);

      if (!hasConfiguredRules(effectiveConfig)) {
        aiView.setResults([], true);
        showToastMessage(ToastKind.Warning, 'No rules configured for this workspace');
        return;
      }

      const configToPass = configState.hasLocal && !configState.hasCustom ? undefined : (effectiveConfig ?? undefined);
      if (configState.hasCustom) {
        logger.info(`[AI Scan] Using custom config from ${customConfigDir}`);
      } else if (configState.hasLocal) {
        logger.info(`[AI Scan] Using local config from ${CONFIG_DIR_NAME}`);
      } else {
        logger.info('[AI Scan] Using global config from extension storage');
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
      const results =
        currentScanModeRef.current === ScanMode.Branch
          ? await scanBranch(currentCompareBranchRef.current, undefined, configToPass, AiExecutionMode.Only)
          : await scanCodebase(undefined, configToPass, AiExecutionMode.Only);

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
