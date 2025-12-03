import { logger } from '../common/lib/logger';
import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { TscannerLspClient } from '../lsp/client';
import { getRustBinaryPath } from './utils';

let lspClient: TscannerLspClient | null = null;

export async function ensureLspClient(): Promise<TscannerLspClient> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    throw new Error('No workspace folder found');
  }

  if (!lspClient) {
    const binaryPath = getRustBinaryPath();
    if (!binaryPath) {
      throw new Error('Rust binary not found');
    }

    lspClient = new TscannerLspClient(binaryPath);
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
  logger.info('Cache cleared via LSP');
}

export function dispose() {
  if (lspClient) {
    lspClient.stop();
    lspClient = null;
  }
}
