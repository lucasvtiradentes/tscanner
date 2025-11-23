# tscanner GitHub Action

GitHub Action that scans PR changes for code quality issues and posts results as PR comments.

## Features

- Scans only changed files in PRs
- Groups issues by rule type
- Collapsible details for each rule
- Updates existing comments instead of creating duplicates
- Timezone-aware timestamps
- Fails workflow if errors are found

## Usage

### Basic

```yaml
- uses: lucasvtiradentes/tscanner/.github/action@main
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

### Advanced

```yaml
- uses: lucasvtiradentes/tscanner/.github/action@main
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    target-branch: 'origin/develop'
    timezone: 'America/Sao_Paulo'
    config-path: '.tscanner/rules.json'
```

## Inputs

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `github-token` | Yes | - | GitHub token for posting comments |
| `target-branch` | No | `origin/main` | Target branch to compare against |
| `timezone` | No | `UTC` | Timezone for timestamps |
| `config-path` | No | `.tscanner/rules.json` | Path to tscanner config |

## Development

```bash
cd packages/github-action
pnpm install
pnpm build
```

The action uses `@vercel/ncc` to compile everything into a single `dist/index.js` file.

## Publishing

The `dist/` folder must be committed for GitHub Actions to work. Run `pnpm build` before committing.
