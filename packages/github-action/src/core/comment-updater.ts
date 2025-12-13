import { COMMENT_MARKER } from '../constants';
import { type Octokit, githubHelper } from '../lib/actions-helper';
import type { ActionScanResult } from './scanner/scanner';
import {
  type CommitHistoryEntry,
  buildCommitHistorySection,
  buildIssuesReport,
  buildSuccessReport,
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

type CommentUpdateParams = {
  octokit: Octokit;
  owner: string;
  repo: string;
  prNumber: number;
  scanResult: ActionScanResult;
  commitSha: string;
  commitMessage: string;
  targetBranch?: string;
};

function buildCommentBody(
  result: ActionScanResult,
  commitSha: string,
  commitMessage: string,
  owner: string,
  repo: string,
  prNumber: number,
  targetBranch: string | undefined,
  commitHistory: CommitHistoryEntry[],
): string {
  const historyData = serializeCommitHistory(commitHistory);
  const historySection = buildCommitHistorySection(commitHistory);

  const reportParams = {
    result,
    targetBranch,
    commitSha,
    commitMessage,
    extraSection: historySection,
    issuesViewParams: { result, owner, repo, prNumber },
  };

  const report = result.totalIssues === 0 ? buildSuccessReport(reportParams) : buildIssuesReport(reportParams);

  return `${COMMENT_MARKER}
${historyData}
${report}`;
}

const MAX_HISTORY_ENTRIES = 10;

export async function updateOrCreateComment(params: CommentUpdateParams) {
  const { octokit, owner, repo, prNumber, scanResult, commitSha, commitMessage, targetBranch } = params;

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
