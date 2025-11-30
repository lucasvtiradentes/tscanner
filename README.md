<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-motivation">Motivation</a> • <a href="#-quick-start">Quick Start</a> • <a href="#-rules">Rules</a><br /><a href="#-configuration">Configuration</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-contributing">Contributing</a> • <a href="#-license">License</a>
</div>

<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

<!-- <DYNFIELD:OVERVIEW> -->
## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Define what "good code" means for your project. TScanner enforces your patterns in real-time in the code editor, blocks violations in PRs, and runs in CI. 38 ready-to-use rules + custom rules via regex, scripts, or AI.
<!-- </DYNFIELD:OVERVIEW> -->

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
  <em>scanning the codebase via CLI</em>
</div>
<!-- </DYNFIELD:CLI_DEMO_IMAGE> -->

<br />

<!-- <DYNFIELD:GITHUB_ACTION_DEMO_IMAGE> -->
<div align="center">
  <img width="80%" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-pr-comment-issues-found.png" alt="GitHub Action PR Comment">
  <br>
  <em>issues detected in the latest commit pushed to a PR</em>
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

<!-- <DYNFIELD:FEATURES> -->
## ⭐ Features<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- **Your Rules, Enforced** - 38 built-in checks + define your own with regex, scripts, or AI
- **See Issues Instantly** - Real-time feedback in code editor as you type, no manual scan needed
- **Focus on What Matters** - Scan your branch changes only, or audit the full codebase
- **Catch Before Merge** - PR comments show violations with clickable links to exact lines
- **Sub-second Scans** - Rust engine processes hundreds of files in <1s
<!-- </DYNFIELD:FEATURES> -->

<!-- <DYNFIELD:MOTIVATION> -->
## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI generates code fast, but it doesn't know your project's conventions, preferred patterns, or forbidden shortcuts. You end up reviewing the same issues over and over.

TScanner lets you define those rules once. Every AI-generated file, every PR, every save: automatically checked against your standards. Stop repeating yourself in code reviews.

<br />

<div align="center">


<details>
<summary>Understand why TScanner is not just for AI generated code</summary>
<br />

<div align="left">

We highlight AI in our messaging because it attracts attention, but TScanner solves a problem as old as programming itself.

Developers forget conventions. Teams struggle to maintain consistency. Code reviews become repetitive. These issues exist whether you use Copilot or type every character yourself.

TScanner automates the enforcement of your standards, freeing you from being the "pattern police" in every pull request.

</div>

</details>

<details>
<summary>How I put the pieces together to build TScanner?</summary>
<br />

<div align="left">

I'm a heavy user of AI coding tools, always testing the latest and building personal projects that make my life easier at home and at work. I realized that despite going incredibly fast, AI never followed the patterns I care about: named components in array functions for React, types over interfaces, and so on.

Even with a lot of working code, I wanted something fast to answer "what patterns and conventions are broken in this project?" or "what code smells should I fix in my PR before merging to main?".

One day I was reflecting on why Rust-based tools for the TypeScript ecosystem are so ridiculously fast (<a href="https://github.com/oven-sh/bun">Bun</a>, <a href="https://github.com/vercel/turborepo">Turborepo</a>, <a href="https://github.com/biomejs/biome">Biome</a>). Then I had this crazy idea: build a tool as fast as <a href="https://github.com/biomejs/biome">Biome</a> but focused on code quality, my way, for my needs. And that's how TScanner was born.

</div>

</details>

<details>
<summary>Why not just push PRs to some industry open source linters?</summary>
<br />

<div align="left">

Because not everything can be solved with regex or AST rules. Sometimes we need more flexibility to ensure our project is correct.

Maybe you need a script rule to verify that every React component has a corresponding test file, or that route definitions match your actual folder structure. Or maybe you want an AI rule to ensure your team is following the agreed-upon patterns for data fetching with React Query, or that error boundaries are implemented correctly in critical paths.

Regex doesn't solve everything. TScanner was built to support multiple rule types: AST, regex, scripts, and even AI-powered validations. This flexibility is what makes it different.

</div>

</details>

</div>

<!-- </DYNFIELD:MOTIVATION> -->

## 🚀 Quick Start<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

TScanner is available as a [VSCode Extension](packages/vscode-extension#readme), [CLI](packages/cli#readme), and [GitHub Action](packages/github-action#readme).

### [VSCode Extension](packages/vscode-extension#readme)

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
3. Go to Settings Menu → "Manage Rules" → enable desired rules → click "Save"
4. Issues appear automatically in the sidebar (if any)
5. Click any issue to jump to its location
<!-- </DYNFIELD:QUICK_START_VSCODE_EXTENSION> -->

### [CLI](packages/cli#readme) <a href="https://www.npmjs.com/package/tscanner"><img src="https://img.shields.io/npm/v/tscanner?label=npm&logo=npm&logoColor=white&labelColor=CB3837&color=374151" alt="npm"></a>

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

### [GitHub Action](packages/github-action#readme) <a href="https://github.com/marketplace/actions/tscanner-action"><img src="https://img.shields.io/badge/Marketplace-black.svg?logo=github&logoColor=white&labelColor=181717&color=374151" alt="GitHub Marketplace"></a>

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
      - uses: lucasvtiradentes/tscanner-action@v0.0.22
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
    <td>38 ready-to-use AST rules</td>
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
<summary>Built-in rules (38)</summary>
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
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_explicit_any.rs"><code>no-explicit-any</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Detects usage of TypeScript 'any' type (<code>: any</code> and <code>as any</code>). Using 'any' defeats the purpose of TypeScript's type system.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-explicit-any"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-explicit-any"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_implicit_any.rs"><code>no-implicit-any</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Detects function parameters without type annotations that implicitly have 'any' type.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_inferrable_types.rs"><code>no-inferrable-types</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows explicit type annotations on variables initialized with literal values. TypeScript can infer these types automatically.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-inferrable-types"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-inferrable-types"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_non_null_assertion.rs"><code>no-non-null-assertion</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows the non-null assertion operator (!). Use proper null checks or optional chaining instead.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-non-null-assertion"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-non-null-assertion"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_single_or_array_union.rs"><code>no-single-or-array-union</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows union types that combine a type with its array form (e.g., <code>string | string[]</code>, <code>number | number[]</code>). Prefer using a consistent type to avoid handling multiple cases in function implementations.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_unnecessary_type_assertion.rs"><code>no-unnecessary-type-assertion</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows type assertions on values that are already of the asserted type (e.g., "hello" as string, 123 as number).</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-unnecessary-type-assertion"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
</table>

<div align="left">

#### Code Quality (13)

</div>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="450">Description</th>
    <th width="100">Also in</th>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/max_function_length.rs"><code>max-function-length</code></a></div></td>
    <td align="left">Enforces a maximum number of statements in functions (default: 50). Long functions are harder to understand and maintain.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/max-lines-per-function"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/max_params.rs"><code>max-parameters</code></a></div></td>
    <td align="left">Limits the number of parameters in a function. Functions with many parameters should use an options object instead.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/max-params"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_async_without_await.rs"><code>no-async-without-await</code></a></div></td>
    <td align="left">Disallows async functions that don't use await. The async keyword is unnecessary if await is never used.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/require-await"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/use-await"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_console.rs"><code>no-console</code></a><br/><br/><img src="https://img.shields.io/badge/regex--rule-6C757D" alt="Regex rule"></div></td>
    <td align="left">Disallow the use of console methods. Console statements should be removed before committing to production.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-console"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-console"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_else_return.rs"><code>no-else-return</code></a></div></td>
    <td align="left">Disallows else blocks after return statements. The else is unnecessary since the function already returned.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-else-return"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-useless-else"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_empty_class.rs"><code>no-empty-class</code></a></div></td>
    <td align="left">Disallows empty classes without methods or properties.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_empty_function.rs"><code>no-empty-function</code></a></div></td>
    <td align="left">Disallows empty functions and methods. Empty functions are often leftovers from incomplete code.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-empty-function"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_empty_interface.rs"><code>no-empty-interface</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows empty interface declarations. Empty interfaces are equivalent to {} and usually indicate incomplete code.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-empty-interface"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-empty-interface"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_nested_ternary.rs"><code>no-nested-ternary</code></a></div></td>
    <td align="left">Disallows nested ternary expressions. Nested ternaries are hard to read and should be replaced with if-else statements.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-nested-ternary"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-nested-ternary"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_return_await.rs"><code>no-return-await</code></a></div></td>
    <td align="left">Disallows redundant 'return await' in async functions. The await is unnecessary since the function already returns a Promise.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/return-await"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_todo_comments.rs"><code>no-todo-comments</code></a><br/><br/><img src="https://img.shields.io/badge/regex--rule-6C757D" alt="Regex rule"></div></td>
    <td align="left">Detects TODO, FIXME, and similar comment markers.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-warning-comments"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_unused_vars.rs"><code>no-unused-variables</code></a></div></td>
    <td align="left">Detects variables that are declared but never used in the code.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-unused-vars"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-unused-variables"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_useless_catch.rs"><code>no-useless-catch</code></a></div></td>
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
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/consistent_return.rs"><code>consistent-return</code></a></div></td>
    <td align="left">Requires consistent return behavior in functions. Either all code paths return a value or none do.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/consistent-return"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_constant_condition.rs"><code>no-constant-condition</code></a></div></td>
    <td align="left">Disallows constant expressions in conditions (if/while/for/ternary). Likely a programming error.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-constant-condition"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-constant-condition"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_floating_promises.rs"><code>no-floating-promises</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Disallows floating promises (promises used as statements without await, .then(), or .catch()). Unhandled promises can lead to silent failures.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/no-floating-promises"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-floating-promises"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_unreachable_code.rs"><code>no-unreachable-code</code></a></div></td>
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
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_shadow.rs"><code>no-shadow</code></a></div></td>
    <td align="left">Disallows variable declarations that shadow variables in outer scopes. Shadowing can lead to confusing code and subtle bugs.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-shadow"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_var.rs"><code>no-var</code></a></div></td>
    <td align="left">Disallows the use of 'var' keyword. Use 'let' or 'const' instead for block-scoped variables.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-var"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-var"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_const.rs"><code>prefer-const</code></a></div></td>
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
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_absolute_imports.rs"><code>no-absolute-imports</code></a></div></td>
    <td align="left">Disallows absolute imports without alias. Prefer relative or aliased imports.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_alias_imports.rs"><code>no-alias-imports</code></a></div></td>
    <td align="left">Disallows aliased imports (starting with @). Prefer relative imports.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_default_export.rs"><code>no-default-export</code></a></div></td>
    <td align="left">Disallows default exports. Named exports are preferred for better refactoring support and explicit imports.</td>
    <td align="left"><a href="https://biomejs.dev/linter/rules/no-default-export"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_duplicate_imports.rs"><code>no-duplicate-imports</code></a></div></td>
    <td align="left">Disallows multiple import statements from the same module. Merge them into a single import.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/no-duplicate-imports"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/no-duplicate-json-keys"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_dynamic_import.rs"><code>no-dynamic-import</code></a></div></td>
    <td align="left">Disallows dynamic import() expressions. Dynamic imports make static analysis harder and can impact bundle optimization.</td>
    <td align="left"></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_forwarded_exports.rs"><code>no-forwarded-exports</code></a></div></td>
    <td align="left">Disallows re-exporting from other modules. This includes direct re-exports (export { X } from 'module'), star re-exports (export * from 'module'), and re-exporting imported values.</td>
    <td align="left"><a href="https://biomejs.dev/linter/rules/no-re-export-all"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_nested_require.rs"><code>no-nested-require</code></a></div></td>
    <td align="left">Disallows require() calls inside functions, blocks, or conditionals. Require statements should be at the top level for static analysis.</td>
    <td align="left"><a href="https://eslint.org/docs/latest/rules/global-require"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/no_relative_imports.rs"><code>no-relative-imports</code></a></div></td>
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
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_interface_over_type.rs"><code>prefer-interface-over-type</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
    <td align="left">Suggests using 'interface' keyword instead of 'type' for consistency.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/consistent-type-definitions"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_nullish_coalescing.rs"><code>prefer-nullish-coalescing</code></a></div></td>
    <td align="left">Suggests using nullish coalescing (??) instead of logical OR (||) for default values. The || operator treats 0, "", and false as falsy, which may not be intended.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/prefer-nullish-coalescing"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_optional_chain.rs"><code>prefer-optional-chain</code></a></div></td>
    <td align="left">Suggests using optional chaining (?.) instead of logical AND (&&) chains for null checks.</td>
    <td align="left"><a href="https://typescript-eslint.io/rules/prefer-optional-chain"><img src="https://img.shields.io/badge/-ESLint-4B32C3?logo=eslint&logoColor=white" alt="ESLint"></a> <a href="https://biomejs.dev/linter/rules/use-optional-chain"><img src="https://img.shields.io/badge/-Biome-60A5FA?logo=biome&logoColor=white" alt="Biome"></a></td>
  </tr>
  <tr>
    <td align="left"><div align="center"><a href="https://github.com/lucasvtiradentes/tscanner/blob/main/packages/core/crates/core/src/rules/prefer_type_over_interface.rs"><code>prefer-type-over-interface</code></a><br/><br/><img src="https://img.shields.io/badge/ts--only-3178C6?logo=typescript&logoColor=white" alt="TypeScript only"></div></td>
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


<!-- <DYNFIELD:COMMON_SECTION_CONFIG> -->
## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To scan your code, you need to set up the rules in the TScanner config folder. Here's how to get started:

1. **VSCode Extension**: TScanner icon in the status bar → `Manage Rules` → Select desired rules → `Save`
2. **CLI**: Run `tscanner init` in your project root
3. **Manual**: Copy the default config below to `.tscanner/config.jsonc`

The default configuration is:

```json
{
  "$schema": "https://unpkg.com/tscanner@0.0.25/schema.json",
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
  "codeEditor": {
    "highlightErrors": true,
    "highlightWarnings": false,
    "scanIntervalSeconds": 0
  },
  "cli": {
    "groupBy": "file",
    "noCache": false,
    "showSettings": true,
    "showIssueSeverity": true,
    "showIssueSourceLine": true,
    "showIssueRuleName": true,
    "showIssueDescription": false,
    "showSummary": true
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

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and development workflow.
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
