<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner - CLI</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-motivation">Motivation</a> • <a href="#-quick-start">Quick Start</a> • <a href="#-usage">Usage</a> • <a href="#-configuration">Configuration</a><br />
  <a href="#-use-cases">Use Cases</a> • <a href="#-rules">Rules</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-contributing">Contributing</a> • <a href="#-license">License</a>
</div>

<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Scan your codebase from the terminal. Run before commits, in CI pipelines, or as part of your build. Get JSON output for tooling or pretty output for humans. Same rules as the VS Code extension.

<!-- <DYNFIELD:CLI_DEMO_IMAGE> -->
<div align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-cli-demo.png" alt="CLI Scan Screenshot">
  <br>
  <em>scanning the codebase via CLI</em>
</div>
<!-- </DYNFIELD:CLI_DEMO_IMAGE> -->

<br />

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
        <b><a href="https://github.com/lucasvtiradentes/tscanner/tree/main/packages/github-action#readme">GitHub Action</a></b>
        <br />
        <br />
        <a href="https://github.com/marketplace/actions/tscanner-action"><img src="https://img.shields.io/badge/Marketplace-black.svg?logo=github&logoColor=white&labelColor=181717&color=374151" alt="GitHub Marketplace"></a>
      </div>
    </td>
    <td>CICD integration with analysis summary attached to PR comments</td>
  </tr>
</table>

</details>
<!-- </DYNFIELD:WAYS_TO_USE_TSCANNER> -->

</div>

<!-- <DYNFIELD:FEATURES> -->
## ⭐ Features<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- **Your Rules, Enforced** - 39 built-in checks + define your own with regex, scripts, or AI
- **Sub-second Scans** - Rust engine processes hundreds of files in <1s, with smart caching
- **Focus on What Matters** - Scan your branch changes only, or audit the full codebase
- **CI-Ready** - JSON output for automation, exit codes for pipelines
<!-- </DYNFIELD:FEATURES> -->

<!-- <DYNFIELD:MOTIVATION> -->
## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI generates code fast, but it doesn't know your project's conventions, preferred patterns, or forbidden shortcuts. You end up reviewing the same issues over and over.

TScanner lets you define those rules once. Every AI-generated file, every PR, every save: automatically checked against your standards. Stop repeating yourself in code reviews.
<!-- </DYNFIELD:MOTIVATION> -->

## 🚀 Quick Start<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

<!-- <DYNFIELD:QUICK_START_CLI> -->
1. Install globally

```bash
npm install -g tscanner
```

2. Initialize configuration

```bash
tscanner init
```

3. Use it

```bash
# Scan workspace
tscanner check

# Scan only changed files vs branch
tscanner check --branch origin/main
```
<!-- </DYNFIELD:QUICK_START_CLI> -->

## 📖 Usage<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

### Commands Overview

| Command | Description | Flags |
|---------|-------------|-------|
| `init [path]` | Create `.tscanner/config.jsonc` configuration | - |
| `check [path]` | Scan files and report issues | `--no-cache`, `--json`, `--pretty`, `--by-rule`, `--branch`, `--file`, `--rule`, `--continue-on-error`, `--config` |
| `rules [path]` | List all available rules with metadata | `--config` |
| `--help` or `-h` | Show help information | - |
| `--version` or `-V` | Show version number | - |

### Check Command Flags

| Flag | Description | Example |
|------|-------------|---------|
| `--no-cache` | Skip cache and force full scan | `tscanner check --no-cache` |
| `--json` | Output results as JSON | `tscanner check --json` |
| `--pretty` | Pretty output with rule definitions | `tscanner check --pretty` |
| `--by-rule` | Group issues by rule instead of file | `tscanner check --by-rule` |
| `--branch <BRANCH>` | Only scan files changed vs branch | `tscanner check --branch main` |
| `--file <PATTERN>` | Filter by file glob pattern | `tscanner check --file "src/**"` |
| `--rule <RULE>` | Filter by specific rule | `tscanner check --rule no-any-type` |
| `--continue-on-error` | Don't exit with error code | `tscanner check --continue-on-error` |
| `--config <DIR>` | Custom config directory | `tscanner check --config ./custom` |

### Examples

<details>
<summary><b>Initialize Configuration</b></summary>

```bash
# Create .tscanner/config.jsonc in current directory
tscanner init

# Create config in specific directory
tscanner init /path/to/project
```

Creates `.tscanner/config.jsonc` with default rule configuration (see [Configuration](#-configuration) section).

</details>

<details>
<summary><b>Scan Files</b></summary>

```bash
# Basic scan
tscanner check

# Scan specific directory
tscanner check /path/to/project

# Skip cache (force full rescan)
tscanner check --no-cache

# Output as JSON
tscanner check --json

# Pretty output with rule definitions
tscanner check --pretty

# Group results by rule instead of file
tscanner check --by-rule
```

**Example output:**
```
Scanning...

src/index.ts
  ✖ 5:10 Found ': any' type annotation [no-any-type]
  ⚠ 10:7 'count' is never reassigned, use 'const' instead [prefer-const]

src/utils.ts
  ⚠ 15:3 console.log found [no-console-log]

✖ 2 errors, 2 warnings
Scanned 2 files in 45ms
```

**Exit codes:**
- `0` - No errors found
- `1` - Errors found or configuration missing

</details>

<details>
<summary><b>Advanced Filtering</b></summary>

```bash
# Only scan files changed compared to branch
tscanner check --branch origin/main
tscanner check --branch develop

# Filter by file pattern (glob)
tscanner check --file "src/**/*.ts"
tscanner check --file "components/**/*.tsx"

# Filter by specific rule
tscanner check --rule no-console-log
tscanner check --rule no-any-type

# Combine filters
tscanner check --branch main --file "src/**" --rule no-console-log

# Continue on error (don't exit with code 1)
tscanner check --continue-on-error

# Use custom config location
tscanner check --config /path/to/config/dir
```

</details>

<details>
<summary><b>List Rules</b></summary>

```bash
# Show all available rules
tscanner rules

# Show rules for specific project
tscanner rules /path/to/project

# Use custom config location
tscanner rules --config /path/to/config/dir
```

**Output shows:**
- Rule name and description
- Current status (enabled/disabled)
- Severity level (error/warning)
- Rule type (ast/regex)

</details>

<!-- <DYNFIELD:COMMON_SECTION_CONFIG> -->
## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To scan your code, you need to set up the rules in the TScanner config folder. Here's how to get started:

1. **VSCode Extension**: TScanner icon in the status bar → `Manage Rules` → Select desired rules → `Save`
2. **CLI**: Run `tscanner init` in your project root
3. **Manual**: Copy the default config below to `.tscanner/config.json`

The default configuration is:

```json
{
  "$schema": "https://unpkg.com/tscanner@0.0.22/schema.json",
  "builtinRules": {
    "no-any-type": {}
  },
  "customRules": {},
  "files": {
    "include": [
      "**/*.ts",
      "**/*.tsx",
      "**/*.js",
      "**/*.jsx",
      "**/*.mjs",
      "**/*.cjs"
    ],
    "exclude": [
      "**/node_modules/**",
      "**/dist/**",
      "**/build/**",
      "**/.git/**"
    ]
  },
  "lsp": {
    "errors": true,
    "warnings": false
  }
}
```

**Inline Disables:**

```typescript
// tscanner-disable-next-line no-any-type
const data: any = fetchData();

// tscanner-disable-file
// Entire file is skipped
```

<details>
<summary><strong>Additional info about configuration</strong></summary>

<br/>

All configuration fields are **optional** with sensible defaults. The minimum required config is just enabling the rules you want:

```json
{
  "builtinRules": {
    "no-any-type": {}
  }
}
```

With this minimal config, TScanner will scan all `.ts/.tsx/.js/.jsx/.mjs/.cjs` files, excluding `node_modules/`, `dist/`, `build/`, and `.git/` directories.

**Understanding `files.include` and `files.exclude`:**

- `files.include`: Glob patterns for files to scan (default: `["**/*.{ts,tsx,js,jsx,mjs,cjs}"]`)
- `files.exclude`: Glob patterns for files/folders to ignore (default: `["node_modules/**", "dist/**", "build/**", ".git/**"]`)

Example with per-rule file patterns:

```json
{
  "builtinRules": {
    "no-any-type": {},
    "no-console-log": {
      "exclude": ["src/utils/logger.ts"]
    },
    "max-function-length": {
      "include": ["src/core/**/*.ts"]
    }
  }
}
```

This config:
- Runs `no-any-type` on all files (uses global `files` patterns)
- Runs `no-console-log` on all files except `src/utils/logger.ts`
- Runs `max-function-length` only on files inside `src/core/`

</details>
<!-- </DYNFIELD:COMMON_SECTION_CONFIG> -->

## 🎯 Use Cases<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

<details>
<summary><b>CI/CD Pipeline</b></summary>

It is recommended to use [TScanner GitHub action](https://github.com/lucasvtiradentes/tscanner/tree/main/packages/github-action), but you can also set up your own workflow:

```yaml
name: Code Quality

on: [push, pull_request]

jobs:
  tscanner:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: Install TScanner
        run: npm install -g tscanner
      - name: Run TScanner validation
        run: tscanner check
```

</details>

<details>
<summary><b>Pre-commit Hook</b></summary>

```bash
#!/bin/sh
if command -v tscanner &> /dev/null && [ -f .tscanner/config.jsonc ]; then
  tscanner check --no-cache
fi
```

</details>

<details>
<summary><b>Git Pre-push Hook</b></summary>

```bash
#!/bin/sh
if command -v tscanner &> /dev/null && [ -f .tscanner/config.jsonc ]; then
  tscanner check --branch origin/main --no-cache
fi
```

</details>

<details>
<summary><b>VS Code Tasks</b></summary>

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "tscanner: Check",
      "type": "shell",
      "command": "tscanner check"
    },
    {
      "label": "tscanner: Check (No Cache)",
      "type": "shell",
      "command": "tscanner check --no-cache"
    },
    {
      "label": "tscanner: Check (Branch Changes)",
      "type": "shell",
      "command": "tscanner check --branch origin/main"
    }
  ]
}
```

</details>

<details>
<summary><b>Package.json Scripts</b></summary>

```json
{
  "scripts": {
    "lint": "tscanner check",
    "lint:nocache": "tscanner check --no-cache",
    "lint:branch": "tscanner check --branch origin/main",
    "lint:json": "tscanner check --json > lint-results.json"
  }
}
```

</details>

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
    <td><b><a href="packages/core/crates/core/src/rules">Built-in</a></b></td>
    <td>39 ready-to-use AST rules</td>
    <td><code>no-any-type</code>, <code>prefer-const</code>, <code>no-console-log</code></td>
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

<div align="center">

<details>
<summary>Built-in rules (39)</summary>
<br />

<div align="left">

#### Type Safety (6)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="450">Description</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_any_type.rs"><code>no-any-type</code></a> <sup>TS</sup></td>
    <td align="left">Detects usage of TypeScript 'any' type (<code>: any</code> and <code>as any</code>). Using 'any' defeats the purpose of TypeScript's type system.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-explicit-any"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-explicit-any"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_implicit_any.rs"><code>no-implicit-any</code></a> <sup>TS</sup></td>
    <td align="left">Detects function parameters without type annotations that implicitly have 'any' type.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_inferrable_types.rs"><code>no-inferrable-types</code></a> <sup>TS</sup></td>
    <td align="left">Disallows explicit type annotations on variables initialized with literal values. TypeScript can infer these types automatically.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-inferrable-types"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-inferrable-types"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_non_null_assertion.rs"><code>no-non-null-assertion</code></a> <sup>TS</sup></td>
    <td align="left">Disallows the non-null assertion operator (!). Use proper null checks or optional chaining instead.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-non-null-assertion"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-non-null-assertion"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_single_or_array_union.rs"><code>no-single-or-array-union</code></a> <sup>TS</sup></td>
    <td align="left">Disallows union types that combine a type with its array form (e.g., <code>string | string[]</code>, <code>number | number[]</code>). Prefer using a consistent type to avoid handling multiple cases in function implementations.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_unnecessary_type_assertion.rs"><code>no-unnecessary-type-assertion</code></a> <sup>TS</sup></td>
    <td align="left">Disallows type assertions on values that are already of the asserted type (e.g., "hello" as string, 123 as number).</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-unnecessary-type-assertion"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
</table>

<div align="left">

#### Code Quality (14)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="450">Description</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/max_function_length.rs"><code>max-function-length</code></a></td>
    <td align="left">Enforces a maximum number of statements in functions (default: 50). Long functions are harder to understand and maintain.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/max-lines-per-function"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/max_params.rs"><code>max-parameters</code></a></td>
    <td align="left">Limits the number of parameters in a function. Functions with many parameters should use an options object instead.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/max-params"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_async_without_await.rs"><code>no-async-without-await</code></a></td>
    <td align="left">Disallows async functions that don't use await. The async keyword is unnecessary if await is never used.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/require-await"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/use-await"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_console_log.rs"><code>no-console-log</code></a></td>
    <td align="left">Finds console.log() statements in code. Console statements should be removed before committing to production.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-console"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-console"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_else_return.rs"><code>no-else-return</code></a></td>
    <td align="left">Disallows else blocks after return statements. The else is unnecessary since the function already returned.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-else-return"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-useless-else"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_empty_class.rs"><code>no-empty-class</code></a></td>
    <td align="left">Disallows empty classes without methods or properties.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_empty_function.rs"><code>no-empty-function</code></a></td>
    <td align="left">Disallows empty functions and methods. Empty functions are often leftovers from incomplete code.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-empty-function"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_empty_interface.rs"><code>no-empty-interface</code></a> <sup>TS</sup></td>
    <td align="left">Disallows empty interface declarations. Empty interfaces are equivalent to {} and usually indicate incomplete code.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-empty-interface"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-empty-interface"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_magic_numbers.rs"><code>no-magic-numbers</code></a></td>
    <td align="left">Detects magic numbers in code (literals other than 0, 1, -1). Use named constants instead for better readability and maintainability.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-magic-numbers"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_nested_ternary.rs"><code>no-nested-ternary</code></a></td>
    <td align="left">Disallows nested ternary expressions. Nested ternaries are hard to read and should be replaced with if-else statements.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-nested-ternary"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-nested-ternary"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_return_await.rs"><code>no-return-await</code></a></td>
    <td align="left">Disallows redundant 'return await' in async functions. The await is unnecessary since the function already returns a Promise.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/return-await"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_todo_comments.rs"><code>no-todo-comments</code></a></td>
    <td align="left">Detects TODO, FIXME, and similar comment markers.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-warning-comments"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_unused_vars.rs"><code>no-unused-variables</code></a></td>
    <td align="left">Detects variables that are declared but never used in the code.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-unused-vars"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-unused-variables"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_useless_catch.rs"><code>no-useless-catch</code></a></td>
    <td align="left">Disallows catch blocks that only rethrow the caught error. Remove the try-catch or add meaningful error handling.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-useless-catch"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-useless-catch"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
</table>

<div align="left">

#### Bug Prevention (4)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="450">Description</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/consistent_return.rs"><code>consistent-return</code></a></td>
    <td align="left">Requires consistent return behavior in functions. Either all code paths return a value or none do.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/consistent-return"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_constant_condition.rs"><code>no-constant-condition</code></a></td>
    <td align="left">Disallows constant expressions in conditions (if/while/for/ternary). Likely a programming error.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-constant-condition"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-constant-condition"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_floating_promises.rs"><code>no-floating-promises</code></a> <sup>TS</sup></td>
    <td align="left">Disallows floating promises (promises used as statements without await, .then(), or .catch()). Unhandled promises can lead to silent failures.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-floating-promises"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-floating-promises"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_unreachable_code.rs"><code>no-unreachable-code</code></a></td>
    <td align="left">Detects code after return, throw, break, or continue statements. This code will never execute.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-unreachable"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-unreachable"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
</table>

<div align="left">

#### Variables (3)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="450">Description</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_shadow.rs"><code>no-shadow</code></a></td>
    <td align="left">Disallows variable declarations that shadow variables in outer scopes. Shadowing can lead to confusing code and subtle bugs.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-shadow"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_var.rs"><code>no-var</code></a></td>
    <td align="left">Disallows the use of 'var' keyword. Use 'let' or 'const' instead for block-scoped variables.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-var"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-var"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_const.rs"><code>prefer-const</code></a></td>
    <td align="left">Suggests using 'const' instead of 'let' when variables are never reassigned.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/prefer-const"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/use-const"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
</table>

<div align="left">

#### Imports (8)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="450">Description</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_absolute_imports.rs"><code>no-absolute-imports</code></a></td>
    <td align="left">Disallows absolute imports without alias. Prefer relative or aliased imports.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_alias_imports.rs"><code>no-alias-imports</code></a></td>
    <td align="left">Disallows aliased imports (starting with @). Prefer relative imports.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_default_export.rs"><code>no-default-export</code></a></td>
    <td align="left">Disallows default exports. Named exports are preferred for better refactoring support and explicit imports.</td>
    <td align="left"><a href="https://biomejs.dev/linter/rules/no-default-export"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_duplicate_imports.rs"><code>no-duplicate-imports</code></a></td>
    <td align="left">Disallows multiple import statements from the same module. Merge them into a single import.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-duplicate-imports"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-duplicate-json-keys"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_dynamic_import.rs"><code>no-dynamic-import</code></a></td>
    <td align="left">Disallows dynamic import() expressions. Dynamic imports make static analysis harder and can impact bundle optimization.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_forwarded_exports.rs"><code>no-forwarded-exports</code></a></td>
    <td align="left">Disallows re-exporting from other modules. This includes direct re-exports (export { X } from 'module'), star re-exports (export * from 'module'), and re-exporting imported values.</td>
    <td align="left"><a href="https://biomejs.dev/linter/rules/no-re-export-all"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_nested_require.rs"><code>no-nested-require</code></a></td>
    <td align="left">Disallows require() calls inside functions, blocks, or conditionals. Require statements should be at the top level for static analysis.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/global-require"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_relative_imports.rs"><code>no-relative-imports</code></a></td>
    <td align="left">Detects relative imports (starting with './' or '../'). Prefer absolute imports with @ prefix for better maintainability.</td>
    <td align="left"></td>
  </tr>
</table>

<div align="left">

#### Style (4)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="450">Description</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_interface_over_type.rs"><code>prefer-interface-over-type</code></a> <sup>TS</sup></td>
    <td align="left">Suggests using 'interface' keyword instead of 'type' for consistency.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/consistent-type-definitions"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_nullish_coalescing.rs"><code>prefer-nullish-coalescing</code></a></td>
    <td align="left">Suggests using nullish coalescing (??) instead of logical OR (||) for default values. The || operator treats 0, "", and false as falsy, which may not be intended.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/prefer-nullish-coalescing"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_optional_chain.rs"><code>prefer-optional-chain</code></a></td>
    <td align="left">Suggests using optional chaining (?.) instead of logical AND (&&) chains for null checks.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/prefer-optional-chain"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/use-optional-chain"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_type_over_interface.rs"><code>prefer-type-over-interface</code></a> <sup>TS</sup></td>
    <td align="left">Suggests using 'type' keyword instead of 'interface' for consistency. Type aliases are more flexible and composable.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/consistent-type-definitions"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
</table>

</details>

<details>
<summary>Regex rules examples</summary>
<br />
<div align="left">

Define patterns to match in your code using regular expressions:

```json
{
  "customRules": {
    "no-todos": {
      "type": "regex",
      "pattern": "TODO:|FIXME:",
      "message": "Remove TODO comments before merging"
    },
    "no-debug-logs": {
      "type": "regex",
      "pattern": "console\\.(log|debug|info)",
      "message": "Remove debug statements"
    }
  }
}
```

</div>
</details>

<details>
<summary>Script rules examples</summary>
<br />
<div align="left">

Soon!

</div>
</details>

<details>
<summary>AI rules examples</summary>
<br />
<div align="left">

Soon!

</div>
</details>

</div>
<!-- </DYNFIELD:RULES> -->

<!-- <DYNFIELD:INSPIRATIONS> -->
## 💡 Inspirations<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- [Biome](https://github.com/biomejs/biome) - High-performance Rust-based linter and formatter for web projects
- [VSCode Bookmarks](https://github.com/alefragnani/vscode-bookmarks) - Bookmarks Extension for Visual Studio Code
<!-- </DYNFIELD:INSPIRATIONS> -->

<!-- <DYNFIELD:CONTRIBUTING> -->
## 🤝 Contributing<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Contributions are welcome! See [CONTRIBUTING.md](https://github.com/lucasvtiradentes/tscanner/blob/main/CONTRIBUTING.md) for setup instructions and development workflow.
<!-- </DYNFIELD:CONTRIBUTING> -->

## 📜 License<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

MIT License - see [LICENSE](LICENSE) file for details.

<!-- <DYNFIELD:FOOTER> -->
<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/lino@main/.github/image/divider.png" />
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
