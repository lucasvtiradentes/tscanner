<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-motivation">Motivation</a> • <a href="#-quick-start">Quick Start</a> • <a href="#-rules">Rules</a> • <a href="#-configuration">Configuration</a> • <a href="#-architecture">Architecture</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-contributing">Contributing</a> • <a href="#-license">License</a>
</div>

<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

High-performance TypeScript code quality scanner. 39 built-in rules plus custom patterns via regex, scripts, or AI validation. Integrates with CI/CD, git hooks, and VS Code/Cursor.

<!-- <DYNFIELD:VSCODE_EXTENSION_DEMO_IMAGE> -->
<div align="center">
  <img width="50%" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-vscode-demo.png" alt="VS Code Extension Demo">
  <br>
  <em>issues detected in real time in the code editor</em>
</div>
<!-- </DYNFIELD:VSCODE_EXTENSION_DEMO_IMAGE> -->

<br />

<div align="center"> 

<details>
  <summary>Other images</summary>

<br />

<!-- <DYNFIELD:CLI_DEMO_IMAGE> -->
<div align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-cli-demo.png" alt="CLI Scan Screenshot">
  <br>
  <em>scanning codebase via CLI</em>
</div>
<!-- </DYNFIELD:CLI_DEMO_IMAGE> -->

<br />

<!-- <DYNFIELD:GITHUB_ACTION_DEMO_IMAGE> -->
<div align="center">
  <img width="80%" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-pr-comment-issues-found.png" alt="GitHub Action PR Comment">
  <br>
  <em>issues detected in the latest push in a PR</em>
</div>
<!-- </DYNFIELD:GITHUB_ACTION_DEMO_IMAGE> -->

</details>

<details>
<summary>Use cases for this project</summary>
<br />

<table>
  <tr>
    <td><b>Project Consistency</b></td>
    <td>Enforce architectural patterns across your codebase - import styles, type preferences, naming conventions, and code organization rules that matter to your project.</td>
  </tr>
  <tr>
    <td><b>PR Quality Gates</b></td>
    <td>Automated PR comments show exactly which patterns were violated before merge. Reviewers can focus on logic instead of style issues.</td>
  </tr>
  <tr>
    <td><b>AI Code Validation</b></td>
    <td>See real-time quality feedback on AI-generated code. Quickly identify violations and request targeted refactoring before accepting changes.</td>
  </tr>
  <tr>
    <td><b>Flexible Customization</b></td>
    <td>Built-in rules cover common cases, but unique project requirements can use custom script and AI rules for complex validation logic.</td>
  </tr>
</table> 

</details>

</div>

## ⭐ Features<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- **39 Built-in Rules** - AST-based TypeScript/TSX validation for type safety, imports, and code quality
- **Custom Rules** - Regex patterns, JavaScript scripts, or AI-powered validation
- **Multiple Scanning Modes** - Full codebase or only files changed in your branch
- **Works Everywhere** - CLI, VS Code extension, and GitHub Action
- **Rust-Powered Speed** - 100-500 files in <1s with parallel processing and smart caching

<!-- <DYNFIELD:MOTIVATION> -->
## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI-assisted code is great for fast iteration, but working code is just one requirement. It also needs to follow project patterns, be type-safe, and avoid code smells.

With real-time feedback on violations in the code editor and PR checks before merging, you get the best of both worlds:

1. Fast iteration
2. High-quality code that follows your standards
<!-- </DYNFIELD:MOTIVATION> -->

## 🚀 Quick Start<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

<!-- <DYNFIELD:WAYS_TO_USE_TSCANNER> -->
<table>
  <tr>
    <th>Package</th>
    <th>Description</th>
  </tr>
  <tr>
    <td>
      <div align="center">
        <b><a href="packages/vscode-extension#readme">VSCode Extension</a></b>
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
        <b><a href="packages/cli#readme">CLI</a></b>
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
        <b><a href="packages/github-action#readme">GitHub Action</a></b>
        <br />
        <br />
        <a href="https://github.com/marketplace/actions/tscanner-action"><img src="https://img.shields.io/badge/Marketplace-black.svg?logo=github&logoColor=white&labelColor=181717&color=374151" alt="GitHub Marketplace"></a>
      </div>
    </td>
    <td>CICD integration with analysis summary attached to PR comments</td>
  </tr>
</table>
<!-- </DYNFIELD:WAYS_TO_USE_TSCANNER> -->


<!-- <DYNFIELD:QUICK_START_VSCODE_EXTENSION> -->
### [VSCode Extension](packages/vscode-extension#readme)

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

<!-- <DYNFIELD:QUICK_START_CLI> -->
### [CLI](packages/cli#readme)

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

### [GitHub Action](packages/github-action#readme)

<!-- <DYNFIELD:QUICK_START_GITHUB_ACTION> -->
1. Create `.github/workflows/tscanner.yml`:

```yaml
name: Code Quality

on:
  pull_request:
    branches: [main]

jobs:
  tscanner:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: lucasvtiradentes/tscanner-action@v0.0.17
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
```

2. Add TScanner config to your repo (run `tscanner init` or create `.tscanner/config.jsonc`)
3. Open a PR and watch the magic happen!
<!-- </DYNFIELD:QUICK_START_GITHUB_ACTION> -->


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

## 🏗️ Architecture<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

```
CLI/VSCode/GitHub Action (TypeScript)
            ↓
   JSON-RPC Protocol (GZIP compressed)
            ↓
    tscanner-core (Rust)
    ├─ Scanner (Rayon parallel processing)
    ├─ Parser (SWC AST)
    ├─ Rule Registry (23+ built-in + custom)
    ├─ Cache (DashMap memory + disk persistence)
    ├─ File Watcher (notify)
    └─ Config (.tscanner/config.jsonc)
```

**Communication:**
- Line-delimited JSON-RPC over stdin/stdout
- GZIP compression for large result sets (>10KB)
- Real-time file watching for incremental updates

<!-- <DYNFIELD:INSPIRATIONS> -->
## 💡 Inspirations<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- [Biome](https://github.com/biomejs/biome) - High-performance Rust-based linter and formatter for web projects
- [VSCode Bookmarks](https://github.com/alefragnani/vscode-bookmarks) - Bookmarks Extension for Visual Studio Code
<!-- </DYNFIELD:INSPIRATIONS> -->

<!-- <DYNFIELD:CONTRIBUTING> -->
## 🤝 Contributing<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and development workflow.

**Quick Setup:**

```bash
git clone https://github.com/lucasvtiradentes/tscanner.git
cd tscanner
pnpm install
pnpm run build
```
<!-- </DYNFIELD:CONTRIBUTING> -->

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
