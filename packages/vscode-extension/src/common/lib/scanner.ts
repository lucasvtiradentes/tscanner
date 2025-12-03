import { existsSync } from 'node:fs';
import { join } from 'node:path';
import * as vscode from 'vscode';
import { TscannerLspClient } from '../../lsp';
import { BINARY_BASE_NAME, PLATFORM_TARGET_MAP, getServerBinaryName } from '../constants';
import type { Issue, IssueResult, TscannerConfig } from '../types';
import { parseSeverity } from '../types';
import { getExtensionPath } from '../utils/extension-helper';
import { LOG_FILE_PATH, logger } from '../utils/logger';
import { getCurrentWorkspaceFolder, openTextDocument } from './vscode-utils';

const CONFIG_ERROR_PREFIX = 'TSCANNER_CONFIG_ERROR:';

type ConfigError = {
  invalidFields: string[];
  version: string;
};

function getDocsUrl(version: string): string {
  return `https://github.com/lucasvtiradentes/tscanner/tree/vscode-extension-v${version}?tab=readme-ov-file#%EF%B8%8F-configuration`;
}

function parseConfigError(errorMessage: string): ConfigError | null {
  if (!errorMessage.includes(CONFIG_ERROR_PREFIX)) {
    return null;
  }

  const match = errorMessage.match(/invalid_fields=\[([^\]]*)\];version=([^\s]+)/);
  if (!match) {
    return null;
  }

  const [, fieldsStr, version] = match;
  const invalidFields = fieldsStr.split(',').filter(Boolean);

  return { invalidFields, version };
}

function showConfigErrorToast(configError: ConfigError) {
  const fieldsText = configError.invalidFields.join(', ');
  const message = `TScanner config error: invalid fields [${fieldsText}]. Please check the docs for v${configError.version}`;

  vscode.window.showErrorMessage(message, 'Open Docs').then((selection) => {
    if (selection === 'Open Docs') {
      vscode.env.openExternal(vscode.Uri.parse(getDocsUrl(configError.version)));
    }
  });
}

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

let lspClient: TscannerLspClient | null = null;

export function getRustBinaryPath(): string | null {
  const extensionPath = getExtensionPath();
  if (!extensionPath) {
    logger.error('Extension path not found');
    return null;
  }

  logger.debug(`Extension path: ${extensionPath}`);

  const platform = process.platform;
  const arch = process.arch;
  const target = PLATFORM_TARGET_MAP[`${platform}-${arch}`];
  const binaryName = target
    ? `${BINARY_BASE_NAME}-${target}${platform === 'win32' ? '.exe' : ''}`
    : getServerBinaryName();

  const bundledBinary = join(extensionPath, 'out', 'binaries', binaryName);
  logger.debug(`Checking bundled binary: ${bundledBinary}`);
  if (existsSync(bundledBinary)) {
    logger.info(`Found bundled binary: ${bundledBinary}`);
    return bundledBinary;
  }

  logger.error(`Rust binary not found. Searched: ${bundledBinary}`);
  return null;
}

async function ensureLspClient(): Promise<TscannerLspClient> {
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

export async function scanWorkspace(
  fileFilter?: Set<string>,
  config?: TscannerConfig,
  branch?: string,
): Promise<IssueResult[]> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    return [];
  }

  const binaryPath = getRustBinaryPath();
  if (!binaryPath) {
    logger.error('Rust binary not found');
    vscode.window
      .showErrorMessage(
        `TScanner: Rust binary not found. Please build the Rust core:\n\ncd packages/rust-core && cargo build --release\n\nCheck logs at ${LOG_FILE_PATH} for details.`,
        'Open Logs',
      )
      .then((selection) => {
        if (selection === 'Open Logs') {
          openTextDocument(vscode.Uri.file(LOG_FILE_PATH)).then((doc) => {
            vscode.window.showTextDocument(doc);
          });
        }
      });
    throw new Error('Rust binary not found');
  }

  try {
    const client = await ensureLspClient();

    const scanStart = Date.now();
    const result = await client.scan(workspaceFolder.uri.fsPath, config, branch);
    const scanTime = Date.now() - scanStart;

    logger.info(`Scan completed: ${result.total_issues} issues in ${result.duration_ms}ms (client: ${scanTime}ms)`);

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
  } catch (error) {
    logger.error(`Scan failed: ${error}`);

    const errorMessage = String(error);
    const configError = parseConfigError(errorMessage);

    if (configError) {
      showConfigErrorToast(configError);
    } else {
      vscode.window
        .showErrorMessage(`TScanner: Scan error: ${error}\n\nCheck logs at ${LOG_FILE_PATH}`, 'Open Logs')
        .then((selection) => {
          if (selection === 'Open Logs') {
            openTextDocument(vscode.Uri.file(LOG_FILE_PATH)).then((doc) => {
              vscode.window.showTextDocument(doc);
            });
          }
        });
    }

    throw error;
  }
}

export async function scanFile(filePath: string): Promise<IssueResult[]> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    return [];
  }

  try {
    const client = await ensureLspClient();
    const result = await client.scanFile(workspaceFolder.uri.fsPath, filePath);

    logger.debug(`scanFile() returned ${result.issues.length} results for ${filePath}`);

    const uri = vscode.Uri.file(result.file);
    return result.issues.map((issue) => mapIssueToResult(uri, issue));
  } catch (error) {
    logger.error(`Failed to scan file ${filePath}: ${error}`);
    throw error;
  }
}

export type ScanContentResult = {
  issues: IssueResult[];
  relatedFiles: string[];
};

export async function scanContent(
  filePath: string,
  content: string,
  config?: TscannerConfig,
): Promise<ScanContentResult> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    return { issues: [], relatedFiles: [] };
  }

  try {
    const client = await ensureLspClient();
    const result = await client.scanContent(workspaceFolder.uri.fsPath, filePath, content, config);

    logger.debug(`scanContent() returned ${result.issues.length} results for ${filePath}`);

    const issues = result.issues.map((issue) => {
      const issueFile = issue.file || result.file;
      const uri = vscode.Uri.file(issueFile);
      return mapIssueToResult(uri, issue);
    });

    return {
      issues,
      relatedFiles: result.related_files ?? [],
    };
  } catch (error) {
    logger.error(`Failed to scan content for ${filePath}: ${error}`);
    throw error;
  }
}

export async function clearCache(): Promise<void> {
  const client = await ensureLspClient();
  await client.clearCache();
  logger.info('Cache cleared via LSP');
}

export function getLspClient(): TscannerLspClient | null {
  return lspClient;
}

export async function startLspClient(): Promise<void> {
  await ensureLspClient();
}

export function dispose() {
  if (lspClient) {
    lspClient.stop();
    lspClient = null;
  }
}
