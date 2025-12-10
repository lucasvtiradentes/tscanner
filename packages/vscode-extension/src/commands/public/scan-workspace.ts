import {
  type AiExecutionMode,
  CONFIG_DIR_NAME,
  GitHelper,
  ScanMode,
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
import {
  ContextKey,
  WorkspaceStateKey,
  setContextKey,
  setWorkspaceState,
  updateState,
} from '../../common/state/workspace-state';
import { serializeResults } from '../../common/types';
import type { RegularIssuesView } from '../../issues-panel';
import { scan } from '../../scanner/scan';
import { resetIssueIndex } from './issue-navigation';

export function createScanWorkspaceCommand(ctx: CommandContext, regularView: RegularIssuesView) {
  const { context, treeView, stateRefs, updateStatusBar } = ctx;
  const { isSearchingRef, currentScanModeRef, currentCompareBranchRef, currentConfigDirRef } = stateRefs;

  return registerCommand(Command.FindIssue, async (options?: { silent?: boolean; aiMode?: AiExecutionMode }) => {
    if (isSearchingRef.current) {
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

    const configDir = currentConfigDirRef.current;
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

    if (currentScanModeRef.current === ScanMode.Branch) {
      const branchExistsCheck = await GitHelper.branchExists(
        workspaceFolder.uri.fsPath,
        currentCompareBranchRef.current,
      );
      if (!branchExistsCheck) {
        logger.warn(`Branch does not exist: ${currentCompareBranchRef.current}`);
        const action = await showToastMessage(
          ToastKind.Error,
          `Branch '${currentCompareBranchRef.current}' does not exist in this repository`,
          'Change Branch',
          'Switch to Workspace Mode',
        );

        if (action === 'Change Branch') {
          await executeCommand(Command.OpenSettingsMenu);
        } else if (action === 'Switch to Workspace Mode') {
          currentScanModeRef.current = ScanMode.Codebase;
          updateState(context, WorkspaceStateKey.ScanMode, ScanMode.Codebase);
          await updateStatusBar();
          await executeCommand(Command.FindIssue, { silent: true });
        }
        return;
      }
    }

    isSearchingRef.current = true;
    setContextKey(ContextKey.Searching, true);
    regularView.setResults([]);

    logger.info(`Starting scan in ${currentScanModeRef.current} mode`);

    try {
      const startTime = Date.now();
      const branch = currentScanModeRef.current === ScanMode.Branch ? currentCompareBranchRef.current : undefined;
      const results = await scan({
        branch,
        config: configToPass,
        configDir: configDir ?? undefined,
        aiMode: options?.aiMode,
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
        }, 100);
      }
    } catch (error) {
      logger.error(`Error during scan: ${error}`);
      throw error;
    } finally {
      isSearchingRef.current = false;
      setContextKey(ContextKey.Searching, false);
      setContextKey(ContextKey.HasScanned, true);
    }
  });
}
