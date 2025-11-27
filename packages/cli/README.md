<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner - CLI</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-installation">Installation</a> • <a href="#-usage">Usage</a> • <a href="#-configuration">Configuration</a> • <a href="#-use-cases">Use Cases</a> • <a href="#-rules">Rules</a> • <a href="#-architecture">Architecture</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-license">License</a>
</div>

<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Terminal interface for [TScanner](https://github.com/lucasvtiradentes/tscanner): catch code quality issues with built-in rules or define project-specific patterns using regex, scripts, or AI validation. Integrates seamlessly with CI/CD, git hooks, and development workflows.

<!-- <DYNFIELD:CLI_IMAGE> -->
<div align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-cli-demo.png" alt="CLI Scan Screenshot">
  <br>
  <em>scanning codebase via CLI</em>
</div>
<!-- </DYNFIELD:CLI_IMAGE> -->

## ⭐ Features<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- **Blazing Fast** - 100-500 files in <1s with Rust parallel processing
- **Smart Caching** - Skip unchanged files, 80-95% cache hit rate
- **Git-Aware Scanning** - Full codebase or only changed files vs branch
- **39 Built-in Rules** - Type safety, imports, and code quality validation
- **Custom Rules** - Regex patterns, scripts, or AI-powered validation
- **Zero Config** - Works out of the box, JSON/pretty output formats

## 🚀 Installation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

```bash
npm install -g tscanner
pnpm add -g tscanner
yarn global add tscanner
```

After installation, the `tscanner` command will be available globally.

**Supported Platforms:**
- Linux (x64, arm64)
- macOS (Intel, Apple Silicon)
- Windows (x64)

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

## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Create `.tscanner/config.jsonc`:

<!-- <DYNFIELD:DEFAULT_CONFIG> -->
```json
{
  "$schema": "https://unpkg.com/tscanner@0.0.20/schema.json",
  "builtinRules": {
    "no-any-type": {}
  },
  "customRules": {},
  "include": [
    "**/*.ts",
    "**/*.tsx"
  ],
  "exclude": [
    "**/node_modules/**",
    "**/dist/**",
    "**/build/**",
    "**/.git/**"
  ]
}
```
<!-- </DYNFIELD:DEFAULT_CONFIG> -->

## 🎯 Use Cases<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

<details>
<summary><b>CI/CD Pipeline</b></summary>

It is recommended to use [TScanner gh action](https://github.com/lucasvtiradentes/tscanner/tree/main/packages/github-action), but you can also set up your own workflow:

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

## 📋 Rules<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

<!-- <DYNFIELD:RULES> -->
### Built-in Rules (39)

<details>
<summary><b>Type Safety (6)</b></summary>

<div align="center">

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>no-any-type</code></td>
    <td align="left">Detects usage of TypeScript 'any' type (<code>: any</code> and <code>as any</code>). Using 'any' defeats the purpose of TypeScript's type system.</td>
  </tr>
  <tr>
    <td align="left"><code>no-implicit-any</code></td>
    <td align="left">Detects function parameters without type annotations that implicitly have 'any' type.</td>
  </tr>
  <tr>
    <td align="left"><code>no-inferrable-types</code></td>
    <td align="left">Disallows explicit type annotations on variables initialized with literal values. TypeScript can infer these types automatically.</td>
  </tr>
  <tr>
    <td align="left"><code>no-non-null-assertion</code></td>
    <td align="left">Disallows the non-null assertion operator (!). Use proper null checks or optional chaining instead.</td>
  </tr>
  <tr>
    <td align="left"><code>no-single-or-array-union</code></td>
    <td align="left">Disallows union types that combine a type with its array form (e.g., <code>string | string[]</code>, <code>number | number[]</code>). Prefer using a consistent type to avoid handling multiple cases in function implementations.</td>
  </tr>
  <tr>
    <td align="left"><code>no-unnecessary-type-assertion</code></td>
    <td align="left">Disallows type assertions on values that are already of the asserted type (e.g., "hello" as string, 123 as number).</td>
  </tr>
</table>

</div>

</details>

<details>
<summary><b>Code Quality (14)</b></summary>

<div align="center">

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>max-function-length</code></td>
    <td align="left">Enforces a maximum number of statements in functions (default: 50). Long functions are harder to understand and maintain.</td>
  </tr>
  <tr>
    <td align="left"><code>max-params</code></td>
    <td align="left">Limits the number of parameters in a function. Functions with many parameters should use an options object instead.</td>
  </tr>
  <tr>
    <td align="left"><code>no-async-without-await</code></td>
    <td align="left">Disallows async functions that don't use await. The async keyword is unnecessary if await is never used.</td>
  </tr>
  <tr>
    <td align="left"><code>no-console-log</code></td>
    <td align="left">Finds console.log() statements in code. Console statements should be removed before committing to production.</td>
  </tr>
  <tr>
    <td align="left"><code>no-else-return</code></td>
    <td align="left">Disallows else blocks after return statements. The else is unnecessary since the function already returned.</td>
  </tr>
  <tr>
    <td align="left"><code>no-empty-class</code></td>
    <td align="left">Disallows empty classes without methods or properties.</td>
  </tr>
  <tr>
    <td align="left"><code>no-empty-function</code></td>
    <td align="left">Disallows empty functions and methods. Empty functions are often leftovers from incomplete code.</td>
  </tr>
  <tr>
    <td align="left"><code>no-empty-interface</code></td>
    <td align="left">Disallows empty interface declarations. Empty interfaces are equivalent to {} and usually indicate incomplete code.</td>
  </tr>
  <tr>
    <td align="left"><code>no-magic-numbers</code></td>
    <td align="left">Detects magic numbers in code (literals other than 0, 1, -1). Use named constants instead for better readability and maintainability.</td>
  </tr>
  <tr>
    <td align="left"><code>no-nested-ternary</code></td>
    <td align="left">Disallows nested ternary expressions. Nested ternaries are hard to read and should be replaced with if-else statements.</td>
  </tr>
  <tr>
    <td align="left"><code>no-return-await</code></td>
    <td align="left">Disallows redundant 'return await' in async functions. The await is unnecessary since the function already returns a Promise.</td>
  </tr>
  <tr>
    <td align="left"><code>no-todo-comments</code></td>
    <td align="left">Detects TODO, FIXME, and similar comment markers.</td>
  </tr>
  <tr>
    <td align="left"><code>no-unused-vars</code></td>
    <td align="left">Detects variables that are declared but never used in the code.</td>
  </tr>
  <tr>
    <td align="left"><code>no-useless-catch</code></td>
    <td align="left">Disallows catch blocks that only rethrow the caught error. Remove the try-catch or add meaningful error handling.</td>
  </tr>
</table>

</div>

</details>

<details>
<summary><b>Bug Prevention (4)</b></summary>

<div align="center">

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>consistent-return</code></td>
    <td align="left">Requires consistent return behavior in functions. Either all code paths return a value or none do.</td>
  </tr>
  <tr>
    <td align="left"><code>no-constant-condition</code></td>
    <td align="left">Disallows constant expressions in conditions (if/while/for/ternary). Likely a programming error.</td>
  </tr>
  <tr>
    <td align="left"><code>no-floating-promises</code></td>
    <td align="left">Disallows floating promises (promises used as statements without await, .then(), or .catch()). Unhandled promises can lead to silent failures.</td>
  </tr>
  <tr>
    <td align="left"><code>no-unreachable-code</code></td>
    <td align="left">Detects code after return, throw, break, or continue statements. This code will never execute.</td>
  </tr>
</table>

</div>

</details>

<details>
<summary><b>Variables (3)</b></summary>

<div align="center">

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>no-shadow</code></td>
    <td align="left">Disallows variable declarations that shadow variables in outer scopes. Shadowing can lead to confusing code and subtle bugs.</td>
  </tr>
  <tr>
    <td align="left"><code>no-var</code></td>
    <td align="left">Disallows the use of 'var' keyword. Use 'let' or 'const' instead for block-scoped variables.</td>
  </tr>
  <tr>
    <td align="left"><code>prefer-const</code></td>
    <td align="left">Suggests using 'const' instead of 'let' when variables are never reassigned.</td>
  </tr>
</table>

</div>

</details>

<details>
<summary><b>Imports (8)</b></summary>

<div align="center">

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>no-absolute-imports</code></td>
    <td align="left">Disallows absolute imports without alias. Prefer relative or aliased imports.</td>
  </tr>
  <tr>
    <td align="left"><code>no-alias-imports</code></td>
    <td align="left">Disallows aliased imports (starting with @). Prefer relative imports.</td>
  </tr>
  <tr>
    <td align="left"><code>no-default-export</code></td>
    <td align="left">Disallows default exports. Named exports are preferred for better refactoring support and explicit imports.</td>
  </tr>
  <tr>
    <td align="left"><code>no-duplicate-imports</code></td>
    <td align="left">Disallows multiple import statements from the same module. Merge them into a single import.</td>
  </tr>
  <tr>
    <td align="left"><code>no-dynamic-import</code></td>
    <td align="left">Disallows dynamic import() expressions. Dynamic imports make static analysis harder and can impact bundle optimization.</td>
  </tr>
  <tr>
    <td align="left"><code>no-forwarded-exports</code></td>
    <td align="left">Disallows re-exporting from other modules. This includes direct re-exports (export { X } from 'module'), star re-exports (export * from 'module'), and re-exporting imported values.</td>
  </tr>
  <tr>
    <td align="left"><code>no-nested-require</code></td>
    <td align="left">Disallows require() calls inside functions, blocks, or conditionals. Require statements should be at the top level for static analysis.</td>
  </tr>
  <tr>
    <td align="left"><code>no-relative-imports</code></td>
    <td align="left">Detects relative imports (starting with './' or '../'). Prefer absolute imports with @ prefix for better maintainability.</td>
  </tr>
</table>

</div>

</details>

<details>
<summary><b>Style (4)</b></summary>

<div align="center">

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>prefer-interface-over-type</code></td>
    <td align="left">Suggests using 'interface' keyword instead of 'type' for consistency.</td>
  </tr>
  <tr>
    <td align="left"><code>prefer-nullish-coalescing</code></td>
    <td align="left">Suggests using nullish coalescing (??) instead of logical OR (||) for default values. The || operator treats 0, "", and false as falsy, which may not be intended.</td>
  </tr>
  <tr>
    <td align="left"><code>prefer-optional-chain</code></td>
    <td align="left">Suggests using optional chaining (?.) instead of logical AND (&&) chains for null checks.</td>
  </tr>
  <tr>
    <td align="left"><code>prefer-type-over-interface</code></td>
    <td align="left">Suggests using 'type' keyword instead of 'interface' for consistency. Type aliases are more flexible and composable.</td>
  </tr>
</table>

</div>

</details>


<!-- </DYNFIELD:RULES> -->

## 🏗️ Architecture<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

```
CLI (Node.js)              Rust Binary
├─ Platform detector  →    ├─ Scanner
├─ Binary resolver         ├─ Parser (SWC)
├─ Process spawner    ←→   ├─ Rules (39)
└─ Args forwarder          ├─ Cache (DashMap)
                           └─ Config loader
```

**Architecture:**
- Node.js wrapper detects platform (Linux/macOS/Windows, x64/arm64)
- Spawns platform-specific Rust binary with stdio inheritance
- Binary packaged separately per platform via optional dependencies

## 💡 Inspirations<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- [Biome](https://github.com/biomejs/biome) - High-performance Rust-based linter and formatter for web projects
- [VSCode Bookmarks](https://github.com/alefragnani/vscode-bookmarks) - Bookmarks Extension for Visual Studio Code

## 📜 License<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

MIT License - see [LICENSE](LICENSE) file for details.

<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

<div align="center">
  <div>
    <a target="_blank" href="https://www.linkedin.com/in/lucasvtiradentes/"><img src="https://img.shields.io/badge/-linkedin-blue?logo=Linkedin&logoColor=white" alt="LinkedIn"></a>
    <a target="_blank" href="mailto:lucasvtiradentes@gmail.com"><img src="https://img.shields.io/badge/gmail-red?logo=gmail&logoColor=white" alt="Gmail"></a>
    <a target="_blank" href="https://x.com/lucasvtiradente"><img src="https://img.shields.io/badge/-X-black?logo=X&logoColor=white" alt="X"></a>
    <a target="_blank" href="https://github.com/lucasvtiradentes"><img src="https://img.shields.io/badge/-github-gray?logo=Github&logoColor=white" alt="Github"></a>
  </div>
</div>
