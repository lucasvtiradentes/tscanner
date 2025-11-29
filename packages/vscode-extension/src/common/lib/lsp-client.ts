import * as vscode from 'vscode';
import {
  LanguageClient,
  type LanguageClientOptions,
  type ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';
import { logger } from '../utils/logger';

export class TscannerLspClient {
  private client: LanguageClient | null = null;

  constructor(private binaryPath: string) {}

  async start(workspaceRoot: string): Promise<void> {
    if (this.client) {
      logger.info('LSP client already running');
      return;
    }

    logger.info(`Starting LSP client: ${this.binaryPath}`);

    const serverOptions: ServerOptions = {
      command: this.binaryPath,
      args: ['--lsp'],
      transport: TransportKind.stdio,
    };

    const clientOptions: LanguageClientOptions = {
      documentSelector: [
        { scheme: 'file', language: 'typescript' },
        { scheme: 'file', language: 'typescriptreact' },
        { scheme: 'file', language: 'javascript' },
        { scheme: 'file', language: 'javascriptreact' },
      ],
      workspaceFolder: vscode.workspace.getWorkspaceFolder(vscode.Uri.file(workspaceRoot)),
      synchronize: {
        fileEvents: [
          vscode.workspace.createFileSystemWatcher('**/*.{ts,tsx,js,jsx}'),
          vscode.workspace.createFileSystemWatcher('**/.tscanner/config.jsonc'),
        ],
      },
    };

    this.client = new LanguageClient('tscanner', 'TScanner LSP', serverOptions, clientOptions);

    try {
      await this.client.start();
      logger.info('LSP client started successfully');
    } catch (error) {
      logger.error(`Failed to start LSP client: ${error}`);
      throw error;
    }
  }

  async stop(): Promise<void> {
    if (this.client) {
      logger.info('Stopping LSP client');
      await this.client.stop();
      this.client = null;
      logger.info('LSP client stopped');
    }
  }

  isRunning(): boolean {
    return this.client !== null;
  }
}
