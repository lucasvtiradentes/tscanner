<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner - GitHub Action</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-usage">Usage</a> • <a href="#-inputs">Inputs</a> • <a href="#-configuration">Configuration</a> • <a href="#-rules">Rules</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-contributing">Contributing</a> • <a href="#-license">License</a>
</div>

<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

GitHub Action for [TScanner](https://github.com/lucasvtiradentes/tscanner): Enforce project-specific patterns, detect anti-patterns, and validate architectural conventions with 39 built-in rules or custom validation (regex, scripts, AI). Integrates into CI/CD workflows with smart PR comments and flexible scan modes.

<table>
  <tr>
    <th>PR Comment - Issues Found</th>
    <th>PR Comment - No Issues Found</th>
  </tr>
  <tr>
    <td><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-pr-comment-issues-found.png" alt="PR Comment - Issues Found"></td>
    <td><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-pr-comment-no-issues.png" alt="PR Comment - No Issues"></td>
  </tr>
</table>

## ⭐ Features<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- **Smart PR Comments** - Auto-posted summary with clickable file links to exact lines
- **Git-Aware Scanning** - Full codebase or only files changed in PR
- **Dual Grouping** - View issues by file or by rule in the same comment
- **39 Built-in Rules** - Type safety, imports, and code quality checks
- **Custom Rules** - Regex patterns, scripts, or AI-powered validation
- **Flexible Control** - Block PR or continue with warnings

## 📖 Usage<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

### Quick Start

**Scan full codebase:**

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

**Scan only changed files (recommended for PRs):**

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    target-branch: 'origin/main'
```

### Additional examples

<details>
<summary><b>Continue on Errors</b></summary>

Scan but don't fail the workflow:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    continue-on-error: 'true'
```

</details>

<details>
<summary><b>Group by Rule</b></summary>

Primary grouping by rule instead of file:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    group-by: 'rule'
```

</details>

<details>
<summary><b>Custom Config Path</b></summary>

Use non-standard config location:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    config-path: 'config/tscanner'
```

</details>

<details>
<summary><b>Specific tscanner Version</b></summary>

Pin to exact CLI version:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    tscanner-version: '0.1.5'
```

</details>

<details>
<summary><b>Full Configuration</b></summary>

All options:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    target-branch: 'origin/develop'
    timezone: 'America/Sao_Paulo'
    config-path: '.tscanner'
    tscanner-version: 'latest'
    continue-on-error: 'false'
    group-by: 'rule'
```

</details>

## 📋 Inputs<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `github-token` | Yes | - | GitHub token for posting PR comments (`${{ secrets.GITHUB_TOKEN }}`) |
| `target-branch` | - | - | Target branch to compare (enables branch mode). Example: `origin/main` |
| `config-path` | - | `.tscanner` | Path to tscanner config directory containing `config.jsonc` |
| `tscanner-version` | - | `latest` | NPM version of tscanner CLI to install |
| `group-by` | - | `file` | Primary grouping mode: `file` or `rule` |
| `continue-on-error` | - | `false` | Continue workflow even if errors found (`true`/`false`) |
| `timezone` | - | `UTC` | Timezone for timestamps in PR comments. Example: `America/New_York` |

<!-- <DYNFIELD:COMMON_SECTION_CONFIG> -->
## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To create TScanner configuration, you can use the following command:

```bash
tscanner init
```

or go the `Status Bar` and click on `Manage Rules`, select the rules you want to enable and click on the `Save` button.

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

## 💡 Inspirations<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- [Biome](https://github.com/biomejs/biome) - High-performance Rust-based linter and formatter for web projects
- [VSCode Bookmarks](https://github.com/alefragnani/vscode-bookmarks) - Bookmarks Extension for Visual Studio Code

<!-- <DYNFIELD:CONTRIBUTING> -->
## 🤝 Contributing<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Contributions are welcome! See [CONTRIBUTING.md](https://github.com/lucasvtiradentes/tscanner/blob/main/CONTRIBUTING.md) for setup instructions and development workflow.

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
