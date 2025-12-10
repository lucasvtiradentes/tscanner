import { existsSync } from 'node:fs';
import { getStatusBarName } from 'src/common/constants';
import {
  type AiExecutionMode,
  CONFIG_DIR_NAME,
  CONFIG_FILE_NAME,
  type ContentScanResult,
  type FileResult,
  type GroupMode,
  JS_EXTENSIONS,
  type RuleMetadata,
  type ScanResult,
  type TscannerConfig,
} from 'tscanner-common';
import * as vscode from 'vscode';
import { LanguageClient, type LanguageClientOptions, type ServerOptions, State } from 'vscode-languageclient/node';
import { ensureBinaryExecutable } from '../common/lib/binary-utils';
import { ClearCacheRequestType } from './requests/clear-cache';
import { FormatResultsRequestType } from './requests/format-results';
import { GetRulesMetadataRequestType } from './requests/get-rules-metadata';
import { ScanRequestType } from './requests/scan';
import { ScanContentRequestType } from './requests/scan-content';
import { ScanFileRequestType } from './requests/scan-file';
import type { AiProgressParams, FormatPrettyResult } from './requests/types';

const AI_PROGRESS_METHOD = 'tscanner/aiProgress';

export class TscannerLspClient {
  private client: LanguageClient | null = null;

  constructor(
    private binaryPath: string,
    private args: string[] = [],
  ) {}

  async start(workspaceRoot: string): Promise<void> {
    if (this.client) {
      return;
    }

    if (!existsSync(this.binaryPath)) {
      throw new Error(`Binary not found: ${this.binaryPath}`);
    }

    ensureBinaryExecutable(this.binaryPath);

    const serverOptions: ServerOptions = {
      command: this.binaryPath,
      args: this.args,
      options: { cwd: workspaceRoot },
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
          vscode.workspace.createFileSystemWatcher(`**/*.{${JS_EXTENSIONS.join(',')}}`),
          vscode.workspace.createFileSystemWatcher(`**/${CONFIG_DIR_NAME}/${CONFIG_FILE_NAME}`),
        ],
      },
    };

    this.client = new LanguageClient('tscanner', `${getStatusBarName()} LSP`, serverOptions, clientOptions);

    await this.client.start();
  }

  async stop(): Promise<void> {
    if (this.client) {
      await this.client.stop();
      this.client = null;
    }
  }

  isRunning(): boolean {
    return this.client !== null && this.client.state === State.Running;
  }

  getState(): string {
    if (!this.client) return 'null';
    switch (this.client.state) {
      case State.Stopped:
        return 'Stopped';
      case State.Starting:
        return 'Starting';
      case State.Running:
        return 'Running';
      default:
        return `Unknown(${this.client.state})`;
    }
  }

  async scan(root: string, config?: TscannerConfig, branch?: string, aiMode?: AiExecutionMode): Promise<ScanResult> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(ScanRequestType, { root, config, branch, ai_mode: aiMode });
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

  onAiProgress(handler: (params: AiProgressParams) => void): vscode.Disposable {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.onNotification(AI_PROGRESS_METHOD, handler);
  }
}
