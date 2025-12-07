import { githubHelper } from '../lib/actions-helper';
import type { ActionScanResult } from './scanner/scanner';
import { buildIssuesReport, buildSuccessReport } from './shared/sections';

type WriteSummaryParams = {
  scanResult: ActionScanResult;
  targetBranch?: string;
  owner: string;
  repo: string;
  prNumber: number;
  commitSha?: string;
  commitMessage?: string;
  timestamp?: string;
};

export function writeSummary(params: WriteSummaryParams): void {
  const { scanResult, targetBranch, owner, repo, prNumber, commitSha, commitMessage, timestamp } = params;

  const reportParams = {
    result: scanResult,
    targetBranch,
    timestamp,
    commitSha,
    commitMessage,
    issuesViewParams: { result: scanResult, owner, repo, prNumber },
  };

  const summary = scanResult.totalIssues === 0 ? buildSuccessReport(reportParams) : buildIssuesReport(reportParams);

  githubHelper.writeSummary(summary);
}
