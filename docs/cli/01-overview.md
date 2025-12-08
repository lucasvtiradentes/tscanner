# CLI Package Overview

## What is it

The CLI package is a Node.js wrapper around the Rust binary that enables cross-platform distribution via npm. It resolves the platform-specific binary at runtime and spawns it as a child process, forwarding all arguments and preserving exit codes.

**Architecture:**

```
tscanner (npm)
├── Node.js wrapper (main.ts)
│   ├── Binary resolution (binary-resolver.ts)
│   │   └── Platform detection (via tscanner-common)
│   └── Child process spawn → Rust binary
└── Platform-specific binaries (optionalDependencies)
    ├── @tscanner/cli-darwin-arm64
    ├── @tscanner/cli-darwin-x64
    ├── @tscanner/cli-linux-arm64
    ├── @tscanner/cli-linux-x64
    └── @tscanner/cli-win32-x64
```

## Why it exists

**Distribution Strategy:**

npm provides universal package distribution across all platforms. The CLI package leverages `optionalDependencies` to download only the binary matching the user's platform during installation.

**Key Benefits:**

| Benefit | Description |
|---------|-------------|
| Cross-platform | Single `npm install tscanner` works on macOS, Linux, Windows |
| Zero compilation | Pre-built binaries eliminate build toolchain requirements |
| Automatic updates | `npm update` pulls latest binaries across all platforms |
| CI/CD ready | Standard npm workflow in GitHub Actions, GitLab CI |

## Installation

```bash
npm install -g tscanner

npm install --save-dev tscanner
```

On installation, the postinstall script resolves and validates the platform-specific binary.

## Basic Usage

```bash
tscanner init

tscanner check

tscanner check --branch origin/main
```

## Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `init` | Create `.tscanner/config.jsonc` configuration | `tscanner init` |
| `check [paths]` | Scan files and report issues | `tscanner check` |
| `config` | Show/validate configuration and list rules | `tscanner config --rules` |
| `lsp` | Start Language Server Protocol server | `tscanner lsp` |
| `--help` | Show help information | `tscanner --help` |
| `--version` | Show version number | `tscanner --version` |

### Init Command Flags

| Flag | Description | Example |
|------|-------------|---------|
| `--all-rules` | Include all available rules in config | `tscanner init --all-rules` |

### Check Command Flags

| Flag | Description | Example |
|------|-------------|---------|
| `--no-cache` | Skip cache and force full scan | `tscanner check --no-cache` |
| `--format <FORMAT>` | Output format: `json`, `pretty`, `default` | `tscanner check --format json` |
| `--group-by <GROUP>` | Group issues by `rule` or `file` | `tscanner check --group-by rule` |
| `--branch <BRANCH>` | Only scan files changed vs branch | `tscanner check --branch main` |
| `--staged` | Only scan staged files | `tscanner check --staged` |
| `--glob <PATTERN>` | Filter by file glob pattern | `tscanner check --glob "src/**"` |
| `--rule <RULE>` | Filter by specific rule | `tscanner check --rule no-explicit-any` |
| `--continue-on-error` | Don't exit with error code | `tscanner check --continue-on-error` |
| `--config-path <DIR>` | Custom config directory | `tscanner check --config-path ./custom` |
| `--include-ai` | Include AI-powered rules in scan | `tscanner check --include-ai` |
| `--only-ai` | Only run AI-powered rules | `tscanner check --only-ai` |

### Config Command Flags

| Flag | Description | Example |
|------|-------------|---------|
| `--rules` | List all available rules and their status | `tscanner config --rules` |
| `--validate` | Validate the configuration file | `tscanner config --validate` |
| `--show` | Show the resolved configuration | `tscanner config --show` |
| `--config-path <DIR>` | Path to .tscanner folder | `tscanner config --config-path ./custom` |

## Exit Codes

| Code | Condition |
|------|-----------|
| `0` | No errors found |
| `1` | Errors found, configuration missing, or binary resolution failed |

The `--continue-on-error` flag forces exit code `0` regardless of errors found.

## Package Relationships

```
CLI Package
├── Calls → Rust binary (via child_process.spawn)
├── Used by → GitHub Action (runs tscanner check in CI)
└── Shares schema → tscanner-common (platform detection)
```

**Data Flow:**

1. User runs `tscanner check`
2. Node.js wrapper resolves platform-specific binary path
3. Spawns Rust binary with forwarded arguments
4. Rust binary executes scanner, returns exit code
5. Node.js wrapper propagates exit code to shell

**GitHub Action Integration:**

The GitHub Action installs the CLI package and executes `tscanner check --branch` to scan PR changes. Results are posted as PR comments.
