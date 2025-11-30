import { Severity, pluralize } from 'tscanner-common';
import { COMMENT_MARKER } from '../constants';
import { type Octokit, githubHelper } from '../lib/actions-helper';
import { formatTimestamp } from '../utils/format-timestamp';
import { buildPrFileUrl } from '../utils/url-builder';
import type { RuleGroup, ScanResult } from './scanner';

const ICONS = {
  SUCCESS: '✅',
  ERROR: '❌',
  WARNING: '⚠️',
  ERROR_BADGE: '🔴',
  WARNING_BADGE: '🟡',
  RULE_ICON: '📋',
  FILE_ICON: '📁',
} as const;

type CommitHistory = {
  sha: string;
  message: string;
  totalIssues: number;
  errors: number;
  warnings: number;
};

function parseCommitHistory(commentBody: string): CommitHistory[] {
  const historyMatch = commentBody.match(/<!-- COMMIT_HISTORY:(.*?)-->/s);
  if (!historyMatch) return [];

  try {
    return JSON.parse(historyMatch[1]) as CommitHistory[];
  } catch {
    return [];
  }
}

function serializeCommitHistory(history: CommitHistory[]): string {
  return `<!-- COMMIT_HISTORY:${JSON.stringify(history)}-->`;
}

export type CommentUpdateParams = {
  octokit: Octokit;
  owner: string;
  repo: string;
  prNumber: number;
  scanResult: ScanResult;
  timezone: string;
  commitSha: string;
  commitMessage: string;
  targetBranch?: string;
};

function buildStatsTable(result: ScanResult, targetBranch?: string): string {
  const { totalIssues, totalErrors, totalWarnings, totalFiles, totalRules } = result;
  const modeLabel = targetBranch ? `branch (${targetBranch})` : 'codebase';

  return `| Metric | Value |
|--------|-------|
| Total Issues | ${totalIssues} |
| Errors | ${totalErrors} |
| Warnings | ${totalWarnings} |
| Files | ${totalFiles} |
| Rules | ${totalRules} |
| Mode | ${modeLabel} |`;
}

function buildTopOffenders(ruleGroups: RuleGroup[], limit = 5): string {
  const sorted = [...ruleGroups].sort((a, b) => b.issueCount - a.issueCount).slice(0, limit);

  if (sorted.length === 0) return '';

  let output = '\n**Top offenders:**\n';
  for (const rule of sorted) {
    const badge = rule.severity === Severity.Error ? ICONS.ERROR_BADGE : ICONS.WARNING_BADGE;
    output += `- ${badge} \`${rule.ruleName}\` - ${rule.issueCount} ${pluralize(rule.issueCount, 'issue')}\n`;
  }
  return output;
}

function buildCommitHistorySection(history: CommitHistory[]): string {
  if (history.length === 0) return '';

  let output = '\n<details>\n<summary><strong>📈 Scan history</strong></summary>\n<br />\n\n';
  output += '| Commit | Issues | Errors | Warnings |\n';
  output += '|--------|--------|--------|----------|\n';

  for (const entry of history) {
    const label = entry.message ? `\`${entry.sha}\` - ${entry.message}` : `\`${entry.sha}\``;
    output += `| ${label} | ${entry.totalIssues} | ${entry.errors} | ${entry.warnings} |\n`;
  }

  output += '\n</details>\n';
  return output;
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

function buildGroupedByFileView(result: ScanResult, owner: string, repo: string, prNumber: number): string {
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

  let output = '';
  for (const [filePath, ruleMap] of fileMap) {
    const totalIssues = Array.from(ruleMap.values()).reduce((sum, issues) => sum + issues.length, 0);
    const ruleCount = ruleMap.size;
    const summary = `<strong>${filePath}</strong> - ${totalIssues} ${pluralize(totalIssues, 'issue')} - ${ruleCount} ${pluralize(ruleCount, 'rule')}`;
    output += `<details>\n<summary>${summary}</summary>\n<br />\n\n`;

    for (const [ruleName, issues] of ruleMap) {
      output += `<strong>${ruleName}</strong> - ${issues.length} ${pluralize(issues.length, 'issue')}\n\n`;

      for (const issue of issues) {
        const fileUrl = buildPrFileUrl(owner, repo, prNumber, filePath, issue.line);
        output += `- <a href="${fileUrl}">${issue.line}:${issue.column}</a> - <code>${escapeHtml(issue.lineText.trim())}</code>\n`;
      }

      output += '\n';
    }

    output += '</details>\n\n';
  }

  return output;
}

function buildGroupedByRuleView(ruleGroups: RuleGroup[], owner: string, repo: string, prNumber: number): string {
  let output = '';

  for (const group of ruleGroups) {
    const badge = group.severity === Severity.Error ? ICONS.ERROR_BADGE : ICONS.WARNING_BADGE;
    const summary = `${badge} <strong>${group.ruleName}</strong> - ${group.issueCount} ${pluralize(group.issueCount, 'issue')} - ${group.fileCount} ${pluralize(group.fileCount, 'file')}`;

    output += `<details>\n<summary>${summary}</summary>\n<br />\n\n`;

    for (const file of group.files) {
      const fileIssueCount = file.issues.length;
      output += `<strong>${file.filePath}</strong> - ${fileIssueCount} ${pluralize(fileIssueCount, 'issue')}\n\n`;

      for (const issue of file.issues) {
        const fileUrl = buildPrFileUrl(owner, repo, prNumber, file.filePath, issue.line);
        output += `- <a href="${fileUrl}">${issue.line}:${issue.column}</a> - <code>${escapeHtml(issue.lineText.trim())}</code>\n`;
      }

      output += '\n';
    }

    output += '</details>\n\n';
  }

  return output;
}

function buildCommentBody(
  result: ScanResult,
  timezone: string,
  commitSha: string,
  commitMessage: string,
  owner: string,
  repo: string,
  prNumber: number,
  targetBranch: string | undefined,
  commitHistory: CommitHistory[],
): string {
  const timestamp = formatTimestamp(timezone);
  const { totalIssues, totalErrors, ruleGroupsByRule } = result;
  const modeLabel = targetBranch ? `branch (${targetBranch})` : 'codebase';

  if (totalIssues === 0) {
    const commitInfo = commitMessage ? `\`${commitSha}\` - ${commitMessage}` : `\`${commitSha}\``;
    const historySection = buildCommitHistorySection(commitHistory);
    const historyData = serializeCommitHistory(commitHistory);

    return `${COMMENT_MARKER}
${historyData}
## ${ICONS.SUCCESS} TScanner - No Issues Found

| Metric | Value |
|--------|-------|
| Total Issues | 0 |
| Mode | ${modeLabel} |

All files passed validation!
${historySection}
---
**Last updated:** ${timestamp}
**Last commit analyzed:** ${commitInfo}`;
  }

  const icon = totalErrors > 0 ? ICONS.ERROR : ICONS.WARNING;
  const title = totalErrors > 0 ? 'Errors Found' : 'Warnings Found';
  const statsTable = buildStatsTable(result, targetBranch);
  const topOffenders = buildTopOffenders(ruleGroupsByRule);

  let comment = `${COMMENT_MARKER}
${serializeCommitHistory(commitHistory)}
## ${icon} TScanner - ${title}

${statsTable}
${topOffenders}
---

`;

  const { totalFiles, totalRules } = result;
  const groupedByRule = buildGroupedByRuleView(ruleGroupsByRule, owner, repo, prNumber);
  const groupedByFile = buildGroupedByFileView(result, owner, repo, prNumber);
  const historySection = buildCommitHistorySection(commitHistory);

  comment += `<div align="center">\n\n<details>\n<summary><strong>${ICONS.RULE_ICON} Issues grouped by rule (${totalRules})</strong></summary>\n<br />\n\n<div align="left">\n${groupedByRule}\n</div></details>\n\n</div>\n\n---\n\n`;
  comment += `<div align="center">\n\n<details>\n<summary><strong>${ICONS.FILE_ICON} Issues grouped by file (${totalFiles})</strong></summary>\n<br />\n\n<div align="left">\n${groupedByFile}\n</div></details>\n\n</div>\n\n`;
  comment += historySection;

  const commitInfo = commitMessage ? `\`${commitSha}\` - ${commitMessage}` : `\`${commitSha}\``;

  comment += `---\n**Last updated:** ${timestamp}  \n**Last commit analyzed:** ${commitInfo}`;

  return comment;
}

const MAX_HISTORY_ENTRIES = 10;

export async function updateOrCreateComment(params: CommentUpdateParams) {
  const { octokit, owner, repo, prNumber, scanResult, timezone, commitSha, commitMessage, targetBranch } = params;

  const existingComments = await octokit.rest.issues.listComments({
    owner,
    repo,
    issue_number: prNumber,
  });

  const botComment = existingComments.data.find((c) => c.user?.type === 'Bot' && c.body?.includes(COMMENT_MARKER));

  let commitHistory: CommitHistory[] = [];
  if (botComment?.body) {
    commitHistory = parseCommitHistory(botComment.body);
  }

  const existingEntry = commitHistory.find((h) => h.sha === commitSha);
  if (!existingEntry) {
    commitHistory.unshift({
      sha: commitSha,
      message: commitMessage,
      totalIssues: scanResult.totalIssues,
      errors: scanResult.totalErrors,
      warnings: scanResult.totalWarnings,
    });
    commitHistory = commitHistory.slice(0, MAX_HISTORY_ENTRIES);
  }

  const comment = buildCommentBody(
    scanResult,
    timezone,
    commitSha,
    commitMessage,
    owner,
    repo,
    prNumber,
    targetBranch,
    commitHistory,
  );

  if (botComment) {
    await octokit.rest.issues.updateComment({
      owner,
      repo,
      comment_id: botComment.id,
      body: comment,
    });
    githubHelper.logInfo(`Updated existing comment #${botComment.id}`);
  } else {
    await octokit.rest.issues.createComment({
      owner,
      repo,
      issue_number: prNumber,
      body: comment,
    });
    githubHelper.logInfo('Created new comment');
  }
}
