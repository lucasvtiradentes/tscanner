# CLI Package Overview

## What is it

The CLI package is a Node.js wrapper around the Rust binary that enables cross-platform distribution via npm. It resolves the platform-specific binary at runtime and spawns it as a child process, forwarding all arguments and preserving exit codes.

**Architecture:**

```
tscanner (npm)
├── Node.js wrapper (main.ts)
│   ├── Platform detection (platform.ts)
│   ├── Binary resolution (binary-resolver.ts)
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
| `init [path]` | Create `.tscanner/config.jsonc` configuration | `tscanner init` |
| `check [path]` | Scan files and report issues | `tscanner check` |
| `rules [path]` | List all available rules with metadata | `tscanner rules` |
| `--help` | Show help information | `tscanner --help` |
| `--version` | Show version number | `tscanner --version` |

### Check Command Flags

| Flag | Description | Example |
|------|-------------|---------|
| `--no-cache` | Skip cache and force full scan | `tscanner check --no-cache` |
| `--json` | Output results as JSON | `tscanner check --json` |
| `--pretty` | Pretty output with rule definitions | `tscanner check --pretty` |
| `--by-rule` | Group issues by rule instead of file | `tscanner check --by-rule` |
| `--branch <BRANCH>` | Only scan files changed vs branch | `tscanner check --branch main` |
| `--file <PATTERN>` | Filter by file glob pattern | `tscanner check --file "src/**"` |
| `--rule <RULE>` | Filter by specific rule | `tscanner check --rule no-explicit-any` |
| `--continue-on-error` | Don't exit with error code | `tscanner check --continue-on-error` |
| `--config <DIR>` | Custom config directory | `tscanner check --config ./custom` |

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
