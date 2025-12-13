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
  LSP_CLIENT_ID,
  LspMethod,
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

export class TscannerLspClient {
  private client: LanguageClient | null = null;

  private ensureClient(): LanguageClient {
    if (!this.client) throw new Error('LSP client not started');
    return this.client;
  }

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

    this.client = new LanguageClient(LSP_CLIENT_ID, `${getStatusBarName()} LSP`, serverOptions, clientOptions);

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

  async scan(
    root: string,
    config?: TscannerConfig,
    configDir?: string,
    branch?: string,
    staged?: boolean,
    aiMode?: AiExecutionMode,
    noCache?: boolean,
  ): Promise<ScanResult> {
    return this.ensureClient().sendRequest(ScanRequestType, {
      root,
      config,
      config_dir: configDir,
      branch,
      staged,
      ai_mode: aiMode,
      no_cache: noCache,
    });
  }

  async scanFile(root: string, file: string): Promise<FileResult> {
    return this.ensureClient().sendRequest(ScanFileRequestType, { root, file });
  }

  async scanContent(
    root: string,
    file: string,
    content: string,
    config?: TscannerConfig,
    configDir?: string,
  ): Promise<ContentScanResult> {
    return this.ensureClient().sendRequest(ScanContentRequestType, {
      root,
      file,
      content,
      config,
      config_dir: configDir,
    });
  }

  async clearCache(): Promise<void> {
    await this.ensureClient().sendRequest(ClearCacheRequestType);
  }

  async getRulesMetadata(): Promise<RuleMetadata[]> {
    return this.ensureClient().sendRequest(GetRulesMetadataRequestType);
  }

  async formatResults(root: string, results: ScanResult, groupMode: GroupMode): Promise<FormatPrettyResult> {
    return this.ensureClient().sendRequest(FormatResultsRequestType, {
      root,
      results,
      group_mode: groupMode,
    });
  }

  onAiProgress(handler: (params: AiProgressParams) => void): vscode.Disposable {
    return this.ensureClient().onNotification(LspMethod.AiProgress, handler);
  }
}
