import { Severity } from 'tscanner-common';
import { githubHelper } from '../lib/actions-helper';
import type { ScanResult } from './scanner';

export function writeAnnotations(scanResult: ScanResult): void {
  for (const group of scanResult.ruleGroups) {
    for (const file of group.files) {
      for (const issue of file.issues) {
        const ruleName = issue.ruleName ?? group.ruleName;
        const message = `[${ruleName}] ${issue.message}`;
        const properties = {
          title: ruleName,
          file: file.filePath,
          startLine: issue.line,
          startColumn: issue.column,
        };

        if (group.severity === Severity.Error) {
          githubHelper.addAnnotationError(message, properties);
        } else {
          githubHelper.addAnnotationWarning(message, properties);
        }
      }
    }
  }
}
