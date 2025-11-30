import { Severity } from 'tscanner-common';
import { githubHelper } from '../lib/actions-helper';
import type { ScanResult } from './scanner';

export function writeAnnotations(scanResult: ScanResult): void {
  githubHelper.logInfo('');
  githubHelper.logInfo('📝 Writing annotations...');
  githubHelper.logInfo(`ruleGroups count: ${scanResult.ruleGroups.length}`);
  githubHelper.logInfo(`ruleGroupsByRule count: ${scanResult.ruleGroupsByRule.length}`);

  let annotationCount = 0;

  for (const group of scanResult.ruleGroupsByRule) {
    githubHelper.logInfo(
      `Processing rule: ${group.ruleName} (${group.issueCount} issues, ${group.files.length} files)`,
    );

    for (const file of group.files) {
      githubHelper.logInfo(`  File: ${file.filePath} (${file.issues.length} issues)`);

      for (const issue of file.issues) {
        const ruleName = issue.ruleName ?? group.ruleName;
        const message = `[${ruleName}] ${issue.message}`;
        const properties = {
          title: ruleName,
          file: file.filePath,
          startLine: issue.line,
          startColumn: issue.column,
        };

        githubHelper.logInfo(`    -> Line ${issue.line}: ${ruleName}`);

        if (group.severity === Severity.Error) {
          githubHelper.addAnnotationError(message, properties);
        } else {
          githubHelper.addAnnotationWarning(message, properties);
        }
        annotationCount++;
      }
    }
  }

  githubHelper.logInfo(`Total annotations written: ${annotationCount}`);
}
