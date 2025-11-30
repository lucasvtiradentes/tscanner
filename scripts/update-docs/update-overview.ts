import path from 'node:path';
import { DynMarkdown, getJson } from 'markdown-helper';

type TFields = 'OVERVIEW';

const rootDir = path.resolve(__dirname, '..', '..');
const rulesJson: unknown[] = getJson(path.join(rootDir, 'assets/generated/rules.json'));
const RULES_COUNT = rulesJson.length;

function getOverviewContent(): string {
  return `## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Define what "good code" means for your project. TScanner enforces your patterns in real-time in the code editor, blocks violations in PRs, and runs in CI. ${RULES_COUNT} ready-to-use rules + custom rules via regex, scripts, or AI.`;
}

export function updateOverview() {
  const filePath = path.join(rootDir, 'README.md');
  const readme = new DynMarkdown<TFields>(filePath);
  readme.updateField('OVERVIEW', getOverviewContent());
  readme.saveFile();

  console.log(`✓ Updated OVERVIEW section (${RULES_COUNT} rules)`);
}
