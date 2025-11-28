<a name="TOC"></a>


<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner - VS Code Extension</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-motivation">Motivation</a> • <a href="#-quick-start">Quick Start</a> • <a href="#-usage">Usage</a><br /><a href="#-configuration">Configuration</a> • <a href="#-rules">Rules</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-contributing">Contributing</a> • <a href="#-license">License</a>
</div>

<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

See code issues the moment you type, not after you commit. TScanner shows violations in a sidebar panel with one-click navigation to each problem. Scan your whole project or just the files you changed in your current branch.

<!-- <DYNFIELD:VSCODE_EXTENSION_DEMO_IMAGE> -->
<div align="center">
  <img width="50%" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-demo.png" alt="VS Code Extension Demo">
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
    <td>Terminal scanning, CI/CD integration, pre-commit hooks</td>
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
- **See Issues Instantly** - Real-time feedback in code editor as you type, no manual scan needed
- **Focus on What Matters** - Scan your branch changes only, or audit the full codebase
- **Copy for AI** - Export issues to clipboard, paste into chat for bulk fixes
- **Sub-second Scans** - Rust engine processes hundreds of files in <1s
<!-- </DYNFIELD:FEATURES> -->

<!-- <DYNFIELD:MOTIVATION> -->
## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI generates code fast, but it doesn't know your project's conventions, preferred patterns, or forbidden shortcuts. You end up reviewing the same issues over and over.

TScanner lets you define those rules once. Every AI-generated file, every PR, every save: automatically checked against your standards. Stop repeating yourself in code reviews.
<!-- </DYNFIELD:MOTIVATION> -->

## 🚀 Quick Start<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

<!-- <DYNFIELD:QUICK_START_VSCODE_EXTENSION> -->
1. Install the extension:

<div align="center">

<table>
  <tr>
    <th>Search "TScanner" in Extensions</th>
    <th>Install from marketplace</th>
  </tr>
  <tr>
    <td><img width="300" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-install.png" alt="TScanner installation"></td>
    <td>
      <div align="center">
      <a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/VS%20Code-Marketplace-007ACC?logo=visual-studio-code&logoColor=white" alt="VS Code"></a><br/>
      <a href="https://open-vsx.org/extension/lucasvtiradentes/tscanner-vscode"><img src="https://img.shields.io/badge/Open%20VSX-Registry-a60ee5?logo=eclipse-ide&logoColor=white" alt="Open VSX"></a>
      </div>
    </td>
  </tr>
</table>
</div>

2. Click TScanner icon in activity bar
3. Go to Settings Menu → "Manage Rules" → enable desired rules -> click "Save"
4. Issues appear automatically in the sidebar (if any)
5. Click any issue to jump to its location
<!-- </DYNFIELD:QUICK_START_VSCODE_EXTENSION> -->

## 📖 Usage<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

### Scan Modes

You have two scanning options, switchable via status bar click:

<div align="center">
<table>
  <tr>
    <th>Codebase</th>
    <th>Branch</th>
  </tr>
  <tr>
    <td>
      <div align="center">
        <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-scan-codebase.png" alt="Codebase mode">
      </div>
    </td>
    <td>
      <div align="center">
        <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-scan-branch.png" alt="Branch mode">
      </div>
    </td>
  </tr>
  <tr>
    <td>Analyze all files in the codebase</td>
    <td>Scan only modified files in current branch <br />compared to target branch</td>
  </tr>
</table>
</div>

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
        <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-1.png" alt="VS Code">
      </div>
    </td>
    <td>
      <div align="center">
        <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-2.png" alt="VS Code">
      </div>
    </td>
    <td>
      <div align="center">
        <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-3.png" alt="VS Code">
      </div>
    </td>
    <td>
      <div align="center">
        <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-view-4.png" alt="VS Code">
      </div>
    </td>
  </tr>
</table>
</div>

### Copy issues by rule/file

Copy all issues to clipboard in a structured format, ready to paste into an AI agent for automatic fixes.

<div align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-copy-issues.png" alt="VS Code">
</div>

<br />

<div align="center">

<details>
<summary>Example output - by rule</summary>

<br />

<div align="left">

```plain
TScanner report searching for all the issues of the rule "no-non-null-assertion" in the codebase mode

cli command: tscanner check --rule "no-non-null-assertion"
found issues: 5 issues

Rules triggered:

  no-non-null-assertion: Avoid non-null assertion operator (!). Use proper null checks or optional chaining instead.

Issues grouped by rule:

no-non-null-assertion (5 issues, 4 files)

  packages/github-action/src/core/comment-updater.ts (2 issues)
    ⚠ 36:23 -> const ruleMap = fileMap.get(file.filePath)!;
    ⚠ 44:9 -> ruleMap.get(ruleName)!.push({

  packages/github-action/src/core/scanner.ts (1 issues)
    ⚠ 180:7 -> fileMap.get(issue.file)!.push({

  packages/vscode-extension/src/commands/internal/copy.ts (1 issues)
    ⚠ 28:5 -> fileMap.get(filePath)!.push(result);

  packages/vscode-extension/src/common/utils/git-helper.ts (1 issues)
    ⚠ 118:12 -> return changedFilesCache.get(cacheKey)!;

Issues: 5 (0 errors, 5 warnings)
Files: 4
Rules: 1
```

</div>

</details>


<details>
<summary>Example output - by file</summary>

<br />

<div align="left">

```plain
TScanner report searching for all the issues in file "packages/github-action/src/core/cli-executor.ts" in the codebase mode

cli command: tscanner check --file "packages/github-action/src/core/cli-executor.ts"
found issues: 3 issues

Rules triggered:

  no-floating-promises     : Promise-returning expression used without handling. Use await, .then(), .catch(), or assign to a variable.
  prefer-nullish-coalescing: Use nullish coalescing (??) instead of logical OR (||). The || operator treats 0, "", and false as falsy, while ?? only checks for null/undefined.

Issues grouped by file:

packages/github-action/src/core/cli-executor.ts - 3 issues - 2 rules

  no-floating-promises (2 issues)
    ⚠ 12:3 -> githubHelper.logInfo(`Using local CLI: ${cliPath}`);
    ⚠ 40:3 -> githubHelper.logInfo(`Using published ${PACKAGE_NAME} from npm: ${packageSpec}`);

  prefer-nullish-coalescing (1 issues)
    ⚠ 9:25 -> const workspaceRoot = process.env.GITHUB_WORKSPACE || process.cwd();

Issues: 3 (0 errors, 3 warnings)
Files: 1
Rules: 2
```

</div>

</details>

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
    <td align="left"><code>tscanner: Scan Workspace</code></td>
    <td align="center">-</td>
  </tr>
  <tr>
    <td align="left"><code>tscanner: Hard Scan (Clear Cache & Rescan)</code></td>
    <td align="center">-</td>
  </tr>
  <tr>
    <td align="left"><code>tscanner: Go to Next Issue</code></td>
    <td align="center"><code>f8</code></td>
  </tr>
  <tr>
    <td align="left"><code>tscanner: Go to Previous Issue</code></td>
    <td align="center"><code>shift+f8</code></td>
  </tr>
  <tr>
    <td align="left"><code>tscanner: Show Logs</code></td>
    <td align="center">-</td>
  </tr>
</table>

</div>
<!-- </DYNFIELD:COMMANDS> -->

### Other Details 

<div align="center">

<details>
<summary><b>Settings Menu</b></summary>

<br />

<div align="left">

- **Manage Rules**: Multi-select UI for 39 built-in rules with enable/disable toggles
- **Scan Settings**: Choose workspace or branch mode, select target branch
- **Config Files**: Edit `.tscanner/config.jsonc` or create from template

</div>

</details>

<details>
<summary><b>Issue Navigation</b></summary>

<br />

<div align="left">

- **Click to Jump**: Click any issue to open file at exact line/column
- **Keyboard**: F8 (next issue), Shift+F8 (previous issue)
- **Context Menu**: Right-click for copy path options
- **Badge Count**: Sidebar shows total issue count

</div>

</details>

<details>
<summary><b>Status Bar</b></summary>

<br />

<div align="left">

- **Scan Mode**: Shows "Codebase" or "Branch: {name}"
- **Click**: Opens Settings Menu
- **Config Status**: Green checkmark if `.tscanner/config.jsonc` exists

</div>

</details>

<details>
<summary><b>Branch Mode</b></summary>

<br />

<div align="left">

1. Extension runs `git diff {branch}...HEAD` to detect changed files
2. Parses hunks to extract modified line ranges
3. Scans all files but filters issues to modified lines only

Perfect for PR validation - see only issues you introduced.

</div>

</details>

</div>


<!-- <DYNFIELD:COMMON_SECTION_CONFIG> -->
## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To scan your code, you need to set up the rules in the TScanner config folder. Here's how to get started:

1. **VSCode Extension**: Click on TScanner icon in the status bar → `Manage Rules` → Select desired rules → `Save`
2. **CLI**: Run `tscanner init` in your project root
3. **Manual**: Copy the default config below to `.tscanner/config.json`

The default configuration is:

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

**Inline Disables:**

```typescript
// tscanner-disable-next-line no-any-type
const data: any = fetchData();

// tscanner-disable-file
// Entire file is skipped
```
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

#### Code Quality (14)

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

#### Bug Prevention (4)

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

#### Variables (3)

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

#### Imports (8)

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

#### Style (4)

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

MIT License - see [LICENSE](../../LICENSE) file for details.

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
