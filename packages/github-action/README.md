<a name="TOC"></a>

<div align="center">
<img width="128" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/logo.png" alt="tscanner GitHub Action logo">
<h4>tscanner - GitHub Action</h4>
<p>
  <a href="https://github.com/marketplace/actions/tscanner-pr-validator"><img src="https://img.shields.io/badge/GitHub-Marketplace-blue.svg" alt="GitHub Marketplace"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <br>
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-usage">Usage</a> • <a href="#-inputs">Inputs</a> • <a href="#-license">License</a>
</p>

</div>

<a href="#"><img src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/divider.png" /></a>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

GitHub Action for [Tscanner](https://github.com/lucasvtiradentes/tscanner): Enforce project-specific patterns, detect anti-patterns, and validate architectural conventions with 23+ built-in rules or custom validation (regex, scripts, AI). Integrates into CI/CD workflows with smart PR comments and flexible scan modes.

<img src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/pr-comment-errors-found.png" alt="PR Comment Screenshot" width="100%">

## ⭐ Features<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

- **23+ Built-in Rules** - AST-based validation for TypeScript/TSX
- **Custom Rules** - Regex patterns, JavaScript scripts, or AI-powered validation
- **Two Scan Modes** - Full codebase or only changed files
- **Smart PR Comments** - Automatic PR annotations with dual grouping (rule + file)
- **Direct File Links** - Jump to exact line in PR files view
- **Flexible Control** - Continue or fail workflow on errors
- **CI/CD Integration** - Works with any GitHub Actions workflow

## 🚀 Usage<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

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
      - uses: lucasvtiradentes/tscanner-action@v0.0.5
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
```

**Scan only changed files (recommended for PRs):**

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.5
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    target-branch: 'origin/main'
```

### Advanced Examples

<details>
<summary><b>Continue on Errors</b></summary>

Scan but don't fail the workflow:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.5
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    continue-on-error: 'true'
```

</details>

<details>
<summary><b>Group by Rule</b></summary>

Primary grouping by rule instead of file:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.5
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    group-by: 'rule'
```

</details>

<details>
<summary><b>Custom Config Path</b></summary>

Use non-standard config location:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.5
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    config-path: 'config/tscanner'
```

</details>

<details>
<summary><b>Specific tscanner Version</b></summary>

Pin to exact CLI version:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.5
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    tscanner-version: '0.1.5'
```

</details>

<details>
<summary><b>Full Configuration</b></summary>

All options:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.5
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

## 📋 Inputs<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `github-token` | ✅ Yes | - | GitHub token for posting PR comments (`${{ secrets.GITHUB_TOKEN }}`) |
| `target-branch` | No | - | Target branch to compare (enables branch mode). Example: `origin/main` |
| `config-path` | No | `.tscanner` | Path to tscanner config directory containing `config.jsonc` |
| `tscanner-version` | No | `latest` | NPM version of tscanner CLI to install |
| `group-by` | No | `file` | Primary grouping mode: `file` or `rule` |
| `continue-on-error` | No | `false` | Continue workflow even if errors found (`true`/`false`) |
| `timezone` | No | `UTC` | Timezone for timestamps in PR comments. Example: `America/New_York` |

## 📜 License<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

MIT License - see [LICENSE](LICENSE) file for details.
