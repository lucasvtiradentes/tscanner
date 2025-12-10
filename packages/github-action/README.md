<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner - GitHub Action</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-motivation">Motivation</a> • <a href="#-quick-start">Quick Start</a> • <a href="#-usage">Usage</a><br />
  <a href="#-configuration">Configuration</a> • <a href="#-rules">Rules</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-contributing">Contributing</a> • <a href="#-license">License</a>
</div>

<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Block bad code before it reaches main. TScanner posts a comment on every PR showing exactly which rules were violated, with clickable links to the problematic lines. Reviewers focus on logic, not style debates.

<!-- <DYNFIELD:GITHUB_ACTION_DEMO_IMAGE> -->
<div align="center">
  <img width="80%" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-pr-comment-issues-found.png" alt="GitHub Action PR Comment">
  <br>
  <em>issues detected in the latest commit pushed to a PR</em>
</div>
<!-- </DYNFIELD:GITHUB_ACTION_DEMO_IMAGE> -->

<br />

<div align="center">

<details>
  <summary>Other images</summary>

<br />

<div align="center">
  <img width="80%" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-pr-comment-no-issues.png" alt="PR Comment - No Issues">
  <br>
  <em>no issues detected in the PR</em>
</div>

</details>

</div>

<div align="center">

<!-- <DYNFIELD:WAYS_TO_USE_TSCANNER> -->
<details>
<summary>Other ways to use TScanner</summary>
<br />

<table>
  <tr>
    <th>Package</th>
    <th>Description</th>
  </tr>
  <tr>
    <td>
      <div align="center">
        <b><a href="https://github.com/lucasvtiradentes/tscanner/tree/main/packages/vscode-extension#readme">VSCode Extension</a></b>
        <br />
        <br />
        <a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/VS%20Code-Extension-blue.svg" alt="VS Marketplace"></a><br /><a href="https://open-vsx.org/extension/lucasvtiradentes/tscanner-vscode"><img src="https://img.shields.io/open-vsx/v/lucasvtiradentes/tscanner-vscode?label=Open%20VSX&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&labelColor=a60ee5&color=374151" alt="Open VSX"></a>
      </div>
    </td>
    <td>Real-time sidebar integration with Git-aware branch scanning</td>
  </tr>
  <tr>
    <td>
      <div align="center">
        <b><a href="https://github.com/lucasvtiradentes/tscanner/tree/main/packages/cli#readme">CLI</a></b>
        <br />
        <br />
        <a href="https://www.npmjs.com/package/tscanner"><img src="https://img.shields.io/npm/v/tscanner?label=npm&logo=npm&logoColor=white&labelColor=CB3837&color=374151" alt="npm"></a>
      </div>
    </td>
    <td>Terminal scanning, CI/CD integration, pre-commit hooks</td>
  </tr>
</table>

</details>
<!-- </DYNFIELD:WAYS_TO_USE_TSCANNER> -->

</div>

<!-- <DYNFIELD:FEATURES> -->
## ⭐ Features<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- **Your Rules, Enforced** - 38 built-in checks + define your own with regex, scripts, or AI
- **Focus on What Matters** - Scan your branch changes only, or audit the full codebase
- **Catch Before Merge** - PR comments show violations with clickable links to exact lines
- **One Comment, Updated** - No spam, same comment updated on each push
- **Block or Warn** - Fail the check or just inform, your choice
<!-- </DYNFIELD:FEATURES> -->

<!-- <DYNFIELD:MOTIVATION> -->
## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI generates code fast, but it doesn't know your project's conventions, preferred patterns, or forbidden shortcuts. You end up reviewing the same issues over and over.

TScanner lets you define those rules once. Every AI-generated file, every PR, every save: automatically checked against your standards. Stop repeating yourself in code reviews.

<div align="center">

<details>
<summary>Use cases for this project</summary>
<br />

<div align="left">

- **Project Consistency** - Enforce import styles, naming conventions, and code organization rules
- **PR Quality Gates** - Auto-comment violations before merge so reviewers focus on logic
- **AI Code Validation** - Real-time feedback on AI-generated code before accepting
- **Flexible Customization** - Built-in rules + custom scripts and AI rules for complex logic 

</div>

</details>

</div>

<!-- </DYNFIELD:MOTIVATION> -->

## 🚀 Quick Start<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

<!-- <DYNFIELD:QUICK_START_INSTALL> -->
1. Install locally

```bash
npm install -D tscanner
```

2. Initialize configuration

```bash
tscanner init
```

> **Tip:** Use `tscanner init --full` for a complete config with example regex, script, and AI rules.
<!-- </DYNFIELD:QUICK_START_INSTALL> -->

After that you can already setup the GitHub Action:

<!-- <DYNFIELD:QUICK_START_GITHUB_ACTION> -->
3. Create `.github/workflows/tscanner.yml`:

```yaml
name: Code Quality
on: [pull_request]

jobs:
  tscanner:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: lucasvtiradentes/tscanner-action@v0.0.29
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
```

4. Open a PR to see it in action
<!-- </DYNFIELD:QUICK_START_GITHUB_ACTION> -->

## 📖 Usage<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

### Inputs

<div align="center">
<table>
  <tr>
    <th>Input</th>
    <th>Required</th>
    <th>Default</th>
    <th>Description</th>
  </tr>
  <tr>
    <td><code>github-token</code></td>
    <td>Yes</td>
    <td>-</td>
    <td>GitHub token for posting PR comments</td>
  </tr>
  <tr>
    <td><code>target-branch</code></td>
    <td>-</td>
    <td>-</td>
    <td>Branch to compare (enables diff mode). Example: <code>origin/main</code></td>
  </tr>
  <tr>
    <td><code>config-path</code></td>
    <td>-</td>
    <td><code>.tscanner</code></td>
    <td>Path to tscanner config directory</td>
  </tr>
  <tr>
    <td><code>tscanner-version</code></td>
    <td>-</td>
    <td><code>latest</code></td>
    <td>NPM version of tscanner CLI</td>
  </tr>
  <tr>
    <td><code>group-by</code></td>
    <td>-</td>
    <td><code>file</code></td>
    <td>Grouping mode: <code>file</code> or <code>rule</code></td>
  </tr>
  <tr>
    <td><code>continue-on-error</code></td>
    <td>-</td>
    <td><code>false</code></td>
    <td>Continue workflow even if errors found</td>
  </tr>
  <tr>
    <td><code>timezone</code></td>
    <td>-</td>
    <td><code>UTC</code></td>
    <td>Timezone for PR comment timestamps</td>
  </tr>
  <tr>
    <td><code>annotations</code></td>
    <td>-</td>
    <td><code>true</code></td>
    <td>Add inline annotations in PR diff</td>
  </tr>
  <tr>
    <td><code>summary</code></td>
    <td>-</td>
    <td><code>true</code></td>
    <td>Write results to GitHub Step Summary</td>
  </tr>
  <tr>
    <td><code>ai-mode</code></td>
    <td>-</td>
    <td><code>ignore</code></td>
    <td>AI rules: <code>ignore</code>, <code>include</code>, <code>only</code></td>
  </tr>
</table>
</div>

### Permissions

For annotations on **all lines** (not just changed lines), add `checks: write`:

```yaml
permissions:
  contents: read
  pull-requests: write
  checks: write
```

Without it, annotations only appear on lines in the PR diff (GitHub limitation).

### Scan Modes

<div align="center">
<table>
  <tr>
    <th>Mode</th>
    <th>When to use</th>
    <th>Config</th>
  </tr>
  <tr>
    <td><b>Changed files</b></td>
    <td>Recommended for PRs</td>
    <td><code>target-branch: 'origin/main'</code></td>
  </tr>
  <tr>
    <td><b>Full codebase</b></td>
    <td>Audit entire repo</td>
    <td>Omit <code>target-branch</code></td>
  </tr>
</table>
</div>


### AI Rules Setup

To enable AI-powered rules in your workflow, you need:

1. **AI provider CLI installed** (`claude` or `gemini`)
2. **OAuth credentials** from your local machine

> **Note:** OAuth tokens have refresh tokens that auto-renew. You only need to update the secret if authentication stops working.

<div align="center">

<details>
<summary><strong>Claude Setup (Claude Max subscription)</strong></summary>

<br/>

<div align="left">


1. **Local setup** - Run in your terminal:
```bash
npm install -g @anthropic-ai/claude-code
claude  # Login with your Claude Max account
```

2. **Copy credentials** - Get the content of `~/.claude/.credentials.json`

3. **Add GitHub Secret** - Go to repo Settings → Secrets → Actions → New secret:
   - Name: `CLAUDE_CREDENTIALS`
   - Value: paste the JSON content

4. **Workflow**:
```yaml
name: Code Quality
on: [pull_request]

jobs:
  tscanner:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Claude CLI
        run: npm install -g @anthropic-ai/claude-code

      - name: Setup Claude credentials
        run: |
          mkdir -p ~/.claude
          echo '${{ secrets.CLAUDE_CREDENTIALS }}' > ~/.claude/.credentials.json

      - uses: lucasvtiradentes/tscanner-action@v0.0.29
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          ai-mode: include
```

</div>

</details>

<details>
<summary><strong>Gemini Setup (FREE - 1000 req/day)</strong></summary>

<br/>

<div align="left">

1. **Local setup** - Run in your terminal:
```bash
npm install -g @google/gemini-cli
gemini  # Login with your Google account
```

2. **Copy credentials** - Get the content of `~/.gemini/oauth_creds.json`

3. **Add GitHub Secret** - Go to repo Settings → Secrets → Actions → New secret:
   - Name: `GEMINI_CREDENTIALS`
   - Value: paste the JSON content

4. **Workflow**:
```yaml
name: Code Quality
on: [pull_request]

jobs:
  tscanner:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Gemini CLI
        run: npm install -g @google/gemini-cli

      - name: Setup Gemini credentials
        run: |
          mkdir -p ~/.gemini
          echo '${{ secrets.GEMINI_CREDENTIALS }}' > ~/.gemini/oauth_creds.json
          echo '{"security":{"auth":{"selectedType":"oauth-personal"}}}' > ~/.gemini/settings.json

      - uses: lucasvtiradentes/tscanner-action@v0.0.29
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          ai-mode: include
```

</div>

</details>

<details>
<summary><strong>Manual Dispatch (AI rules only)</strong></summary>

<br/>

<div align="left">

Useful for testing AI rules without running all other checks:

```yaml
name: AI Rules Test
on:
  workflow_dispatch:

jobs:
  ai-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Claude CLI
        run: npm install -g @anthropic-ai/claude-code

      - name: Setup Claude credentials
        run: |
          mkdir -p ~/.claude
          echo '${{ secrets.CLAUDE_CREDENTIALS }}' > ~/.claude/.credentials.json

      - uses: lucasvtiradentes/tscanner-action@v0.0.29
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          ai-mode: only
          continue-on-error: true
```

</div>

</details>

</div>


### Full Configuration

```yaml
name: Code Quality
on: [pull_request]

jobs:
  tscanner:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write
      checks: write
    steps:
      - uses: actions/checkout@v4

      # Optional: cache for faster runs
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'pnpm'

      - uses: lucasvtiradentes/tscanner-action@v0.0.29
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          target-branch: 'origin/main'        # omit to scan full codebase
          config-path: '.tscanner'
          tscanner-version: 'latest'
          group-by: 'file'                    # or 'rule'
          continue-on-error: 'false'
          timezone: 'UTC'
          annotations: 'true'
          summary: 'true'
          ai-mode: 'ignore'                 # or 'include', 'only'
```

<!-- <DYNFIELD:COMMON_SECTION_CONFIG> -->
## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To scan your code, you need to set up the rules in the TScanner config folder. Here's how to get started:

1. **CLI**: Run `tscanner init` in your project root (**Recommended**)
2. **Manual**: Copy the default config below to `.tscanner/config.jsonc`

<div align="center">
<details>
<summary><strong>Default configuration</strong></summary>

<br/>

<div align="left">

```json
{
  "$schema": "https://unpkg.com/tscanner@0.0.32/schema.json",
  "files": {
    "include": [
      "**/*.ts",
      "**/*.tsx",
      "**/*.js",
      "**/*.jsx",
      "**/*.mjs",
      "**/*.cjs",
      "**/*mts",
      "**/*cts"
    ],
    "exclude": [
      "**/node_modules/**",
      "**/dist/**",
      "**/build/**",
      "**/.git/**",
      "**/.next/**"
    ]
  },
  "codeEditor": {
    "highlightErrors": true,
    "highlightWarnings": true,
    "highlightInfos": true,
    "highlightHints": true,
    "scanInterval": 0,
    "aiScanInterval": 0
  }
}
```

</div>
</details>

<details>
<summary><strong>Additional info about configuration</strong></summary>

<br/>

<div align="left">

All configuration fields are **optional** with sensible defaults. The minimum required config is just enabling the rules you want:

```json
{
  "rules": {
    "builtin": {
      "no-explicit-any": {}
    }
  }
}
```

With this minimal config, TScanner will scan all `.ts/.tsx/.js/.jsx/.mjs/.cjs` files, excluding `node_modules/`, `dist/`, `build/`, and `.git/` directories.

**Understanding `files.include` and `files.exclude`:**

- `files.include`: Glob patterns for files to scan (default: `["**/*.ts", "**/*.tsx", "**/*.js", "**/*.jsx", "**/*.mjs", "**/*.cjs"]`)
- `files.exclude`: Glob patterns for files/folders to ignore (default: `["**/node_modules/**", "**/dist/**", "**/build/**", "**/.git/**"]`)

Example with per-rule file patterns:

```json
{
  "rules": {
    "builtin": {
      "no-explicit-any": {},
      "no-console": {
        "exclude": ["src/utils/logger.ts"]
      },
      "max-function-length": {
        "include": ["src/core/**/*.ts"]
      }
    }
  }
}
```

This config:
- Runs `no-explicit-any` on all files (uses global `files` patterns)
- Runs `no-console` on all files except `src/utils/logger.ts`
- Runs `max-function-length` only on files inside `src/core/`

**Inline Disables:**

```typescript
// tscanner-ignore-next-line no-explicit-any
const data: any = fetchData();

// tscanner-ignore
// Entire file is skipped
```

</div>
</details>

</div>
<!-- </DYNFIELD:COMMON_SECTION_CONFIG> -->

<!-- <DYNFIELD:RULES> -->
## 📋 Rules<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Customize TScanner to validate what matters to your project while maintaining consistency.

<div align="center">

<table>
  <tr>
    <th width="100">Type</th>
    <th width="250">Use Case</th>
    <th width="400">Example</th>
  </tr>
  <tr>
    <td><b>Built-in</b></td>
    <td>38 ready-to-use AST rules</td>
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


  <br />
  
<div align="center">

<details>
<summary>Built-in rules (38)</summary>
<br />

<div align="left">

#### Type Safety (6)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="400">Description</th>
    <th width="150">Options</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/type_safety/no_explicit_any.rs"><code>no-explicit-any</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Detects usage of TypeScript 'any' type (<code>: any</code> and <code>as any</code>). Using 'any' defeats the purpose of TypeScript's type system.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-explicit-any"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-explicit-any"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/type_safety/no_implicit_any.rs"><code>no-implicit-any</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Detects function parameters without type annotations that implicitly have 'any' type.</td>
    <td align="left"></td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/type_safety/no_inferrable_types.rs"><code>no-inferrable-types</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows explicit type annotations on variables initialized with literal values. TypeScript can infer these types automatically.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-inferrable-types"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-inferrable-types"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/type_safety/no_non_null_assertion.rs"><code>no-non-null-assertion</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows the non-null assertion operator (!). Use proper null checks or optional chaining instead.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-non-null-assertion"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-non-null-assertion"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/type_safety/no_single_or_array_union.rs"><code>no-single-or-array-union</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows union types that combine a type with its array form (e.g., <code>string | string[]</code>, <code>number | number[]</code>). Prefer using a consistent type to avoid handling multiple cases in function implementations.</td>
    <td align="left"></td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/type_safety/no_unnecessary_type_assertion.rs"><code>no-unnecessary-type-assertion</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows type assertions on values that are already of the asserted type (e.g., "hello" as string, 123 as number).</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-unnecessary-type-assertion"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
</table>

<div align="left">

#### Code Quality (13)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="400">Description</th>
    <th width="150">Options</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/max_function_length.rs"><code>max-function-length</code></a><br/><br/><img src="https://img.shields.io/badge/configurable-green" alt="Configurable"></div></td>
    <td align="left">Enforces a maximum number of statements in functions. Long functions are harder to understand and maintain.</td>
    <td align="left"><code>maxLength</code>: 50</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/max-lines-per-function"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/max_params.rs"><code>max-parameters</code></a><br/><br/><img src="https://img.shields.io/badge/configurable-green" alt="Configurable"></div></td>
    <td align="left">Limits the number of parameters in a function. Functions with many parameters should use an options object instead.</td>
    <td align="left"><code>maxParams</code>: 4</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/max-params"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_async_without_await.rs"><code>no-async-without-await</code></a></div></td>
    <td align="left">Disallows async functions that don't use await. The async keyword is unnecessary if await is never used.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/require-await"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/use-await"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_console.rs"><code>no-console</code></a><br/><br/><img src="https://img.shields.io/badge/regex--rule-6C757D" alt="Regex rule"> <img src="https://img.shields.io/badge/configurable-green" alt="Configurable"></div></td>
    <td align="left">Disallow the use of console methods. Console statements should be removed before committing to production.</td>
    <td align="left"><code>methods</code>: [21 items]</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-console"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-console"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_else_return.rs"><code>no-else-return</code></a></div></td>
    <td align="left">Disallows else blocks after return statements. The else is unnecessary since the function already returned.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-else-return"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-useless-else"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_empty_class.rs"><code>no-empty-class</code></a></div></td>
    <td align="left">Disallows empty classes without methods or properties.</td>
    <td align="left"></td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_empty_function.rs"><code>no-empty-function</code></a></div></td>
    <td align="left">Disallows empty functions and methods. Empty functions are often leftovers from incomplete code.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-empty-function"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_empty_interface.rs"><code>no-empty-interface</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows empty interface declarations. Empty interfaces are equivalent to {} and usually indicate incomplete code.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-empty-interface"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-empty-interface"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_nested_ternary.rs"><code>no-nested-ternary</code></a></div></td>
    <td align="left">Disallows nested ternary expressions. Nested ternaries are hard to read and should be replaced with if-else statements.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-nested-ternary"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-nested-ternary"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_return_await.rs"><code>no-return-await</code></a></div></td>
    <td align="left">Disallows redundant 'return await' in async functions. The await is unnecessary since the function already returns a Promise.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/return-await"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_todo_comments.rs"><code>no-todo-comments</code></a><br/><br/><img src="https://img.shields.io/badge/regex--rule-6C757D" alt="Regex rule"> <img src="https://img.shields.io/badge/configurable-green" alt="Configurable"></div></td>
    <td align="left">Detects TODO comments (case insensitive). Configure 'keywords' option to detect additional markers like FIXME, HACK, etc.</td>
    <td align="left"><code>keywords</code>: [1 items]</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-warning-comments"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_unused_vars.rs"><code>no-unused-variables</code></a></div></td>
    <td align="left">Detects variables that are declared but never used in the code.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-unused-vars"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-unused-variables"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/code_quality/no_useless_catch.rs"><code>no-useless-catch</code></a></div></td>
    <td align="left">Disallows catch blocks that only rethrow the caught error. Remove the try-catch or add meaningful error handling.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-useless-catch"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-useless-catch"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
</table>

<div align="left">

#### Bug Prevention (4)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="400">Description</th>
    <th width="150">Options</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/bug_prevention/consistent_return.rs"><code>consistent-return</code></a></div></td>
    <td align="left">Requires consistent return behavior in functions. Either all code paths return a value or none do.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/consistent-return"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/bug_prevention/no_constant_condition.rs"><code>no-constant-condition</code></a></div></td>
    <td align="left">Disallows constant expressions in conditions (if/while/for/ternary). Likely a programming error.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-constant-condition"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-constant-condition"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/bug_prevention/no_floating_promises.rs"><code>no-floating-promises</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows floating promises (promises used as statements without await, .then(), or .catch()). Unhandled promises can lead to silent failures.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-floating-promises"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-floating-promises"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/bug_prevention/no_unreachable_code.rs"><code>no-unreachable-code</code></a></div></td>
    <td align="left">Detects code after return, throw, break, or continue statements. This code will never execute.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-unreachable"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-unreachable"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
</table>

<div align="left">

#### Variables (3)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="400">Description</th>
    <th width="150">Options</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/variables/no_shadow.rs"><code>no-shadow</code></a></div></td>
    <td align="left">Disallows variable declarations that shadow variables in outer scopes. Shadowing can lead to confusing code and subtle bugs.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-shadow"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/variables/no_var.rs"><code>no-var</code></a></div></td>
    <td align="left">Disallows the use of 'var' keyword. Use 'let' or 'const' instead for block-scoped variables.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-var"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-var"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/variables/prefer_const.rs"><code>prefer-const</code></a></div></td>
    <td align="left">Suggests using 'const' instead of 'let' when variables are never reassigned.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/prefer-const"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/use-const"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
</table>

<div align="left">

#### Imports (8)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="400">Description</th>
    <th width="150">Options</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/imports/no_absolute_imports.rs"><code>no-absolute-imports</code></a></div></td>
    <td align="left">Disallows absolute imports without alias. Prefer relative or aliased imports.</td>
    <td align="left"></td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/imports/no_alias_imports.rs"><code>no-alias-imports</code></a></div></td>
    <td align="left">Disallows aliased imports (starting with @). Prefer relative imports.</td>
    <td align="left"></td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/imports/no_default_export.rs"><code>no-default-export</code></a></div></td>
    <td align="left">Disallows default exports. Named exports are preferred for better refactoring support and explicit imports.</td>
    <td align="left"></td>
    <td align="left"><a href="https://biomejs.dev/linter/rules/no-default-export"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/imports/no_duplicate_imports.rs"><code>no-duplicate-imports</code></a></div></td>
    <td align="left">Disallows multiple import statements from the same module. Merge them into a single import.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-duplicate-imports"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-duplicate-json-keys"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/imports/no_dynamic_import.rs"><code>no-dynamic-import</code></a></div></td>
    <td align="left">Disallows dynamic import() expressions. Dynamic imports make static analysis harder and can impact bundle optimization.</td>
    <td align="left"></td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/imports/no_forwarded_exports.rs"><code>no-forwarded-exports</code></a></div></td>
    <td align="left">Disallows re-exporting from other modules. This includes direct re-exports (export { X } from 'module'), star re-exports (export * from 'module'), and re-exporting imported values.</td>
    <td align="left"></td>
    <td align="left"><a href="https://biomejs.dev/linter/rules/no-re-export-all"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/imports/no_nested_require.rs"><code>no-nested-require</code></a></div></td>
    <td align="left">Disallows require() calls inside functions, blocks, or conditionals. Require statements should be at the top level for static analysis.</td>
    <td align="left"></td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/global-require"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/imports/no_relative_imports.rs"><code>no-relative-imports</code></a></div></td>
    <td align="left">Detects relative imports (starting with './' or '../'). Prefer absolute imports with @ prefix for better maintainability.</td>
    <td align="left"></td>
    <td align="left"></td>
  </tr>
</table>

<div align="left">

#### Style (4)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="400">Description</th>
    <th width="150">Options</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/style/prefer_interface_over_type.rs"><code>prefer-interface-over-type</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Suggests using 'interface' keyword instead of 'type' for consistency.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/consistent-type-definitions"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/style/prefer_nullish_coalescing.rs"><code>prefer-nullish-coalescing</code></a></div></td>
    <td align="left">Suggests using nullish coalescing (??) instead of logical OR (||) for default values. The || operator treats 0, "", and false as falsy, which may not be intended.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/prefer-nullish-coalescing"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/style/prefer_optional_chain.rs"><code>prefer-optional-chain</code></a></div></td>
    <td align="left">Suggests using optional chaining (?.) instead of logical AND (&&) chains for null checks.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/prefer-optional-chain"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/use-optional-chain"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin/style/prefer_type_over_interface.rs"><code>prefer-type-over-interface</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Suggests using 'type' keyword instead of 'interface' for consistency. Type aliases are more flexible and composable.</td>
    <td align="left"></td>
    <td align="left"><a href="https://typescript-eslint.io/rules/consistent-type-definitions"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
</table>

</details>

<details>
<summary>Regex rules examples</summary>
<br />
<div align="left">

Define patterns to match in your code using regular expressions:

**Config** (`.tscanner/config.jsonc`):
```json
{
  "rules": {
    "regex": {
      "no-todos": {
        "pattern": "TODO:|FIXME:",
        "message": "Remove TODO comments before merging",
        "severity": "warning"
      },
      "no-debug-logs": {
        "pattern": "console\\.(log|debug|info)",
        "message": "Remove debug statements",
        "severity": "warning",
        "exclude": ["**/*.test.ts"]
      }
    }
  }
}
```

> 💡 See a real example in the [`.tscanner/`](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner) folder of this project.

</div>
</details>

<details>
<summary>Script rules examples</summary>
<br />
<div align="left">

Run custom scripts that receive file data via stdin and output issues as JSON:

**Config** (`.tscanner/config.jsonc`):
```json
{
  "rules": {
    "script": {
      "no-debug-comments": {
        "command": "npx tsx .tscanner/script-rules/no-debug-comments.ts",
        "message": "Debug comments should be removed",
        "severity": "warning"
      }
    }
  }
}
```

**Script** (`.tscanner/script-rules/no-debug-comments.ts`):
```typescript
#!/usr/bin/env npx tsx

import { stdin } from 'node:process';

type ScriptFile = {
  path: string;
  content: string;
  lines: string[];
};

type ScriptInput = {
  files: ScriptFile[];
  options?: Record<string, unknown>;
  workspaceRoot: string;
};

type ScriptIssue = {
  file: string;
  line: number;
  column?: number;
  message: string;
};

async function main() {
  let data = '';
  for await (const chunk of stdin) {
    data += chunk;
  }

  const input: ScriptInput = JSON.parse(data);
  const issues: ScriptIssue[] = [];

  for (const file of input.files) {
    for (let i = 0; i < file.lines.length; i++) {
      const line = file.lines[i];
      if (/\/\/\s*(DEBUG|HACK|XXX|TEMP)\b/i.test(line)) {
        issues.push({
          file: file.path,
          line: i + 1,
          message: `Debug comment found: "${line.trim().substring(0, 50)}"`,
        });
      }
    }
  }

  console.log(JSON.stringify({ issues }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
```

> 💡 See a real example in the [`.tscanner/`](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner) folder of this project.

</div>
</details>

<details>
<summary>AI rules examples</summary>
<br />
<div align="left">

Use AI prompts to perform semantic code analysis:

**Config** (`.tscanner/config.jsonc`):
```json
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
```

**Prompt** (`.tscanner/ai-rules/find-complexity.md`):
```markdown
# Find Complex Functions

Analyze the provided code and identify functions that are overly complex.

## What to look for

1. Functions with high cyclomatic complexity (many branches/loops)
2. Deeply nested code blocks (3+ levels)
3. Functions doing too many things (violating single responsibility)
4. Long parameter lists that should be objects

## Output format

Report each complex function with:
- The function name
- Why it's complex
- A brief suggestion for improvement

{{FILES}}
```

> 💡 See a real example in the [`.tscanner/`](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner) folder of this project.

</div>
</details>

</div>
<!-- </DYNFIELD:RULES> -->

<!-- <DYNFIELD:INSPIRATIONS> -->
## 💡 Inspirations<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- [Biome](https://github.com/biomejs/biome) - High-performance Rust-based linter and formatter for web projects
- [ESLint](https://github.com/eslint/eslint) - Find and fix problems in your JavaScript code
- [Vitest](https://github.com/vitest-dev/vitest) - Next generation testing framework powered by Vite
- [VSCode Bookmarks](https://github.com/alefragnani/vscode-bookmarks) - Bookmarks Extension for Visual Studio Code

<div align="center">
  <details>
  <summary>How each project was used?</summary>

<br />

<div align="left">
<ul>
  <li><a href="https://github.com/biomejs/biome">Biome</a>:
    <ul>
      <li>multi-crate Rust architecture (cli, core, server separation)</li>
      <li>LSP server implementation for real-time IDE diagnostics</li>
      <li>parallel file processing with Rayon</li>
      <li>SWC parser integration for JavaScript/TypeScript AST</li>
      <li>visitor pattern for AST node traversal</li>
      <li>file-level result caching strategy</li>
    </ul>
  </li>
  <li><a href="https://github.com/eslint/eslint">ESLint</a>:
    <ul>
      <li>inline suppression system (disable-next-line, disable-file patterns)</li>
      <li>precursor on javascript linting concepts</li>
      <li>inspiration for rule ideas and detection patterns</li>
    </ul>
  </li>
  <li><a href="https://github.com/vitest-dev/vitest">Vitest</a>:
    <ul>
      <li>glob pattern matching techniques for file discovery</li>
    </ul>
  </li>
  <li><a href="https://github.com/alefragnani/vscode-bookmarks">VSCode Bookmarks</a>:
    <ul>
      <li>sidebar icon badge displaying issue count</li>
    </ul>
  </li>
</ul>
</div>

  </details>
</div>

<div align="center">
  <details>
  <summary>Notes about the huge impact Biome has on this project</summary>

<br />

<div align="left">
This project only makes sense because it is fast, and it can only be fast because we applied the same techniques from the amazing Biome project.
Once you experience a project powered by Biome and compare it to the traditional ESLint + Prettier setup, it feels like we were being fooled our entire careers.
The speed difference is so dramatic that going back to the old tools feels almost unbearable.
I am deeply grateful to the Biome team for open-sourcing such an incredible project and paving the way for high-performance JavaScript tooling.
</div>

  </details>
</div>
<!-- </DYNFIELD:INSPIRATIONS> -->

<!-- <DYNFIELD:CONTRIBUTING> -->
## 🤝 Contributing<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Contributions are welcome! See [CONTRIBUTING.md](https://github.com/lucasvtiradentes/tscanner/blob/main/CONTRIBUTING.md) for setup instructions and development workflow.
<!-- </DYNFIELD:CONTRIBUTING> -->

## 📜 License<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

MIT License - see [LICENSE](LICENSE) file for details.

<!-- <DYNFIELD:FOOTER> -->
<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

<br />

<div align="center">
  <div>
    <a target="_blank" href="https://www.linkedin.com/in/lucasvtiradentes/"><img src="https://img.shields.io/badge/-linkedin-blue?logo=Linkedin&logoColor=white" alt="LinkedIn"></a>
    <a target="_blank" href="mailto:lucasvtiradentes@gmail.com"><img src="https://img.shields.io/badge/gmail-red?logo=gmail&logoColor=white" alt="Gmail"></a>
    <a target="_blank" href="https://x.com/lucasvtiradente"><img src="https://img.shields.io/badge/-X-black?logo=X&logoColor=white" alt="X"></a>
    <a target="_blank" href="https://github.com/lucasvtiradentes"><img src="https://img.shields.io/badge/-github-gray?logo=Github&logoColor=white" alt="Github"></a>
  </div>
</div>
<!-- </DYNFIELD:FOOTER> -->
