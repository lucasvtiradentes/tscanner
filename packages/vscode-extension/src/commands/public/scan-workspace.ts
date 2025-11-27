import type * as vscode from 'vscode';
import { hasCustomConfig, hasLocalConfig, loadEffectiveConfig } from '../../common/lib/config-manager';
import { scanWorkspace } from '../../common/lib/scanner';
import {
  Command,
  ContextKey,
  ScanMode,
  ToastKind,
  ViewMode,
  WorkspaceStateKey,
  executeCommand,
  getCurrentWorkspaceFolder,
  registerCommand,
  setContextKey,
  setWorkspaceState,
  showToastMessage,
  updateState,
} from '../../common/lib/vscode-utils';
import { type IssueResult, hasConfiguredRules } from '../../common/types';
import { branchExists } from '../../common/utils/git-helper';
import { logger } from '../../common/utils/logger';
import type { IssuesPanelContent } from '../../issues-panel/panel-content';
import { resetIssueIndex } from './issue-navigation';

export function createScanWorkspaceCommand(
  panelContent: IssuesPanelContent,
  context: vscode.ExtensionContext,
  treeView: vscode.TreeView<any>,
  updateBadge: () => void,
  updateStatusBar: () => Promise<void>,
  isSearchingRef: { current: boolean },
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  currentCustomConfigDirRef: { current: string | null },
) {
  return registerCommand(Command.FindIssue, async (options?: { silent?: boolean }) => {
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

    const customConfigDir = currentCustomConfigDirRef.current;
    const effectiveConfig = await loadEffectiveConfig(context, workspaceFolder.uri.fsPath, customConfigDir);
    const hasLocal = await hasLocalConfig(workspaceFolder.uri.fsPath);
    const hasCustom = customConfigDir ? await hasCustomConfig(workspaceFolder.uri.fsPath, customConfigDir) : false;

    if (!hasConfiguredRules(effectiveConfig)) {
      panelContent.setResults([]);
      updateBadge();
      if (!options?.silent) {
        const action = await showToastMessage(
          ToastKind.Warning,
          'No rules configured for this workspace',
          'Configure Rules',
        );
        if (action === 'Configure Rules') {
          await executeCommand(Command.ManageRules);
        }
      }
      return;
    }

    const configToPass = hasLocal && !hasCustom ? undefined : (effectiveConfig ?? undefined);
    if (hasCustom) {
      logger.info(`Using custom config from ${customConfigDir}`);
    } else if (hasLocal) {
      logger.info('Using local config from .tscanner');
    } else {
      logger.info('Using global config from extension storage');
    }

    if (currentScanModeRef.current === ScanMode.Branch) {
      const branchExistsCheck = await branchExists(workspaceFolder.uri.fsPath, currentCompareBranchRef.current);
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

    logger.info(`Starting scan in ${currentScanModeRef.current} mode`);

    try {
      const startTime = Date.now();
      let results: IssueResult[];

      if (currentScanModeRef.current === ScanMode.Branch) {
        results = await scanWorkspace(undefined, configToPass, currentCompareBranchRef.current);
      } else {
        results = await scanWorkspace(undefined, configToPass);
      }

      const elapsed = Date.now() - startTime;
      logger.info(`Search completed in ${elapsed}ms, found ${results.length} results`);

      resetIssueIndex();
      panelContent.setResults(results);

      const serializedResults = results.map((r) => {
        const { uri, ...rest } = r;
        return {
          ...rest,
          uriString: uri.toString(),
        };
      });
      setWorkspaceState(context, WorkspaceStateKey.CachedResults, serializedResults);
      updateBadge();

      if (panelContent.viewMode === ViewMode.Tree) {
        setTimeout(() => {
          const folders = panelContent.getAllFolderItems();
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
    }
  });
}
