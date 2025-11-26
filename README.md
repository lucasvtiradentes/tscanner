<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://i.ibb.co/1tyQ1m40/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-ways-to-use">Ways to use</a> • <a href="#-features">Features</a> • <a href="#-use-cases">Use Cases</a> • <a href="#-architecture">Architecture</a> • <a href="#-quick-start">Quick Start</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-license">License</a>
</div>

<a href="#"><img src="https://i.ibb.co/CKW9djzW/divider.png" /></a>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://i.ibb.co/YBVkRcnC/up-arrow.png" width="22"></a>

High-performance TypeScript code quality scanner. 23+ built-in rules plus custom patterns via regex, scripts, or AI validation. Integrates with CI/CD, git hooks, and VS Code/Cursor.

<table>
<tr>
<th>issues detected in real time in the code editor</th>
<th>issues detected in the latest push in a PR</th>
</tr>
<tr>
<td width="50%"><img src="https://i.ibb.co/8DZqQqn6/tscanner-vscode-demo.png" alt="VS Code Extension Screenshot" width="100%"></td>
<td width="50%"><img src="https://i.ibb.co/5W7GNQPv/tscanner-pr-comment-issues-found.png" alt="VS Code Extension Screenshot" width="100%"></td>
</tr>
</table>

## 📦 Ways to use<a href="#TOC"><img align="right" src="https://i.ibb.co/YBVkRcnC/up-arrow.png" width="22"></a>

<table>
  <tr>
    <th>Package</th>
    <th>Description</th>
    <th>Download</th>
  </tr>
  <tr>
    <td><b><a href="packages/cli">CLI</a></b></td>
    <td>Terminal scanning, CI/CD integration, pre-commit hooks</td>
    <td><a href="https://www.npmjs.com/package/tscanner"><img src="https://img.shields.io/badge/Npm-Package-red.svg" alt="npm"></a></td>
  </tr>
  <tr>
    <td><b><a href="packages/vscode-extension">VSCode Extension</a></b></td>
    <td>Real-time sidebar integration with Git-aware branch scanning</td>
    <td><a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/Vscode-Extension-blue.svg" alt="VS Marketplace"></a></td>
  </tr>
  <tr>
    <td><b><a href="packages/github-action">GitHub Action</a></b></td>
    <td>CICD integration with analysis summary attached to PR comments</td>
    <td><a href="https://github.com/marketplace/actions/tscanner-pr-validator"><img src="https://img.shields.io/badge/GitHub-Marketplace-black.svg" alt="GitHub Marketplace"></a></td>
  </tr>
</table>

## ⭐ Features<a href="#TOC"><img align="right" src="https://i.ibb.co/YBVkRcnC/up-arrow.png" width="22"></a>

- **23+ Built-in Rules** - AST-based TypeScript/TSX validation for type safety, imports, and code quality
- **Custom Rules** - Regex patterns, JavaScript scripts, or AI-powered validation
- **Git-Aware Scanning** - Full codebase or only files changed in your branch
- **Works Everywhere** - CLI, VS Code extension, and GitHub Action with zero config
- **Rust-Powered Speed** - 100-500 files in <1s with parallel processing and smart caching

## 🎯 Use Cases<a href="#TOC"><img align="right" src="https://i.ibb.co/YBVkRcnC/up-arrow.png" width="22"></a>

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

## 🏗️ Architecture<a href="#TOC"><img align="right" src="https://i.ibb.co/YBVkRcnC/up-arrow.png" width="22"></a>

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

## 🚀 Quick Start<a href="#TOC"><img align="right" src="https://i.ibb.co/YBVkRcnC/up-arrow.png" width="22"></a>


### CLI

```bash
# Install globally
npm install -g tscanner
pnpm add -g tscanner
yarn global add tscanner

# Initialize configuration
tscanner init

# Scan workspace
tscanner check

# Scan only changed files vs branch
tscanner check --branch main

# Output as JSON
tscanner check --json
```

### VSCode Extension

1. Install from VSCode marketplace or run:
   ```bash
   code --install-extension lucasvtiradentes.tscanner-vscode
   ```
2. Click TScanner icon in activity bar
3. Issues appear automatically in the sidebar
4. Configure rules via settings menu

### GitHub Action

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
      - uses: lucasvtiradentes/tscanner-action@v0.0.11
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          target-branch: 'origin/main'  # Optional: scan only changed files
```

### Configuration

Create `.tscanner/config.jsonc`:

```json
{
  "schema": "https://unpkg.com/tscanner@0.0.14/schema.json",
  "builtinRules": {
    "no-any-type": {
      "enabled": true,
      "severity": "error"
    },
    "no-console-log": {
      "enabled": true,
      "severity": "warning"
    }
  },
  "customRules": {
    "no-todos": {
      "type": "regex",
      "pattern": "TODO:|FIXME:",
      "message": "Remove TODO comments",
      "severity": "warning"
    }
  },
  "include": ["**/*.{ts,tsx}"],
  "exclude": ["node_modules/**", "dist/**", "build/**", ".git/**"]
}
```

**Inline Disables:**

```typescript
// tscanner-disable-next-line no-any-type
const data: any = fetchData();

// tscanner-disable-file
// Entire file is skipped
```

## 💡 Inspirations<a href="#TOC"><img align="right" src="https://i.ibb.co/YBVkRcnC/up-arrow.png" width="22"></a>

- [Biome](https://github.com/biomejs/biome) - Biome is a performant toolchain for web projects, it aims to provide developer tools to maintain the health of said projects.
- [VSCode Bookmarks](https://github.com/alefragnani/vscode-bookmarks) - Bookmarks Extension for Visual Studio Code

## 📜 License<a href="#TOC"><img align="right" src="https://i.ibb.co/YBVkRcnC/up-arrow.png" width="22"></a>

MIT License - see [LICENSE](LICENSE) file for details.

<a href="#"><img src="https://i.ibb.co/CKW9djzW/divider.png" /></a>

<div align="center">
  <div>
    <a target="_blank" href="https://www.linkedin.com/in/lucasvtiradentes/"><img src="https://img.shields.io/badge/-linkedin-blue?logo=Linkedin&logoColor=white" alt="LinkedIn"></a>
    <a target="_blank" href="mailto:lucasvtiradentes@gmail.com"><img src="https://img.shields.io/badge/gmail-red?logo=gmail&logoColor=white" alt="Gmail"></a>
    <a target="_blank" href="https://x.com/lucasvtiradente"><img src="https://img.shields.io/badge/-X-black?logo=X&logoColor=white" alt="X"></a>
    <a target="_blank" href="https://github.com/lucasvtiradentes"><img src="https://img.shields.io/badge/-github-gray?logo=Github&logoColor=white" alt="Github"></a>
  </div>
</div>
