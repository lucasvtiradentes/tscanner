import { githubHelper } from '../lib/actions-helper';
import type { ScanResult } from './scanner';
import { buildMostTriggeredTable } from './shared/formatting';
import { buildIssuesReport, buildSuccessReport } from './shared/sections';

export type WriteSummaryParams = {
  scanResult: ScanResult;
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
  const mostTriggered = buildMostTriggeredTable(scanResult.ruleGroupsByRule);

  const reportParams = {
    result: scanResult,
    targetBranch,
    timestamp,
    commitSha,
    commitMessage,
    extraSection: mostTriggered,
    issuesViewParams: { result: scanResult, owner, repo, prNumber },
  };

  const summary = scanResult.totalIssues === 0 ? buildSuccessReport(reportParams) : buildIssuesReport(reportParams);

  githubHelper.writeSummary(summary);
}
