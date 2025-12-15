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
  ensureBinaryExecutable,
} from 'tscanner-common';
import * as vscode from 'vscode';
import {
  type InitializeResult,
  LanguageClient,
  type LanguageClientOptions,
  type ServerOptions,
  State,
} from 'vscode-languageclient/node';
import { ClearCacheRequestType } from './requests/clear-cache';
import { FormatResultsRequestType } from './requests/format-results';
import { GetRulesMetadataRequestType } from './requests/get-rules-metadata';
import { ScanRequestType } from './requests/scan';
import { ScanContentRequestType } from './requests/scan-content';
import { ScanFileRequestType } from './requests/scan-file';
import type { AiProgressParams, FormatPrettyResult, ValidateConfigResult } from './requests/types';
import { ValidateConfigRequestType } from './requests/validate-config';

export class TscannerLspClient {
  private client: LanguageClient | null = null;

  private async ensureClient(): Promise<LanguageClient> {
    if (!this.client) throw new Error('LSP client not started');

    const maxWaitMs = 10000;
    const startTime = Date.now();

    while (this.client.state !== State.Running) {
      if (Date.now() - startTime > maxWaitMs) {
        throw new Error(`LSP client failed to start within ${maxWaitMs}ms (state: ${this.getState()})`);
      }
      await new Promise((resolve) => setTimeout(resolve, 100));
    }

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
    const client = await this.ensureClient();
    return client.sendRequest(ScanRequestType, {
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
    const client = await this.ensureClient();
    return client.sendRequest(ScanFileRequestType, { root, file });
  }

  async scanContent(
    root: string,
    file: string,
    content: string,
    config?: TscannerConfig,
    configDir?: string,
    branch?: string,
    uncommitted?: boolean,
  ): Promise<ContentScanResult> {
    const client = await this.ensureClient();

    return client.sendRequest(ScanContentRequestType, {
      root,
      file,
      content,
      config,
      config_dir: configDir,
      branch,
      uncommitted,
    });
  }

  async clearCache(): Promise<void> {
    const client = await this.ensureClient();
    await client.sendRequest(ClearCacheRequestType);
  }

  async getRulesMetadata(): Promise<RuleMetadata[]> {
    const client = await this.ensureClient();
    return client.sendRequest(GetRulesMetadataRequestType);
  }

  async formatResults(root: string, results: ScanResult, groupMode: GroupMode): Promise<FormatPrettyResult> {
    const client = await this.ensureClient();
    return client.sendRequest(FormatResultsRequestType, {
      root,
      results,
      group_mode: groupMode,
    });
  }

  async validateConfig(configPath: string): Promise<ValidateConfigResult> {
    const client = await this.ensureClient();
    return client.sendRequest(ValidateConfigRequestType, {
      config_path: configPath,
    });
  }

  onAiProgress(handler: (params: AiProgressParams) => void): vscode.Disposable {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.onNotification(LspMethod.AiProgress, handler);
  }

  async getServerVersion(): Promise<string | null> {
    if (!this.client) {
      return null;
    }

    const maxWaitMs = 5000;
    const startTime = Date.now();

    while (this.client.state !== State.Running || !this.client.initializeResult) {
      if (Date.now() - startTime > maxWaitMs) {
        return null;
      }
      await new Promise((resolve) => setTimeout(resolve, 50));
    }

    const initResult = this.client.initializeResult as InitializeResult & {
      capabilities: { serverInfo?: { name: string; version: string } };
    };
    return initResult.capabilities?.serverInfo?.version ?? null;
  }
}
