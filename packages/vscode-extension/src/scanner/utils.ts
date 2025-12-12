import { CONFIG_ERROR_PREFIX, type Issue, PACKAGE_DISPLAY_NAME, REPO_URL } from 'tscanner-common';
import * as vscode from 'vscode';
import { LOG_FILE_PATH } from '../common/lib/logger';
import { openTextDocument } from '../common/lib/vscode-utils';
import { type IssueResult, parseSeverity } from '../common/types';

type ConfigError = {
  invalidFields: string[];
  version: string;
};

function getDocsUrl(version: string): string {
  return `${REPO_URL}/tree/vscode-extension-v${version}?tab=readme-ov-file#%EF%B8%8F-configuration`;
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
  const message = `${PACKAGE_DISPLAY_NAME} config error: invalid fields [${fieldsText}]. Please check the docs for v${configError.version}`;

  vscode.window.showErrorMessage(message, 'Open Docs').then((selection) => {
    if (selection === 'Open Docs') {
      vscode.env.openExternal(vscode.Uri.parse(getDocsUrl(configError.version)));
    }
  });
}

export function showScanErrorToast(error: unknown) {
  vscode.window
    .showErrorMessage(`${PACKAGE_DISPLAY_NAME}: Scan error: ${error}\n\nCheck logs at ${LOG_FILE_PATH}`, 'Open Logs')
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
    ruleType: issue.rule_type,
  };
}
