import { Severity } from 'tscanner-common';

export enum Alignment {
  Center = 'center',
  Left = 'left',
}

export function alignSection(alignment: Alignment, content: string): string {
  return `<div align="${alignment}">\n${content}\n</div>`;
}

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
  return commitMessage ? `<code>${commitSha}</code> - ${commitMessage}` : `<code>${commitSha}</code>`;
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

  let rows = '';
  for (const rule of mostTriggered) {
    const badge = getSeverityBadge(rule.severity);
    rows += `<tr><td>${badge} <code>${rule.ruleName}</code></td><td>${rule.issueCount}</td></tr>\n`;
  }

  const table = `**Most triggered rules:**

<table>
<tr><th>Rule</th><th>Issues</th></tr>
${rows}</table>`;

  return `${alignSection(Alignment.Center, table)}\n`;
}
