import { PACKAGE_DISPLAY_NAME, pluralize } from 'tscanner-common';
import { buildPrFileUrl } from '../../utils/url-builder';
import type { ActionScanResult } from '../scanner/scanner';
import {
  Alignment,
  ICONS,
  alignSection,
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

type ScanSummaryParams = {
  result: ActionScanResult;
  targetBranch?: string;
  timestamp?: string;
  commitSha?: string;
  commitMessage?: string;
};

type IssuesViewParams = {
  result: ActionScanResult;
  owner: string;
  repo: string;
  prNumber: number;
};

function buildScanSummaryTable(params: ScanSummaryParams): string {
  const { result, targetBranch, timestamp, commitSha, commitMessage } = params;
  const { totalIssues, totalErrors, totalWarnings, totalFiles, filesWithIssues, totalRules, totalEnabledRules } =
    result;
  const modeLabel = getModeLabel(targetBranch);
  const issuesBreakdown = getIssuesBreakdown(totalErrors, totalWarnings);

  let rows = `<tr><td>Issues found</td><td>${totalIssues}${issuesBreakdown}</td></tr>
<tr><td>Triggered rules</td><td>${totalRules}/${totalEnabledRules}</td></tr>
<tr><td>Files with issues</td><td>${filesWithIssues}/${totalFiles}</td></tr>
<tr><td>Scan mode</td><td>${modeLabel}</td></tr>`;

  if (commitSha) {
    const commitInfo = formatCommitInfo(commitSha, commitMessage);
    rows += `\n<tr><td>Last commit</td><td>${commitInfo}</td></tr>`;
  }

  if (timestamp) {
    rows += `\n<tr><td>Last updated</td><td>${timestamp}</td></tr>`;
  }

  return `<table>
<tr><th>Metric</th><th>Value</th></tr>
${rows}
</table>`;
}

export function buildCommitHistorySection(history: CommitHistoryEntry[]): string {
  if (history.length === 0) return '';

  let rows = '';
  for (const entry of history) {
    const label = formatCommitInfo(entry.sha, entry.message);
    rows += `<tr><td>${label}</td><td>${entry.totalIssues}</td><td>${entry.errors}</td><td>${entry.warnings}</td></tr>\n`;
  }

  const table = `<table>
<tr><th>Commit</th><th>Issues</th><th>Errors</th><th>Warnings</th></tr>
${rows}</table>`;

  const details = `<details>
<summary><strong>📈 Scan history</strong></summary>
<br />

${table}

</details>`;

  return `\n${alignSection(Alignment.Center, details)}\n`;
}

function buildIssuesByRuleSection(params: IssuesViewParams): string {
  const { result, owner, repo, prNumber } = params;
  const { ruleGroupsByRule } = result;

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
        const lineText = issue.lineText?.trim() || '';
        content += `- <a href="${fileUrl}">${issue.line}:${issue.column}</a> - <code>${escapeHtml(lineText)}</code>\n`;
      }

      content += '\n';
    }

    content += '</details>\n\n';
  }

  const innerContent = alignSection(Alignment.Left, content);

  const details = `<details>
<summary><strong>${ICONS.RULE_ICON} Issues grouped by rule</strong></summary>
<br />

${innerContent}
</details>`;

  return `${alignSection(Alignment.Center, details)}\n\n`;
}

function buildIssuesByFileSection(params: IssuesViewParams): string {
  const { result, owner, repo, prNumber } = params;

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
        const lineText = issue.lineText?.trim() || '';
        content += `- <a href="${fileUrl}">${issue.line}:${issue.column}</a> - <code>${escapeHtml(lineText)}</code>\n`;
      }

      content += '\n';
    }

    content += '</details>\n\n';
  }

  const innerContent = alignSection(Alignment.Left, content);

  const details = `<details>
<summary><strong>${ICONS.FILE_ICON} Issues grouped by file</strong></summary>
<br />

${innerContent}
</details>`;

  return `${alignSection(Alignment.Center, details)}\n\n`;
}

function buildScanHeader(totalErrors: number, hasIssues: boolean): string {
  if (!hasIssues) {
    return `## ${ICONS.SUCCESS} ${PACKAGE_DISPLAY_NAME} - No Issues Found`;
  }
  const icon = getStatusIcon(totalErrors);
  const title = getStatusTitle(totalErrors);
  return `## ${icon} ${PACKAGE_DISPLAY_NAME} - ${title}`;
}

function buildNoIssuesTable(modeLabel: string, commitInfo?: string, timestamp?: string): string {
  let rows = `<tr><td>Issues</td><td>0</td></tr>
<tr><td>Scan mode</td><td>${modeLabel}</td></tr>`;

  if (commitInfo) {
    rows += `\n<tr><td>Last commit</td><td>${commitInfo}</td></tr>`;
  }

  if (timestamp) {
    rows += `\n<tr><td>Last updated</td><td>${timestamp}</td></tr>`;
  }

  return `<table>
<tr><th>Metric</th><th>Value</th></tr>
${rows}
</table>`;
}

type BuildReportParams = {
  result: ActionScanResult;
  targetBranch?: string;
  timestamp?: string;
  commitSha?: string;
  commitMessage?: string;
  extraSection?: string;
  issuesViewParams?: IssuesViewParams;
};

export function buildSuccessReport(params: BuildReportParams): string {
  const { targetBranch, timestamp, commitSha, commitMessage, extraSection } = params;
  const header = buildScanHeader(0, false);
  const modeLabel = getModeLabel(targetBranch);
  const commitInfo = commitSha ? formatCommitInfo(commitSha, commitMessage) : undefined;
  const table = buildNoIssuesTable(modeLabel, commitInfo, timestamp);

  let report = `${header}

${alignSection(Alignment.Center, table)}`;

  if (extraSection) {
    report += `\n${extraSection}`;
  }

  return report;
}

export function buildIssuesReport(params: BuildReportParams): string {
  const { result, targetBranch, timestamp, commitSha, commitMessage, extraSection, issuesViewParams } = params;
  const { totalErrors } = result;

  const header = buildScanHeader(totalErrors, true);
  const statsTable = buildScanSummaryTable({ result, targetBranch, timestamp, commitSha, commitMessage });

  let report = `${header}

${alignSection(Alignment.Center, statsTable)}
`;

  if (issuesViewParams) {
    report += buildIssuesByRuleSection(issuesViewParams);
    report += buildIssuesByFileSection(issuesViewParams);
  }

  if (extraSection) {
    report += extraSection;
  }

  return report;
}
