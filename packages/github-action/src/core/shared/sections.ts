import { pluralize } from 'tscanner-common';
import { buildPrFileUrl } from '../../utils/url-builder';
import type { ScanResult } from '../scanner';
import {
  ICONS,
  escapeHtml,
  formatCommitInfo,
  getIssuesBreakdown,
  getModeLabel,
  getSeverityBadge,
  getStatusIcon,
  getStatusTitle,
} from './formatting';

export type CommitHistoryEntry = {
  sha: string;
  message: string;
  totalIssues: number;
  errors: number;
  warnings: number;
};

export type ScanSummaryParams = {
  result: ScanResult;
  targetBranch?: string;
  timestamp?: string;
  commitSha?: string;
  commitMessage?: string;
};

export type IssuesViewParams = {
  result: ScanResult;
  owner: string;
  repo: string;
  prNumber: number;
};

export function buildScanSummaryTable(params: ScanSummaryParams): string {
  const { result, targetBranch, timestamp, commitSha, commitMessage } = params;
  const { totalIssues, totalErrors, totalWarnings, totalFiles, totalRules } = result;
  const modeLabel = getModeLabel(targetBranch);
  const issuesBreakdown = getIssuesBreakdown(totalErrors, totalWarnings);

  let table = `| Metric | Value |
|--------|-------|
| Issues found | ${totalIssues}${issuesBreakdown} |
| Scanned files | ${totalFiles} |
| Triggered rules | ${totalRules} |
| Scan mode | ${modeLabel} |`;

  if (commitSha) {
    const commitInfo = formatCommitInfo(commitSha, commitMessage);
    table += `\n| Last commit | ${commitInfo} |`;
  }

  if (timestamp) {
    table += `\n| Last updated | ${timestamp} |`;
  }

  return table;
}

export function buildCommitHistorySection(history: CommitHistoryEntry[]): string {
  if (history.length === 0) return '';

  let output = '\n<div align="center"><details>\n<summary><strong>📈 Scan history</strong></summary>\n<br />\n\n';
  output += '| Commit | Issues | Errors | Warnings |\n';
  output += '|--------|--------|--------|----------|\n';

  for (const entry of history) {
    const label = formatCommitInfo(entry.sha, entry.message);
    output += `| ${label} | ${entry.totalIssues} | ${entry.errors} | ${entry.warnings} |\n`;
  }

  output += '\n</details></div>\n';
  return output;
}

export function buildIssuesByRuleSection(params: IssuesViewParams): string {
  const { result, owner, repo, prNumber } = params;
  const { ruleGroupsByRule, totalRules } = result;

  let content = '';
  for (const group of ruleGroupsByRule) {
    const badge = getSeverityBadge(group.severity);
    const summary = `${badge} <strong>${group.ruleName}</strong> - ${group.issueCount} ${pluralize(group.issueCount, 'issue')} - ${group.fileCount} ${pluralize(group.fileCount, 'file')}`;

    content += `<details>\n<summary>${summary}</summary>\n<br />\n\n`;

    for (const file of group.files) {
      const fileIssueCount = file.issues.length;
      content += `<strong>${file.filePath}</strong> - ${fileIssueCount} ${pluralize(fileIssueCount, 'issue')}\n\n`;

      for (const issue of file.issues) {
        const fileUrl = buildPrFileUrl(owner, repo, prNumber, file.filePath, issue.line);
        content += `- <a href="${fileUrl}">${issue.line}:${issue.column}</a> - <code>${escapeHtml(issue.lineText.trim())}</code>\n`;
      }

      content += '\n';
    }

    content += '</details>\n\n';
  }

  return `<div align="center">

<details>
<summary><strong>${ICONS.RULE_ICON} Issues grouped by rule (${totalRules})</strong></summary>
<br />

<div align="left">
${content}
</div></details>

</div>

`;
}

export function buildIssuesByFileSection(params: IssuesViewParams): string {
  const { result, owner, repo, prNumber } = params;
  const { totalFiles } = result;

  const fileMap = new Map<string, Map<string, Array<{ line: number; column: number; lineText: string }>>>();

  for (const group of result.ruleGroups) {
    for (const file of group.files) {
      if (!fileMap.has(file.filePath)) {
        fileMap.set(file.filePath, new Map());
      }
      const ruleMap = fileMap.get(file.filePath)!;

      const ruleName = file.issues[0]?.ruleName || group.ruleName;
      if (!ruleMap.has(ruleName)) {
        ruleMap.set(ruleName, []);
      }

      for (const issue of file.issues) {
        ruleMap.get(ruleName)!.push({
          line: issue.line,
          column: issue.column,
          lineText: issue.lineText,
        });
      }
    }
  }

  let content = '';
  for (const [filePath, ruleMap] of fileMap) {
    const issueCount = Array.from(ruleMap.values()).reduce((sum, issues) => sum + issues.length, 0);
    const ruleCount = ruleMap.size;
    const summary = `<strong>${filePath}</strong> - ${issueCount} ${pluralize(issueCount, 'issue')} - ${ruleCount} ${pluralize(ruleCount, 'rule')}`;
    content += `<details>\n<summary>${summary}</summary>\n<br />\n\n`;

    for (const [ruleName, issues] of ruleMap) {
      content += `<strong>${ruleName}</strong> - ${issues.length} ${pluralize(issues.length, 'issue')}\n\n`;

      for (const issue of issues) {
        const fileUrl = buildPrFileUrl(owner, repo, prNumber, filePath, issue.line);
        content += `- <a href="${fileUrl}">${issue.line}:${issue.column}</a> - <code>${escapeHtml(issue.lineText.trim())}</code>\n`;
      }

      content += '\n';
    }

    content += '</details>\n\n';
  }

  return `<div align="center">

<details>
<summary><strong>${ICONS.FILE_ICON} Issues grouped by file (${totalFiles})</strong></summary>
<br />

<div align="left">
${content}
</div></details>

</div>

`;
}

export function buildScanHeader(totalErrors: number, hasIssues: boolean): string {
  if (!hasIssues) {
    return `## ${ICONS.SUCCESS} TScanner - No Issues Found`;
  }
  const icon = getStatusIcon(totalErrors);
  const title = getStatusTitle(totalErrors);
  return `## ${icon} TScanner - ${title}`;
}

export function buildNoIssuesMessage(): string {
  return 'All files passed validation!';
}
