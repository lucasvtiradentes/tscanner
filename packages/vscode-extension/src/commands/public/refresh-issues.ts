import {
  type AiExecutionMode,
  CONFIG_DIR_NAME,
  GitHelper,
  ScanMode,
  VSCODE_EXTENSION,
  ViewMode,
  hasConfiguredRules,
} from 'tscanner-common';
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
import { StoreKey, extensionStore } from '../../common/state/extension-store';
import { ContextKey, WorkspaceStateKey, setContextKey, setWorkspaceState } from '../../common/state/workspace-state';
import { serializeResults } from '../../common/types';
import type { RegularIssuesView } from '../../issues-panel';
import { scan } from '../../scanner/scan';
import { resetIssueIndex } from './issue-navigation';

export function createRefreshIssuesCommand(ctx: CommandContext, regularView: RegularIssuesView) {
  const { context, treeView, updateStatusBar } = ctx;

  return registerCommand(Command.RefreshIssues, async (options?: { silent?: boolean; aiMode?: AiExecutionMode }) => {
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
    const config = await loadConfig(workspaceFolder.uri.fsPath, configDir);

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
    const scanUseCache = config?.codeEditor?.scanUseCache ?? true;

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

    extensionStore.set(StoreKey.IsSearching, true);
    regularView.setResults([]);

    logger.info(`Starting scan in ${scanMode} mode`);

    try {
      const startTime = Date.now();
      const branch = scanMode === ScanMode.Branch ? compareBranch : undefined;
      const results = await scan({
        branch,
        config: configToPass,
        configDir: configDir ?? undefined,
        aiMode: options?.aiMode,
        noCache: !scanUseCache,
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
    } catch (error) {
      logger.error(`Error during scan: ${error}`);
      throw error;
    } finally {
      extensionStore.set(StoreKey.IsSearching, false);
      setContextKey(ContextKey.HasScanned, true);
    }
  });
}
