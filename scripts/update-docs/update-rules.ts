import path from 'node:path';
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

type TFields = 'RULES';

const rootDir = path.resolve(__dirname, '..', '..');
const rulesJson: RuleMetadata[] = getJson(path.join(rootDir, 'assets/rules.json'));

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

let rulesContent = `## 📋 Rules<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>\n\n### Built-in Rules (${rulesJson.length})\n\n`;

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

  rulesContent += `<div align="center">\n\n${table.getTable()}\n\n</div>\n\n</details>\n\n`;
}

const builtInRulesContent = `### Built-in Rules (${rulesJson.length})\n\n${rulesContent.replace(/^## 📋 Rules.*\n\n### Built-in Rules.*\n\n/, '')}`;

const readmePaths = [
  { path: path.join(rootDir, 'packages/core/README.md'), content: rulesContent },
  { path: path.join(rootDir, 'README.md'), content: builtInRulesContent },
  { path: path.join(rootDir, 'packages/cli/README.md'), content: builtInRulesContent },
  { path: path.join(rootDir, 'packages/vscode-extension/README.md'), content: builtInRulesContent },
  { path: path.join(rootDir, 'packages/github-action/README.md'), content: builtInRulesContent },
];

for (const { path: filePath, content } of readmePaths) {
  const readme = new DynMarkdown<TFields>(filePath);
  readme.updateField('RULES', content);
  readme.saveFile();
}

console.log(`✓ Updated RULES in ${readmePaths.length} files (${rulesJson.length} rules)`);
