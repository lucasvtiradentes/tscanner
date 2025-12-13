import { DISPLAY_ICONS, Severity } from 'tscanner-common';

export enum Alignment {
  Center = 'center',
  Left = 'left',
}

export function alignSection(alignment: Alignment, content: string): string {
  return `<div align="${alignment}">\n${content}\n</div>`;
}

export const ICONS = {
  SUCCESS: DISPLAY_ICONS.success,
  ERROR: DISPLAY_ICONS.error,
  WARNING: DISPLAY_ICONS.warning,
  ERROR_BADGE: DISPLAY_ICONS.error,
  WARNING_BADGE: DISPLAY_ICONS.warning,
  INFO_BADGE: DISPLAY_ICONS.info,
  HINT_BADGE: DISPLAY_ICONS.hint,
  RULE_ICON: '📋',
  FILE_ICON: '📁',
  BUILTIN: DISPLAY_ICONS.builtin,
  REGEX: DISPLAY_ICONS.regex,
  SCRIPT: DISPLAY_ICONS.script,
  AI: DISPLAY_ICONS.ai,
} as const;

export function getModeLabel(targetBranch?: string): string {
  return targetBranch ? `branch (${targetBranch})` : 'codebase';
}

export function getIssuesBreakdown(
  totalErrors: number,
  totalWarnings: number,
  totalInfos: number,
  totalHints: number,
): string {
  const parts: string[] = [];
  if (totalErrors > 0) parts.push(`${ICONS.ERROR_BADGE} ${totalErrors}`);
  if (totalWarnings > 0) parts.push(`${ICONS.WARNING_BADGE} ${totalWarnings}`);
  if (totalInfos > 0) parts.push(`${ICONS.INFO_BADGE} ${totalInfos}`);
  if (totalHints > 0) parts.push(`${ICONS.HINT_BADGE} ${totalHints}`);
  return parts.length > 0 ? ` (${parts.join(', ')})` : '';
}

export function getRulesBreakdown(breakdown: { builtin: number; regex: number; script: number; ai: number }): string {
  const parts: string[] = [];
  if (breakdown.builtin > 0) parts.push(`${ICONS.BUILTIN} ${breakdown.builtin}`);
  if (breakdown.regex > 0) parts.push(`${ICONS.REGEX} ${breakdown.regex}`);
  if (breakdown.script > 0) parts.push(`${ICONS.SCRIPT} ${breakdown.script}`);
  if (breakdown.ai > 0) parts.push(`${ICONS.AI} ${breakdown.ai}`);
  return parts.length > 0 ? ` (${parts.join(', ')})` : '';
}

export function getStatusIcon(totalErrors: number): string {
  return totalErrors > 0 ? ICONS.ERROR : ICONS.WARNING;
}

export function getStatusTitle(totalErrors: number): string {
  return totalErrors > 0 ? 'Errors Found' : 'Warnings Found';
}

export function getSeverityBadge(severity: Severity): string {
  switch (severity) {
    case Severity.Error:
      return ICONS.ERROR_BADGE;
    case Severity.Warning:
      return ICONS.WARNING_BADGE;
    case Severity.Info:
      return ICONS.INFO_BADGE;
    case Severity.Hint:
      return ICONS.HINT_BADGE;
  }
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
