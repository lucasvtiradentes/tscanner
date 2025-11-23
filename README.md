<a name="TOC"></a>

<div align="center">
<h4>Tscanner</h4>
<p>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <br>
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-architecture">Architecture</a> • <a href="#-quick-start">Quick Start</a> • <a href="#-development">Development</a>
</p>
</div>

<a href="#"><img src="./.github/image/divider.png" /></a>

## 🎺 Overview<a href="#TOC"><img align="right" src="./.github/image/up_arrow.png" width="22"></a>

Tscanner is a fast, flexible code quality scanner for TypeScript codebases. Enforce project-specific patterns, detect anti-patterns, and validate architectural conventions with 23+ built-in rules or custom regex patterns.

<img src="./.github/image/vscode-demo.png" />

**Three ways to use:**
- **[VSCode Extension](packages/vscode-extension)** - Real-time sidebar integration with Git-aware branch scanning
- **[GitHub Action](packages/github-action)** - CICD integration with analysis summary being attached to the same PR comment on every push
- **[CLI](packages/cli)** - Terminal scanning, pre-commit hooks

## ⭐ Features<a href="#TOC"><img align="right" src="./.github/image/up_arrow.png" width="22"></a>

- **23 AST-based rules** - Type safety, import conventions, best practices
- **Custom rules** - Regex patterns, scripts, or AI-powered validation
- **Blazing fast** - Rust parallel processing with smart caching (100-500 files in <1s)
- **Flexible severity** - Errors block CI, warnings report only
- **Inline control** - Disable rules per line/file with `tscanner-disable` directives

<details>
<summary>VSCode Extension</summary>
<br />

- Tree/list sidebar views with issue badges
- Branch mode: scan only changed files vs target branch
- Click-to-navigate to exact line/column
- F8/Shift+F8 keyboard navigation
- Live file watching with incremental updates

</details>

<details>
<summary>CLI</summary>
<br />

- Terminal scanning with JSON output
- CI/CD integration (exit codes 0/1)
- Pre-commit hook support
- Cross-platform binaries (Linux, macOS, Windows)

</details>

<details>
<summary>GitHub Action</summary>
<br />

- Automated PR comments with direct file links
- Codebase or branch scan modes
- Smart single-comment updates (no spam)
- Collapsible grouped views (by file or rule)

</details>

## 💡 Use Cases

**Project Consistency**

Enforce architectural patterns across your codebase - import styles, type preferences, naming conventions, and code organization rules that matter to your project.

**PR Quality Gates**

Automated PR comments show exactly which patterns were violated before merge. Reviewers can focus on logic instead of style issues.

**AI Code Validation**

See real-time quality feedback on AI-generated code. Quickly identify violations and request targeted refactoring before accepting changes.

**Flexible Customization**

Built-in rules cover common cases, but unique project requirements can use custom script and AI rules for complex validation logic. 

## 📦 Architecture<a href="#TOC"><img align="right" src="./.github/image/up_arrow.png" width="22"></a>

```
CLI/VSCode/GitHub Action (TypeScript)
            ↓
      JSON-RPC Protocol
            ↓
    tscanner-core (Rust)
    ├─ Scanner (Rayon parallel)
    ├─ Parser (SWC AST)
    ├─ Rules (23 built-in + custom)
    ├─ Cache (DashMap + disk)
    └─ Config (.tscanner/rules.json)
```

## 🚀 Quick Start<a href="#TOC"><img align="right" src="./.github/image/up_arrow.png" width="22"></a>


### CLI

```bash
npm install -g tscanner
tscanner init
tscanner check
```

### VSCode Extension

1. Install from VSCode marketplace
2. Click tscanner icon in activity bar
3. Configure rules via settings menu

### GitHub Action

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.1
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

### Configuration

Create `.tscanner/rules.json`:

```json
{
  "builtinRules": {
    "no-any-type": { "severity": "error" },
    "prefer-const": { "enabled": true, "severity": "warning" }
  },
  "customRules": {
    "no-todos": {
      "type": "regex",
      "pattern": "TODO:|FIXME:",
      "message": "Remove TODO comments before committing",
      "severity": "warning"
    }
  },
  "include": ["**/*.{ts,tsx}"],
  "exclude": ["node_modules/**", "dist/**", "build/**", ".git/**"]
}
```

## 📜 License<a href="#TOC"><img align="right" src="./.github/image/up_arrow.png" width="22"></a>

MIT License - see [LICENSE](LICENSE) file for details.
