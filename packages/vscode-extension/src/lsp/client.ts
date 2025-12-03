import * as vscode from 'vscode';
import {
  LanguageClient,
  type LanguageClientOptions,
  type ServerOptions,
  Trace,
  TransportKind,
} from 'vscode-languageclient/node';
import { CONFIG_DIR_NAME, CONFIG_FILE_NAME } from '../common/constants';
import { logger } from '../common/lib/logger';
import type {
  ContentScanResult,
  FileResult,
  GroupMode,
  RuleMetadata,
  ScanResult,
  TscannerConfig,
} from '../common/types';
import { ClearCacheRequestType } from './requests/clear-cache';
import { FormatResultsRequestType } from './requests/format-results';
import { GetRulesMetadataRequestType } from './requests/get-rules-metadata';
import { ScanRequestType } from './requests/scan';
import { ScanContentRequestType } from './requests/scan-content';
import { ScanFileRequestType } from './requests/scan-file';
import type { FormatPrettyResult } from './requests/types';

export class TscannerLspClient {
  private client: LanguageClient | null = null;

  constructor(
    private binaryPath: string,
    private args: string[] = [],
  ) {}

  async start(workspaceRoot: string): Promise<void> {
    if (this.client) {
      logger.info('LSP client already running');
      return;
    }

    logger.info(`Starting LSP: ${this.binaryPath} ${this.args.join(' ')}`);

    const config = vscode.workspace.getConfiguration('tscanner');
    const trace = config.get<string>('trace.server') || 'off';

    const serverOptions: ServerOptions = {
      command: this.binaryPath,
      args: this.args,
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
          vscode.workspace.createFileSystemWatcher(`**/${CONFIG_DIR_NAME}/${CONFIG_FILE_NAME}`),
        ],
      },
      traceOutputChannel: trace !== 'off' ? vscode.window.createOutputChannel('TScanner LSP Trace') : undefined,
    };

    this.client = new LanguageClient('tscanner', 'TScanner LSP', serverOptions, clientOptions);

    if (trace !== 'off') {
      await this.client.setTrace(trace === 'verbose' ? Trace.Verbose : Trace.Messages);
    }

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
      logger.info('LSP client stopped\n\n\n');
    }
  }

  isRunning(): boolean {
    return this.client !== null;
  }

  async scan(root: string, config?: TscannerConfig, branch?: string): Promise<ScanResult> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(ScanRequestType, { root, config, branch });
  }

  async scanFile(root: string, file: string): Promise<FileResult> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(ScanFileRequestType, { root, file });
  }

  async scanContent(root: string, file: string, content: string, config?: TscannerConfig): Promise<ContentScanResult> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(ScanContentRequestType, { root, file, content, config });
  }

  async clearCache(): Promise<void> {
    if (!this.client) throw new Error('LSP client not started');
    await this.client.sendRequest(ClearCacheRequestType);
  }

  async getRulesMetadata(): Promise<RuleMetadata[]> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(GetRulesMetadataRequestType);
  }

  async formatResults(root: string, results: ScanResult, groupMode: GroupMode): Promise<FormatPrettyResult> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(FormatResultsRequestType, {
      root,
      results,
      group_mode: groupMode,
    });
  }
}
