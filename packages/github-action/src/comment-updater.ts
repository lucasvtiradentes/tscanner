import * as core from '@actions/core';
import { createHash } from 'node:crypto';
import type { GitHub } from '@actions/github/lib/utils';
import type { ScanResult } from './scanner';

type Octokit = InstanceType<typeof GitHub>;

export type CommentUpdateParams = {
  octokit: Octokit;
  owner: string;
  repo: string;
  prNumber: number;
  scanResult: ScanResult;
  timezone: string;
  commitSha: string;
  commitMessage: string;
};

function pluralize(count: number, singular: string): string {
  return count === 1 ? singular : `${singular}s`;
}

function createFileHash(filePath: string): string {
  return createHash('sha256').update(filePath).digest('hex');
}

function buildGroupedByFileView(result: ScanResult, owner: string, repo: string, prNumber: number): string {
  const fileMap = new Map<string, Array<{ line: number; column: number; lineText: string; ruleName: string }>>();

  for (const group of result.ruleGroups) {
    for (const file of group.files) {
      if (!fileMap.has(file.filePath)) {
        fileMap.set(file.filePath, []);
      }
      for (const issue of file.issues) {
        fileMap.get(file.filePath)!.push({
          line: issue.line,
          column: issue.column,
          lineText: issue.lineText,
          ruleName: issue.ruleName || group.ruleName,
        });
      }
    }
  }

  let output = '';
  for (const [filePath, issues] of fileMap) {
    const summary = `<strong>${filePath}</strong> - ${issues.length} ${pluralize(issues.length, 'issue')}`;
    output += `<details>\n<summary>${summary}</summary>\n\n`;

    for (const issue of issues) {
      const fileUrl = `https://github.com/${owner}/${repo}/pull/${prNumber}/files#diff-${createFileHash(filePath)}R${issue.line}`;
      output += `- [Line ${issue.line}:${issue.column}](${fileUrl}) - **${issue.ruleName}** - \`${issue.lineText.trim()}\`\n`;
    }

    output += '\n</details>\n\n';
  }

  return output;
}

function buildGroupedByRuleView(result: ScanResult, owner: string, repo: string, prNumber: number): string {
  const ruleMap = new Map<
    string,
    { severity: 'error' | 'warning'; files: Map<string, Array<{ line: number; column: number; lineText: string }>> }
  >();

  for (const group of result.ruleGroups) {
    if (!ruleMap.has(group.ruleName)) {
      ruleMap.set(group.ruleName, { severity: group.severity, files: new Map() });
    }

    const ruleData = ruleMap.get(group.ruleName)!;

    for (const file of group.files) {
      if (!ruleData.files.has(file.filePath)) {
        ruleData.files.set(file.filePath, []);
      }
      for (const issue of file.issues) {
        ruleData.files.get(file.filePath)!.push({
          line: issue.line,
          column: issue.column,
          lineText: issue.lineText,
        });
      }
    }
  }

  let output = '';
  for (const [ruleName, ruleData] of ruleMap) {
    const totalIssues = Array.from(ruleData.files.values()).reduce((sum, issues) => sum + issues.length, 0);
    const icon = ruleData.severity === 'error' ? '✗' : '⚠';
    const summary = `${icon} <strong>${ruleName}</strong> - ${totalIssues} ${pluralize(totalIssues, 'issue')} - ${ruleData.files.size} ${pluralize(ruleData.files.size, 'file')}`;

    output += `<details>\n<summary>${summary}</summary>\n\n`;

    for (const [filePath, issues] of ruleData.files) {
      output += `\n**${filePath}**\n`;
      for (const issue of issues) {
        const fileUrl = `https://github.com/${owner}/${repo}/pull/${prNumber}/files#diff-${createFileHash(filePath)}R${issue.line}`;
        output += `- [Line ${issue.line}:${issue.column}](${fileUrl}) - \`${issue.lineText.trim()}\`\n`;
      }
    }

    output += '\n</details>\n\n';
  }

  return output;
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

function buildCommentBody(
  result: ScanResult,
  timezone: string,
  commitSha: string,
  commitMessage: string,
  owner: string,
  repo: string,
  prNumber: number,
): string {
  const timestamp = formatTimestamp(timezone);
  const { totalIssues, totalErrors, totalWarnings, totalFiles, totalRules } = result;

  if (totalIssues === 0) {
    return `<!-- tscanner-pr-comment -->
## ✅ tscanner - No Issues Found

All changed files passed validation!

---
**Last updated:** ${timestamp}
**Last commit analyzed:** \`${commitSha}\``;
  }

  let comment = `<!-- tscanner-pr-comment -->
## tscanner - Issues Found

**Issues:** ${totalIssues} (${totalErrors} ${pluralize(totalErrors, 'error')}, ${totalWarnings} ${pluralize(totalWarnings, 'warning')})
**Files:** ${totalFiles}
**Rules:** ${totalRules}

---

`;

  const groupedByFile = buildGroupedByFileView(result, owner, repo, prNumber);
  const groupedByRule = buildGroupedByRuleView(result, owner, repo, prNumber);

  comment += `<details>\n<summary><strong>📁 Issues grouped by file</strong></summary>\n\n${groupedByFile}\n</details>\n\n`;
  comment += `<details>\n<summary><strong>📋 Issues grouped by rule</strong></summary>\n\n${groupedByRule}\n</details>\n\n`;

  const commitInfo = commitMessage ? `\`${commitSha}\` - ${commitMessage}` : `\`${commitSha}\``;

  comment += `---\n**Last updated:** ${timestamp}  \n**Last commit analyzed:** ${commitInfo}`;

  return comment;
}

export async function updateOrCreateComment(params: CommentUpdateParams): Promise<void> {
  const { octokit, owner, repo, prNumber, scanResult, timezone, commitSha, commitMessage } = params;

  const comment = buildCommentBody(scanResult, timezone, commitSha, commitMessage, owner, repo, prNumber);

  const existingComments = await octokit.rest.issues.listComments({
    owner,
    repo,
    issue_number: prNumber,
  });

  const botComment = existingComments.data.find(
    (c) => c.user?.type === 'Bot' && c.body?.includes('<!-- tscanner-pr-comment -->'),
  );

  if (botComment) {
    await octokit.rest.issues.updateComment({
      owner,
      repo,
      comment_id: botComment.id,
      body: comment,
    });
    core.info(`Updated existing comment #${botComment.id}`);
  } else {
    await octokit.rest.issues.createComment({
      owner,
      repo,
      issue_number: prNumber,
      body: comment,
    });
    core.info('Created new comment');
  }
}
