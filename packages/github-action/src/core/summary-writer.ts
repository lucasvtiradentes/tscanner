import { Severity, pluralize } from 'tscanner-common';
import { githubHelper } from '../lib/actions-helper';
import type { ScanResult } from './scanner';

const ICONS = {
  SUCCESS: '✅',
  ERROR: '❌',
  WARNING: '⚠️',
  ERROR_BADGE: '🔴',
  WARNING_BADGE: '🟡',
} as const;

export function writeSummary(scanResult: ScanResult, targetBranch?: string): void {
  const { totalIssues, totalErrors, totalWarnings, totalFiles, totalRules, ruleGroupsByRule } = scanResult;

  const modeLabel = targetBranch ? `branch (${targetBranch})` : 'codebase';

  if (totalIssues === 0) {
    const summary = `## ${ICONS.SUCCESS} TScanner - No Issues Found

| Metric | Value |
|--------|-------|
| Scan mode | ${modeLabel} |

All files passed validation!`;
    githubHelper.writeSummary(summary);
    return;
  }

  const icon = totalErrors > 0 ? ICONS.ERROR : ICONS.WARNING;
  const title = totalErrors > 0 ? 'Errors Found' : 'Warnings Found';

  const issuesBreakdown = ` (${ICONS.ERROR_BADGE} ${totalErrors}, ${ICONS.WARNING_BADGE} ${totalWarnings})`;

  let summary = `## ${icon} TScanner - ${title}

| Metric | Value |
|--------|-------|
| Issues | ${totalIssues}${issuesBreakdown} |
| Scanned files | ${totalFiles} |
| Triggered rules | ${totalRules} |
| Scan mode | ${modeLabel} |

`;

  const mostTriggered = [...ruleGroupsByRule].sort((a, b) => b.issueCount - a.issueCount).slice(0, 5);

  if (mostTriggered.length > 0) {
    summary += '**Most triggered rules:**\n';
    for (const rule of mostTriggered) {
      const badge = rule.severity === Severity.Error ? ICONS.ERROR_BADGE : ICONS.WARNING_BADGE;
      summary += `- ${badge} \`${rule.ruleName}\` - ${rule.issueCount} ${pluralize(rule.issueCount, 'issue')}\n`;
    }
  }

  githubHelper.writeSummary(summary);
}
