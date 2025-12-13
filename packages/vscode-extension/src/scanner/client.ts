import { EXTENSION_DISPLAY_NAME } from 'src/common/scripts-constants';
import { PACKAGE_NAME } from 'tscanner-common';
import { logger } from '../common/lib/logger';
import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { LOCATOR_SOURCE_LABELS_VERBOSE, Locator, promptInstall } from '../locator';
import { TscannerLspClient } from '../lsp/client';

let lspClient: TscannerLspClient | null = null;

export async function ensureLspClient(): Promise<TscannerLspClient> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    logger.info('No workspace folder open, LSP client not started');
    throw new Error('No workspace folder found');
  }

  if (!lspClient) {
    const locator = new Locator(workspaceFolder.uri.fsPath);
    const result = await locator.locate();

    if (!result) {
      const installed = await promptInstall();
      if (installed) {
        const retryResult = await locator.locate();
        if (!retryResult) {
          throw new Error(`${EXTENSION_DISPLAY_NAME} binary not found after installation. Please restart VSCode.`);
        }
        logger.info(`Using tscanner from: ${LOCATOR_SOURCE_LABELS_VERBOSE[retryResult.source]} (${retryResult.path})`);
        const args = [...(retryResult.args ?? []), 'lsp'];
        lspClient = new TscannerLspClient(retryResult.path, args);
      } else {
        throw new Error(
          `${EXTENSION_DISPLAY_NAME} binary not found.\n\nInstall with: npm install -g ${PACKAGE_NAME}\nOr add as dev dependency: npm install -D ${PACKAGE_NAME}`,
        );
      }
    } else {
      logger.info(`Using tscanner from: ${LOCATOR_SOURCE_LABELS_VERBOSE[result.source]} (${result.path})`);
      const args = [...(result.args ?? []), 'lsp'];
      lspClient = new TscannerLspClient(result.path, args);
    }

    await lspClient.start(workspaceFolder.uri.fsPath);
  }

  return lspClient;
}

export function getLspClient(): TscannerLspClient | null {
  return lspClient;
}

export async function startLspClient(): Promise<void> {
  await ensureLspClient();
}

export async function clearCache(): Promise<void> {
  const client = await ensureLspClient();
  await client.clearCache();
}

export function dispose() {
  if (lspClient) {
    lspClient.stop();
    lspClient = null;
  }
}
