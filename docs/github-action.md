# TScanner GitHub Action - Technical Documentation

## Overview

The TScanner GitHub Action is a CI/CD integration package that automatically scans TypeScript/JavaScript code in pull requests and posts detailed code quality reports as PR comments. It acts as a wrapper around the TScanner CLI, executing scans and presenting results in an easily digestible format directly within GitHub's pull request interface.

**Key Capabilities:**
- Automated code scanning triggered by PR events
- Smart PR comment management (creates/updates a single comment)
- Dual scan modes: full codebase or branch-diff only
- Dual grouping modes: by file or by rule
- Flexible error handling (block PR or continue with warnings)
- Timezone-aware timestamps
- Clickable file links pointing to exact issue locations in PR diff

## Package Structure

```
packages/github-action/
├── src/
│   ├── index.ts                      # Main entry point and orchestration
│   ├── constants.ts                  # Shared constants and re-exports
│   │
│   ├── core/                         # Core business logic
│   │   ├── input-validator.ts        # Action input parsing and validation
│   │   ├── cli-executor.ts           # TScanner CLI execution abstraction
│   │   ├── scanner.ts                # Scan orchestration and result parsing
│   │   └── comment-updater.ts        # PR comment generation and management
│   │
│   ├── lib/                          # GitHub integration helpers
│   │   ├── actions-helper.ts         # GitHub Actions API wrapper
│   │   └── git-helper.ts             # Git operations wrapper
│   │
│   └── utils/                        # Utility functions
│       ├── config-validator.ts       # Configuration file validation
│       ├── format-timestamp.ts       # Timezone-aware timestamp formatting
│       ├── pluralize.ts              # Text pluralization helper
│       └── url-builder.ts            # GitHub PR file URL generation
│
├── action.yml                        # GitHub Action manifest (inputs/outputs)
├── package.json                      # Package metadata and dependencies
├── tsconfig.json                     # TypeScript configuration
└── tsup.config.ts                    # Build configuration (bundles to dist/index.js)
```

## Code Organization

### Entry Point (`index.ts`)

The `ActionRunner` class orchestrates the entire action flow:

1. **Input Validation**: Parses and validates action inputs via `getActionInputs()`
2. **Config Validation**: Verifies TScanner config file exists at specified path
3. **PR Context Check**: For branch mode, validates this is running in a PR context
4. **Scan Execution**: Executes scan via `executeScan()` method
5. **Comment Management**: Updates or creates PR comment with results
6. **Error Handling**: Reports final status (success/failure) based on scan results and `continue-on-error` setting

**Key Methods:**
- `run()`: Main orchestration method
- `executeScan()`: Determines scan mode and calls scanner
- `handlePRComment()`: Fetches commit metadata and updates PR comment
- `getCommitMessageFromApi()`: Retrieves commit message from GitHub API
- `handleScanResults()`: Logs results and sets action status

### Core Modules

#### 1. Input Validator (`core/input-validator.ts`)

Handles action input parsing and validation using Zod schemas.

**Input Schema Structure:**
```typescript
type ActionInputs = {
  token: string;
  timezone: string;
  configPath: string;
  tscannerVersion: string;
  groupBy: 'file' | 'rule';
  continueOnError: boolean;
  devMode: boolean;
  mode: 'branch' | 'codebase';
  targetBranch?: string; // Required only for branch mode
};
```

**Validation Strategy:**
- Uses discriminated union schema (`mode` field discriminates between branch/codebase)
- Branch mode schema requires `targetBranch` property
- Codebase mode schema omits `targetBranch` property
- Automatic mode detection: if `target-branch` input provided → branch mode, else → codebase mode
- Default values provided for optional inputs

**Input Parsing:**
```typescript
export function getActionInputs(): ActionInputs {
  const token = githubHelper.getInput('github-token', { required: true });
  const targetBranch = githubHelper.getInput('target-branch');
  const mode = targetBranch ? ScanMode.Branch : ScanMode.Codebase;
  // ... parse other inputs
  return actionInputsSchema.parse(rawInputs); // Throws if validation fails
}
```

#### 2. CLI Executor (`core/cli-executor.ts`)

Provides abstraction layer for executing TScanner CLI in two modes: dev mode (local monorepo) and prod mode (published npm package).

**Executor Interface:**
```typescript
type CliExecutor = {
  execute: (args: string[]) => Promise<string>;  // Returns JSON output
  displayResults: (args: string[]) => Promise<unknown>;  // Displays pretty output
};
```

**Dev Mode Executor:**
- Used when `dev-mode: 'true'` input provided
- Points to local CLI at `$GITHUB_WORKSPACE/packages/cli/dist/main.js`
- Executes via `node` command
- Purpose: Testing action changes during monorepo development

**Prod Mode Executor:**
- Default execution mode
- Downloads and runs published TScanner CLI from npm
- Uses `npx <package-name>@<version>` for execution
- Version specified via `tscanner-version` input (default: 'latest')

**Execution Pattern:**
```typescript
const executor = devMode ? createDevModeExecutor() : createProdModeExecutor(version);

// Get JSON output (silent execution)
const output = await executor.execute(['check', '--json', ...args]);

// Display pretty output to GitHub Actions log
await executor.displayResults(['check', '--pretty', ...args]);
```

#### 3. Scanner (`core/scanner.ts`)

Orchestrates the scanning process and transforms CLI JSON output into structured TypeScript data.

**Scan Flow:**

1. **CLI Invocation**: Executes TScanner CLI twice in parallel:
   - Once grouped by file (`--json`)
   - Once grouped by rule (`--json --by-rule`)

2. **Output Parsing**: Parses JSON output into typed structures:
   ```typescript
   type CliJsonOutputByFile = {
     files: Array<{ file: string; issues: Issue[] }>;
     summary: { total_files, total_issues, errors, warnings };
   };

   type CliJsonOutputByRule = {
     rules: Array<{ rule: string; count: number; issues: Issue[] }>;
     summary: { total_files, total_issues, errors, warnings };
   };
   ```

3. **Data Transformation**: Converts CLI output to internal format:
   - Groups issues by file and by rule
   - Sorts groups by severity (errors first) then issue count (descending)
   - Extracts file paths, line numbers, column numbers, rule names, line text

4. **Pretty Output Display**: Shows formatted scan results in GitHub Actions logs

**Scan Options:**
```typescript
type ScanOptions = {
  targetBranch?: string;        // If provided, enables branch mode
  devMode: boolean;             // Use local CLI vs npm package
  tscannerVersion: string;      // CLI version to install
  groupBy: GroupMode;           // Primary grouping preference
  configPath: string;           // Path to config directory
};
```

**Result Structure:**
```typescript
type ScanResult = {
  totalIssues: number;
  totalErrors: number;
  totalWarnings: number;
  totalFiles: number;
  totalRules: number;
  groupBy: GroupMode;
  ruleGroups: RuleGroup[];          // Grouped by file
  ruleGroupsByRule: RuleGroup[];    // Grouped by rule
};

type RuleGroup = {
  ruleName: string;       // File path or rule name depending on grouping
  severity: Severity;     // Error or Warning
  issueCount: number;
  fileCount: number;
  files: FileIssues[];
};

type FileIssues = {
  filePath: string;
  issues: Issue[];
};

type Issue = {
  line: number;
  column: number;
  message: string;
  lineText: string;
  ruleName?: string;
};
```

#### 4. Comment Updater (`core/comment-updater.ts`)

Generates and manages the PR comment that displays scan results.

**Comment Management Flow:**

1. **Build Comment Body**: Generates markdown comment with:
   - Header with icon and status (✅ No Issues / ❌ Errors / ⚠️ Warnings)
   - Summary statistics (issues, mode, target branch)
   - Collapsible sections for both groupings (by rule and by file)
   - Clickable file links to exact PR diff locations
   - Timestamp with timezone
   - Commit SHA and message

2. **Find Existing Comment**: Searches PR comments for previous bot comment using marker
   - Marker: `<!-- tscanner-pr-comment -->`
   - Filters for Bot user type

3. **Update or Create**:
   - If existing comment found → updates via GitHub API
   - If no comment found → creates new comment

**Comment Structure:**

```markdown
<!-- tscanner-pr-comment -->
## ❌ TScanner - Errors Found

**Issues:** 15 (12 errors, 3 warnings)
**Mode:** branch (origin/main)

---

<div align="center">
  <details>
    <summary><strong>📋 Issues grouped by rule (5)</strong></summary>
    <!-- Nested details for each rule -->
  </details>
</div>

---

<div align="center">
  <details>
    <summary><strong>📁 Issues grouped by file (8)</strong></summary>
    <!-- Nested details for each file -->
  </details>
</div>

---
**Last updated:** 01/27/2025, 14:23:45 (UTC-3)
**Last commit analyzed:** `abc1234` - fix: update validation logic
```

**URL Generation:**

File links point to exact line in PR diff view:
```typescript
// URL format: https://github.com/{owner}/{repo}/pull/{pr}/files#diff-{hash}R{line}
function buildPrFileUrl(owner, repo, prNumber, filePath, line): string {
  const fileHash = createHash('sha256').update(filePath).digest('hex');
  return `https://github.com/${owner}/${repo}/pull/${prNumber}/files#diff-${fileHash}R${line}`;
}
```

**HTML Safety:**

All code snippets (line text) are escaped to prevent XSS:
```typescript
function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}
```

### Library Modules

#### 1. Actions Helper (`lib/actions-helper.ts`)

Wrapper class around GitHub Actions SDK providing simplified API for common operations.

**GitHub Actions SDK Integrations:**
- `@actions/core`: Logging, input parsing, action status
- `@actions/exec`: Command execution
- `@actions/github`: Octokit client and context

**Methods:**

| Method | Purpose |
|--------|---------|
| `getOctokit(token)` | Creates authenticated GitHub API client |
| `getContext()` | Returns current GitHub Actions context |
| `logInfo/Warning/Error/Debug(msg)` | Structured logging to Actions console |
| `setFailed(msg)` | Marks action as failed with message |
| `getInput(name, opts)` | Retrieves action input value |
| `execCommand(cmd, args, opts)` | Executes shell command |
| `execCommandWithOutput(cmd, args)` | Executes command and captures output |

**Usage Pattern:**
```typescript
// Get inputs
const token = githubHelper.getInput('github-token', { required: true });

// Execute commands
await githubHelper.execCommand('git', ['fetch', 'origin', 'main']);

// Log messages
githubHelper.logInfo('Scanning started...');
githubHelper.logError('Scan failed');

// Access GitHub context
const { owner, repo } = githubHelper.getContext().repo;
const prNumber = githubHelper.getContext().payload.pull_request?.number;

// Create GitHub API client
const octokit = githubHelper.getOctokit(token);
await octokit.rest.issues.createComment({ owner, repo, issue_number: prNumber, body });
```

#### 2. Git Helper (`lib/git-helper.ts`)

Provides Git-specific operations used by the scanner.

**Methods:**

```typescript
class GitHelper {
  // Fetches target branch from remote (required for branch mode)
  async fetchBranch(targetBranch: string): Promise<void> {
    const branchName = targetBranch.replace('origin/', '');
    await githubHelper.execCommand('git', ['fetch', 'origin', branchName]);
  }

  // Retrieves commit message for given SHA
  async getCommitMessage(commitSha: string): Promise<string> {
    const message = await githubHelper.execCommandWithOutput(
      'git',
      ['log', '-1', '--pretty=%s', commitSha]
    );
    return message || '';
  }
}
```

**Usage:**
```typescript
// In branch mode, fetch target branch before scanning
await gitHelper.fetchBranch('origin/main');

// Get commit message for PR comment
const message = await gitHelper.getCommitMessage('abc123def');
```

### Utility Modules

#### 1. Config Validator (`utils/config-validator.ts`)

Validates TScanner configuration file existence.

```typescript
export function validateConfigFiles(configPath: string): void {
  const configFilePath = path.join(configPath, CONFIG_FILE_NAME); // config.jsonc

  if (fs.existsSync(configFilePath)) {
    githubHelper.logInfo(`Using config.jsonc from ${configPath}`);
  } else {
    githubHelper.logWarning(`No config file found. Using defaults.`);
  }
}
```

**Note:** This is a soft validation that only warns if config is missing. The CLI will use its default configuration if no file exists.

#### 2. Format Timestamp (`utils/format-timestamp.ts`)

Generates timezone-aware timestamps for PR comments.

```typescript
export function formatTimestamp(timezone: string): string {
  const now = new Date();

  // Format: MM/DD/YYYY, HH:MM:SS (UTC±offset)
  const formatted = now.toLocaleString('en-US', {
    timeZone: timezone,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });

  const offset = getTimezoneOffset(timezone); // e.g., "+3" or "-5"
  return `${formatted} (UTC${offset})`;
}
```

**Examples:**
- `timezone: 'UTC'` → `01/27/2025, 17:23:45 (UTC)`
- `timezone: 'America/Sao_Paulo'` → `01/27/2025, 14:23:45 (UTC-3)`
- `timezone: 'Asia/Tokyo'` → `01/28/2025, 02:23:45 (UTC+9)`

#### 3. Pluralize (`utils/pluralize.ts`)

Simple pluralization helper for English words.

```typescript
export function pluralize(count: number, singular: string): string {
  return count === 1 ? singular : `${singular}s`;
}
```

**Usage:**
```typescript
pluralize(1, 'error')  // "error"
pluralize(5, 'error')  // "errors"
pluralize(0, 'issue')  // "issues"
```

#### 4. URL Builder (`utils/url-builder.ts`)

Generates GitHub PR file URLs with line anchors.

```typescript
export function buildPrFileUrl(
  owner: string,
  repo: string,
  prNumber: number,
  filePath: string,
  line: number
): string {
  const fileHash = createFileHash(filePath); // SHA-256 hash
  return `https://github.com/${owner}/${repo}/pull/${prNumber}/files#diff-${fileHash}R${line}`;
}

function createFileHash(filePath: string): string {
  return createHash('sha256').update(filePath).digest('hex');
}
```

**URL Format:**
- Base: `https://github.com/owner/repo/pull/123/files`
- Fragment: `#diff-{sha256-hash}R{line-number}`
- The `R` prefix indicates right-side (new code) in diff view

## External Dependencies

### Production Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `@actions/core` | ^1.11.1 | GitHub Actions SDK - inputs, outputs, logging, status |
| `@actions/exec` | ^1.1.1 | GitHub Actions SDK - command execution |
| `@actions/github` | ^6.0.0 | GitHub Actions SDK - Octokit client, context |
| `tscanner-common` | workspace:* | Shared types and constants from monorepo |
| `zod` | ^4.1.12 | Runtime type validation for action inputs |

### Development Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `@types/node` | ^22.10.2 | Node.js type definitions |
| `tsup` | ^8.5.1 | TypeScript bundler (builds to single dist/index.js) |
| `tsx` | ^4.20.6 | TypeScript execution for scripts |
| `typescript` | ^5.7.2 | TypeScript compiler |

### Shared Package (`tscanner-common`)

The action imports shared constants and types from the monorepo's common package:

```typescript
export {
  CONFIG_DIR_NAME,      // '.tscanner'
  CONFIG_FILE_NAME,     // 'config.jsonc'
  PACKAGE_NAME,         // 'tscanner'
  GroupMode,            // 'file' | 'rule'
  ScanMode,             // 'branch' | 'codebase'
  Severity,             // 'error' | 'warning'
  pluralize             // Pluralization utility
} from 'tscanner-common';
```

## Communication with Other Packages

### TScanner CLI Integration

The GitHub Action communicates with the TScanner CLI package via shell execution:

**Installation:**
- Dev Mode: Uses pre-built local CLI at `packages/cli/dist/main.js`
- Prod Mode: Downloads from npm via `npx tscanner@{version}`

**Execution:**
```bash
# Branch mode scan
npx tscanner@latest check --json --continue-on-error --config .tscanner --branch origin/main

# Codebase mode scan
npx tscanner@latest check --json --continue-on-error --config .tscanner

# Additional execution for rule grouping
npx tscanner@latest check --json --continue-on-error --config .tscanner --by-rule

# Display results in GitHub Actions log
npx tscanner@latest check --pretty --config .tscanner
```

**Communication Protocol:**
1. Action invokes CLI with appropriate flags
2. CLI reads config from `.tscanner/config.jsonc`
3. CLI executes scan and writes JSON to stdout
4. Action parses JSON stdout into TypeScript types
5. Action transforms data and generates PR comment

**CLI Output Formats:**

The action relies on two CLI output formats:

1. **By File** (`--json`):
```json
{
  "files": [
    {
      "file": "src/utils/helper.ts",
      "issues": [
        {
          "rule": "no-any-type",
          "severity": "error",
          "line": 42,
          "column": 15,
          "message": "Detected usage of 'any' type",
          "line_text": "const data: any = fetchData();"
        }
      ]
    }
  ],
  "summary": {
    "total_files": 8,
    "total_issues": 15,
    "errors": 12,
    "warnings": 3
  }
}
```

2. **By Rule** (`--json --by-rule`):
```json
{
  "rules": [
    {
      "rule": "no-any-type",
      "count": 5,
      "issues": [
        {
          "file": "src/utils/helper.ts",
          "line": 42,
          "column": 15,
          "message": "Detected usage of 'any' type",
          "severity": "error",
          "line_text": "const data: any = fetchData();"
        }
      ]
    }
  ],
  "summary": {
    "total_files": 8,
    "total_issues": 15,
    "errors": 12,
    "warnings": 3
  }
}
```

### TScanner Core Integration

The action does NOT directly communicate with the Rust core. All interaction happens through the CLI:

```
GitHub Action → TScanner CLI → TScanner Core (Rust)
                     ↓
              JSON output via stdout
```

The CLI acts as the bridge between the Node.js action runtime and the Rust scanning engine.

## Action Inputs and Outputs

### Input Configuration (`action.yml`)

```yaml
inputs:
  github-token:
    description: 'GitHub token for posting comments'
    required: true

  target-branch:
    description: 'Target branch to compare against (only for branch mode)'
    required: false

  timezone:
    description: 'Timezone for timestamps (e.g., America/Sao_Paulo)'
    required: false
    default: 'UTC'

  config-path:
    description: 'Path to directory containing config.jsonc'
    required: false
    default: '.tscanner'

  tscanner-version:
    description: 'TScanner version to use (e.g., "1.0.0", "latest")'
    required: false
    default: 'latest'

  dev-mode:
    description: 'Use local TScanner CLI from monorepo (internal development only)'
    required: false
    default: 'false'

  group-by:
    description: 'Group issues by "rule" or "file"'
    required: false
    default: 'file'

  continue-on-error:
    description: 'Continue execution even when errors are found'
    required: false
    default: 'false'
```

### Input Details

| Input | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `github-token` | string | ✅ | - | GitHub token (`${{ secrets.GITHUB_TOKEN }}`) for PR comment posting |
| `target-branch` | string | ❌ | - | Enables branch mode; compares against this branch |
| `timezone` | string | ❌ | `'UTC'` | IANA timezone name for timestamp formatting |
| `config-path` | string | ❌ | `'.tscanner'` | Directory containing `config.jsonc` |
| `tscanner-version` | string | ❌ | `'latest'` | npm version to install (`latest`, `1.0.0`, etc.) |
| `dev-mode` | boolean | ❌ | `false` | Use local monorepo CLI (development only) |
| `group-by` | enum | ❌ | `'file'` | Primary grouping: `'file'` or `'rule'` |
| `continue-on-error` | boolean | ❌ | `false` | Don't fail action on scan errors |

### Outputs

The action does NOT define explicit outputs in `action.yml`. All communication with users happens via:

1. **PR Comments**: Detailed scan results posted as comment
2. **Action Status**: Success/failure reflected in GitHub Actions UI
3. **Action Logs**: Structured logs visible in workflow run details

## Workflow Integration

### Basic Setup

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

### Advanced Configurations

**Branch Mode (Recommended):**
```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    target-branch: 'origin/main'  # Only scan changed files
```

**Custom Config Location:**
```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    config-path: 'config/tscanner'  # Non-standard location
```

**Continue on Errors:**
```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    continue-on-error: 'true'  # Don't fail workflow
```

**Group by Rule:**
```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    group-by: 'rule'  # Primary grouping by rule instead of file
```

**Timezone Configuration:**
```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    timezone: 'America/New_York'  # EDT/EST timestamps
```

**Specific Version:**
```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    tscanner-version: '0.1.5'  # Pin to specific CLI version
```

**Full Configuration:**
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

### Workflow Triggers

**Pull Request Events:**
```yaml
on:
  pull_request:
    types: [opened, synchronize, reopened]
    branches: [main, develop]
```

**Multiple Events:**
```yaml
on:
  pull_request:
    branches: [main]
  push:
    branches: [main]  # Also run on direct pushes to main
```

### Permissions

Required GitHub token permissions:
```yaml
permissions:
  contents: read        # Read repository code
  pull-requests: write  # Post/update PR comments
```

### Complete Example

```yaml
name: Code Quality

on:
  pull_request:
    types: [opened, synchronize, reopened]
    branches: [main, develop]

permissions:
  contents: read
  pull-requests: write

jobs:
  tscanner:
    name: TScanner Analysis
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Full history for accurate git diff

      - name: Run TScanner
        uses: lucasvtiradentes/tscanner-action@v0.0.17
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          target-branch: 'origin/main'
          timezone: 'America/New_York'
          continue-on-error: 'false'
```

## Implementation Details

### Build System

**Build Tool:** tsup (TypeScript bundler)

**Configuration** (`tsup.config.ts`):
```typescript
export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs'],               // CommonJS for Node.js 20 runtime
  outDir: 'dist',
  clean: true,
  minify: true,
  noExternal: [/.*/],            // Bundle all dependencies
});
```

**Output:**
- Single bundled file: `dist/index.js`
- All dependencies (except Node.js built-ins) are inlined
- Minified and optimized for fast startup
- No external npm packages required at runtime

**Build Command:**
```bash
pnpm run build  # → tsup → dist/index.js
```

### TypeScript Configuration

**Compiler Options** (`tsconfig.json`):
```json
{
  "compilerOptions": {
    "target": "ES2022",           // Modern JS features
    "module": "commonjs",         // Required for GitHub Actions runtime
    "lib": ["ES2022"],
    "outDir": "./dist",
    "strict": true,               // Full type safety
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "moduleResolution": "node",
    "baseUrl": ".",
    "paths": {
      "tscanner-common": ["../../shared/tscanner-common/src/index.ts"]
    }
  }
}
```

**Path Mapping:**
- Maps `tscanner-common` to monorepo shared package
- Allows importing shared types/constants during development
- Bundler (tsup) resolves and inlines during build

### Action Runtime

**Runtime Environment:**
- Node.js 20 (specified in `action.yml`: `runs.using: 'node20'`)
- Linux runner (GitHub-hosted or self-hosted)
- Environment variables from GitHub Actions context

**Execution Flow:**

1. GitHub Actions downloads action repository
2. GitHub Actions loads `dist/index.js` with Node.js 20
3. Entry point (`index.ts`) instantiates `ActionRunner`
4. `ActionRunner.run()` executes
5. Results posted to PR, status set, execution completes

**Environment Variables Used:**

| Variable | Source | Purpose |
|----------|--------|---------|
| `GITHUB_WORKSPACE` | Actions | Repository checkout path |
| `GITHUB_TOKEN` | Input | GitHub API authentication |
| `GITHUB_REPOSITORY` | Actions | owner/repo slug |
| `GITHUB_EVENT_PATH` | Actions | Webhook payload JSON |

### Error Handling Strategy

**Validation Errors:**
- Input validation failures → immediate action failure
- Missing required inputs → Zod throws with descriptive message
- Invalid config → warning logged, continues with defaults

**Scan Errors:**
- CLI execution failures → captured and logged
- JSON parse errors → detailed error with partial output logged
- No issues found → success status with positive message

**GitHub API Errors:**
- Comment posting failures → logged but doesn't fail action
- Commit message fetch failures → falls back to empty string
- API rate limiting → retries handled by `@actions/github`

**Continue on Error Mode:**
```typescript
if (scanResult.totalErrors > 0) {
  const loggerMethod = inputs.continueOnError
    ? githubHelper.logInfo
    : githubHelper.setFailed;
  loggerMethod(`Found ${scanResult.totalErrors} error(s)`);
}
```

### Performance Optimizations

**Parallel Execution:**
```typescript
// Execute both grouping modes simultaneously
const [scanOutputFile, scanOutputRule] = await Promise.all([
  executor.execute(argsFile),
  executor.execute(argsRule)
]);
```

**Minimal CLI Installations:**
- Dev mode: No installation, uses pre-built local binary
- Prod mode: `npx` caches downloaded packages across workflow runs

**Efficient Comment Updates:**
- Searches existing comments once
- Updates in-place instead of deleting and recreating
- Reduces API calls and maintains comment history/reactions

**Selective Branch Fetching:**
```typescript
// Only fetch target branch when needed (branch mode)
if (inputs.mode === ScanMode.Branch) {
  await gitHelper.fetchBranch(inputs.targetBranch);
}
```

### Security Considerations

**Input Sanitization:**
- All user inputs validated via Zod schemas
- No direct shell interpolation of user inputs
- Command arguments passed as arrays (not string concatenation)

**HTML Escaping:**
- All code snippets in comments are HTML-escaped
- Prevents XSS injection via malicious code patterns

**Token Handling:**
- GitHub token passed securely via `@actions/github`
- Never logged or exposed in output
- Scoped to minimum required permissions

**Dependency Pinning:**
- Dependencies use caret ranges (^) for semver compatibility
- Build produces single bundled file (no runtime deps)
- Reduces supply chain attack surface

### Comment Marker System

The action uses HTML comments as markers to identify its own comments:

```typescript
const COMMENT_MARKER = '<!-- tscanner-pr-comment -->';
```

**Comment Detection:**
```typescript
const botComment = existingComments.data.find(
  c => c.user?.type === 'Bot' && c.body?.includes(COMMENT_MARKER)
);
```

**Why This Works:**
- HTML comments invisible to users viewing PR
- Persistent across comment edits
- Allows safe identification without database lookups
- Prevents accidental updates to wrong comments
- Filters by Bot user type for additional safety

### Scan Mode Detection

The action automatically determines scan mode based on inputs:

```typescript
const mode = targetBranch ? ScanMode.Branch : ScanMode.Codebase;
```

**Branch Mode:**
- Triggered by: `target-branch` input provided
- Behavior: CLI uses `--branch origin/main` flag
- Git diff: Fetches target branch and compares
- Use case: Only scan files changed in PR

**Codebase Mode:**
- Triggered by: No `target-branch` input
- Behavior: CLI scans all files matching include/exclude patterns
- No git operations: Scans entire workspace
- Use case: Comprehensive codebase validation

### Logging Strategy

**Structured Logging Levels:**

```typescript
// Informational messages (always shown)
githubHelper.logInfo('Scanning started...');

// Warning messages (yellow in Actions UI)
githubHelper.logWarning('No config file found. Using defaults.');

// Error messages (red in Actions UI)
githubHelper.logError('Failed to parse scan output');

// Debug messages (only shown with ACTIONS_STEP_DEBUG=true)
githubHelper.logDebug(`Raw output: ${output.substring(0, 500)}`);
```

**Log Output Examples:**

```
ℹ️ Scanning [changed files vs origin/main] group by: [file]
ℹ️ Using tscanner CLI from npm: tscanner@latest
ℹ️ Scan completed: 15 issues found

📊 Scan Results:

src/utils/helper.ts - 5 issues (3 errors, 2 warnings)
...

ℹ️ Updated existing comment #123456
❌ Found 12 error(s)
```

### Version Compatibility

**Action Version:** `v0.0.17`

**Compatible CLI Versions:**
- `latest` (default, recommended)
- Any version `>=0.1.0` supporting `--json` output format

**Node.js Requirement:**
- Runtime: Node.js 20 (specified in `action.yml`)
- Build: Node.js >=18 (for development)

**GitHub Actions API:**
- `@actions/core`: v1.11.1+
- `@actions/exec`: v1.1.1+
- `@actions/github`: v6.0.0+

### Development Mode

The `dev-mode` input enables testing local CLI changes within the monorepo:

**When to Use:**
- Developing/testing action changes locally
- Testing CLI changes before publishing
- Debugging integration issues

**Setup:**
```yaml
- uses: ./packages/github-action  # Local action
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    dev-mode: 'true'
```

**Behavior:**
- Skips `npx` installation
- Points to `$GITHUB_WORKSPACE/packages/cli/dist/main.js`
- Requires CLI to be pre-built in monorepo

**Important:**
- Only works in monorepo context
- Never use in production workflows
- Published action ignores this flag for external users

### Future Enhancements

Based on code structure, potential areas for enhancement:

1. **Output Variables:**
   - Expose scan results as action outputs
   - Allow downstream jobs to access issue counts

2. **Annotations:**
   - Use GitHub's annotation API for inline file comments
   - Highlight issues directly in diff view

3. **Status Checks:**
   - Create GitHub status check for scan results
   - Allow protecting branches based on scan status

4. **Custom Templates:**
   - Support user-defined comment templates
   - Allow customizing markdown output

5. **Performance Metrics:**
   - Track and report scan execution time
   - Monitor CLI download/installation time

6. **Artifact Upload:**
   - Upload full JSON scan results as workflow artifact
   - Enable historical analysis and trending

## Summary

The TScanner GitHub Action provides seamless CI/CD integration for code quality scanning. It wraps the TScanner CLI, orchestrates scans, and presents results through rich PR comments. The action's architecture emphasizes:

- **Type Safety**: Zod schemas validate all inputs
- **Error Handling**: Graceful degradation with informative messages
- **Performance**: Parallel execution and minimal API calls
- **Security**: Input sanitization and HTML escaping
- **Maintainability**: Clear separation of concerns across modules
- **Developer Experience**: Smart defaults and flexible configuration

The action integrates tightly with GitHub's PR workflow, providing immediate feedback on code quality while maintaining a clean, professional presentation that developers actually want to read.
