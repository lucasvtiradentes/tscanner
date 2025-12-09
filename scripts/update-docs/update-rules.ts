import fs from 'node:fs';
import path from 'node:path';
import { DynMarkdown, MarkdownTable, type TRowContent, getJson } from 'markdown-helper';
import { PACKAGE_DISPLAY_NAME } from 'tscanner-common';

type RuleOption = {
  name: string;
  description: string;
  type: 'integer' | 'boolean' | 'string' | 'array';
  default: unknown;
  minimum?: number;
  items?: string;
};

type RuleMetadata = {
  displayName: string;
  description: string;
  ruleType: 'ast' | 'regex';
  defaultSeverity: 'error' | 'warning';
  defaultEnabled: boolean;
  category: string;
  sourcePath?: string;
  typescriptOnly?: boolean;
  equivalentEslintRule?: string;
  equivalentBiomeRule?: string;
  options?: RuleOption[];
};

type TFields = 'RULES';

const rootDir = path.resolve(__dirname, '..', '..');

export function updateRules() {
  const rulesJson: RuleMetadata[] = getJson(path.join(rootDir, 'assets/generated/rules.json'));

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

  let builtInRulesTableContent = '';

  for (const cat of categoryOrder) {
    const rules = rulesByCategory[cat];
    if (!rules || rules.length === 0) continue;

    const categoryName = categoryMap[cat] || cat;

    builtInRulesTableContent += `<div align="left">\n\n#### ${categoryName} (${rules.length})\n\n</div>\n\n`;

    const headerContent = [
      { content: 'Rule', width: 250 },
      { content: 'Description', width: 400 },
      { content: 'Options', width: 150 },
      { content: 'Also in', width: 100 },
    ] as const satisfies TRowContent;

    const table = new MarkdownTable(headerContent);

    for (const rule of rules) {
      const description = rule.description.replace(/`([^`]+)`/g, '<code>$1</code>');
      const ruleName = rule.displayName
        .toLowerCase()
        .replace(/\s+/g, '-')
        .replace(/[^a-z0-9-]/g, '');

      const ruleBadges: string[] = [];
      if (rule.typescriptOnly) {
        ruleBadges.push(
          '<img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only">',
        );
      }
      if (rule.ruleType === 'regex') {
        ruleBadges.push('<img src="https://img.shields.io/badge/regex--rule-6C757D" alt="Regex rule">');
      }
      if (rule.options && rule.options.length > 0) {
        ruleBadges.push('<img src="https://img.shields.io/badge/configurable-green" alt="Configurable">');
      }
      const badgesHtml = ruleBadges.length > 0 ? `<br/><br/>${ruleBadges.join(' ')}` : '';
      const ruleLink = rule.sourcePath
        ? `<a href="https://github.com/lucasvtiradentes/tscanner/blob/main/${rule.sourcePath}"><code>${ruleName}</code></a>`
        : `<code>${ruleName}</code>`;
      const ruleCell = `<div align="center">${ruleLink}${badgesHtml}</div>`;

      const equivalentBadges: string[] = [];
      if (rule.equivalentEslintRule) {
        equivalentBadges.push(
          `<a href="${rule.equivalentEslintRule}"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a>`,
        );
      }
      if (rule.equivalentBiomeRule) {
        equivalentBadges.push(
          `<a href="${rule.equivalentBiomeRule}"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a>`,
        );
      }
      const equivalentCell = equivalentBadges.join(' ');

      let optionsCell = '';
      if (rule.options && rule.options.length > 0) {
        const optionsList = rule.options.map((opt) => {
          const defaultVal = Array.isArray(opt.default) ? `[${opt.default.length} items]` : String(opt.default);
          return `<code>${opt.name}</code>: ${defaultVal}`;
        });
        optionsCell = optionsList.join('<br/>');
      }

      table.addBodyRow([
        { content: ruleCell, align: 'left' },
        { content: description, align: 'left' },
        { content: optionsCell, align: 'left' },
        { content: equivalentCell, align: 'left' },
      ]);
    }

    builtInRulesTableContent += `${table.getTable()}\n\n`;
  }

  const builtInRulesContent = `<details>
<summary>Built-in rules (${rulesJson.length})</summary>
<br />

${builtInRulesTableContent.trim()}

</details>`;

  const rulesIntroTable = `## 📋 Rules<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Customize ${PACKAGE_DISPLAY_NAME} to validate what matters to your project while maintaining consistency.

<div align="center">

<table>
  <tr>
    <th width="100">Type</th>
    <th width="250">Use Case</th>
    <th width="400">Example</th>
  </tr>
  <tr>
    <td><b>Built-in</b></td>
    <td>${rulesJson.length} ready-to-use AST rules</td>
    <td><code>no-explicit-any</code>, <code>prefer-const</code>, <code>no-console</code></td>
  </tr>
  <tr>
    <td><b>Regex</b></td>
    <td>Simple text patterns</td>
    <td>Match <code>TODO</code> comments, banned imports, naming conventions</td>
  </tr>
  <tr>
    <td><b>Script</b></td>
    <td>Complex logic via JS</td>
    <td>Validate file naming, check if tests exist, enforce folder structure</td>
  </tr>
  <tr>
    <td><b>AI</b></td>
    <td>Semantic validation via prompts</td>
    <td>Enforce React Hook Form usage, validate API integration patterns with SWR/TanStack</td>
  </tr>
</table>

</div>

`;

  const scriptRuleExample = fs
    .readFileSync(path.join(rootDir, 'assets/configs/script-rule-example.ts'), 'utf-8')
    .trim();
  const aiRuleExample = fs.readFileSync(path.join(rootDir, 'assets/configs/ai-rule-example.md'), 'utf-8').trim();

  const customRulesContent = `<details>
<summary>Regex rules examples</summary>
<br />
<div align="left">

Define patterns to match in your code using regular expressions:

**Config** (\`.tscanner/config.jsonc\`):
\`\`\`json
{
  "rules": {
    "regex": {
      "no-todos": {
        "pattern": "TODO:|FIXME:",
        "message": "Remove TODO comments before merging",
        "severity": "warning"
      },
      "no-debug-logs": {
        "pattern": "console\\\\.(log|debug|info)",
        "message": "Remove debug statements",
        "severity": "warning",
        "exclude": ["**/*.test.ts"]
      }
    }
  }
}
\`\`\`

> 💡 See a real example in the [\`.tscanner/\`](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner) folder of this project.

</div>
</details>

<details>
<summary>Script rules examples</summary>
<br />
<div align="left">

Run custom scripts that receive file data via stdin and output issues as JSON:

**Config** (\`.tscanner/config.jsonc\`):
\`\`\`json
{
  "rules": {
    "script": {
      "no-debug-comments": {
        "command": "npx tsx .tscanner/scripts/no-debug-comments.ts",
        "message": "Debug comments should be removed",
        "severity": "warning"
      }
    }
  }
}
\`\`\`

**Script** (\`.tscanner/scripts/no-debug-comments.ts\`):
\`\`\`typescript
${scriptRuleExample}
\`\`\`

> 💡 See a real example in the [\`.tscanner/\`](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner) folder of this project.

</div>
</details>

<details>
<summary>AI rules examples</summary>
<br />
<div align="left">

Use AI prompts to perform semantic code analysis:

**Config** (\`.tscanner/config.jsonc\`):
\`\`\`json
{
  "aiRules": {
    "find-complexity": {
      "prompt": "find-complexity.md",
      "mode": "content",
      "message": "Function is too complex, consider refactoring",
      "severity": "warning",
      "enabled": true
    }
  },
  "ai": {
    "provider": "claude",
    "timeout": 120
  }
}
\`\`\`

**Prompt** (\`.tscanner/prompts/find-complexity.md\`):
\`\`\`markdown
${aiRuleExample}
\`\`\`

> 💡 See a real example in the [\`.tscanner/\`](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner) folder of this project.

</div>
</details>`;

  const fullRulesContent = `${rulesIntroTable}
  <br />
  
<div align="center">

${builtInRulesContent}

${customRulesContent}

</div>`;

  const readmePaths = [
    path.join(rootDir, 'README.md'),
    path.join(rootDir, 'packages/cli/README.md'),
    path.join(rootDir, 'packages/vscode-extension/README.md'),
    path.join(rootDir, 'packages/github-action/README.md'),
  ];

  for (const filePath of readmePaths) {
    const readme = new DynMarkdown<TFields>(filePath);
    readme.updateField('RULES', fullRulesContent);
    readme.saveFile();
  }

  console.log(`✓ Updated RULES in ${readmePaths.length} files (${rulesJson.length} rules)`);
}
