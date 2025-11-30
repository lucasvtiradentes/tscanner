import { githubHelper } from '../lib/actions-helper';
import type { ScanResult } from './scanner';
import { buildMostTriggeredTable, getModeLabel } from './shared/formatting';
import { buildNoIssuesMessage, buildScanHeader, buildScanSummaryTable } from './shared/sections';

export function writeSummary(scanResult: ScanResult, targetBranch?: string): void {
  const { totalIssues, totalErrors, ruleGroupsByRule } = scanResult;

  if (totalIssues === 0) {
    const header = buildScanHeader(0, false);
    const modeLabel = getModeLabel(targetBranch);
    const summary = `${header}

| Metric | Value |
|--------|-------|
| Scan mode | ${modeLabel} |

${buildNoIssuesMessage()}`;
    githubHelper.writeSummary(summary);
    return;
  }

  const header = buildScanHeader(totalErrors, true);
  const statsTable = buildScanSummaryTable({ result: scanResult, targetBranch });
  const mostTriggered = buildMostTriggeredTable(ruleGroupsByRule);

  const summary = `${header}

${statsTable}

${mostTriggered}`;

  githubHelper.writeSummary(summary);
}
