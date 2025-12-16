import { CONFIG_ERROR_PREFIX, type Issue } from 'tscanner-common';
import type * as vscode from 'vscode';
import { type IssueResult, parseSeverity } from '../common/types';

type ConfigError = {
  invalidFields: string[];
  version: string;
};

export function parseConfigError(errorMessage: string): ConfigError | null {
  if (errorMessage.includes(CONFIG_ERROR_PREFIX)) {
    const match = errorMessage.match(/invalid_fields=\[([^\]]*)\];version=([^\s]+)/);
    if (match) {
      const [, fieldsStr, version] = match;
      const invalidFields = fieldsStr.split(',').filter(Boolean);
      return { invalidFields, version };
    }
  }

  const invalidFieldsWarningMatch = errorMessage.match(
    /Config contains invalid fields \[([^\]]+)\] which will be ignored/,
  );
  if (invalidFieldsWarningMatch) {
    const fieldsStr = invalidFieldsWarningMatch[1];
    const invalidFields = fieldsStr
      .split(',')
      .map((f) => f.trim())
      .filter(Boolean);
    return {
      invalidFields,
      version: 'unknown',
    };
  }

  const invalidFieldMatch = errorMessage.match(/Invalid field: (\w+)/);
  if (invalidFieldMatch) {
    return {
      invalidFields: [invalidFieldMatch[1]],
      version: 'unknown',
    };
  }

  return null;
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
