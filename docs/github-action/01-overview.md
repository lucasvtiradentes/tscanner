# GitHub Action Overview

## What is it

The GitHub Action is a CI/CD integration that runs TScanner on pull requests and posts scan results as PR comments. It automatically detects code quality issues in changed files and provides inline feedback directly in the PR conversation.

**Architecture:**

```
GitHub PR Event
├── Action triggers (on pull_request)
├── Installs tscanner CLI (npm)
├── Executes scan command
│   ├── Full codebase scan OR
│   └── Changed files only (--branch mode)
├── Parses JSON output
└── Posts/updates PR comment
```

## Why it exists

**Automated PR Review:**

Code quality checks run automatically on every PR push. Reviewers see violations immediately without manual scanning. The action posts a single comment that updates on each push, avoiding notification spam.

**Key Benefits:**

| Benefit | Description |
|---------|-------------|
| Shift-left testing | Catch issues before merge, not after deployment |
| Consistent enforcement | Same rules applied to all PRs regardless of reviewer |
| Reduce review time | Automated checks handle style/quality, reviewers focus on logic |
| Git-aware scanning | Only scan changed files to minimize CI runtime |

## How it works

**Execution Flow:**

```
1. PR opened/updated → GitHub triggers workflow
2. Action installs tscanner CLI via npm
3. Action validates .tscanner/config.jsonc exists
4. CLI scans files (full or branch mode):
   - Full: tscanner check --json
   - Branch: tscanner check --json --branch origin/main
5. CLI outputs JSON with issues/metadata
6. Action parses JSON output
7. Action creates/updates PR comment with results
8. Action exits with code 1 if errors found (unless continue-on-error)
```

**Comment Management:**

The action uses a marker comment (`<!-- tscanner-action-comment -->`) to identify its comment. On each run, it searches for existing comments and updates them instead of creating duplicates.

## Action Inputs

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `github-token` | Yes | - | GitHub token for posting PR comments (`${{ secrets.GITHUB_TOKEN }}`) |
| `target-branch` | No | - | Target branch to compare (enables branch mode). Example: `origin/main` |
| `config-path` | No | `.tscanner` | Path to tscanner config directory containing `config.jsonc` |
| `tscanner-version` | No | `latest` | NPM version of tscanner CLI to install |
| `group-by` | No | `file` | Primary grouping mode: `file` or `rule` |
| `continue-on-error` | No | `false` | Continue workflow even if errors found (`true`/`false`) |
| `timezone` | No | `UTC` | Timezone for timestamps in PR comments. Example: `America/New_York` |
| `dev-mode` | No | `false` | Use local CLI from monorepo (internal development only) |

## Action Outputs

The action does not expose explicit outputs. Scan results are communicated via:

1. **PR Comment** - Posted to the pull request with formatted results
2. **Exit Code** - `0` for success, `1` for errors (unless `continue-on-error: true`)
3. **Logs** - GitHub Actions logs show scan progress and error details

## Basic Workflow Example

**Minimal Setup (Full Scan):**

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

**Recommended Setup (Branch Mode):**

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
        with:
          fetch-depth: 0
      - uses: lucasvtiradentes/tscanner-action@v0.0.17
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          target-branch: 'origin/main'
```

**Advanced Configuration:**

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

## CLI Integration

The action shells out to the tscanner CLI package, which wraps the Rust binary. Command execution depends on scan mode:

**Full Scan Command:**

```bash
npx tscanner check --json --config .tscanner
```

**Branch Scan Command:**

```bash
npx tscanner check --json --branch origin/main --config .tscanner
```

**JSON Output Structure:**

The CLI outputs JSON that the action parses to generate PR comments:

```json
{
  "totalErrors": 5,
  "totalWarnings": 2,
  "files": [
    {
      "path": "src/utils/helper.ts",
      "issues": [
        {
          "rule": "no-any-type",
          "message": "Avoid using 'any' type",
          "severity": "error",
          "line": 42,
          "column": 10
        }
      ]
    }
  ],
  "metadata": {
    "scanDuration": 1234,
    "filesScanned": 150,
    "rulesEnabled": 12
  }
}
```

**Flow Diagram:**

```
GitHub Action
├── npm install tscanner@{version}
├── npx tscanner check --json [--branch {target}] [--config {path}]
├── Capture stdout (JSON)
├── Parse JSON into ScanResult object
└── Format and post PR comment
```

**Package Relationships:**

```
GitHub Action
├── Installs → CLI package (tscanner npm)
├── Executes → Rust binary (via Node.js wrapper)
├── Parses → JSON output (--json flag)
└── Posts → PR comment (GitHub API)
```

## Configuration Requirements

The action requires a valid TScanner configuration file:

**Expected File:**

```
{workspace}/.tscanner/config.jsonc
```

**Validation:**

The action validates the config file exists before scanning. If missing, it fails with:

```
Config file not found at .tscanner/config.jsonc
```

**Initialization:**

Create the config using one of these methods:

1. Run `npx tscanner init` locally and commit the generated `.tscanner/` directory
2. Install the VSCode extension and use the "Manage Rules" UI
3. Manually create `.tscanner/config.jsonc` with your rule configuration

See [Configuration](../cli/02-configuration.md) for config file structure.
