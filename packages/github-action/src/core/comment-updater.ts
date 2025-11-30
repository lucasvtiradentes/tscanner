import { COMMENT_MARKER } from '../constants';
import { type Octokit, githubHelper } from '../lib/actions-helper';
import { formatTimestamp } from '../utils/format-timestamp';
import type { ScanResult } from './scanner';
import { Alignment, alignSection, formatCommitInfo, getModeLabel } from './shared/formatting';
import {
  type CommitHistoryEntry,
  buildCommitHistorySection,
  buildIssuesByFileSection,
  buildIssuesByRuleSection,
  buildNoIssuesMessage,
  buildNoIssuesTable,
  buildScanHeader,
  buildScanSummaryTable,
} from './shared/sections';

function parseCommitHistory(commentBody: string): CommitHistoryEntry[] {
  const historyMatch = commentBody.match(/<!-- COMMIT_HISTORY:(.*?)-->/s);
  if (!historyMatch) return [];

  try {
    return JSON.parse(historyMatch[1]) as CommitHistoryEntry[];
  } catch {
    return [];
  }
}

function serializeCommitHistory(history: CommitHistoryEntry[]): string {
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

function buildCommentBody(
  result: ScanResult,
  timezone: string,
  commitSha: string,
  commitMessage: string,
  owner: string,
  repo: string,
  prNumber: number,
  targetBranch: string | undefined,
  commitHistory: CommitHistoryEntry[],
): string {
  const timestamp = formatTimestamp(timezone);
  const { totalIssues, totalErrors } = result;
  const historyData = serializeCommitHistory(commitHistory);
  const historySection = buildCommitHistorySection(commitHistory);

  if (totalIssues === 0) {
    const header = buildScanHeader(0, false);
    const modeLabel = getModeLabel(targetBranch);
    const commitInfo = formatCommitInfo(commitSha, commitMessage);
    const table = buildNoIssuesTable(modeLabel, commitInfo, timestamp);

    return `${COMMENT_MARKER}
${historyData}
${header}

${table}

${buildNoIssuesMessage()}
${historySection}`;
  }

  const header = buildScanHeader(totalErrors, true);
  const statsTable = buildScanSummaryTable({
    result,
    targetBranch,
    timestamp,
    commitSha,
    commitMessage,
  });
  const issuesByRule = buildIssuesByRuleSection({ result, owner, repo, prNumber });
  const issuesByFile = buildIssuesByFileSection({ result, owner, repo, prNumber });

  return `${COMMENT_MARKER}
${historyData}
${header}

${alignSection(Alignment.Center, statsTable)}

<br />
${historySection}
---
${issuesByRule}
${issuesByFile}`;
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

  let commitHistory: CommitHistoryEntry[] = [];
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
