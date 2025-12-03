import * as vscode from 'vscode';
import {
  LanguageClient,
  type LanguageClientOptions,
  RequestType,
  RequestType0,
  type ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';
import { CONFIG_DIR_NAME, CONFIG_FILE_NAME } from '../constants';
import type { ContentScanResult, FileResult, GroupMode, RuleMetadata, ScanResult, TscannerConfig } from '../types';
import { logger } from '../utils/logger';

type ScanParams = {
  root: string;
  config?: TscannerConfig;
  branch?: string;
};

type ScanFileParams = {
  root: string;
  file: string;
};

type ScanContentParams = {
  root: string;
  file: string;
  content: string;
  config?: TscannerConfig;
};

type FormatResultsParams = {
  root: string;
  results: ScanResult;
  group_mode: GroupMode;
};

type FormatPrettyResult = {
  output: string;
  summary: {
    total_issues: number;
    error_count: number;
    warning_count: number;
    file_count: number;
    rule_count: number;
  };
};

const ScanRequestType = new RequestType<ScanParams, ScanResult, void>('tscanner/scan');
const ScanFileRequestType = new RequestType<ScanFileParams, FileResult, void>('tscanner/scanFile');
const ScanContentRequestType = new RequestType<ScanContentParams, ContentScanResult, void>('tscanner/scanContent');
const ClearCacheRequestType = new RequestType0<{ cleared: boolean }, void>('tscanner/clearCache');
const GetRulesMetadataRequestType = new RequestType0<RuleMetadata[], void>('tscanner/getRulesMetadata');
const FormatResultsRequestType = new RequestType<FormatResultsParams, FormatPrettyResult, void>(
  'tscanner/formatResults',
);

export type { FormatPrettyResult };

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
      args: [],
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
