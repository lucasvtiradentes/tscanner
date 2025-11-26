import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { DynMarkdown, MarkdownTable, type TRowContent, getJson } from 'markdown-helper';

type RuleMetadata = {
  name: string;
  displayName: string;
  description: string;
  ruleType: 'ast' | 'regex';
  defaultSeverity: 'error' | 'warning';
  defaultEnabled: boolean;
  category: string;
};

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, '..');
const rulesJson: RuleMetadata[] = getJson(path.join(rootDir, 'assets/rules.json'));
const coreReadmePath = path.join(rootDir, 'packages/core/README.md');
const rootReadmePath = path.join(rootDir, 'README.md');

type TFields = 'RULES';
const coreReadme = new DynMarkdown<TFields>(coreReadmePath);
const rootReadme = new DynMarkdown<TFields>(rootReadmePath);

const categoryMap: Record<string, string> = {
  typesafety: 'Type Safety',
  codequality: 'Code Quality',
  style: 'Style',
  performance: 'Performance',
  bugprevention: 'Bug Prevention',
  variables: 'Variables',
  imports: 'Imports',
};

const rulesByCategory = rulesJson.reduce(
  (acc, rule) => {
    const cat = rule.category;
    if (!acc[cat]) acc[cat] = [];
    acc[cat].push(rule);
    return acc;
  },
  {} as Record<string, RuleMetadata[]>,
);

const categoryOrder = ['typesafety', 'codequality', 'bugprevention', 'variables', 'imports', 'style', 'performance'];

let rulesContent = `## 📋 Built-in Rules (${rulesJson.length})<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>\n\n`;

for (const cat of categoryOrder) {
  const rules = rulesByCategory[cat];
  if (!rules || rules.length === 0) continue;

  const categoryName = categoryMap[cat] || cat;

  rulesContent += `<details>\n<summary><b>${categoryName} (${rules.length})</b></summary>\n\n`;

  const headerContent = [
    { content: 'Rule', width: 250 },
    { content: 'Description', width: 500 },
  ] as const satisfies TRowContent;

  const table = new MarkdownTable(headerContent);

  for (const rule of rules) {
    const description = rule.description.replace(/`([^`]+)`/g, '<code>$1</code>');
    table.addBodyRow([
      { content: `<code>${rule.name}</code>`, align: 'left' },
      { content: description, align: 'left' },
    ]);
  }

  rulesContent += `${table.getTable()}\n\n</details>\n\n`;
}

coreReadme.updateField('RULES', rulesContent);
coreReadme.saveFile();

const builtInRulesContent = `### Built-in Rules (${rulesJson.length})\n\n${rulesContent.replace(/^## 📋 Built-in Rules.*\n\n/, '')}`;
rootReadme.updateField('RULES', builtInRulesContent);
rootReadme.saveFile();

console.log(`✓ Updated READMEs with ${rulesJson.length} rules`);
