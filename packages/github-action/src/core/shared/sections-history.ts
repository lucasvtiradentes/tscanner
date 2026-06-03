import { Alignment, ICONS, alignSection, formatCommitInfo } from './formatting';

export type CommitHistoryEntry = {
  sha: string;
  message: string;
  totalIssues: number;
  errors: number;
  warnings: number;
  infos: number;
  hints: number;
};

function formatHistoryDetails(errors: number, warnings: number, infos: number, hints: number): string {
  const parts: string[] = [];
  if (errors > 0) parts.push(`${ICONS.ERROR_BADGE} ${errors}`);
  if (warnings > 0) parts.push(`${ICONS.WARNING_BADGE} ${warnings}`);
  if (infos > 0) parts.push(`${ICONS.INFO_BADGE} ${infos}`);
  if (hints > 0) parts.push(`${ICONS.HINT_BADGE} ${hints}`);
  return parts.length > 0 ? `(${parts.join(', ')})` : '';
}

export function buildCommitHistorySection(history: CommitHistoryEntry[]): string {
  if (history.length === 0) return '';

  let rows = '';
  for (const entry of history) {
    const label = formatCommitInfo(entry.sha, entry.message);
    const details = formatHistoryDetails(entry.errors, entry.warnings, entry.infos, entry.hints);
    rows += `<tr><td>${label}</td><td>${entry.totalIssues}</td><td>${details}</td></tr>\n`;
  }

  const table = `<table>
<tr><th>Commit</th><th>Issues</th><th>Details</th></tr>
${rows}</table>`;

  const details = `<details>
<summary><strong>📈 Scan history</strong></summary>
<br />

${table}

</details>`;

  return `\n${alignSection(Alignment.Center, details)}\n`;
}
