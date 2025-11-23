<a name="TOC"></a>

<div align="center">
<h3>Tscanner GitHub Action</h3>
<p>
  🔍 Code quality scanner for the AI-generated code era
  <br><br>
  <a href="#-features">Features</a> • <a href="#-usage">Usage</a> • <a href="#-inputs">Inputs</a> • <a href="#-development">Development</a>
</p>
</div>

---

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
- uses: lucasvtiradentes/tscanner/.github/action@main
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

### Branch Mode (Changed Files Only)

- Scans only changed files vs target branch
- Enabled automatically when `target-branch` is provided
- Requires pull_request event

```yaml
- uses: lucasvtiradentes/tscanner/.github/action@main
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    target-branch: 'origin/develop'
```

### Continue on Errors

```yaml
- uses: lucasvtiradentes/tscanner/.github/action@main
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    continue-on-error: 'true'
```

### Full Configuration

```yaml
- uses: lucasvtiradentes/tscanner/.github/action@main
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
| `target-branch` | No | - | Target branch to compare (enables branch mode) |
| `timezone` | No | `UTC` | Timezone for timestamps |
| `config-path` | No | `.tscanner` | Path to tscanner config directory |
| `tscanner-version` | No | `latest` | NPM version of tscanner CLI |
| `group-by` | No | `file` | Grouping mode: `file` or `rule` |
| `continue-on-error` | No | `false` | Continue workflow even if errors found |
