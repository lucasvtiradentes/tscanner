import { Severity } from 'tscanner-common';

export const ICONS = {
  SUCCESS: '✅',
  ERROR: '❌',
  WARNING: '⚠️',
  ERROR_BADGE: '🔴',
  WARNING_BADGE: '🟡',
  RULE_ICON: '📋',
  FILE_ICON: '📁',
} as const;

export function getModeLabel(targetBranch?: string): string {
  return targetBranch ? `branch (${targetBranch})` : 'codebase';
}

export function getIssuesBreakdown(totalErrors: number, totalWarnings: number): string {
  if (totalErrors === 0 && totalWarnings === 0) return '';
  return ` (${ICONS.ERROR_BADGE} ${totalErrors}, ${ICONS.WARNING_BADGE} ${totalWarnings})`;
}

export function getStatusIcon(totalErrors: number): string {
  return totalErrors > 0 ? ICONS.ERROR : ICONS.WARNING;
}

export function getStatusTitle(totalErrors: number): string {
  return totalErrors > 0 ? 'Errors Found' : 'Warnings Found';
}

export function getSeverityBadge(severity: Severity): string {
  return severity === Severity.Error ? ICONS.ERROR_BADGE : ICONS.WARNING_BADGE;
}

export function formatCommitInfo(commitSha: string, commitMessage?: string): string {
  return commitMessage ? `\`${commitSha}\` - ${commitMessage}` : `\`${commitSha}\``;
}

export function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

type RuleSummary = {
  ruleName: string;
  issueCount: number;
  severity: Severity;
};

export function buildMostTriggeredTable(rules: RuleSummary[], limit = 5): string {
  const mostTriggered = [...rules].sort((a, b) => b.issueCount - a.issueCount).slice(0, limit);
  if (mostTriggered.length === 0) return '';

  let output = '<div align="center">\n\n';
  output += '**Most triggered rules:**\n\n';
  output += '| Rule | Issues |\n';
  output += '|------|--------|\n';
  for (const rule of mostTriggered) {
    const badge = getSeverityBadge(rule.severity);
    output += `| ${badge} \`${rule.ruleName}\` | ${rule.issueCount} |\n`;
  }
  output += '\n</div>\n';
  return output;
}
