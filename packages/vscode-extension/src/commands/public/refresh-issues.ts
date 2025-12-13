import {
  type AiExecutionMode,
  CONFIG_DIR_NAME,
  GitHelper,
  ScanMode,
  VSCODE_EXTENSION,
  ViewMode,
  hasConfiguredRules,
} from 'tscanner-common';
import { getConfigDirLabel, loadAndCacheConfig } from '../../common/lib/config-manager';
import { logger } from '../../common/lib/logger';
import { ScanType, withScanErrorHandling } from '../../common/lib/scan-helpers';
import {
  Command,
  ToastKind,
  executeCommand,
  getCurrentWorkspaceFolder,
  registerCommand,
  showToastMessage,
} from '../../common/lib/vscode-utils';
import type { CommandContext } from '../../common/state/extension-state';
import { StoreKey, extensionStore } from '../../common/state/extension-store';
import { ContextKey, WorkspaceStateKey, setWorkspaceState } from '../../common/state/workspace-state';
import { serializeResults } from '../../common/types';
import { ScanTrigger, shouldUseCache } from '../../common/types/scan-trigger';
import type { RegularIssuesView } from '../../issues-panel';
import { scan } from '../../scanner/scan';
import { resetIssueIndex } from './issue-navigation';

export function createRefreshIssuesCommand(ctx: CommandContext, regularView: RegularIssuesView) {
  const { context, treeView, updateStatusBar } = ctx;

  return registerCommand(
    Command.RefreshIssues,
    async (options?: { silent?: boolean; aiMode?: AiExecutionMode; trigger?: ScanTrigger }) => {
      if (extensionStore.get(StoreKey.IsSearching)) {
        if (!options?.silent) {
          showToastMessage(ToastKind.Warning, 'Search already in progress');
        }
        return;
      }

      const workspaceFolder = getCurrentWorkspaceFolder();
      if (!workspaceFolder) {
        if (!options?.silent) {
          showToastMessage(ToastKind.Error, 'No workspace folder open');
        }
        return;
      }

      const configDir = extensionStore.get(StoreKey.ConfigDir);
      const config = await loadAndCacheConfig(workspaceFolder.uri.fsPath);

      if (!hasConfiguredRules(config)) {
        regularView.setResults([]);
        if (!options?.silent) {
          showToastMessage(ToastKind.Warning, 'No rules configured. Run "tscanner init" to create config.');
        }
        return;
      }

      const configToPass = configDir ? (config ?? undefined) : undefined;
      if (configDir) {
        logger.info(`Using config from ${getConfigDirLabel(configDir)}`);
      } else {
        logger.info(`Using local config from ${CONFIG_DIR_NAME}`);
      }

      const scanMode = extensionStore.get(StoreKey.ScanMode);
      const compareBranch = extensionStore.get(StoreKey.CompareBranch);

      if (scanMode === ScanMode.Branch) {
        const branchExistsCheck = await GitHelper.branchExists(workspaceFolder.uri.fsPath, compareBranch);
        if (!branchExistsCheck) {
          logger.warn(`Branch does not exist: ${compareBranch}`);
          const action = await showToastMessage(
            ToastKind.Error,
            `Branch '${compareBranch}' does not exist in this repository`,
            'Change Branch',
            'Switch to Workspace Mode',
          );

          if (action === 'Change Branch') {
            await executeCommand(Command.OpenSettingsMenu);
          } else if (action === 'Switch to Workspace Mode') {
            extensionStore.set(StoreKey.ScanMode, ScanMode.Codebase);
            await updateStatusBar();
            await executeCommand(Command.RefreshIssues, { silent: true });
          }
          return;
        }
      }

      regularView.setResults([]);
      logger.info(`Starting scan in ${scanMode} mode`);

      await withScanErrorHandling(
        {
          scanType: ScanType.Regular,
          contextKeyOnComplete: ContextKey.HasScanned,
        },
        async () => {
          const startTime = Date.now();
          const branch = scanMode === ScanMode.Branch ? compareBranch : undefined;
          const staged = scanMode === ScanMode.Uncommitted ? true : undefined;
          const trigger = options?.trigger ?? ScanTrigger.ManualCommand;
          const useCache = shouldUseCache(trigger);
          logger.info(`Scan trigger: ${trigger}, useCache: ${useCache}, noCache flag: ${!useCache}`);

          const results = await scan({
            branch,
            staged,
            config: configToPass,
            configDir: configDir ?? undefined,
            aiMode: options?.aiMode,
            noCache: !useCache,
          });

          const elapsed = Date.now() - startTime;
          logger.info(`Search completed in ${elapsed}ms, found ${results.length} results`);

          resetIssueIndex();
          regularView.setResults(results);
          setWorkspaceState(context, WorkspaceStateKey.CachedResults, serializeResults(results));

          if (regularView.viewMode === ViewMode.Tree) {
            setTimeout(() => {
              const folders = regularView.getAllFolderItems();
              for (const folder of folders) {
                treeView.reveal(folder, { expand: true, select: false, focus: false });
              }
            }, VSCODE_EXTENSION.delays.treeRevealSeconds * 1000);
          }
        },
      );
    },
  );
}
