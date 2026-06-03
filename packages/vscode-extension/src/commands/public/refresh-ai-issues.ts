import { AiExecutionMode, CONFIG_DIR_NAME, ScanMode, hasConfiguredRules } from 'tscanner-common';
import * as vscode from 'vscode';
import { getOrLoadConfig } from '../../common/lib/config-manager';
import { createLogger, logger } from '../../common/lib/logger';
import { ScanType, withScanErrorHandling } from '../../common/lib/scan-helpers';
import { Command, getCurrentWorkspaceFolder, registerCommand } from '../../common/lib/vscode-utils';
import type { CommandContext } from '../../common/state/extension-state';
import { StoreKey, extensionStore } from '../../common/state/extension-store';
import { ContextKey } from '../../common/state/workspace-state';
import { ScanTrigger, shouldUseCache } from '../../common/types/scan-trigger';
import type { AiIssuesView } from '../../issues-panel';
import { getLspClient } from '../../scanner/client';
import { scan } from '../../scanner/scan';

const aiScanLogger = createLogger('AI Scan');
const aiProgressLogger = createLogger('AI Progress');

export type RefreshAiIssuesParams = {
  trigger?: ScanTrigger;
  useCache?: boolean;
};

export function createRefreshAiIssuesCommand(_ctx: CommandContext, aiView: AiIssuesView) {
  return registerCommand(Command.RefreshAiIssues, async (options?: RefreshAiIssuesParams) => {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      return;
    }

    const previousResults = aiView.getResults();
    const previousAiIssues = previousResults.map((issue) => ({
      rule: issue.rule,
      file: vscode.workspace.asRelativePath(issue.uri, false),
      line: issue.line + 1,
      column: issue.column + 1,
      message: issue.message,
      line_text: issue.text || undefined,
    }));
    aiView.setResults([], true);
    let progressDisposable: { dispose(): void } | null = null;

    await withScanErrorHandling(
      {
        scanType: ScanType.Ai,
        contextKeyOnComplete: ContextKey.HasAiScanned,
        onError: (error) => {
          logger.error(`AI scan failed: ${error}`);
          aiView.clearProgress();
          aiView.setResults(previousResults, true);
        },
        onFinally: () => {
          progressDisposable?.dispose();
        },
      },
      async () => {
        const config = await getOrLoadConfig(workspaceFolder.uri.fsPath);

        if (!hasConfiguredRules(config)) {
          aiView.setResults([], true);
          logger.error('No rules configured for this workspace');
          return;
        }

        aiScanLogger.info(`Using local config from ${CONFIG_DIR_NAME}`);

        aiScanLogger.info('Starting AI-only scan (full scan)...');

        const client = getLspClient();
        if (client) {
          progressDisposable = client.onAiProgress((params) => {
            aiProgressLogger.debug(`${params.rule_name}: ${JSON.stringify(params.status)}`);
            aiView.updateProgress(params);
          });
        }

        const startTime = Date.now();
        const scanMode = extensionStore.get(StoreKey.ScanMode);
        const compareBranch = extensionStore.get(StoreKey.CompareBranch);
        const branch = scanMode === ScanMode.Branch ? compareBranch : undefined;
        const trigger = options?.trigger ?? ScanTrigger.ManualCommand;
        const useCache = options?.useCache ?? shouldUseCache(trigger);
        aiScanLogger.info(`AI scan trigger: ${trigger}, useCache: ${useCache}, noCache flag: ${!useCache}`);
        const results = await scan({
          branch,
          aiMode: AiExecutionMode.Only,
          noCache: !useCache,
          previousAiIssues: previousAiIssues.length > 0 ? previousAiIssues : undefined,
        });

        const elapsed = Date.now() - startTime;
        aiScanLogger.info(`Completed in ${elapsed}ms, found ${results.length} AI issues`);

        aiView.setResults(results);
      },
    );
  });
}
