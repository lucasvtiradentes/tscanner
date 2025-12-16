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
    <td>Simple text patterns for any file</td>
    <td>Match <code>TODO</code> comments, banned imports, naming conventions</td>
  </tr>
  <tr>
    <td><b>Script</b></td>
    <td>Complex logic in any language (TS, Python, Rust, Go...)</td>
    <td>Validate file naming, check if tests exist, enforce folder structure, type parity checks</td>
  </tr>
  <tr>
    <td><b>AI</b></td>
    <td>Semantic validation via prompts</td>
    <td>Enforce React Hook Form usage, validate API integration patterns with SWR/TanStack</td>
  </tr>
</table>

</div>

`;

  const scriptInputContract = `{
  "files": [
    {
      "path": "src/utils.ts",
      "content": "export function add(a: number, b: number)...",
      "lines": ["export function add(a: number, b: number)", "..."]
    }
  ],
  "options": { "maxLines": 300 },
  "workspaceRoot": "/path/to/project"
}`;

  const scriptOutputContract = `{
  "issues": [
    { "file": "src/utils.ts", "line": 10, "message": "Issue description" }
  ]
}`;

  const scriptExampleTs = `#!/usr/bin/env npx tsx
import { stdin } from 'node:process';

async function main() {
  let data = '';
  for await (const chunk of stdin) data += chunk;

  const input = JSON.parse(data);
  const issues = [];

  for (const file of input.files) {
    if (file.lines.length > 300) {
      issues.push({ file: file.path, line: 301, message: \`File exceeds 300 lines\` });
    }
  }

  console.log(JSON.stringify({ issues }));
}
main().catch((err) => {
  console.error(err);
  process.exit(1);
});`;

  const scriptExamplePy = `#!/usr/bin/env python3
import json, sys

def main():
    input_data = json.loads(sys.stdin.read())
    issues = []

    for file in input_data["files"]:
        if len(file["lines"]) > 300:
            issues.append({"file": file["path"], "line": 301, "message": "File exceeds 300 lines"})

    print(json.dumps({"issues": issues}))

if __name__ == "__main__":
    main()`;

  const scriptExampleRs = `#!/usr/bin/env rust-script
use std::io::{self, Read};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ScriptFile { path: String, lines: Vec<String> }

#[derive(Deserialize)]
struct ScriptInput { files: Vec<ScriptFile> }

#[derive(Serialize)]
struct ScriptIssue { file: String, line: usize, message: String }

#[derive(Serialize)]
struct ScriptOutput { issues: Vec<ScriptIssue> }

fn main() -> io::Result<()> {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data)?;
    let input: ScriptInput = serde_json::from_str(&data).unwrap();
    let mut issues = Vec::new();

    for file in input.files {
        if file.lines.len() > 300 {
            issues.push(ScriptIssue { file: file.path, line: 301, message: "File exceeds 300 lines".into() });
        }
    }

    println!("{}", serde_json::to_string(&ScriptOutput { issues }).unwrap());
    Ok(())
}`;

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

</div>
</details>

<details>
<summary>Script rules examples</summary>
<br />
<div align="left">

Run custom scripts in **any language** (TypeScript, Python, Rust, Go, etc.) that reads JSON from stdin and outputs JSON to stdout.

**Input contract** (received via stdin):
\`\`\`json
${scriptInputContract}
\`\`\`

**Output contract** (expected via stdout):
\`\`\`json
${scriptOutputContract}
\`\`\`

**Config** (\`.tscanner/config.jsonc\`):
\`\`\`json
{
  "rules": {
    "script": {
      "no-long-files": {
        "command": "npx tsx script-rules/no-long-files.ts",
        "message": "File exceeds 300 lines limit",
        "include": ["**/*.ts", "**/*.rs", "**/*.py", "**/*.go"]
      }
    }
  }
}
\`\`\`

<details>
<summary>TypeScript example</summary>

\`\`\`typescript
${scriptExampleTs}
\`\`\`
</details>

<details>
<summary>Python example</summary>

\`\`\`python
${scriptExamplePy}
\`\`\`
</details>

<details>
<summary>Rust example</summary>

\`\`\`rust
${scriptExampleRs}
\`\`\`
</details>

> 💡 See real examples in the [\`.tscanner/script-rules/\`](${REPO_URL}/tree/main/.tscanner/script-rules) and [\`registry/script-rules/\`](${REPO_URL}/tree/main/registry/script-rules) folders.

</div>
</details>

<details>
<summary>AI rules examples</summary>
<br />
<div align="left">

Use AI prompts (markdown files) to perform semantic code analysis. Works with any AI provider (Claude, OpenAI, Ollama, etc.).

**Modes** - How files are passed to the AI:
| Mode | Description | Best for |
|------|-------------|----------|
| \`paths\` | Only file paths (AI reads files via tools) | Large codebases, many files |
| \`content\` | Full file content in prompt | Small files, quick analysis |
| \`agentic\` | Paths + AI can explore freely | Cross-file analysis, complex patterns |

**Placeholders** - Use in your prompt markdown:
| Placeholder | Replaced with |
|-------------|---------------|
| \`{{FILES}}\` | List of files to analyze (required) |
| \`{{OPTIONS}}\` | Custom options from config (optional) |

**Output contract** - AI must return JSON:
\`\`\`json
{
  "issues": [
    { "file": "src/utils.ts", "line": 10, "column": 1, "message": "Description" }
  ]
}
\`\`\`

**Config** (\`.tscanner/config.jsonc\`):
\`\`\`json
{
  "aiRules": {
    "find-enum-candidates": {
      "prompt": "find-enum-candidates.md",
      "mode": "agentic",
      "message": "Type union could be replaced with an enum",
      "severity": "warning",
      "include": ["**/*.ts", "**/*.tsx", "**/*.rs"]
    },
    "no-dead-code": {
      "prompt": "no-dead-code.md",
      "mode": "content",
      "message": "Dead code detected",
      "severity": "error",
      "include": ["**/*.rs"],
      "options": { "allowTestFiles": true }
    }
  },
  "ai": {
    "provider": "claude"
  }
}
\`\`\`

<details>
<summary>Prompt example (agentic mode)</summary>

\`\`\`markdown
# Enum Candidates Detector

Find type unions that could be replaced with enums.

## What to look for

1. String literal unions: \\\`type Status = 'pending' | 'active'\\\`
2. Repeated string literals across files
3. Type unions used as discriminators

## Exploration hints

- Check how the type is used across files
- Look for related constants

---

## Files

{{FILES}}
\`\`\`
</details>

<details>
<summary>Prompt example (with options)</summary>

\`\`\`markdown
# Dead Code Detector

Detect dead code patterns.

## Rules

1. No \\\`#[allow(dead_code)]\\\` attributes
2. No unreachable code after return/break

## Options

{{OPTIONS}}

## Files

{{FILES}}
\`\`\`
</details>

> 💡 See real examples in the [\`.tscanner/ai-rules/\`](${REPO_URL}/tree/main/.tscanner/ai-rules) and [\`registry/ai-rules/\`](${REPO_URL}/tree/main/registry/ai-rules) folders.

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
