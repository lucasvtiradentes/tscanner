import { ScanMode } from 'tscanner-common';
import { logger } from '../../common/lib/logger';
import { Command, executeCommand } from '../../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../../common/state/extension-store';

const CHECKOUT_COOLDOWN_MS = 2000;

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
    extensionStore.set(StoreKey.IsCheckingOut, true);
    logger.debug('Checkout detected, blocking file watchers...');

    setTimeout(() => {
      extensionStore.set(StoreKey.IsCheckingOut, false);
      logger.debug('Checkout cooldown finished, file watchers unblocked');
    }, CHECKOUT_COOLDOWN_MS);

    const scanMode = extensionStore.get(StoreKey.ScanMode);
    if (scanMode === ScanMode.Branch) {
      logger.debug('Branch changed, refreshing issues (no cache)...');
      executeCommand(Command.RefreshIssues, { silent: true, noCache: true });
    }
  };
}
