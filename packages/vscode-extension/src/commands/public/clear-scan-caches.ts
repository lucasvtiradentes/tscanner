import { logger } from '../../common/lib/logger';
import { Command, ToastKind, registerCommand, showToastMessage } from '../../common/lib/vscode-utils';
import { clearCache } from '../../scanner/client';

export function createClearScanCachesCommand() {
  return registerCommand(Command.ClearScanCaches, async () => {
    try {
      logger.info('Clearing scan caches...');
      await clearCache();
      logger.info('Scan caches cleared successfully');
      showToastMessage(ToastKind.Info, 'Scan caches cleared successfully');
    } catch (error) {
      logger.error(`Failed to clear scan caches: ${error}`);
      showToastMessage(ToastKind.Error, `Failed to clear scan caches: ${error}`);
    }
  });
}
