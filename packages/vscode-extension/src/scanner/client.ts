import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { Locator, promptInstall } from '../locator';
import { TscannerLspClient } from '../lsp/client';

let lspClient: TscannerLspClient | null = null;

export async function ensureLspClient(): Promise<TscannerLspClient> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
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
          throw new Error('TScanner binary not found after installation. Please restart VSCode.');
        }
        const args = [...(retryResult.args ?? []), 'lsp'];
        lspClient = new TscannerLspClient(retryResult.path, args);
      } else {
        throw new Error(
          'TScanner binary not found.\n\n' +
            'Install with: npm install -g tscanner\n' +
            'Or add as dev dependency: npm install -D tscanner',
        );
      }
    } else {
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
