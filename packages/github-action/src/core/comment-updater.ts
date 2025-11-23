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
  targetBranch?: string;
};

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
    const icon = group.severity === Severity.Error ? '✗' : '⚠';
    const summary = `${icon} <strong>${group.ruleName}</strong> - ${group.issueCount} ${pluralize(group.issueCount, 'issue')} - ${group.fileCount} ${pluralize(group.fileCount, 'file')}`;

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
): string {
  const timestamp = formatTimestamp(timezone);
  const { totalIssues, totalErrors, totalWarnings, totalFiles, totalRules, ruleGroupsByRule } = result;

  const buildIssuesSummary = () => {
    if (totalErrors > 0 && totalWarnings > 0) {
      return `${totalIssues} (${totalErrors} ${pluralize(totalErrors, 'error')}, ${totalWarnings} ${pluralize(totalWarnings, 'warning')})`;
    }
    if (totalErrors > 0) {
      return `${totalErrors} (${totalErrors} ${pluralize(totalErrors, 'error')})`;
    }
    if (totalWarnings > 0) {
      return `${totalWarnings} (${totalWarnings} ${pluralize(totalWarnings, 'warning')})`;
    }
    return '0';
  };

  const buildModeLabel = () => {
    return targetBranch ? `branch (${targetBranch})` : 'codebase';
  };

  if (totalIssues === 0) {
    return `${COMMENT_MARKER}
## ✅ Tscanner - No Issues Found

**Issues:** 0
**Mode:** ${buildModeLabel()}

All files passed validation!

---
**Last updated:** ${timestamp}
**Last commit analyzed:** \`${commitSha}\``;
  }

  const icon = totalErrors > 0 ? '❌' : '⚠️';
  const title = totalErrors > 0 ? 'Errors Found' : 'Warnings Found';

  let comment = `${COMMENT_MARKER}
## ${icon} Tscanner - ${title}

**Issues:** ${buildIssuesSummary()}
**Mode:** ${buildModeLabel()}

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

export async function updateOrCreateComment(params: CommentUpdateParams) {
  const { octokit, owner, repo, prNumber, scanResult, timezone, commitSha, commitMessage, targetBranch } = params;

  const comment = buildCommentBody(scanResult, timezone, commitSha, commitMessage, owner, repo, prNumber, targetBranch);

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
