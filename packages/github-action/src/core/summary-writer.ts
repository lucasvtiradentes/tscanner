import { githubHelper } from '../lib/actions-helper';
import type { ScanResult } from './scanner';
import { buildMostTriggeredTable } from './shared/formatting';
import { buildIssuesReport, buildSuccessReport } from './shared/sections';

export function writeSummary(scanResult: ScanResult, targetBranch?: string): void {
  const mostTriggered = buildMostTriggeredTable(scanResult.ruleGroupsByRule);

  const summary =
    scanResult.totalIssues === 0
      ? buildSuccessReport({ result: scanResult, targetBranch })
      : buildIssuesReport({ result: scanResult, targetBranch, extraSection: mostTriggered });

  githubHelper.writeSummary(summary);
}
