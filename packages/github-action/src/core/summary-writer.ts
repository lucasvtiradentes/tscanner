import { Severity, pluralize } from 'tscanner-common';
import { githubHelper } from '../lib/actions-helper';
import type { ScanResult } from './scanner';

export function writeSummary(scanResult: ScanResult, targetBranch?: string): void {
  const { totalIssues, totalErrors, totalWarnings, totalFiles, totalRules, ruleGroupsByRule } = scanResult;

  const modeLabel = targetBranch ? `branch (${targetBranch})` : 'codebase';

  if (totalIssues === 0) {
    const summary = `## ✅ TScanner - No Issues Found

| Metric | Value |
|--------|-------|
| Mode | ${modeLabel} |

All files passed validation!`;
    githubHelper.writeSummary(summary);
    return;
  }

  const icon = totalErrors > 0 ? '❌' : '⚠️';
  const title = totalErrors > 0 ? 'Errors Found' : 'Warnings Found';

  let summary = `## ${icon} TScanner - ${title}

| Metric | Value |
|--------|-------|
| Total Issues | ${totalIssues} |
| Errors | ${totalErrors} |
| Warnings | ${totalWarnings} |
| Files | ${totalFiles} |
| Rules | ${totalRules} |
| Mode | ${modeLabel} |

`;

  const topOffenders = [...ruleGroupsByRule].sort((a, b) => b.issueCount - a.issueCount).slice(0, 5);

  if (topOffenders.length > 0) {
    summary += '**Top offenders:**\n';
    for (const rule of topOffenders) {
      const badge = rule.severity === Severity.Error ? '🔴' : '🟡';
      summary += `- ${badge} \`${rule.ruleName}\` - ${rule.issueCount} ${pluralize(rule.issueCount, 'issue')}\n`;
    }
  }

  githubHelper.writeSummary(summary);
}
