import { type ChildProcess, spawn } from 'node:child_process';
import * as zlib from 'node:zlib';
import type { CliGroupBy } from 'tscanner-common';
import * as vscode from 'vscode';
import {
  type ClearCacheParams,
  type FileResult,
  type GetRulesMetadataParams,
  type Issue,
  type IssueResult,
  type RuleMetadata,
  type ScanContentParams,
  type ScanFileParams,
  type ScanParams,
  type ScanResult,
  type TscannerConfig,
  parseSeverity,
} from '../types';
import { logger } from '../utils/logger';
import { openTextDocument } from './vscode-utils';

function mapIssueToResult(uri: vscode.Uri, issue: Issue, lineText?: string): IssueResult {
  return {
    uri,
    line: issue.line - 1,
    column: issue.column - 1,
    endColumn: issue.end_column - 1,
    text: (lineText ?? issue.line_text ?? '').trim(),
    rule: issue.rule,
    severity: parseSeverity(issue.severity),
    message: issue.message,
  };
}

enum RpcMethod {
  Scan = 'scan',
  ScanFile = 'scanFile',
  ScanContent = 'scanContent',
  GetRulesMetadata = 'getRulesMetadata',
  ClearCache = 'clearCache',
  FormatResults = 'formatResults',
}

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

type FormatResultsParams = {
  root: string;
  results: ScanResult;
  group_mode: CliGroupBy;
};

type RpcRequestMap = {
  [RpcMethod.Scan]: ScanParams;
  [RpcMethod.ScanFile]: ScanFileParams;
  [RpcMethod.ScanContent]: ScanContentParams;
  [RpcMethod.GetRulesMetadata]: GetRulesMetadataParams;
  [RpcMethod.ClearCache]: ClearCacheParams;
  [RpcMethod.FormatResults]: FormatResultsParams;
};

type RpcResponseMap = {
  [RpcMethod.Scan]: ScanResult;
  [RpcMethod.ScanFile]: FileResult;
  [RpcMethod.ScanContent]: FileResult;
  [RpcMethod.GetRulesMetadata]: RuleMetadata[];
  [RpcMethod.ClearCache]: undefined;
  [RpcMethod.FormatResults]: FormatPrettyResult;
};

type RpcRequest = {
  id: number;
  method: string;
  params: unknown;
};

type RpcResponse = {
  id: number;
  result?: unknown;
  error?: string;
};

export class RustClient {
  private process: ChildProcess | null = null;
  private requestId = 0;
  private pendingRequests = new Map<
    number,
    {
      resolve: (result: any) => void;
      reject: (error: Error) => void;
    }
  >();
  private buffer = '';

  constructor(private binaryPath: string) {}

  async start(): Promise<void> {
    if (this.process) {
      logger.info('Rust server already running');
      return;
    }

    logger.info(`Starting Rust server: ${this.binaryPath}`);

    try {
      this.process = spawn(this.binaryPath, [], {
        stdio: ['pipe', 'pipe', 'pipe'],
        env: {
          ...process.env,
          NO_COLOR: '1',
          RUST_LOG_STYLE: 'never',
          RUST_LOG: 'core=warn,server=info',
        },
      });

      logger.info(`Rust server spawned with PID: ${this.process.pid}`);
    } catch (error) {
      logger.error(`Failed to spawn Rust server: ${error}`);
      throw error;
    }

    this.process.stdout?.on('data', (data: Buffer) => {
      const chunkSize = data.length;
      this.buffer += data.toString();

      const lines = this.buffer.split('\n');
      this.buffer = lines.pop() || '';

      for (const line of lines) {
        if (!line.trim()) continue;
        const parseStart = Date.now();
        try {
          let jsonString = line;

          if (line.startsWith('GZIP:')) {
            const decompressStart = Date.now();
            const base64Data = line.substring(5);
            const compressed = Buffer.from(base64Data, 'base64');
            const decompressed = zlib.gunzipSync(compressed);
            jsonString = decompressed.toString('utf8');
            const decompressTime = Date.now() - decompressStart;
            if (decompressTime > 50) {
              logger.debug(
                `Decompression took ${decompressTime}ms (${(compressed.length / 1024).toFixed(1)}KB → ${(decompressed.length / 1024).toFixed(1)}KB)`,
              );
            }
          }

          const response: RpcResponse = JSON.parse(jsonString);
          const parseTime = Date.now() - parseStart;
          if (parseTime > 50) {
            logger.debug(
              `JSON parse took ${parseTime}ms for ${(jsonString.length / 1024 / 1024).toFixed(2)}MB response`,
            );
          }
          const pending = this.pendingRequests.get(response.id);
          if (pending) {
            this.pendingRequests.delete(response.id);
            if (response.error) {
              pending.reject(new Error(response.error));
            } else {
              pending.resolve(response.result);
            }
          }
        } catch (e) {
          logger.error(
            `Failed to parse response (chunk: ${(chunkSize / 1024).toFixed(1)}KB, line: ${(line.length / 1024).toFixed(1)}KB, buffer: ${(this.buffer.length / 1024).toFixed(1)}KB): ${e}`,
          );
        }
      }
    });

    this.process.stderr?.on('data', (data: Buffer) => {
      const text = data.toString().replace(/\x1b\[[0-9;]*m/g, '');
      if (text.trim()) {
        logger.info(`[Rust stderr] ${text}`);
      }
    });

    this.process.on('error', (err) => {
      logger.error(`Rust process error: ${err}`);
    });

    this.process.on('exit', (code) => {
      logger.info(`Rust process exited with code: ${code}`);
      this.process = null;
    });
  }

  async stop(): Promise<void> {
    if (this.process) {
      this.process.kill();
      this.process = null;
    }
  }

  private async sendRequest<M extends RpcMethod>(method: M, params: RpcRequestMap[M]): Promise<RpcResponseMap[M]> {
    if (!this.process) {
      logger.info(`Process not running, starting server for method: ${method}`);
      await this.start();
    }

    if (!this.process || !this.process.stdin) {
      const error = 'Rust server process or stdin not available';
      logger.error(error);
      throw new Error(error);
    }

    const id = ++this.requestId;
    const request: RpcRequest = { id, method, params };

    logger.info(`Sending request ${id}: ${method}`);

    return new Promise((resolve, reject) => {
      this.pendingRequests.set(id, { resolve, reject });

      const json = JSON.stringify(request);
      logger.info(`Writing request to stdin: ${json.substring(0, 200)}...`);
      this.process?.stdin?.write(`${json}\n`);
    });
  }

  async scan(
    workspaceRoot: string,
    fileFilter?: Set<string>,
    config?: TscannerConfig,
    branch?: string,
  ): Promise<IssueResult[]> {
    const result = await this.sendRequest(RpcMethod.Scan, {
      root: workspaceRoot,
      config,
      branch,
    });

    logger.info(`Rust scan completed: ${result.total_issues} issues in ${result.duration_ms}ms`);

    const processStart = Date.now();

    let filesToLoad = [...new Set(result.files.map((f) => f.file))];

    if (fileFilter && fileFilter.size > 0) {
      filesToLoad = filesToLoad.filter((filePath) => {
        const relativePath = vscode.workspace.asRelativePath(vscode.Uri.file(filePath));
        return fileFilter.has(relativePath);
      });

      logger.debug(
        `Filtered ${result.files.length} → ${filesToLoad.length} files to load (fileFilter has ${fileFilter.size} entries)`,
      );
    }

    const results: IssueResult[] = [];

    for (const fileResult of result.files) {
      const uri = vscode.Uri.file(fileResult.file);

      for (const issue of fileResult.issues) {
        let lineText = issue.line_text || '';

        if (!lineText && fileFilter && fileFilter.has(vscode.workspace.asRelativePath(uri))) {
          try {
            const document = await openTextDocument(uri);
            lineText = document.lineAt(issue.line - 1).text;
          } catch (_error) {
            logger.error(`Failed to load line text for: ${fileResult.file}`);
            lineText = '';
          }
        }

        results.push(mapIssueToResult(uri, issue, lineText));
      }
    }

    const processTime = Date.now() - processStart;
    logger.debug(
      `Post-processing ${result.total_issues} issues from ${filesToLoad.length} files took ${processTime}ms total`,
    );

    return results;
  }

  async scanFile(workspaceRoot: string, filePath: string): Promise<IssueResult[]> {
    const result = await this.sendRequest(RpcMethod.ScanFile, {
      root: workspaceRoot,
      file: filePath,
    });

    logger.info(`Rust scan completed for single file: ${result.issues.length} issues`);

    const uri = vscode.Uri.file(result.file);
    return result.issues.map((issue) => mapIssueToResult(uri, issue));
  }

  async getRulesMetadata(): Promise<RuleMetadata[]> {
    return this.sendRequest(RpcMethod.GetRulesMetadata, {});
  }

  async scanContent(
    workspaceRoot: string,
    filePath: string,
    content: string,
    config?: TscannerConfig,
  ): Promise<IssueResult[]> {
    const result = await this.sendRequest(RpcMethod.ScanContent, {
      root: workspaceRoot,
      file: filePath,
      content,
      config,
    });

    logger.debug(`Rust scan completed for content: ${result.issues.length} issues`);

    const uri = vscode.Uri.file(result.file);
    return result.issues.map((issue) => mapIssueToResult(uri, issue));
  }

  async clearCache(): Promise<void> {
    await this.sendRequest(RpcMethod.ClearCache, {});
    logger.info('Rust cache cleared');
  }

  async formatResults(workspaceRoot: string, results: ScanResult, groupMode: CliGroupBy): Promise<FormatPrettyResult> {
    const result = await this.sendRequest(RpcMethod.FormatResults, {
      root: workspaceRoot,
      results,
      group_mode: groupMode,
    });

    logger.info('Rust formatResults completed');
    return result;
  }
}
