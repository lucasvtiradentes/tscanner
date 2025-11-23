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
  return createHash('sha256').update(filePath).digest('hex').substring(0, 32);
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
  const { totalIssues, totalErrors, totalWarnings, ruleGroups } = result;

  if (totalIssues === 0) {
    return `<!-- tscanner-pr-comment -->
## ✅ tscanner - No Issues Found

All changed files passed validation!

---
**Last updated:** ${timestamp}
**Last commit analyzed:** \`${commitSha}\``;
  }

  const errorIcon = totalErrors > 0 ? '✗' : '';
  const warningIcon = totalWarnings > 0 ? '⚠' : '';
  const statusIcon = totalErrors > 0 ? '✗' : '⚠';

  const errorText = totalErrors > 0 ? `${errorIcon} ${totalErrors} ${pluralize(totalErrors, 'error')}` : '';
  const warningText = totalWarnings > 0 ? `${warningIcon} ${totalWarnings} ${pluralize(totalWarnings, 'warning')}` : '';
  const summaryParts = [errorText, warningText].filter((s) => s);

  let comment = `<!-- tscanner-pr-comment -->
## ${statusIcon} tscanner - ${totalIssues} ${pluralize(totalIssues, 'Issue')} Found

**Summary:** ${summaryParts.join(' ')}

---

`;

  for (const group of ruleGroups) {
    const icon = group.severity === 'error' ? '✗' : '⚠';
    const summary = `${icon} <strong>${group.ruleName}</strong> - ${group.issueCount} ${pluralize(group.issueCount, 'issue')} - ${group.fileCount} ${pluralize(group.fileCount, 'file')}`;

    comment += `<details>\n<summary>${summary}</summary>\n\n`;

    for (const file of group.files) {
      comment += `\n**${file.filePath}**\n`;
      for (const issue of file.issues) {
        const fileUrl = `https://github.com/${owner}/${repo}/pull/${prNumber}/files#diff-${createFileHash(file.filePath)}R${issue.line}`;
        comment += `- [Line ${issue.line}:${issue.column}](${fileUrl}) - \`${issue.lineText.trim()}\`\n`;
      }
    }

    comment += '\n</details>\n\n';
  }

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
