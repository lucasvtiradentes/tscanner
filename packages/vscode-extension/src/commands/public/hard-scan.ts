import { logger } from '../../common/lib/logger';
import {
  Command,
  ToastKind,
  executeCommand,
  getCurrentWorkspaceFolder,
  registerCommand,
  showToastMessage,
} from '../../common/lib/vscode-utils';
import { clearCache } from '../../scanner';

export function createHardScanCommand(isSearchingRef: { current: boolean }) {
  return registerCommand(Command.HardScan, async (options?: { showToastMessage?: boolean }) => {
    const shouldShowToast = options?.showToastMessage ?? true;
    if (isSearchingRef.current) {
      if (shouldShowToast) showToastMessage(ToastKind.Warning, 'Search already in progress');
      return;
    }

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      showToastMessage(ToastKind.Error, 'No workspace folder open');
      return;
    }

    logger.info('Starting hard scan (clearing cache)');

    try {
      await clearCache();
      if (shouldShowToast) showToastMessage(ToastKind.Info, 'Cache cleared, rescanning...');
      await executeCommand(Command.FindIssue);
    } catch (error) {
      logger.error(`Hard scan failed: ${error}`);
      showToastMessage(ToastKind.Error, `Hard scan failed: ${error}`);
    }
  });
}
