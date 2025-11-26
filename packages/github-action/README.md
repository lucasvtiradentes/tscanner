<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner - GitHub Action</strong></div>
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-usage">Usage</a> • <a href="#-inputs">Inputs</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-license">License</a>
</div>

<a href="#"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" /></a>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

GitHub Action for [TScanner](https://github.com/lucasvtiradentes/tscanner): Enforce project-specific patterns, detect anti-patterns, and validate architectural conventions with 23+ built-in rules or custom validation (regex, scripts, AI). Integrates into CI/CD workflows with smart PR comments and flexible scan modes.

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
- **23+ Built-in Rules** - Type safety, imports, and code quality checks
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
      - uses: lucasvtiradentes/tscanner-action@v0.0.15
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
```

**Scan only changed files (recommended for PRs):**

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.15
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    target-branch: 'origin/main'
```

### Additional examples

<details>
<summary><b>Continue on Errors</b></summary>

Scan but don't fail the workflow:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.15
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    continue-on-error: 'true'
```

</details>

<details>
<summary><b>Group by Rule</b></summary>

Primary grouping by rule instead of file:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.15
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    group-by: 'rule'
```

</details>

<details>
<summary><b>Custom Config Path</b></summary>

Use non-standard config location:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.15
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    config-path: 'config/tscanner'
```

</details>

<details>
<summary><b>Specific tscanner Version</b></summary>

Pin to exact CLI version:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.15
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    tscanner-version: '0.1.5'
```

</details>

<details>
<summary><b>Full Configuration</b></summary>

All options:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.15
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
