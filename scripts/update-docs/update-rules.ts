import { readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { DynMarkdown, MarkdownTable, type TRowContent, getJson } from 'markdown-helper';
import { PACKAGE_DISPLAY_NAME, REPO_URL } from 'tscanner-common';

enum RuleOptionType {
  Integer = 'integer',
  Boolean = 'boolean',
  String = 'string',
  Array = 'array',
}

enum RuleType {
  Ast = 'ast',
  Regex = 'regex',
}

enum RuleSeverity {
  Error = 'error',
  Warning = 'warning',
}

type RuleOption = {
  name: string;
  description: string;
  type: RuleOptionType;
  default: unknown;
  minimum?: number;
  items?: string;
};

type RuleMetadata = {
  displayName: string;
  description: string;
  ruleType: RuleType;
  defaultSeverity: RuleSeverity;
  defaultEnabled: boolean;
  category: string;
  sourcePath?: string;
  typescriptOnly?: boolean;
  equivalentEslintRule?: string;
  equivalentBiomeRule?: string;
  options?: RuleOption[];
};

type TFields = 'RULES';

const rootDir = resolve(__dirname, '..', '..');

export function updateRules() {
  const rulesJson: RuleMetadata[] = getJson(join(rootDir, 'assets/generated/rules.json'));

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
      if (rule.ruleType === RuleType.Regex) {
        ruleBadges.push('<img src="https://img.shields.io/badge/regex--rule-6C757D" alt="Regex rule">');
      }
      if (rule.options && rule.options.length > 0) {
        ruleBadges.push('<img src="https://img.shields.io/badge/configurable-green" alt="Configurable">');
      }
      const badgesHtml = ruleBadges.length > 0 ? `<br/><br/>${ruleBadges.join(' ')}` : '';
      const ruleLink = rule.sourcePath
        ? `<a href="${REPO_URL}/blob/main/${rule.sourcePath}"><code>${ruleName}</code></a>`
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

  const scriptRuleExample = readFileSync(join(rootDir, 'assets/configs/example-no-long-files.ts'), 'utf-8').trim();
  const aiRuleExample = readFileSync(join(rootDir, 'assets/configs/example-find-enum-candidates.md'), 'utf-8').trim();

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
      "no-rust-deprecated": {
        "pattern": "allow\\\\(deprecated\\\\)",
        "message": "No deprecated methods",
        "include": ["packages/rust-core/**/*.rs"]
      },
      "no-process-env": {
        "pattern": "process\\\\.env",
        "message": "No process env"
      },
      "no-debug-logs": {
        "pattern": "console\\\\.(log|debug|info)",
        "message": "Remove debug statements",
        "exclude": ["**/*.test.ts"]
      }
    }
  }
}
\`\`\`

> 💡 See real examples in the [\`.tscanner/\`](${REPO_URL}/tree/main/.tscanner) folder of this project.

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
      "no-long-files": {
        "command": "npx tsx script-rules/no-long-files.ts",
        "message": "File exceeds 300 lines limit",
        "include": ["packages/**/*.ts", "packages/**/*.rs"]
      }
    }
  }
}
\`\`\`

**Script** (\`.tscanner/script-rules/no-long-files.ts\`):
\`\`\`typescript
${scriptRuleExample}
\`\`\`

> 💡 See real examples in the [\`.tscanner/script-rules/\`](${REPO_URL}/tree/main/.tscanner/script-rules) folder of this project.

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
    "find-enum-candidates": {
      "prompt": "find-enum-candidates.md",
      "mode": "agentic",
      "message": "Type union could be replaced with an enum for better type safety",
      "severity": "warning",
      "include": ["**/*.ts"]
    }
  },
  "ai": {
    "provider": "claude"
  }
}
\`\`\`

**Prompt** (\`.tscanner/ai-rules/find-enum-candidates.md\`):
\`\`\`markdown
${aiRuleExample}
\`\`\`

> 💡 See real examples in the [\`.tscanner/ai-rules/\`](${REPO_URL}/tree/main/.tscanner/ai-rules) folder of this project.

</div>
</details>`;

  const fullRulesContent = `${rulesIntroTable}
  <br />
  
<div align="center">

${builtInRulesContent}

${customRulesContent}

</div>`;

  const readmePaths = [
    join(rootDir, 'README.md'),
    join(rootDir, 'packages/cli/README.md'),
    join(rootDir, 'packages/vscode-extension/README.md'),
    join(rootDir, 'packages/github-action/README.md'),
  ];

  for (const filePath of readmePaths) {
    const readme = new DynMarkdown<TFields>(filePath);
    readme.updateField('RULES', fullRulesContent);
    readme.saveFile();
  }

  console.log(`✓ Updated RULES in ${readmePaths.length} files (${rulesJson.length} rules)`);
}
