import { existsSync } from 'node:fs';
import { join } from 'node:path';
import * as vscode from 'vscode';
import { BINARY_BASE_NAME, PLATFORM_TARGET_MAP, getServerBinaryName } from '../common/constants';
import { getExtensionPath } from '../common/lib/extension-helper';
import { LOG_FILE_PATH, logger } from '../common/lib/logger';
import { openTextDocument } from '../common/lib/vscode-utils';
import type { Issue, IssueResult } from '../common/types';
import { parseSeverity } from '../common/types';

const CONFIG_ERROR_PREFIX = 'TSCANNER_CONFIG_ERROR:';

type ConfigError = {
  invalidFields: string[];
  version: string;
};

function getDocsUrl(version: string): string {
  return `https://github.com/lucasvtiradentes/tscanner/tree/vscode-extension-v${version}?tab=readme-ov-file#%EF%B8%8F-configuration`;
}

export function parseConfigError(errorMessage: string): ConfigError | null {
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

export function showConfigErrorToast(configError: ConfigError) {
  const fieldsText = configError.invalidFields.join(', ');
  const message = `TScanner config error: invalid fields [${fieldsText}]. Please check the docs for v${configError.version}`;

  vscode.window.showErrorMessage(message, 'Open Docs').then((selection) => {
    if (selection === 'Open Docs') {
      vscode.env.openExternal(vscode.Uri.parse(getDocsUrl(configError.version)));
    }
  });
}

export function showScanErrorToast(error: unknown) {
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

export function showBinaryNotFoundError() {
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
}

export function mapIssueToResult(uri: vscode.Uri, issue: Issue, lineText?: string): IssueResult {
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
