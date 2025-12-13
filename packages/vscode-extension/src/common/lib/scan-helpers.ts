import { StoreKey, extensionStore } from '../state/extension-store';
import { type ContextKey, setContextKey } from '../state/workspace-state';
import { createLogger } from './logger';

export enum ScanType {
  Regular = 'regular',
  Ai = 'ai',
}

interface ScanWrapperOptions {
  scanType: ScanType;
  contextKeyOnComplete: ContextKey;
  onError?: (error: unknown) => void;
  onFinally?: () => void;
}

const scanLogger = createLogger('Scan');
const aiScanLogger = createLogger('AI Scan');

const SCAN_CONFIG: Record<ScanType, { storeKey: StoreKey; logger: ReturnType<typeof createLogger> }> = {
  [ScanType.Regular]: { storeKey: StoreKey.IsSearching, logger: scanLogger },
  [ScanType.Ai]: { storeKey: StoreKey.IsAiSearching, logger: aiScanLogger },
};

export async function withScanErrorHandling<T>(
  options: ScanWrapperOptions,
  operation: () => Promise<T>,
): Promise<T | undefined> {
  const { scanType, contextKeyOnComplete, onError, onFinally } = options;
  const config = SCAN_CONFIG[scanType];

  extensionStore.set(config.storeKey, true);

  try {
    return await operation();
  } catch (error) {
    config.logger.error(`Error: ${error}`);
    onError?.(error);
    return undefined;
  } finally {
    extensionStore.set(config.storeKey, false);
    setContextKey(contextKeyOnComplete, true);
    onFinally?.();
  }
}
