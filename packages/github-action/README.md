<div align="center">
<h3>Tscanner GitHub Action</h3>
<p>
  <a href="#-features">Features</a> • <a href="#-usage">Usage</a> • <a href="#-inputs">Inputs</a>
</p>
</div>

---

## 🎺 Overview

GitHub Action for [Tscanner](https://github.com/lucasvtiradentes/tscanner) - automatically scan your codebase for quality issues on every pull request. Scan your entire codebase or just the changed files, with smart PR comments that link directly to problematic lines.

## 🌟 Features

- **Two scan modes:** codebase (all files) or branch (changed files only)
- **Dual grouping:** Issues grouped by rule AND by file in collapsible sections
- **Smart PR comments:** Creates/updates single comment instead of spam
- **Direct file links:** Jump to exact line in PR files view
- **Flexible workflow control:** Continue or fail on errors

## 🚀 Usage

### Codebase Mode

- Scans entire workspace
- Use when no `target-branch` specified

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.1
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

### Branch Mode (Changed Files Only)

- Scans only changed files vs target branch
- Enabled automatically when `target-branch` is provided
- Requires pull_request event

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.1
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    target-branch: 'origin/develop'
```

### Continue on Errors

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.1
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    continue-on-error: 'true'
```

### Full Configuration

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.1
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    target-branch: 'origin/develop'
    timezone: 'America/Sao_Paulo'
    config-path: '.tscanner'
    tscanner-version: 'latest'
    continue-on-error: 'false'
    group-by: 'rule'
```

## 📋 Inputs

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `github-token` | Yes | - | GitHub token for posting comments |
| `target-branch` | - | - | Target branch to compare (enables branch mode) |
| `config-path` | - | `.tscanner` | Path to tscanner config directory |
| `tscanner-version` | - | `latest` | NPM version of tscanner CLI |
| `group-by` | - | `file` | Grouping mode: `file` or `rule` |
| `continue-on-error` | - | `false` | Continue workflow even if errors found |
| `timezone` | - | `UTC` | Timezone for timestamps on pr comment |
