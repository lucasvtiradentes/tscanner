import { githubHelper } from '../lib/actions-helper';
import type { ScanResult } from './scanner';
import { buildMostTriggeredTable, getModeLabel } from './shared/formatting';
import { buildNoIssuesMessage, buildNoIssuesTable, buildScanHeader, buildScanSummaryTable } from './shared/sections';

function buildSuccessSummary(targetBranch?: string): string {
  const header = buildScanHeader(0, false);
  const modeLabel = getModeLabel(targetBranch);
  const table = buildNoIssuesTable(modeLabel);

  return `${header}

${table}

${buildNoIssuesMessage()}`;
}

function buildIssuesSummary(scanResult: ScanResult, targetBranch?: string): string {
  const { totalErrors, ruleGroupsByRule } = scanResult;

  const header = buildScanHeader(totalErrors, true);
  const statsTable = buildScanSummaryTable({ result: scanResult, targetBranch });
  const mostTriggered = buildMostTriggeredTable(ruleGroupsByRule);

  return `${header}

${statsTable}

${mostTriggered}`;
}

export function writeSummary(scanResult: ScanResult, targetBranch?: string): void {
  const summary =
    scanResult.totalIssues === 0 ? buildSuccessSummary(targetBranch) : buildIssuesSummary(scanResult, targetBranch);

  githubHelper.writeSummary(summary);
}
