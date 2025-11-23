import type { ScanResult } from './scanner';

export function formatComment(result: ScanResult, timezone: string, commitSha: string): string {
  const timestamp = formatTimestamp(timezone);
  const { totalIssues, totalErrors, totalWarnings, ruleGroups } = result;

  if (totalIssues === 0) {
    return `<!-- tscanner-pr-comment -->
## ✅ tscanner - No Issues Found

All changed files passed validation!

---
**Last updated:** ${timestamp}
**Last commit analyzed:** \`${commitSha}\``;
  }

  const errorIcon = totalErrors > 0 ? '❌' : '';
  const warningIcon = totalWarnings > 0 ? '⚠️' : '';
  const statusIcon = totalErrors > 0 ? '❌' : '⚠️';

  let comment = `<!-- tscanner-pr-comment -->
## ${statusIcon} tscanner - Issues Found

**Summary:** ${errorIcon} ${totalErrors} error(s) ${warningIcon} ${totalWarnings} warning(s)

---

`;

  for (const group of ruleGroups) {
    const icon = group.severity === 'error' ? '❌' : '⚠️';
    const summary = `${icon} **${group.ruleName}** - ${group.issueCount} issue(s) - ${group.fileCount} file(s)`;

    comment += `<details>\n<summary>${summary}</summary>\n\n`;

    for (const file of group.files) {
      comment += `\n**${file.filePath}**\n`;
      for (const issue of file.issues) {
        comment += `- Line ${issue.line}:${issue.column} - \`${issue.lineText.trim()}\`\n`;
      }
    }

    comment += '\n</details>\n\n';
  }

  comment += `---\n**Last updated:** ${timestamp}  \n**Last commit analyzed:** \`${commitSha}\``;

  return comment;
}

function formatTimestamp(timezone: string): string {
  const now = new Date();

  try {
    const formatted = now.toLocaleString('en-US', {
      timeZone: timezone,
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false,
    });

    const offset = getTimezoneOffset(timezone);
    return `${formatted} (UTC${offset})`;
  } catch {
    return `${now.toISOString()} (UTC)`;
  }
}

function getTimezoneOffset(timezone: string): string {
  if (timezone === 'UTC') return '';

  try {
    const now = new Date();
    const utcDate = new Date(now.toLocaleString('en-US', { timeZone: 'UTC' }));
    const tzDate = new Date(now.toLocaleString('en-US', { timeZone: timezone }));
    const offset = (tzDate.getTime() - utcDate.getTime()) / (1000 * 60 * 60);

    if (offset === 0) return '';
    const sign = offset > 0 ? '+' : '';
    return `${sign}${offset}`;
  } catch {
    return '';
  }
}
