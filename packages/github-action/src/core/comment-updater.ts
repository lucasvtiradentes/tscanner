import { COMMENT_MARKER, Severity } from '../constants';
import { type Octokit, githubHelper } from '../lib/actions-helper';
import { formatTimestamp } from '../utils/format-timestamp';
import { pluralize } from '../utils/pluralize';
import { buildPrFileUrl } from '../utils/url-builder';
import type { RuleGroup, ScanResult } from './scanner';

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
    output += `<details>\n<summary>${summary}</summary>\n<br />\n\n`;

    for (const issue of issues) {
      const fileUrl = buildPrFileUrl(owner, repo, prNumber, filePath, issue.line);
      output += `- [Line ${issue.line}:${issue.column}](${fileUrl}) - **${issue.ruleName}** - \`${issue.lineText.trim()}\`\n`;
    }

    output += '\n</details>\n\n';
  }

  return output;
}

function buildGroupedByRuleView(ruleGroups: RuleGroup[], owner: string, repo: string, prNumber: number): string {
  let output = '';

  for (const group of ruleGroups) {
    const icon = group.severity === Severity.Error ? '✗' : '⚠';
    const summary = `${icon} <strong>${group.ruleName}</strong> - ${group.issueCount} ${pluralize(group.issueCount, 'issue')} - ${group.fileCount} ${pluralize(group.fileCount, 'file')}`;

    output += `<details>\n<summary>${summary}</summary>\n<br />\n\n`;

    for (const file of group.files) {
      const fileIssueCount = file.issues.length;
      output += `<strong>${file.filePath}</strong> - ${fileIssueCount} ${pluralize(fileIssueCount, 'issue')}\n\n`;

      for (const issue of file.issues) {
        const fileUrl = buildPrFileUrl(owner, repo, prNumber, file.filePath, issue.line);
        output += `- <a href="${fileUrl}">${issue.line}:${issue.column}</a> - <code>${issue.lineText.trim()}</code>\n`;
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
): string {
  const timestamp = formatTimestamp(timezone);
  const { totalIssues, totalErrors, totalWarnings, totalFiles, totalRules, ruleGroupsByRule } = result;

  if (totalIssues === 0) {
    return `${COMMENT_MARKER}
## ✅ Tscanner - No Issues Found

All changed files passed validation!

---
**Last updated:** ${timestamp}
**Last commit analyzed:** \`${commitSha}\``;
  }

  let comment = `${COMMENT_MARKER}
## Tscanner - ${totalIssues} Issues Found (${totalErrors} ${pluralize(totalErrors, 'error')}, ${totalWarnings} ${pluralize(totalWarnings, 'warning')})

---

`;

  const groupedByRule = buildGroupedByRuleView(ruleGroupsByRule, owner, repo, prNumber);
  const groupedByFile = buildGroupedByFileView(result, owner, repo, prNumber);

  comment += `<div align="center">\n\n<details>\n<summary><strong>📋 Issues grouped by rule (${totalRules})</strong></summary>\n<br />\n\n<div align="left">\n${groupedByRule}\n</div></details>\n\n</div>\n\n---\n\n`;
  comment += `<div align="center">\n\n<details>\n<summary><strong>📁 Issues grouped by file (${totalFiles})</strong></summary>\n<br />\n\n<div align="left">\n${groupedByFile}\n</div></details>\n\n</div>\n\n`;

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

  const botComment = existingComments.data.find((c) => c.user?.type === 'Bot' && c.body?.includes(COMMENT_MARKER));

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
