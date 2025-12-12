import { ScanMode } from 'tscanner-common';
import { logger } from '../../common/lib/logger';
import { Command, executeCommand } from '../../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../../common/state/extension-store';

export function createCommitHandler() {
  return () => {
    const scanMode = extensionStore.get(StoreKey.ScanMode);
    if (scanMode === ScanMode.Staged || scanMode === ScanMode.Uncommitted) {
      logger.debug('Commit detected, refreshing issues...');
      executeCommand(Command.RefreshIssues, { silent: true });
    }
  };
}

export function createCheckoutHandler() {
  return () => {
    const scanMode = extensionStore.get(StoreKey.ScanMode);
    if (scanMode === ScanMode.Branch) {
      logger.debug('Branch changed, refreshing issues (no cache)...');
      executeCommand(Command.RefreshIssues, { silent: true, noCache: true });
    }
  };
}
