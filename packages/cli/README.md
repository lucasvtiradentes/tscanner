<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner - CLI</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-installation">Installation</a> • <a href="#-usage">Usage</a> • <a href="#-use-cases">Use Cases</a> • <a href="#-architecture">Architecture</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-license">License</a>
</div>

<a href="#"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" /></a>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Terminal interface for [TScanner](https://github.com/lucasvtiradentes/tscanner): catch code quality issues with built-in rules or define project-specific patterns using regex, scripts, or AI validation. Integrates seamlessly with CI/CD, git hooks, and development workflows.

<div align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-cli-demo.png" alt="CLI Scan Screenshot">
</div>

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

Creates `.tscanner/config.jsonc` with default rule configuration:
```json
{
  "schema": "https://unpkg.com/tscanner@0.0.19/schema.json",
  "builtinRules": {
    "no-any-type": {
      "severity": "error"
    },
    "no-console-log": {}
  },
  "customRules": {
    "todo-comment": {
      "type": "regex",
      "pattern": "TODO:",
      "message": "Found TODO comment"
    }
  },
  "include": ["**/*.{ts,tsx}"],
  "exclude": ["node_modules/**", "dist/**", "build/**", ".git/**"]
}
```

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

<a href="#"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" /></a>

<div align="center">
  <div>
    <a target="_blank" href="https://www.linkedin.com/in/lucasvtiradentes/"><img src="https://img.shields.io/badge/-linkedin-blue?logo=Linkedin&logoColor=white" alt="LinkedIn"></a>
    <a target="_blank" href="mailto:lucasvtiradentes@gmail.com"><img src="https://img.shields.io/badge/gmail-red?logo=gmail&logoColor=white" alt="Gmail"></a>
    <a target="_blank" href="https://x.com/lucasvtiradente"><img src="https://img.shields.io/badge/-X-black?logo=X&logoColor=white" alt="X"></a>
    <a target="_blank" href="https://github.com/lucasvtiradentes"><img src="https://img.shields.io/badge/-github-gray?logo=Github&logoColor=white" alt="Github"></a>
  </div>
</div>
