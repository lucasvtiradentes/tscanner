<a name="TOC"></a>


<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner - VS Code Extension</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-motivation">Motivation</a> • <a href="#-workflow">Workflow</a> • <a href="#-quick-start">Quick Start</a> • <a href="#-usage">Usage</a><br /><a href="#-configuration">Configuration</a> • <a href="#-rules">Rules</a> • <a href="#-registry">Registry</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-contributing">Contributing</a> • <a href="#-license">License</a>
</div>

<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

See code quality issues the moment you type, not after you ship. TScanner shows violations you customize in a sidebar panel with one-click navigation to each problem. Scan your whole project, just the files you changed in your current branch or only your current uncommited changes.

<!-- <DYNFIELD:VSCODE_EXTENSION_DEMO_IMAGE> -->
<div align="center">
  <a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-demo.png" target="_blank"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-demo.png" alt="VS Code Extension Demo"></a>
  <br>
  <em>issues detected in real time in the code editor</em>
</div>
<!-- </DYNFIELD:VSCODE_EXTENSION_DEMO_IMAGE> -->

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
        <b><a href="https://github.com/lucasvtiradentes/tscanner/tree/main/packages/cli#readme">CLI</a></b>
        <br />
        <br />
        <a href="https://www.npmjs.com/package/tscanner"><img src="https://img.shields.io/npm/v/tscanner?label=npm&logo=npm&logoColor=white&labelColor=CB3837&color=374151" alt="npm"></a>
      </div>
    </td>
    <td>Fast terminal scanning with pre-commit hook integration</td>
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

- **Your Rules, Enforced** - 38 built-in checks + define your own with regex, scripts, or AI
- **Community Rules** - Install pre-built rules from registry or share your own with the world
- **See Issues Instantly** - Real-time feedback in code editor as you type, no manual scan needed
- **Copy for AI** - Export issues to clipboard, paste into chat for bulk fixes
- **Multiple Scan Modes** - Whole codebase, branch changes, uncommitted changes, or staged changes
- **Sub-second Scans** - Rust engine processes hundreds of files in <1s, with smart caching
- **Not a Blocker** - Issues are warnings by default; set as errors to fail CI/lint-staged
<!-- </DYNFIELD:FEATURES> -->

<!-- <DYNFIELD:MOTIVATION> -->
## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI generates code fast, but it doesn't know your project's conventions, preferred patterns, or forbidden shortcuts. You end up reviewing the same issues over and over.

TScanner lets you define those rules once. Every AI-generated file, every PR, every save: automatically checked against your standards.

<!-- </DYNFIELD:MOTIVATION> -->

<!-- <DYNFIELD:WORKFLOW> -->

## 🔀 Workflow<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

> **Vision:** Go fast with AI and know exactly what to fix before shipping. Detect bad patterns while reviewing code? Ask AI to create [regex](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/config.jsonc), [script](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner/script-rules), or [AI rules](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner/ai-rules) to catch it forever. Use the VSCode extension's "Copy Issues" button to get a [ready-to-paste prompt](https://github.com/lucasvtiradentes/tscanner/blob/main/assets/prompts/fix-tscanner-issues.prompt.md) and let your favorite AI tool fix everything. Before merging, see all issues at a glance in a PR comment from your CI/CD: nothing blocks by default, you decide what matters.

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

**Built-in rules (34 enabled)**: Standard code quality checks like `no-explicit-any`, `prefer-const`, `no-console`, etc.

**Regex rules (3)**:
- `no-rust-deprecated`: Block `#[allow(deprecated)]` in Rust code
- `no-rust-dead-code`: Block `#[allow(dead_code)]` - remove unused code instead
- `no-process-env`: Prevent direct `process.env` access

**Script rules (8)**:
- [`types-parity-match`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/script-rules/types-parity-match.ts): Ensure TypeScript and Rust shared types are in sync
- [`config-schema-match`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/script-rules/config-schema-match.ts): Keep Rust config and TypeScript schema aligned
- [`cli-builder-match`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/script-rules/cli-builder-match.ts): CLI builder must cover all CLI check flags
- [`action-zod-match`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/script-rules/action-zod-match.ts): GitHub Action inputs must match Zod validation
- [`readme-toc-match`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/script-rules/readme-toc-match.ts): README table of contents must match all headings
- [`rust-entry-simple`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/script-rules/rust-entry-simple.ts): `lib.rs` and `mod.rs` should only contain module declarations
- [`no-long-files`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/script-rules/no-long-files.ts): Files cannot exceed 300 lines
- [`no-default-node-imports`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/script-rules/no-default-node-imports.ts): Use named imports for Node.js modules

**AI rules (2)**:
- [`no-dead-code`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/ai-rules/no-dead-code.md): Detect dead code patterns in Rust executors
- [`find-enum-candidates`](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/ai-rules/find-enum-candidates.md): Find type unions that could be enums

> TIP: Check the [`.tscanner/`](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner) folder to see the full config and script implementations.

</div>
</details>

<details>
<summary>How am I using this to improve my code at work?</summary>

<br />

<div align="left">

I basically observe code patterns to enforce/avoid and add custom rules, here are my current rules: 

regex rules: 

```jsonc
"regex": {
  "no-nestjs-logger": {
    "pattern": "import\\s*\\{[^}]*Logger[^}]*\\}\\s*from\\s*['\"]@nestjs/common['\"]",
    "message": "Do not use NestJS Logger. Import from custom logger instead"
  },
  "no-typeorm-for-feature": {
    "pattern": "TypeOrmModule\\.forFeature\\(",
    "message": "Use api/src/way-type-orm.module.ts instead"
  },
  "avoid-typeorm-raw-queries": {
    "pattern": "await this\\.([^.]+)\\.manager\\.query\\(",
    "message": "Avoid using RawQueryBuilder. Use the repository instead",
    "severity": "error"
  },
  "no-static-zod-schema": {
    "pattern": "static\\s+zodSchema\\s*=",
    "message": "Remove 'static zodSchema' from class. The schema is already passed to createZodDto() and this property is redundant"
  }
}
```

script rules:

```jsonc
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
```

ai rules: 

```
soon!
```

Note: my rules at work are not commited to the codebase, so I basically installed tscanner globally and move the `.tscanner` folder into the `.gitignore` file

</div>

</details>

</div>


<!-- </DYNFIELD:WORKFLOW> -->

## 🚀 Quick Start<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

<!-- <DYNFIELD:QUICK_START_INSTALL> -->
1. Install locally

```bash
npm install -D tscanner
```

2. Initialize configuration

```bash
npx tscanner init
```

> TIP: Use `npx tscanner init --full` for a [complete config](https://github.com/lucasvtiradentes/tscanner/blob/main/assets/configs/full.json) with example regex, script, and AI rules.


<!-- </DYNFIELD:QUICK_START_INSTALL> -->

After that you can already install the extension:

<!-- <DYNFIELD:QUICK_START_VSCODE_EXTENSION> -->
3. Install the extension:

<div align="center">

<table>
  <tr>
    <th>Search "TScanner" in Extensions</th>
    <th>Install from marketplace</th>
  </tr>
  <tr>
    <td><a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-install.png" target="_blank"><img width="300" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-install.png" alt="TScanner installation"></a></td>
    <td>
      <div align="center">
      <a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/VS%20Code-Marketplace-007ACC?logo=visual-studio-code&logoColor=white" alt="VS Code"></a><br/>
      <a href="https://open-vsx.org/extension/lucasvtiradentes/tscanner-vscode"><img src="https://img.shields.io/badge/Open%20VSX-Registry-a60ee5?logo=eclipse-ide&logoColor=white" alt="Open VSX"></a>
      </div>
    </td>
  </tr>
</table>
</div>

4. Click TScanner icon in activity bar
5. Issues appear automatically in the sidebar (if any)
6. Click any issue to jump to its location
<!-- </DYNFIELD:QUICK_START_VSCODE_EXTENSION> -->

## 📖 Usage<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

### View Modes

Organize results with 4 combinations:


<div align="center">
<table>
  <tr>
    <th>By rule - flat list</th>
    <th>By rule - tree</th>
    <th>By file - flat list</th>
    <th>By file - tree</th>
  </tr>
  <tr>
    <td>
      <div align="center">
        <a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-1.png" target="_blank"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-1.png" alt="VS Code"></a>
      </div>
    </td>
    <td>
      <div align="center">
        <a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-2.png" target="_blank"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-2.png" alt="VS Code"></a>
      </div>
    </td>
    <td>
      <div align="center">
        <a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-3.png" target="_blank"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-3.png" alt="VS Code"></a>
      </div>
    </td>
    <td>
      <div align="center">
        <a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-4.png" target="_blank"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-4.png" alt="VS Code"></a>
      </div>
    </td>
  </tr>
</table>
</div>

### Copy issues by rule/file

Copy all issues to clipboard in a structured format, ready to paste into an AI agent for automatic fixes.

<div align="center">
  <a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-copy-issues.png" target="_blank"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-copy-issues.png" alt="VS Code"></a>
</div>

<br />

<div align="center">

<details>
<summary>Example output - by rule</summary>

<br />

<div align="left">

```plain
# TScanner Issue Report

This is a report from TScanner, a CLI tool that detects code quality issues in TypeScript/JavaScript projects. Your task is to fix the issues listed below.

## Report Details

Filter: rule "no-console" | Mode: codebase mode | Issues: 3
CLI: tscanner check --rule no-console --group-by rule

Results:

Rules triggered:

  ● no-console: Unexpected call to console.error

Issues grouped by rule:

● no-console (3 issues, 2 files)

  assets/configs/example-no-long-files.ts (2 issues)
    ⚠ 46:3 → console.log(JSON.stringify({ issues }));
    ⚠ 50:3 → console.error(err);

  packages/cli/src/main.ts (1 issues)
    ⚠ 33:5 → console.error(err.message);

Scope:

  Rules: 1
  Files: 2 (0 cached, 2 scanned)


Results:

  Issues: 3 (⚠ 3)
  Triggered rules: 1 (● 1)
  Files with issues: 2
  Duration: 0ms

## Instructions

- Some fixes require changes in multiple files, not just where the issue is reported
- Consider fixing one issue at a time and verifying each fix
- The rule description explains what's wrong - understand it before fixing
- Line numbers are 1-indexed
```

</div>

</details>


<details>
<summary>Example output - by file</summary>

<br />

<div align="left">

```plain
# TScanner Issue Report

This is a report from TScanner, a CLI tool that detects code quality issues in TypeScript/JavaScript projects. Your task is to fix the issues listed below.

## Report Details

Filter: file "packages/github-action/src/core/input-validator.ts" | Mode: codebase mode | Issues: 6
CLI: tscanner check --glob packages/github-action/src/core/input-validator.ts --group-by file

Results:

Rules triggered:

  ● prefer-nullish-coalescing: Use nullish coalescing (??) instead of logical OR (||). The || operator treats 0, "", and false as falsy, while ?? only checks for null/undefined.

Issues grouped by file:

packages/github-action/src/core/input-validator.ts - 6 issues - 1 rules

  ● prefer-nullish-coalescing (6 issues)
    ⚠ 44:20 → const timezone = githubHelper.getInput('timezone') || DEFAULT_INPUTS.timezone;
    ⚠ 45:22 → const configPath = githubHelper.getInput('config-path') || DEFAULT_INPUTS.configPath;
    ⚠ 46:27 → const tscannerVersion = githubHelper.getInput('tscanner-version') || DEFAULT_INPUTS.tscannerVersion;
    ⚠ 48:24 → const groupByInput = githubHelper.getInput('group-by') || DEFAULT_INPUTS.groupBy;
    ⚠ 54:23 → const aiModeInput = githubHelper.getInput('ai-mode') || AiExecutionMode.Ignore;
    ⚠ 78:53 → ...(mode === ScanMode.Branch && { targetBranch: targetBranch || DEFAULT_INPUTS.targetBranch }),

Scope:

  Rules: 1
  Files: 1 (0 cached, 1 scanned)


Results:

  Issues: 6 (⚠ 6)
  Triggered rules: 1 (● 1)
  Files with issues: 1
  Duration: 0ms

## Instructions

- Some fixes require changes in multiple files, not just where the issue is reported
- Consider fixing one issue at a time and verifying each fix
- The rule description explains what's wrong - understand it before fixing
- Line numbers are 1-indexed
```

</div>

</details>

</div>

### Scan Modes

You have four scanning options, switchable via status bar click:

<div align="center">
<table>
  <tr>
    <th>Codebase</th>
    <th>Uncommitted</th>
    <th>Branch</th>
  </tr>
  <tr>
    <td>Analyze all files</td>
    <td>Scan staged + unstaged changes</td>
    <td>Compare to target branch</td>
  </tr>
</table>
</div>

### Status bar 

<div align="center">
  <a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-statusbar.png" target="_blank"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-statusbar.png" alt="VS Code"></a>
</div>

<br />

<div align="center">
  <a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-settings.png" target="_blank"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-settings.png" alt="VS Code"></a>
</div>

### Commands

Access via Command Palette (Ctrl/Cmd + Shift + P):

<!-- <DYNFIELD:COMMANDS> -->
<div align="center">

<table>
  <tr>
    <th width="400">Command</th>
    <th width="100">Keybinding</th>
  </tr>
  <tr>
    <td align="left">tscanner: Clear Scan Caches</td>
    <td align="center">-</td>
  </tr>
  <tr>
    <td align="left">tscanner: Go to Next Issue</td>
    <td align="center">-</td>
  </tr>
  <tr>
    <td align="left">tscanner: Go to Previous Issue</td>
    <td align="center">-</td>
  </tr>
  <tr>
    <td align="left">tscanner: Refresh AI Issues (no cache)</td>
    <td align="center">-</td>
  </tr>
  <tr>
    <td align="left">tscanner: Refresh Issues (no cache)</td>
    <td align="center">-</td>
  </tr>
  <tr>
    <td align="left">tscanner: Show Logs</td>
    <td align="center">-</td>
  </tr>
</table>

</div>
<!-- </DYNFIELD:COMMANDS> -->

### Updating

The VS Code extension and CLI binary must be compatible to work correctly. If you see a version mismatch warning:

**How to update?:**

1. update the npm package 

Local installations:

```bash
npm install tscanner@latest
pnpm update tscanner --latest
yarn upgrade tscanner --latest
```

Global installations:

```bash
npm install -g tscanner@latest
pnpm update -g tscanner --latest
yarn global upgrade tscanner --latest
```

2. update the version in the [.tscanner/config.jsonc](https://github.com/lucasvtiradentes/tscanner/blob/main/.tscanner/config.jsonc) to match the new version or just use `latest` if you are going to regularly update it

```
"$schema": "https://unpkg.com/tscanner@0.1.2/schema.json", // -> tscanner@0.0.1 OR tscanner@latest
```

3. restart vscode

**Why update?**
- New extension features may require updated CLI binary
- CLI updates may include bug fixes and performance improvements
- Version mismatches can cause unexpected behavior

<!-- <DYNFIELD:COMMON_SECTION_CONFIG> -->
## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To scan your code, you need to set up the rules in the TScanner config folder.

<div align="center">
<details>
<summary><strong>Full configuration</strong></summary>

<br/>

<div align="left">

```json
{
  "$schema": "https://unpkg.com/tscanner@latest/schema.json",
  "rules": {
    "builtin": {
      "consistent-return": {},
      "max-function-length": {},
      "max-params": {},
      "no-absolute-imports": {},
      "no-alias-imports": {},
      "no-async-without-await": {},
      "no-console": {},
      "no-constant-condition": {},
      "no-default-export": {},
      "no-duplicate-imports": {},
      "no-dynamic-import": {},
      "no-else-return": {},
      "no-empty-class": {},
      "no-empty-function": {},
      "no-empty-interface": {},
      "no-explicit-any": {},
      "no-floating-promises": {},
      "no-forwarded-exports": {},
      "no-implicit-any": {},
      "no-inferrable-types": {},
      "no-nested-require": {},
      "no-nested-ternary": {},
      "no-non-null-assertion": {},
      "no-relative-imports": {},
      "no-return-await": {},
      "no-shadow": {},
      "no-single-or-array-union": {},
      "no-todo-comments": {},
      "no-unnecessary-type-assertion": {},
      "no-unreachable-code": {},
      "no-unused-vars": {},
      "no-useless-catch": {},
      "no-var": {},
      "prefer-const": {},
      "prefer-interface-over-type": {},
      "prefer-nullish-coalescing": {},
      "prefer-optional-chain": {},
      "prefer-type-over-interface": {}
    },
    "regex": {
      "example-no-console-log": {
        "pattern": "console\\.log",
        "message": "Remove console.log before committing"
      }
    },
    "script": {
      "example-no-long-files": {
        "command": "npx tsx script-rules/example-no-long-files.ts",
        "message": "File exceeds 300 lines limit",
        "include": ["packages/**/*.ts", "packages/**/*.rs"]
      }
    }
  },
  "aiRules": {
    "example-find-enum-candidates": {
      "prompt": "example-find-enum-candidates.md",
      "mode": "agentic",
      "message": "Type union could be replaced with an enum for better type safety",
      "severity": "warning",
      "include": ["**/*.ts"]
    }
  },
  "ai": {
    "provider": "claude"
  },
  "files": {
    "include": ["**/*.ts", "**/*.tsx", "**/*.js", "**/*.jsx", "**/*.mjs", "**/*.cjs"],
    "exclude": ["**/node_modules/**", "**/dist/**", "**/build/**", "**/.git/**"]
  },
  "codeEditor": {
    "highlightErrors": true,
    "highlightWarnings": true,
    "highlightInfos": true,
    "highlightHints": true,
    "autoScanInterval": 0,
    "autoAiScanInterval": 0,
    "startupScan": "cached",
    "startupAiScan": "off"
  }
}
```

</div>
</details>

<details>
<summary><strong>Minimal configuration</strong></summary>

<br/>

<div align="left">

```json
{
  "$schema": "https://unpkg.com/tscanner@latest/schema.json",
  "rules": {
    "builtin": {
      "no-explicit-any": {}
    },
    "regex": {},
    "script": {}
  },
  "aiRules": {},
  "files": {
    "include": ["**/*.ts", "**/*.tsx", "**/*.js", "**/*.jsx", "**/*.mjs", "**/*.cjs"],
    "exclude": ["**/node_modules/**", "**/dist/**", "**/build/**", "**/.git/**"]
  }
}
```

</div>
</details>

<details>
<summary><strong>Additional info</strong></summary>

<br/>

<div align="left">

**Per-rule file patterns:** Each rule can have its own `include`/`exclude` patterns:

```json
{
  "rules": {
    "builtin": {
      "no-console": { "exclude": ["src/logger.ts"] },
      "max-function-length": { "include": ["src/core/**/*.ts"] }
    }
  }
}
```

**Inline disables:**

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

Rules are the core of TScanner. They define what to check, where to check, and how to report issues. Mix built-in rules with custom ones to enforce your team's standards.

<div align="center">

<table>
  <tr>
    <th width="100">Type</th>
    <th width="350">Use Case</th>
    <th width="300">Example</th>
  </tr>
  <tr>
    <td><b>Built-in</b></td>
    <td>Common typescript anti-patterns</td>
    <td><code>no-explicit-any</code>, <code>prefer-const</code>, <code>no-console</code></td>
  </tr>
  <tr>
    <td><b>Regex</b></td>
    <td>Simple text patterns for any file</td>
    <td>Match <code>TODO</code> comments, banned imports</td>
  </tr>
  <tr>
    <td><b>Script</b></td>
    <td>Complex logic in any language (TS, Python, Rust, Go...)</td>
    <td>Enforce folder structure, type parity checks, enforce min/max lines per file</td>
  </tr>
  <tr>
    <td><b>AI</b></td>
    <td>LLM-powered analysis for context-aware patterns (dead code, enum candidates, architectural violations)</td>
    <td>Detect potential enum candidates, check if a complex pattern was followed across multiple files</td>
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
      "no-rust-deprecated": {
        "pattern": "allow\\(deprecated\\)",
        "message": "No deprecated methods",
        "include": ["packages/rust-core/**/*.rs"]
      },
      "no-process-env": {
        "pattern": "process\\.env",
        "message": "No process env"
      },
      "no-debug-logs": {
        "pattern": "console\\.(log|debug|info)",
        "message": "Remove debug statements",
        "exclude": ["**/*.test.ts"]
      }
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

Run custom scripts in **any language** (TypeScript, Python, Rust, Go, etc.) that reads JSON from stdin and outputs JSON to stdout.

**Input contract** (received via stdin):
```json
{
  "files": [
    {
      "path": "src/utils.ts",
      "content": "export function add(a: number, b: number)...",
      "lines": ["export function add(a: number, b: number)", "..."]
    }
  ],
  "options": { "maxLines": 300 },
  "workspaceRoot": "/path/to/project"
}
```

**Output contract** (expected via stdout):
```json
{
  "issues": [
    { "file": "src/utils.ts", "line": 10, "message": "Issue description" }
  ]
}
```

**Config** (`.tscanner/config.jsonc`):
```json
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
```

<details>
<summary>TypeScript example</summary>

```typescript
#!/usr/bin/env npx tsx
import { stdin } from 'node:process';

async function main() {
  let data = '';
  for await (const chunk of stdin) data += chunk;

  const input = JSON.parse(data);
  const issues = [];

  for (const file of input.files) {
    if (file.lines.length > 300) {
      issues.push({ file: file.path, line: 301, message: `File exceeds 300 lines` });
    }
  }

  console.log(JSON.stringify({ issues }));
}
main().catch((err) => {
  console.error(err);
  process.exit(1);
});
```
</details>

<details>
<summary>Python example</summary>

```python
#!/usr/bin/env python3
import json, sys

def main():
    input_data = json.loads(sys.stdin.read())
    issues = []

    for file in input_data["files"]:
        if len(file["lines"]) > 300:
            issues.append({"file": file["path"], "line": 301, "message": "File exceeds 300 lines"})

    print(json.dumps({"issues": issues}))

if __name__ == "__main__":
    main()
```
</details>

<details>
<summary>Rust example</summary>

```rust
#!/usr/bin/env rust-script
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
}
```
</details>

> 💡 See real examples in the [`.tscanner/script-rules/`](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner/script-rules) and [`registry/script-rules/`](https://github.com/lucasvtiradentes/tscanner/tree/main/registry/script-rules) folders.

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
| `paths` | Only file paths (AI reads files via tools) | Large codebases, many files |
| `content` | Full file content in prompt | Small files, quick analysis |
| `agentic` | Paths + AI can explore freely | Cross-file analysis, complex patterns |

**Placeholders** - Use in your prompt markdown:
| Placeholder | Replaced with |
|-------------|---------------|
| `{{FILES}}` | List of files to analyze (required) |
| `{{OPTIONS}}` | Custom options from config (optional) |

**Output contract** - AI must return JSON:
```json
{
  "issues": [
    { "file": "src/utils.ts", "line": 10, "column": 1, "message": "Description" }
  ]
}
```

**Config** (`.tscanner/config.jsonc`):
```json
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
```

<details>
<summary>Prompt example (agentic mode)</summary>

```markdown
# Enum Candidates Detector

Find type unions that could be replaced with enums.

## What to look for

1. String literal unions: \`type Status = 'pending' | 'active'\`
2. Repeated string literals across files
3. Type unions used as discriminators

## Exploration hints

- Check how the type is used across files
- Look for related constants

---

## Files

{{FILES}}
```
</details>

<details>
<summary>Prompt example (with options)</summary>

```markdown
# Dead Code Detector

Detect dead code patterns.

## Rules

1. No \`#[allow(dead_code)]\` attributes
2. No unreachable code after return/break

## Options

{{OPTIONS}}

## Files

{{FILES}}
```
</details>

> 💡 See real examples in the [`.tscanner/ai-rules/`](https://github.com/lucasvtiradentes/tscanner/tree/main/.tscanner/ai-rules) and [`registry/ai-rules/`](https://github.com/lucasvtiradentes/tscanner/tree/main/registry/ai-rules) folders.

</div>
</details>

</div>
<!-- </DYNFIELD:RULES> -->

<!-- <DYNFIELD:REGISTRY> -->
## 📦 Registry<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

The registry is a collection of community rules ready to install with a single command.

```bash
npx tscanner registry                     # List all available rules (and you chose the ones you want to install)
npx tscanner registry no-long-files       # Install a specific rule
npx tscanner registry --kind script       # Filter by type (ai, script, regex)
npx tscanner registry --category security # Filter by category
npx tscanner registry --latest            # Use rules from main branch instead of current version
```

<div align="center">

**Available rules (5)**

<table>
  <tr>
    <th width="33%">Rule</th>
    <th width="17%">Type</th>
    <th width="17%">Language</th>
    <th width="33%">Description</th>
  </tr>
  <tr>
    <td><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/registry/ai-rules/find-enum-candidates/prompt.md"><code>find-enum-candidates</code></a></td>
    <td><img src="https://img.shields.io/badge/ai-8B5CF6" alt="ai"></td>
    <td><img src="https://img.shields.io/badge/Markdown-083fa1?logo=markdown&logoColor=white" alt="Markdown"></td>
    <td>Find string literal unions that could be replaced with enums</td>
  </tr>
  <tr>
    <td><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/registry/script-rules/no-long-files/script.ts"><code>no-long-files</code></a></td>
    <td><img src="https://img.shields.io/badge/script-10B981" alt="script"></td>
    <td><img src="https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white" alt="TypeScript"></td>
    <td>Enforce maximum lines per file limit</td>
  </tr>
  <tr>
    <td><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/registry/script-rules/no-empty-files/script.py"><code>no-empty-files</code></a></td>
    <td><img src="https://img.shields.io/badge/script-10B981" alt="script"></td>
    <td><img src="https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white" alt="Python"></td>
    <td>Enforce minimum lines per file</td>
  </tr>
  <tr>
    <td><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/registry/script-rules/no-fixme-comments/script.rs"><code>no-fixme-comments</code></a></td>
    <td><img src="https://img.shields.io/badge/script-10B981" alt="script"></td>
    <td><img src="https://img.shields.io/badge/Rust-DEA584?logo=rust&logoColor=white" alt="Rust"></td>
    <td>Disallow FIXME/XXX comments in code</td>
  </tr>
  <tr>
    <td><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/registry/regex-rules/no-process-env/config.jsonc"><code>no-process-env</code></a></td>
    <td><img src="https://img.shields.io/badge/regex-6C757D" alt="regex"></td>
    <td>-</td>
    <td>Disallow direct process.env access</td>
  </tr>
</table>

</div>

<br />

> **Want to share your rule?** Open a PR adding your rule to the [`registry/`](https://github.com/lucasvtiradentes/tscanner/tree/main/registry) folder. Once merged, everyone can install it with `npx tscanner registry your-rule-name`.

<!-- </DYNFIELD:REGISTRY> -->

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

MIT License - see [LICENSE](../../LICENSE) file for details.

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
