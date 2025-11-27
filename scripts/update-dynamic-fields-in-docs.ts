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

type VscodeCommand = {
  command: string;
  title: string;
  icon?: string;
};

type VscodeKeybinding = {
  command: string;
  key: string;
  when?: string;
};

type CommandPaletteEntry = {
  command: string;
  when: string;
};

type VscodePackageJson = {
  contributes: {
    commands: VscodeCommand[];
    keybindings: VscodeKeybinding[];
    menus: {
      commandPalette: CommandPaletteEntry[];
    };
  };
};

const rootDir = path.resolve(__dirname, '..');
const rulesJson: RuleMetadata[] = getJson(path.join(rootDir, 'assets/rules.json'));
const vscodePackageJson: VscodePackageJson = getJson(path.join(rootDir, 'packages/vscode-extension/package.json'));

type TFields = 'RULES' | 'DEFAULT_CONFIG' | 'COMMANDS';

const readmePaths = {
  root: path.join(rootDir, 'README.md'),
  core: path.join(rootDir, 'packages/core/README.md'),
  cli: path.join(rootDir, 'packages/cli/README.md'),
  vscode: path.join(rootDir, 'packages/vscode-extension/README.md'),
  githubAction: path.join(rootDir, 'packages/github-action/README.md'),
};

const readmes = {
  root: new DynMarkdown<TFields>(readmePaths.root),
  core: new DynMarkdown<TFields>(readmePaths.core),
  cli: new DynMarkdown<TFields>(readmePaths.cli),
  vscode: new DynMarkdown<TFields>(readmePaths.vscode),
  githubAction: new DynMarkdown<TFields>(readmePaths.githubAction),
};

const defaultConfigJson = getJson(path.join(rootDir, 'assets/default-config.json'));
const defaultConfigContent = `\`\`\`json\n${JSON.stringify(defaultConfigJson, null, 2)}\n\`\`\``;

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

  rulesContent += `${table.getTable()}\n\n</details>\n\n`;
}

const builtInRulesContent = `### Built-in Rules (${rulesJson.length})\n\n${rulesContent.replace(/^## 📋 Rules.*\n\n### Built-in Rules.*\n\n/, '')}`;

readmes.core.updateField('RULES', rulesContent);
readmes.core.updateField('DEFAULT_CONFIG', defaultConfigContent);
readmes.core.saveFile();

readmes.root.updateField('RULES', builtInRulesContent);
readmes.root.updateField('DEFAULT_CONFIG', defaultConfigContent);
readmes.root.saveFile();

readmes.cli.updateField('RULES', builtInRulesContent);
readmes.cli.updateField('DEFAULT_CONFIG', defaultConfigContent);
readmes.cli.saveFile();

const hiddenCommands = new Set(
  vscodePackageJson.contributes.menus.commandPalette
    .filter((entry) => entry.when === 'false')
    .map((entry) => entry.command),
);

const keybindingsMap = new Map(vscodePackageJson.contributes.keybindings.map((kb) => [kb.command, kb.key]));

const commandDescriptions: Record<string, string> = {
  'tscanner.findIssue': 'Run scan (Codebase or Branch mode)',
  'tscanner.hardScan': 'Clear cache and rescan workspace',
  'tscanner.goToNextIssue': 'Jump to next issue in the list',
  'tscanner.goToPreviousIssue': 'Jump to previous issue in the list',
  'tscanner.showLogs': 'View extension logs file',
};

const visibleCommands = vscodePackageJson.contributes.commands.filter(
  (cmd) => !hiddenCommands.has(cmd.command) && cmd.title.startsWith('tscanner:'),
);

const commandsHeaderContent = [
  { content: 'Command', width: 300 },
  { content: 'Description', width: 250 },
  { content: 'Keybinding', width: 100 },
] as const satisfies TRowContent;

const commandsTable = new MarkdownTable(commandsHeaderContent);

for (const cmd of visibleCommands) {
  const keybinding = keybindingsMap.get(cmd.command);
  const description = commandDescriptions[cmd.command] || '-';
  commandsTable.addBodyRow([
    { content: `<code>${cmd.title}</code>`, align: 'left' },
    { content: description, align: 'left' },
    { content: keybinding ? `<code>${keybinding}</code>` : '-', align: 'center' },
  ]);
}

const commandsContent = `<div align="center">\n\n${commandsTable.getTable()}\n\n</div>`;

readmes.vscode.updateField('RULES', builtInRulesContent);
readmes.vscode.updateField('DEFAULT_CONFIG', defaultConfigContent);
readmes.vscode.updateField('COMMANDS', commandsContent);
readmes.vscode.saveFile();

readmes.githubAction.updateField('RULES', builtInRulesContent);
readmes.githubAction.updateField('DEFAULT_CONFIG', defaultConfigContent);
readmes.githubAction.saveFile();

console.log(
  `✓ Updated all READMEs with ${rulesJson.length} rules, default config, and ${visibleCommands.length} commands`,
);
