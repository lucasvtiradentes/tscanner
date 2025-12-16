import { join, resolve } from 'node:path';
import { DynMarkdown } from 'markdown-helper';
import { REPO_URL } from 'tscanner-common';

type TFields = 'WORKFLOW';

const rootDir = resolve(__dirname, '..', '..');

export function updateWorkflow() {
  const getWorkflowContent = () => {
    return `
## 🔀 Workflow<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

> **Vision:** Go fast with AI and know exactly what to fix before shipping. Detect bad patterns while reviewing code? Ask AI to create [regex](${REPO_URL}/blob/main/.tscanner/config.jsonc), [script](${REPO_URL}/tree/main/.tscanner/script-rules), or [AI rules](${REPO_URL}/tree/main/.tscanner/ai-rules) to catch it forever. Use the VSCode extension's "Copy Issues" button to get a [ready-to-paste prompt](${REPO_URL}/blob/main/assets/prompts/fix-tscanner-issues.prompt.md) and let your favorite AI tool fix everything. Before merging, see all issues at a glance in a PR comment from your CI/CD: nothing blocks by default, you decide what matters.

<div align="center">
  <a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-and-the-coding-workflow.png" target="_blank"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-and-the-coding-workflow.png" alt="TScanner and the coding workflow"></a>
  <br />
  <em>How TScanner fits into the average coding workflow</em>
</div>

<br />

<div align="center">

<details>
<summary>How does TScanner prevent issues from reaching production?</summary>
<br />
<div align="left">

- **Code Editor**: See issues in real-time while coding. Add to lint-staged to prevent committing errors.
- **Before PR**: Check all issues in your branch compared to origin/main and fix them before opening a PR.
- **CI/CD**: Every push to a PR is checked automatically. Get a single comment with clickable links to the exact lines.

</div>
</details>

<details>
<summary>Why does this matter?</summary>
<br />
<div align="left">

- **Go fast with confidence**: Know exactly what issues to fix before committing or merging.
- **Zero rejected PRs**: Over time, eliminate PR rejections due to styling or poor code quality patterns.
- **AI-powered quality**: Use AI rules to detect patterns that traditional linters miss, and let AI help fix AI-generated code.
- **Your job**: Observe code patterns to enforce/avoid and add TScanner rules for that.

</div>
</details>

</div>

<div align="center">

<details>
<summary>How TScanner maintains its own codebase?</summary>
<br />
<div align="left">

We use TScanner to maintain this very codebase. Here's our setup:

**Built-in rules (34 enabled)**: Standard code quality checks like \`no-explicit-any\`, \`prefer-const\`, \`no-console\`, etc.

**Regex rules (3)**:
- \`no-rust-deprecated\`: Block \`#[allow(deprecated)]\` in Rust code
- \`no-rust-dead-code\`: Block \`#[allow(dead_code)]\` - remove unused code instead
- \`no-process-env\`: Prevent direct \`process.env\` access

**Script rules (8)**:
- [\`types-parity-match\`](${REPO_URL}/blob/main/.tscanner/script-rules/types-parity-match.ts): Ensure TypeScript and Rust shared types are in sync
- [\`config-schema-match\`](${REPO_URL}/blob/main/.tscanner/script-rules/config-schema-match.ts): Keep Rust config and TypeScript schema aligned
- [\`cli-builder-match\`](${REPO_URL}/blob/main/.tscanner/script-rules/cli-builder-match.ts): CLI builder must cover all CLI check flags
- [\`action-zod-match\`](${REPO_URL}/blob/main/.tscanner/script-rules/action-zod-match.ts): GitHub Action inputs must match Zod validation
- [\`readme-toc-match\`](${REPO_URL}/blob/main/.tscanner/script-rules/readme-toc-match.ts): README table of contents must match all headings
- [\`rust-entry-simple\`](${REPO_URL}/blob/main/.tscanner/script-rules/rust-entry-simple.ts): \`lib.rs\` and \`mod.rs\` should only contain module declarations
- [\`no-long-files\`](${REPO_URL}/blob/main/.tscanner/script-rules/no-long-files.ts): Files cannot exceed 300 lines
- [\`no-default-node-imports\`](${REPO_URL}/blob/main/.tscanner/script-rules/no-default-node-imports.ts): Use named imports for Node.js modules

**AI rules (2)**:
- [\`no-dead-code\`](${REPO_URL}/blob/main/.tscanner/ai-rules/no-dead-code.md): Detect dead code patterns in Rust executors
- [\`find-enum-candidates\`](${REPO_URL}/blob/main/.tscanner/ai-rules/find-enum-candidates.md): Find type unions that could be enums

> TIP: Check the [\`.tscanner/\`](${REPO_URL}/tree/main/.tscanner) folder to see the full config and script implementations.

</div>
</details>

<details>
<summary>How am I using this to improve my code at work?</summary>

<br />

<div align="left">

I basically observe code patterns to enforce/avoid and add custom rules, here are my current rules: 

regex rules: 

\`\`\`jsonc
"regex": {
  "no-nestjs-logger": {
    "pattern": "import\\\\s*\\\\{[^}]*Logger[^}]*\\\\}\\\\s*from\\\\s*['\\"]@nestjs/common['\\"]",
    "message": "Do not use NestJS Logger. Import from custom logger instead"
  },
  "no-typeorm-for-feature": {
    "pattern": "TypeOrmModule\\\\.forFeature\\\\(",
    "message": "Use api/src/way-type-orm.module.ts instead"
  },
  "avoid-typeorm-raw-queries": {
    "pattern": "await this\\\\.([^.]+)\\\\.manager\\\\.query\\\\(",
    "message": "Avoid using RawQueryBuilder. Use the repository instead",
    "severity": "error"
  },
  "no-static-zod-schema": {
    "pattern": "static\\\\s+zodSchema\\\\s*=",
    "message": "Remove 'static zodSchema' from class. The schema is already passed to createZodDto() and this property is redundant"
  }
}
\`\`\`

script rules:

\`\`\`jsonc
"script": {
  "entity-registered-in-typeorm-module": {
    "command": "npx tsx script-rules/entity-registered-in-typeorm-module.ts",
    "message": "Entity must be registered in way-type-orm.module.ts",
    "severity": "error",
    "include": ["api/src/**/*.entity.ts", "api/src/way-type-orm.module.ts"]
  },
  "entity-registered-in-setup-nest": {
    "command": "npx tsx script-rules/entity-registered-in-setup-nest.ts",
    "message": "Entity must be registered in setup-nest.ts for tests",
    "severity": "error",
    "include": ["api/src/**/*.entity.ts", "api/test/helpers/setup-nest.ts"]
  },
  "no-long-files": {
    "command": "npx tsx script-rules/no-long-files.ts",
    "message": "File exceeds 600 lines limit",
    "include": ["**/*.ts"]
  }
}
\`\`\`

ai rules: 

\`\`\`
soon!
\`\`\`

Note: my rules at work are not commited to the codebase, so I basically installed tscanner globally and move the \`.tscanner\` folder into the \`.gitignore\` file

</div>

</details>

</div>

`;
  };

  const readmeConfigs = [
    { path: 'README.md' },
    { path: 'packages/cli/README.md' },
    { path: 'packages/github-action/README.md' },
    { path: 'packages/vscode-extension/README.md' },
  ];

  readmeConfigs.forEach(({ path: filePath }) => {
    const readme = new DynMarkdown<TFields>(join(rootDir, filePath));
    readme.updateField('WORKFLOW', getWorkflowContent());
    readme.saveFile();
  });

  console.log('✓ Updated WORKFLOW section');
}
