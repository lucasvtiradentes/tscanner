# TScanner CLI Package - Technical Documentation

## Overview

The TScanner CLI package (`packages/cli`) is a Node.js wrapper that provides terminal interface for the TScanner code quality scanner. It acts as a thin launcher that detects the platform, resolves the appropriate Rust binary, and spawns it with passed command-line arguments.

The CLI enables integration with CI/CD pipelines, git hooks, development workflows, and manual command-line usage while maintaining the high-performance Rust core implementation.

## Package Information

- **Package Name**: `tscanner`
- **Version**: `0.0.20`
- **License**: MIT
- **Binary Command**: `tscanner`
- **Node Engine**: `>=18`
- **Main Entry**: `./dist/main.js`

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Terminal                            │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│              Node.js CLI Wrapper (TypeScript)                    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ main.ts                                                    │  │
│  │  - Entry point                                             │  │
│  │  - Signal handling (SIGINT, SIGTERM)                       │  │
│  │  - Exit code propagation                                   │  │
│  └──────────────────┬─────────────────────────────────────────┘  │
│                     │                                             │
│                     ▼                                             │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ binary-resolver.ts                                         │  │
│  │  - Platform detection                                      │  │
│  │  - Binary path resolution (dev/CI/production)              │  │
│  │  - Package.json optionalDependencies lookup                │  │
│  └──────────────────┬─────────────────────────────────────────┘  │
│                     │                                             │
│                     ▼                                             │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ platform.ts                                                │  │
│  │  - getPlatformKey() → "linux-x64", "darwin-arm64", etc.    │  │
│  │  - getBinaryName() → "tscanner" or "tscanner.exe"          │  │
│  └─────────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       │ spawn() with stdio: 'inherit'
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Rust Binary (tscanner)                         │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ CLI Commands (Rust - crates/cli/src/main.rs)              │  │
│  │  - clap argument parsing                                   │  │
│  │  - Commands: check, rules, init                            │  │
│  │  - Flags: --no-cache, --json, --branch, etc.               │  │
│  └──────────────────┬─────────────────────────────────────────┘  │
│                     │                                             │
│                     ▼                                             │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Core Library (Rust - crates/core)                          │  │
│  │  - Scanner (Rayon parallel processing)                     │  │
│  │  - Parser (SWC AST)                                        │  │
│  │  - Rule Registry (39+ rules)                               │  │
│  │  - Cache (DashMap with disk persistence)                   │  │
│  │  - Config loader (.tscanner/config.jsonc)                  │  │
│  └─────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## File Structure

```
packages/cli/
├── src/
│   ├── main.ts              # Entry point: spawn binary with args
│   ├── binary-resolver.ts   # Resolve platform-specific binary path
│   └── platform.ts          # Platform detection utilities
├── npm/                     # Platform-specific binary packages
│   ├── cli-darwin-arm64/
│   │   ├── package.json     # macOS ARM64 binary package
│   │   └── tscanner         # Rust binary (darwin-arm64)
│   ├── cli-darwin-x64/
│   │   ├── package.json     # macOS x64 binary package
│   │   └── tscanner         # Rust binary (darwin-x64)
│   ├── cli-linux-arm64/
│   │   ├── package.json     # Linux ARM64 binary package
│   │   └── tscanner         # Rust binary (linux-arm64)
│   ├── cli-linux-x64/
│   │   ├── package.json     # Linux x64 binary package
│   │   └── tscanner         # Rust binary (linux-x64)
│   └── cli-win32-x64/
│       ├── package.json     # Windows x64 binary package
│       └── tscanner.exe     # Rust binary (win32-x64)
├── dist/
│   └── main.js              # Bundled output from tsup
├── package.json             # Main package with optionalDependencies
├── postinstall.js           # Post-install hook: chmod binary
├── schema.json              # JSON schema for config.jsonc
├── tsup.config.ts           # Build configuration
└── tsconfig.json            # TypeScript configuration
```

## Module-by-Module Breakdown

### 1. main.ts - Entry Point

**Purpose**: Main entry point that spawns the Rust binary and handles process lifecycle.

**Key Responsibilities**:
- Import binary resolver
- Spawn Rust binary with `child_process.spawn()`
- Forward all CLI arguments (`process.argv.slice(2)`)
- Inherit stdio streams (stdout, stderr, stdin)
- Handle exit codes and signals
- Propagate termination signals (SIGINT, SIGTERM)

**Implementation Details**:

```typescript
import { spawn } from 'node:child_process';
import { getBinaryPath } from './binary-resolver';

function main(): void {
  try {
    const binaryPath = getBinaryPath();

    // Spawn binary with inherited stdio
    const child = spawn(binaryPath, process.argv.slice(2), {
      stdio: 'inherit',      // Forward stdout/stderr/stdin
      windowsHide: true,     // Hide console window on Windows
    });

    // Handle exit
    child.on('exit', (code, signal) => {
      if (signal) {
        process.kill(process.pid, signal);
      } else {
        process.exit(code || 0);
      }
    });

    // Forward signals to child
    process.on('SIGINT', () => {
      child.kill('SIGINT');
      child.kill('SIGTERM');
    });

    process.on('SIGTERM', () => {
      child.kill('SIGTERM');
    });
  } catch (error) {
    console.error(error.message);
    process.exit(1);
  }
}
```

**Signal Handling**:
- **SIGINT** (Ctrl+C): Forwards to child process, sends both SIGINT and SIGTERM
- **SIGTERM**: Forwards to child process
- Child exit code is propagated to parent process

**Error Handling**:
- Catches errors from `getBinaryPath()` (unsupported platform, missing binary)
- Exits with code 1 on errors

### 2. binary-resolver.ts - Binary Path Resolution

**Purpose**: Resolve the correct Rust binary path for the current platform.

**Key Responsibilities**:
- Get platform key (e.g., "linux-x64", "darwin-arm64")
- Resolve binary from multiple locations (dev, CI, production)
- Handle missing binaries with helpful error messages

**Resolution Strategy** (in order):

1. **Development Path**: `npm/cli-{platform}/tscanner`
   - Used during local development/monorepo
   - Example: `npm/cli-linux-x64/tscanner`

2. **CI Path**: `npm/{platform}/tscanner`
   - Used in CI builds
   - Example: `npm/linux-x64/tscanner`

3. **Production Path**: `require.resolve()`
   - Resolves from `node_modules/@tscanner/cli-{platform}`
   - Used after `npm install` in production

**Implementation**:

```typescript
export function getBinaryPath(): string {
  const platformKey = getPlatformKey();
  const packageName = PLATFORM_PACKAGE_MAP[platformKey];
  const binaryName = getBinaryName();

  // 1. Check dev path
  const devPath = join(__dirname, '..', 'npm', `cli-${platformKey}`, binaryName);
  if (existsSync(devPath)) {
    return devPath;
  }

  // 2. Check CI path
  const ciPath = join(__dirname, '..', 'npm', platformKey, binaryName);
  if (existsSync(ciPath)) {
    return ciPath;
  }

  // 3. Resolve from node_modules
  try {
    const binaryPath = require.resolve(`${packageName}/${binaryName}`);
    return binaryPath;
  } catch (e) {
    throw new Error(
      `Failed to find TScanner binary for ${platformKey}
Please try reinstalling: npm install tscanner
Error: ${error.message}`
    );
  }
}
```

**Dependencies**:
- `PLATFORM_PACKAGE_MAP` from `tscanner-common`
- `getPlatformKey()` from `platform.ts`
- `getBinaryName()` from `tscanner-common`

### 3. platform.ts - Platform Detection

**Purpose**: Detect the current platform and architecture.

**Exported Functions**:

1. **`getPlatformKey(): string`**
   - Returns platform identifier (e.g., "linux-x64", "darwin-arm64")
   - Throws error for unsupported platforms

2. **`getBinaryName(): string`**
   - Returns binary name with platform-specific extension
   - Windows: "tscanner.exe"
   - Unix: "tscanner"

**Supported Platforms**:

| Platform Key | OS | Architecture |
|--------------|-----|--------------|
| `linux-x64` | Linux | x86_64 |
| `linux-arm64` | Linux | ARM64 |
| `darwin-x64` | macOS | x86_64 (Intel) |
| `darwin-arm64` | macOS | ARM64 (Apple Silicon) |
| `win32-x64` | Windows | x86_64 |

**Implementation**:

```typescript
export const PLATFORM_MAP: Record<string, string> = {
  'linux-x64': '@tscanner/cli-linux-x64',
  'linux-arm64': '@tscanner/cli-linux-arm64',
  'darwin-x64': '@tscanner/cli-darwin-x64',
  'darwin-arm64': '@tscanner/cli-darwin-arm64',
  'win32-x64': '@tscanner/cli-win32-x64',
};

export function getPlatformKey(): string {
  const platform = process.platform;
  const arch = process.arch;

  // Map to platform key
  if (platform === 'linux') {
    if (arch === 'x64') return 'linux-x64';
    if (arch === 'arm64') return 'linux-arm64';
  }

  if (platform === 'darwin') {
    if (arch === 'x64') return 'darwin-x64';
    if (arch === 'arm64') return 'darwin-arm64';
  }

  if (platform === 'win32') {
    if (arch === 'x64') return 'win32-x64';
  }

  // Unsupported platform
  throw new Error(
    `Unsupported platform: ${platform}-${arch}
tscanner is only available for:
  - Linux (x64, arm64)
  - macOS (x64, arm64)
  - Windows (x64)`
  );
}

export function getBinaryName(): string {
  return `tscanner${process.platform === 'win32' ? '.exe' : ''}`;
}
```

### 4. postinstall.js - Post-Install Hook

**Purpose**: Run after package installation to ensure binary is executable.

**Key Responsibilities**:
- Detect current platform
- Locate installed binary via `require.resolve()`
- Set executable permissions on Unix (`chmod 0o755`)
- Log installation status
- Silent in workspace environments (monorepo)

**Implementation**:

```javascript
const platformKey = getPlatformKey();
const isWorkspace = process.env.PNPM_SCRIPT_SRC_DIR !== undefined;

if (!platformKey) {
  if (!isWorkspace) {
    logger.warn('Warning: tscanner does not have a prebuilt binary...');
  }
  process.exit(0);
}

const packageName = PLATFORM_MAP[platformKey];

try {
  const packagePath = require.resolve(packageName);
  const binaryName = process.platform === 'win32' ? 'tscanner.exe' : 'tscanner';
  const binaryPath = join(packagePath, '..', binaryName);

  // Set executable on Unix
  if (process.platform !== 'win32') {
    try {
      chmodSync(binaryPath, 0o755);
    } catch (_e) {
      // Ignore chmod errors
    }
  }

  if (!isWorkspace) {
    logger.log(`✅ tscanner binary installed successfully (${platformKey})`);
  }
} catch (_e) {
  if (!isWorkspace) {
    logger.warn('Warning: Failed to install tscanner binary...');
  }
}
```

**Workspace Detection**:
- Checks `process.env.PNPM_SCRIPT_SRC_DIR`
- Suppresses logs in monorepo workspace environments

### 5. tsup.config.ts - Build Configuration

**Purpose**: Bundle TypeScript source into a single CommonJS file.

**Configuration**:

```typescript
export default defineConfig({
  entry: ['src/main.ts'],    // Entry point
  format: ['cjs'],            // CommonJS format
  outDir: 'dist',             // Output directory
  clean: true,                // Clean before build
  minify: false,              // No minification
  sourcemap: false,           // No sourcemaps
  dts: false,                 // No type definitions
  shims: false,               // No shims
});
```

**Build Output**:
- Single file: `dist/main.js`
- No tree-shaking (wrapper is tiny)
- No bundling of dependencies (only imports `tscanner-common`)

## External Dependencies

### Production Dependencies

1. **zod** (`^4.1.12`)
   - Runtime type validation
   - Used for config schema validation
   - Not directly used in CLI wrapper (used by Rust binary via schema.json)

### Development Dependencies

1. **@types/node** (`^22.0.0`)
   - Node.js type definitions
   - Used for TypeScript compilation

2. **tscanner-common** (`workspace:*`)
   - Shared utilities and constants
   - Platform detection helpers
   - Schema definitions

3. **tsup** (`^8.5.1`)
   - TypeScript bundler
   - Builds `src/main.ts` → `dist/main.js`

4. **tsx** (`^4.19.2`)
   - TypeScript execution for development
   - Used in `dev` script

5. **typescript** (`^5.7.0`)
   - TypeScript compiler
   - Used for type checking

6. **zod-to-json-schema** (`^3.25.0`)
   - Converts Zod schemas to JSON Schema
   - Generates `schema.json` for editor autocomplete

### Optional Dependencies

Platform-specific binary packages (only one gets installed per platform):

```json
{
  "@tscanner/cli-darwin-arm64": "0.0.20",
  "@tscanner/cli-darwin-x64": "0.0.20",
  "@tscanner/cli-linux-arm64": "0.0.20",
  "@tscanner/cli-linux-x64": "0.0.20",
  "@tscanner/cli-win32-x64": "0.0.20"
}
```

Each platform package contains:
- Binary executable (`tscanner` or `tscanner.exe`)
- `package.json` with `os` and `cpu` constraints

## Communication with Other Packages

### 1. CLI → Rust Core (packages/core)

**Communication Method**: Direct process execution

**Flow**:
1. CLI wrapper spawns Rust binary
2. Forwards all command-line arguments
3. Rust binary parses arguments with `clap`
4. Rust binary executes command (check/rules/init)
5. Rust binary outputs to stdout/stderr
6. CLI wrapper inherits stdio (no processing)
7. Exit code propagated back to shell

**No JSON-RPC**:
- Unlike VSCode extension, CLI does not use JSON-RPC
- Direct stdio inheritance for performance
- No intermediate processing or protocol overhead

**Arguments Passed**:
```bash
# User runs:
tscanner check --branch main --json

# CLI wrapper spawns:
/path/to/tscanner check --branch main --json
```

### 2. CLI → tscanner-common (shared/tscanner-common)

**Shared Utilities**:

1. **Platform Detection**:
   ```typescript
   import { getPlatformKey, getBinaryName } from 'tscanner-common';
   ```

2. **Constants**:
   ```typescript
   import { PLATFORM_PACKAGE_MAP } from 'tscanner-common';
   ```

3. **Schema Types** (indirectly via schema.json):
   - CLI generates `schema.json` from Zod schemas
   - Rust binary uses schema for config validation

### 3. CLI vs VSCode Extension

**Key Differences**:

| Aspect | CLI Package | VSCode Extension |
|--------|-------------|------------------|
| Communication | Direct process spawn | JSON-RPC protocol |
| Binary Usage | One-shot execution | Long-running server |
| Output Format | Text/JSON to stdout | Structured JSON responses |
| Caching | Rust binary handles | Rust server handles |
| File Watching | N/A | Rust server watches |
| UI | Terminal text | VSCode TreeView |

**Similarities**:
- Both spawn platform-specific Rust binary
- Both use same binary resolver pattern
- Both support same scan modes (workspace/branch)

### 4. CLI vs GitHub Action

**CLI is Used By GitHub Action**:

```yaml
# GitHub Action runs:
- name: Install TScanner
  run: npm install -g tscanner

- name: Run scan
  run: tscanner check --json > results.json
```

**GitHub Action Package**:
- Wraps CLI with GitHub Actions SDK
- Parses JSON output from CLI
- Posts PR comments with results
- Sets workflow status based on exit code

## Configuration Handling

### Config Schema (schema.json)

The CLI package includes a JSON Schema file (`schema.json`) that defines the structure of `.tscanner/config.jsonc`.

**Generation**:
- Generated from Zod schemas in `tscanner-common`
- Uses `zod-to-json-schema` package
- Provides IDE autocomplete and validation

**Schema Structure**:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "TscannerConfig",
  "type": "object",
  "properties": {
    "builtinRules": {
      "type": "object",
      "additionalProperties": { "$ref": "#/definitions/BuiltinRuleConfig" }
    },
    "customRules": {
      "type": "object",
      "additionalProperties": { "$ref": "#/definitions/CustomRuleConfig" }
    },
    "include": {
      "type": "array",
      "items": { "type": "string" }
    },
    "exclude": {
      "type": "array",
      "items": { "type": "string" }
    }
  }
}
```

**Usage**:
- Users reference schema in config: `"$schema": "https://unpkg.com/tscanner@0.0.20/schema.json"`
- Provides autocomplete in VSCode/IntelliJ
- Validates config structure before scanning

### Config Loading

Config loading is handled by **Rust binary**, not CLI wrapper:

1. CLI passes `--config` flag (optional)
2. Rust binary loads `.tscanner/config.jsonc`
3. Rust binary validates against schema
4. Rust binary applies rules during scan

**CLI Responsibilities**:
- None (no config parsing in wrapper)
- Only forwards `--config` argument

**Rust Binary Responsibilities**:
- Load and parse JSON/JSONC
- Validate against schema
- Apply include/exclude patterns
- Enable/disable rules based on config

## Entry Points and Command Flow

### Entry Point

**Binary**: `tscanner` (defined in `package.json` bin field)

**Execution Flow**:

```
User Terminal
    ↓
$ tscanner check --branch main
    ↓
Node.js (/usr/bin/env node)
    ↓
dist/main.js (CLI wrapper)
    ↓
getBinaryPath() → Resolve Rust binary
    ↓
spawn(binary, ['check', '--branch', 'main'])
    ↓
Rust Binary (crates/cli/src/main.rs)
    ↓
clap::Parser::parse()
    ↓
Commands::Check { branch: Some("main"), ... }
    ↓
cmd_check() → Core Scanner
    ↓
Output to stdout/stderr
    ↓
Exit with code (0 or 1)
```

### Command Handling

**Rust Binary Commands** (defined in `crates/cli/src/main.rs`):

1. **`tscanner check [PATH]`**
   - Scan files for issues
   - Flags: `--no-cache`, `--json`, `--pretty`, `--by-rule`, `--branch`, `--file`, `--rule`, `--continue-on-error`, `--config`
   - Default path: `.` (current directory)

2. **`tscanner rules [PATH]`**
   - List all available rules with metadata
   - Flags: `--config`
   - Shows: name, description, enabled status, severity

3. **`tscanner init [PATH]`**
   - Create default `.tscanner/config.jsonc`
   - No additional flags
   - Writes default config to disk

**CLI Wrapper Role**:
- Transparent pass-through
- No command parsing
- No argument validation
- All logic in Rust binary

### Exit Codes

| Code | Meaning | Triggered By |
|------|---------|--------------|
| 0 | Success (no errors found) | Clean scan with no errors |
| 1 | Errors found or invalid config | Scan found errors, missing config, invalid args |

**Exit Code Propagation**:
```typescript
child.on('exit', (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
  } else {
    process.exit(code || 0);  // Forward exit code
  }
});
```

## Important Implementation Details

### 1. Platform Binary Distribution

**Problem**: Single npm package must support 5 platforms

**Solution**: Optional dependencies

```json
{
  "optionalDependencies": {
    "@tscanner/cli-darwin-arm64": "0.0.20",
    "@tscanner/cli-darwin-x64": "0.0.20",
    "@tscanner/cli-linux-arm64": "0.0.20",
    "@tscanner/cli-linux-x64": "0.0.20",
    "@tscanner/cli-win32-x64": "0.0.20"
  }
}
```

**Behavior**:
- npm installs **only the matching platform package**
- Other platforms are silently skipped
- Each platform package has `os` and `cpu` constraints
- Example: On macOS ARM64, only `@tscanner/cli-darwin-arm64` installs

**Platform Package Structure**:

```json
{
  "name": "@tscanner/cli-darwin-arm64",
  "version": "0.0.20",
  "os": ["darwin"],
  "cpu": ["arm64"],
  "main": "tscanner",
  "files": ["tscanner"]
}
```

### 2. Multi-Environment Binary Resolution

**3 Environments Supported**:

1. **Development** (monorepo):
   - Path: `packages/cli/npm/cli-{platform}/tscanner`
   - Used when developing in monorepo
   - Binary copied by build script

2. **CI** (GitHub Actions):
   - Path: `packages/cli/npm/{platform}/tscanner`
   - Used during automated builds
   - Binary placed by CI build step

3. **Production** (installed package):
   - Path: `node_modules/@tscanner/cli-{platform}/tscanner`
   - Used after `npm install tscanner`
   - Binary installed via optional dependencies

**Why Multiple Paths?**:
- Different build/publish workflows
- Monorepo vs standalone package
- Local development vs production use

### 3. Executable Permissions (Unix)

**Problem**: Binaries downloaded from npm are not executable

**Solution**: `postinstall.js` hook

```javascript
if (process.platform !== 'win32') {
  try {
    chmodSync(binaryPath, 0o755);
  } catch (_e) {
    // Ignore errors (file might not exist or already executable)
  }
}
```

**Why Necessary**:
- npm does not preserve executable permissions
- Unix requires `+x` flag to execute files
- Windows uses `.exe` extension (no chmod needed)

### 4. Signal Handling for Graceful Shutdown

**Problem**: User presses Ctrl+C, need to terminate child gracefully

**Solution**: Forward signals to child process

```typescript
process.on('SIGINT', () => {
  child.kill('SIGINT');
  child.kill('SIGTERM');  // Also send TERM for redundancy
});

process.on('SIGTERM', () => {
  child.kill('SIGTERM');
});
```

**Why Both SIGINT and SIGTERM?**:
- SIGINT: User-initiated interrupt (Ctrl+C)
- SIGTERM: System-initiated termination
- Sending both ensures child receives termination signal

### 5. TypeScript Path Mapping

**tsconfig.json** includes path mapping for monorepo:

```json
{
  "paths": {
    "tscanner-common": ["../../shared/tscanner-common/src/index.ts"]
  }
}
```

**Purpose**:
- Import shared utilities during development
- Resolves to `tscanner-common` package in production
- Enables type checking across monorepo

### 6. Build Process

**Commands**:

```bash
# Build CLI wrapper
pnpm run build  # Runs tsup → dist/main.js

# Development run
pnpm run dev    # Runs tsx src/main.ts

# Type check
pnpm run typecheck  # Runs tsc --noEmit
```

**Build Output**:
- Input: `src/main.ts` (TypeScript)
- Output: `dist/main.js` (CommonJS)
- No bundling of Rust binary (kept separate)

**Published Files** (from `package.json` files field):
```json
{
  "files": ["dist", "schema.json", "postinstall.js"]
}
```

### 7. Stdio Inheritance

**Key Decision**: Use `stdio: 'inherit'` instead of `'pipe'`

```typescript
spawn(binaryPath, args, {
  stdio: 'inherit',  // Forward stdout/stderr/stdin directly
  windowsHide: true,
});
```

**Benefits**:
- No intermediate buffering
- Preserves colors and formatting
- Real-time output (no lag)
- Lower memory usage
- Simpler code (no stream handling)

**Tradeoff**:
- Cannot intercept or modify output
- Cannot implement custom formatting
- Acceptable for CLI (terminal output expected)

### 8. No Caching in Wrapper

**Design Choice**: CLI wrapper has **zero** business logic

**Caching is Rust Binary's Job**:
- Memory cache: DashMap concurrent hash map
- Disk cache: `~/.cache/tscanner/cache_{hash}.json`
- Cache invalidation: file mtime + config hash
- Cache control: `--no-cache` flag passed to Rust

**CLI Wrapper**:
- Does not parse `--no-cache`
- Does not manage cache
- Transparently forwards flag to Rust

## Common Workflows

### 1. Install and Initialize

```bash
# Install globally
npm install -g tscanner

# Creates .tscanner/config.jsonc
tscanner init

# Edit config with your editor
code .tscanner/config.jsonc
```

### 2. Scan Current Directory

```bash
# Basic scan
tscanner check

# Skip cache (force rescan)
tscanner check --no-cache
```

### 3. Scan Changed Files (Git-aware)

```bash
# Only scan files changed vs main branch
tscanner check --branch origin/main

# Useful for pre-push hooks
git diff --name-only origin/main
tscanner check --branch origin/main
```

### 4. CI/CD Integration

```yaml
# .github/workflows/quality.yml
- name: Setup Node
  uses: actions/setup-node@v4
  with:
    node-version: '20'

- name: Install TScanner
  run: npm install -g tscanner

- name: Run scan
  run: tscanner check --json > scan-results.json

- name: Upload results
  uses: actions/upload-artifact@v4
  with:
    name: scan-results
    path: scan-results.json
```

### 5. Pre-commit Hook

```bash
#!/bin/sh
# .git/hooks/pre-commit

if command -v tscanner &> /dev/null && [ -f .tscanner/config.jsonc ]; then
  echo "Running TScanner..."
  tscanner check --no-cache
  if [ $? -ne 0 ]; then
    echo "TScanner found issues. Commit aborted."
    exit 1
  fi
fi
```

### 6. Filter Results

```bash
# Only show specific rule
tscanner check --rule no-console-log

# Only check specific files
tscanner check --file "src/**/*.ts"

# Combine filters
tscanner check --branch main --file "src/**" --rule no-any-type
```

### 7. JSON Output for Parsing

```bash
# Output as JSON
tscanner check --json > results.json

# Pretty-print with jq
tscanner check --json | jq '.files[] | select(.issues | length > 0)'

# Count total issues
tscanner check --json | jq '.total_issues'
```

## Comparison with Other Packages

### vs VSCode Extension

| Feature | CLI | VSCode Extension |
|---------|-----|------------------|
| Binary Communication | Direct spawn | JSON-RPC over stdio |
| Execution Model | One-shot | Long-running server |
| File Watching | None | Rust server watches |
| Output Format | Text/JSON | TreeView UI |
| Caching | Rust handles | Rust handles |
| Git Integration | `--branch` flag | Git diff in extension |
| Use Case | Terminal/CI | IDE integration |

### vs GitHub Action

| Feature | CLI | GitHub Action |
|---------|-----|---------------|
| Core | Rust binary | Wraps CLI |
| Install | `npm install -g` | Uses CLI via npm |
| Output | stdout/stderr | PR comments |
| Configuration | `.tscanner/config.jsonc` | Same config |
| Exit Code | 0/1 | Sets workflow status |

## Performance Characteristics

### Startup Time

- **Node.js Wrapper**: ~50ms (spawn overhead)
- **Rust Binary**: ~10-50ms (load + parse)
- **Total**: ~60-100ms before scanning starts

### Scan Performance

**Controlled by Rust Binary** (not wrapper):
- 100-500 files: <1 second
- Parallel processing with Rayon
- Cache hit rate: 80-95%
- Memory usage: ~50-200MB

### Build Size

- **Node.js Wrapper**: ~15 KB (bundled)
- **Rust Binary**: ~5-8 MB (per platform)
- **Total Package**: ~6 MB (one binary + wrapper)

## Error Handling

### Unsupported Platform

```
Error: Unsupported platform: freebsd-x64
tscanner is only available for:
  - Linux (x64, arm64)
  - macOS (x64, arm64)
  - Windows (x64)
```

### Missing Binary

```
Error: Failed to find TScanner binary for linux-x64
Please try reinstalling: npm install tscanner
Error: Cannot find module '@tscanner/cli-linux-x64'
```

### Invalid Arguments

```
error: unexpected argument '--invalid-flag' found
Usage: tscanner <COMMAND>
For more information, try '--help'.
```

### Config Errors

```
Error: Configuration file not found: .tscanner/config.jsonc
Run 'tscanner init' to create a default configuration.
```

## Troubleshooting

### Binary Not Found

**Problem**: `ENOENT: no such file or directory`

**Solutions**:
1. Reinstall: `npm install -g tscanner`
2. Check platform: `node -p "process.platform-process.arch"`
3. Verify optional dependency installed: `npm ls @tscanner/cli-*`

### Permission Denied

**Problem**: `EACCES: permission denied`

**Solutions**:
1. Run `chmod +x` manually: `chmod +x $(which tscanner)`
2. Reinstall (triggers postinstall): `npm install -g tscanner`
3. Check postinstall ran: `npm config get ignore-scripts` (should be false)

### Slow Scans

**Problem**: Scans taking longer than expected

**Solutions**:
1. Check cache: `ls ~/.cache/tscanner/`
2. Force cache rebuild: `tscanner check --no-cache`
3. Reduce file count: Add exclusions to config
4. Check disk I/O: Cache might be on slow drive

## Future Enhancements

1. **Watch Mode**: Add `--watch` flag for continuous scanning
2. **Custom Binary Path**: Allow `TSCANNER_BINARY_PATH` env var
3. **Verbose Mode**: Add `--verbose` for debugging
4. **Progress Indicator**: Show progress during long scans
5. **Config Validation**: Add `tscanner validate-config` command

## Summary

The TScanner CLI package is a minimal, high-performance wrapper that:

1. **Detects Platform**: Identifies OS/architecture
2. **Resolves Binary**: Finds correct Rust executable
3. **Spawns Process**: Executes with inherited stdio
4. **Forwards Signals**: Handles Ctrl+C gracefully
5. **Propagates Exit Code**: Returns scan result status

The wrapper adds ~60-100ms overhead but provides:
- Cross-platform compatibility (5 platforms)
- Zero-config npm installation
- Standard npm package distribution
- Seamless integration with Node.js ecosystem

All business logic, caching, parsing, and scanning is handled by the Rust binary, keeping the wrapper thin and maintainable.
