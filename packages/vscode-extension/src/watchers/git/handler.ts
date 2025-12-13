import { ScanMode } from 'tscanner-common';
import { logger } from '../../common/lib/logger';
import { Command, executeCommand } from '../../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../../common/state/extension-store';
import { ScanTrigger } from '../../common/types/scan-trigger';

export function createCommitHandler() {
  return () => {
    const scanMode = extensionStore.get(StoreKey.ScanMode);
    if (scanMode === ScanMode.Staged || scanMode === ScanMode.Uncommitted) {
      logger.debug('Commit detected, refreshing issues...');
      executeCommand(Command.RefreshIssues, { silent: true, trigger: ScanTrigger.GitCommit });
    }
  };
}

export function createCheckoutHandler() {
  return () => {
    const scanMode = extensionStore.get(StoreKey.ScanMode);
    if (scanMode === ScanMode.Branch) {
      logger.debug('Checkout detected in branch mode, refreshing issues...');
      executeCommand(Command.RefreshIssues, { silent: true, trigger: ScanTrigger.GitCheckout });
    }
  };
}
