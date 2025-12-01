import { existsSync } from 'node:fs';
import { join } from 'node:path';
import * as vscode from 'vscode';
import { BINARY_BASE_NAME, PLATFORM_TARGET_MAP, getServerBinaryName } from '../constants';
import type { IssueResult, TscannerConfig } from '../types';
import { getExtensionPath } from '../utils/extension-helper';
import { LOG_FILE_PATH, logger } from '../utils/logger';
import { TscannerLspClient } from './lsp-client';
import { RustClient, type ScanContentResult } from './rust-client';
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

let rustClient: RustClient | null = null;
let lspClient: TscannerLspClient | null = null;

async function ensureRustClient(): Promise<RustClient> {
  const binaryPath = getRustBinaryPath();
  if (!binaryPath) {
    throw new Error('Rust binary not found');
  }

  if (!rustClient) {
    rustClient = new RustClient(binaryPath);
    await rustClient.start();
  }

  return rustClient;
}

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

  const devBinaryRelease = join(extensionPath, '..', '..', 'core', 'target', 'release', getServerBinaryName());
  logger.debug(`Checking dev release binary: ${devBinaryRelease}`);
  if (existsSync(devBinaryRelease)) {
    logger.info(`Found dev release binary: ${devBinaryRelease}`);
    return devBinaryRelease;
  }

  const devBinaryDebug = join(extensionPath, '..', '..', 'core', 'target', 'debug', binaryName);
  logger.debug(`Checking dev debug binary: ${devBinaryDebug}`);
  if (existsSync(devBinaryDebug)) {
    logger.info(`Found dev debug binary: ${devBinaryDebug}`);
    return devBinaryDebug;
  }

  logger.error(`Rust binary not found. Searched: ${bundledBinary}, ${devBinaryRelease}, ${devBinaryDebug}`);
  return null;
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
        `TScanner: Rust binary not found. Please build the Rust core:\n\ncd packages/core && cargo build --release\n\nCheck logs at ${LOG_FILE_PATH} for details.`,
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
    logger.info('Using Rust backend for scanning');
    const client = await ensureRustClient();

    const scanStart = Date.now();
    const results = await client.scan(workspaceFolder.uri.fsPath, fileFilter, config, branch);
    const scanTime = Date.now() - scanStart;

    logger.info(`scanWorkspace() took ${scanTime}ms to return ${results.length} results`);
    return results;
  } catch (error) {
    logger.error(`Rust backend failed: ${error}`);

    const errorMessage = String(error);
    const configError = parseConfigError(errorMessage);

    if (configError) {
      showConfigErrorToast(configError);
    } else {
      vscode.window
        .showErrorMessage(`TScanner: Rust backend error: ${error}\n\nCheck logs at ${LOG_FILE_PATH}`, 'Open Logs')
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
    const client = await ensureRustClient();
    const results = await client.scanFile(workspaceFolder.uri.fsPath, filePath);
    logger.debug(`scanFile() returned ${results.length} results for ${filePath}`);
    return results;
  } catch (error) {
    logger.error(`Failed to scan file ${filePath}: ${error}`);
    throw error;
  }
}

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
    const client = await ensureRustClient();
    const result = await client.scanContent(workspaceFolder.uri.fsPath, filePath, content, config);
    logger.debug(`scanContent() returned ${result.issues.length} results for ${filePath}`);
    return result;
  } catch (error) {
    logger.error(`Failed to scan content for ${filePath}: ${error}`);
    throw error;
  }
}

export async function clearCache(): Promise<void> {
  const client = await ensureRustClient();
  await client.clearCache();
  logger.info('Cache cleared via RPC');
}

export function getRustClient(): RustClient | null {
  return rustClient;
}

export async function startLspClient(): Promise<void> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    logger.warn('No workspace folder found, cannot start LSP client');
    return;
  }

  const binaryPath = getRustBinaryPath();
  if (!binaryPath) {
    logger.error('Rust binary not found, cannot start LSP client');
    return;
  }

  if (!lspClient) {
    lspClient = new TscannerLspClient(binaryPath);
    try {
      await lspClient.start(workspaceFolder.uri.fsPath);
      logger.info('LSP client started successfully');
    } catch (error) {
      logger.error(`Failed to start LSP client: ${error}`);
      lspClient = null;
    }
  }
}

export function dispose() {
  if (rustClient) {
    rustClient.stop();
    rustClient = null;
  }
  if (lspClient) {
    lspClient.stop();
    lspClient = null;
  }
}
