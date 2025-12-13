import { AiExecutionMode, CONFIG_DIR_NAME, ScanMode, hasConfiguredRules } from 'tscanner-common';
import { getConfigDirLabel, getOrLoadConfig } from '../../common/lib/config-manager';
import { createLogger } from '../../common/lib/logger';
import { ScanType, withScanErrorHandling } from '../../common/lib/scan-helpers';
import {
  Command,
  ToastKind,
  getCurrentWorkspaceFolder,
  registerCommand,
  showToastMessage,
} from '../../common/lib/vscode-utils';
import type { CommandContext } from '../../common/state/extension-state';
import { StoreKey, extensionStore } from '../../common/state/extension-store';
import { ContextKey } from '../../common/state/workspace-state';
import type { AiIssuesView } from '../../issues-panel';
import { getLspClient } from '../../scanner/client';
import { scan } from '../../scanner/scan';

const aiScanLogger = createLogger('AI Scan');
const aiProgressLogger = createLogger('AI Progress');

export function createRefreshAiIssuesCommand(_ctx: CommandContext, aiView: AiIssuesView) {
  return registerCommand(Command.RefreshAiIssues, async () => {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      showToastMessage(ToastKind.Error, 'No workspace folder open');
      return;
    }

    aiView.setResults([], true);
    let progressDisposable: { dispose(): void } | null = null;

    await withScanErrorHandling(
      {
        scanType: ScanType.Ai,
        contextKeyOnComplete: ContextKey.HasAiScanned,
        onError: (error) => {
          showToastMessage(ToastKind.Error, `AI scan failed: ${error}`);
          aiView.clearProgress();
        },
        onFinally: () => {
          progressDisposable?.dispose();
        },
      },
      async () => {
        const configDir = extensionStore.get(StoreKey.ConfigDir);
        const config = await getOrLoadConfig(workspaceFolder.uri.fsPath);

        if (!hasConfiguredRules(config)) {
          aiView.setResults([], true);
          showToastMessage(ToastKind.Warning, 'No rules configured for this workspace');
          return;
        }

        const configToPass = configDir ? (config ?? undefined) : undefined;
        if (configDir) {
          aiScanLogger.info(`Using config from ${getConfigDirLabel(configDir)}`);
        } else {
          aiScanLogger.info(`Using local config from ${CONFIG_DIR_NAME}`);
        }

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
        const results = await scan({
          branch,
          config: configToPass,
          configDir: configDir ?? undefined,
          aiMode: AiExecutionMode.Only,
          noCache: true,
        });

        const elapsed = Date.now() - startTime;
        aiScanLogger.info(`Completed in ${elapsed}ms, found ${results.length} AI issues`);

        aiView.setResults(results);
      },
    );
  });
}
