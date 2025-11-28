# TScanner VSCode Extension - Technical Documentation

## Overview

TScanner is a real-time TypeScript code quality scanner implemented as a VSCode extension. It provides instant visual feedback for code issues through a sidebar panel, with support for both full codebase scanning and git-aware branch scanning. The extension communicates with a Rust backend server via JSON-RPC for high-performance analysis.

**Key Features:**
- Real-time issue detection with file watchers
- Git-aware branch scanning mode
- Multiple view modes (list/tree, grouped by file/rule)
- Quick navigation between issues (F8/Shift+F8)
- Interactive settings menu for rule management
- Persistent caching for performance

## Architecture

### Technology Stack

- **Frontend**: TypeScript, VSCode Extension API
- **Backend**: Rust server (JSON-RPC over stdin/stdout)
- **Build**: esbuild for bundling
- **Communication**: Line-delimited JSON with GZIP compression

### High-Level Components

```
extension.ts (entry point)
├── IssuesPanelContent (TreeDataProvider)
│   ├── tree-builder.ts (hierarchy construction)
│   └── tree-items.ts (node types)
├── StatusBarManager (status bar UI)
├── Commands (all user actions)
│   ├── public commands (scan, navigation, logs)
│   └── internal commands (view modes, copy, refresh)
├── Settings Menu (configuration UI)
│   ├── manage-rules.ts
│   ├── scan-mode.ts
│   └── config-location.ts
├── Common Libraries
│   ├── scanner.ts (Rust client management)
│   ├── rust-client.ts (JSON-RPC client)
│   ├── config-manager.ts (config CRUD)
│   ├── git-helper.ts (git operations)
│   └── vscode-utils.ts (VSCode API wrappers)
└── File Watcher (real-time updates)
```

## Code Organization

### Directory Structure

```
src/
├── extension.ts                    # Extension activation/deactivation
├── common/                         # Shared utilities and libraries
│   ├── constants.ts               # Extension-level constants
│   ├── scripts-constants.ts       # Build-time constants
│   ├── types.ts                   # Type definitions
│   ├── lib/
│   │   ├── scanner.ts             # Rust binary management
│   │   ├── rust-client.ts         # JSON-RPC client implementation
│   │   ├── config-manager.ts      # Configuration CRUD operations
│   │   └── vscode-utils.ts        # VSCode API wrappers
│   └── utils/
│       ├── logger.ts              # File-based logging
│       ├── git-helper.ts          # Git operations (diff, branches)
│       ├── extension-helper.ts    # Extension path resolution
│       └── issue-comparator.ts    # Filter issues by git ranges
├── status-bar/
│   └── status-bar-manager.ts      # Status bar UI management
├── issues-panel/
│   ├── panel-content.ts           # TreeDataProvider implementation
│   ├── panel-icon.ts              # Badge counter management
│   └── utils/
│       ├── tree-builder.ts        # Build folder hierarchy
│       └── tree-items.ts          # Tree node definitions
├── commands/
│   ├── index.ts                   # Command registration
│   ├── public/                    # User-facing commands
│   │   ├── scan-workspace.ts     # Main scan command
│   │   ├── hard-scan.ts          # Cache-clearing scan
│   │   ├── issue-navigation.ts   # F8/Shift+F8 navigation
│   │   └── show-logs.ts          # Open log file
│   └── internal/                  # UI control commands
│       ├── refresh.ts            # Rescan trigger
│       ├── navigation.ts         # File opening
│       ├── view-mode.ts          # View mode cycling
│       └── copy.ts               # Copy issues to clipboard
└── settings-menu/
    ├── index.ts                   # Settings menu exports
    ├── open-settings-menu.ts      # Main menu UI
    ├── manage-rules.ts            # Rule selection UI
    ├── scan-mode.ts               # Scan mode switcher
    └── config-location.ts         # Config storage location
```

### File Descriptions

#### Core Files

**extension.ts** (210 lines)
- Extension lifecycle (activate/deactivate)
- Component initialization
- File watcher setup for real-time updates
- Initial scan trigger
- Workspace state restoration

**common/constants.ts** (48 lines)
- Dev/prod environment detection
- Command ID generation with dev suffix
- Binary name resolution
- Platform-specific paths

**common/types.ts** (64 lines)
- Re-exports from tscanner-common
- VSCode-specific types (IssueResult with Uri)
- Tree node types (FolderNode, FileNode)

#### Scanner & Communication

**scanner.ts** (187 lines)
- Rust binary path resolution (bundled vs dev builds)
- RustClient lifecycle management
- Scan operations (workspace, file, content)
- Cache clearing
- Error handling and user notifications

**rust-client.ts** (365 lines)
- JSON-RPC client implementation
- Child process management (spawn, stdio)
- GZIP decompression for large responses
- Request/response correlation via ID
- Method definitions:
  - `scan` - Full workspace scan
  - `scanFile` - Single file scan
  - `scanContent` - In-memory content scan
  - `getRulesMetadata` - Fetch available rules
  - `clearCache` - Invalidate Rust cache
  - `formatResults` - Format issues for clipboard

**config-manager.ts** (264 lines)
- Configuration file location resolution:
  - Global (extension storage)
  - Local (.tscanner/config.json)
  - Custom (user-specified directory)
- CRUD operations for all config types
- Auto-managed marker for sync tracking
- Default config generation
- JSONC parsing with error handling

#### Git Integration

**git-helper.ts** (251 lines)
- VSCode Git API integration
- Branch operations:
  - `getCurrentBranch()` - Get active branch
  - `getAllBranches()` - List local and remote branches
  - `branchExists()` - Validate branch name
- File change detection:
  - `getChangedFiles()` - Git diff with caching (30s TTL)
  - `getModifiedLineRanges()` - Parse diff hunks for line ranges
- Cache invalidation for git operations
- File content at specific refs

**issue-comparator.ts** (22 lines)
- Filter issues to modified line ranges
- Used in branch scan mode to show only new issues

#### UI Components

**panel-content.ts** (167 lines)
- Implements `vscode.TreeDataProvider<PanelContentItem>`
- View mode state (list/tree)
- Group mode state (default/rule)
- Result storage and updates
- Tree structure generation:
  - By file: flat list or folder hierarchy
  - By rule: grouped by rule name, then files
- Event emitters for tree refresh

**panel-icon.ts** (15 lines)
- Badge count management
- Updates on result changes

**tree-builder.ts** (83 lines)
- Builds folder hierarchy from flat issue list
- Converts absolute paths to workspace-relative
- Handles nested folder structures
- Issue counting for folder nodes

**tree-items.ts** (64 lines)
- Tree item definitions:
  - `RuleGroupItem` - Rule name + issue count
  - `FolderResultItem` - Folder with child count
  - `FileResultItem` - File with issue count
  - `LineResultItem` - Individual issue (clickable)
- Icons and context values for each type

**status-bar-manager.ts** (61 lines)
- Status bar item creation
- Dynamic text updates:
  - Scan mode display (Codebase vs Branch)
  - Config status indicator
  - Custom config directory label
- Click handler to open settings menu

#### Commands

**scan-workspace.ts** (153 lines)
- Main scanning logic
- Config validation and loading
- Branch existence check
- Silent mode support for automatic scans
- Result serialization to workspace state
- Tree expansion in tree view mode
- Filtering by scan mode (workspace vs branch)

**hard-scan.ts** (39 lines)
- Cache clearing
- Git cache invalidation
- Rescan trigger

**issue-navigation.ts** (66 lines)
- Next/previous issue navigation
- Index tracking with wraparound
- Cursor positioning and viewport centering
- Status bar feedback

**show-logs.ts** (15 lines)
- Opens log file in editor

**view-mode.ts** (69 lines)
- Cycles through 4 view states:
  - List + Group by file
  - Tree + Group by file
  - List + Group by rule
  - Tree + Group by rule
- Updates workspace state and context keys

**copy.ts** (190 lines)
- Copy issues to clipboard with formatting
- Three copy modes:
  - By rule: all issues for a specific rule
  - By file: all issues in a file
  - By folder: all issues in folder tree
- Calls Rust `formatResults` RPC method
- Includes scan mode context and CLI command

#### Settings Menu

**open-settings-menu.ts** (115 lines)
- Main settings menu UI
- Options:
  - Manage Rules
  - Manage Scan Mode
  - Manage Config Location
  - Open Config File
- Conditional option display based on config state

**manage-rules.ts** (271 lines)
- Fetches rules metadata from Rust server
- Multi-select quick pick UI
- Rule categorization with icons:
  - Type Safety (shield)
  - Variables (symbol-variable)
  - Imports (package)
  - Code Quality (beaker)
  - Bug Prevention (bug)
  - Style (symbol-color)
  - Performance (dashboard)
- Custom rule support (regex, script, AI)
- Config save with location selection
- Automatic rescan after save

**scan-mode.ts** (183 lines)
- Switch between Codebase and Branch modes
- Branch selection UI:
  - Local branches
  - Remote branches (origin/*)
  - Current branch exclusion
- Compare branch persistence
- Cache invalidation on mode change
- Automatic rescan

**config-location.ts** (516 lines)
- Config location management:
  - Extension Storage (global)
  - Project Folder (.tscanner)
  - Custom Path (user-selected)
- Folder picker UI with navigation
- Config migration between locations
- Overwrite confirmation dialogs
- First-time setup flow

#### Utilities

**logger.ts** (42 lines)
- File-based logging to `$TMPDIR/tscannerlogs.txt`
- UTC-3 timestamp formatting
- Log levels: INFO, ERROR, WARN, DEBUG
- Context tagging

**vscode-utils.ts** (175 lines)
- Workspace state management with validation
- Context key updates
- Command registration wrappers
- Toast message helpers
- Workspace folder access

**extension-helper.ts** (13 lines)
- Extension path resolution (prod vs dev)

## External Dependencies

### Runtime Dependencies

**jsonc-parser** (^3.3.1)
- Parse JSONC (JSON with comments) config files
- Handles syntax errors gracefully

**zod** (^4.1.12)
- Workspace state schema validation
- Type-safe state persistence

### Dev Dependencies

**@types/vscode** (^1.93.0)
- VSCode Extension API type definitions

**@types/node** (^22.0.0)
- Node.js type definitions

**esbuild** (^0.24.0)
- Bundler for extension code
- Minification and tree-shaking

**tsx** (^4.20.6)
- TypeScript execution for build scripts

**typescript** (^5.7.0)
- TypeScript compiler

**tscanner-common** (workspace:*)
- Shared types and constants from monorepo

## Communication with Rust Backend

### JSON-RPC Protocol

The extension communicates with the Rust server using JSON-RPC over stdin/stdout:

**Request Format:**
```json
{
  "id": 1,
  "method": "scan",
  "params": {
    "root": "/path/to/workspace",
    "config": { ... },
    "branch": "main"
  }
}
```

**Response Format (GZIP compressed):**
```
GZIP:H4sIAAAAAAAAA+2da3PbuJLH...base64...
```

After decompression:
```json
{
  "id": 1,
  "result": {
    "files": [
      {
        "file": "/path/to/file.ts",
        "issues": [
          {
            "rule": "no-any-type",
            "line": 42,
            "column": 10,
            "severity": "error",
            "message": "...",
            "line_text": "const x: any = 123"
          }
        ]
      }
    ],
    "total_issues": 1,
    "duration_ms": 150
  }
}
```

### RPC Methods

1. **scan** - Full workspace scan
   - Params: `{ root, config?, branch? }`
   - Returns: `ScanResult` with files and issues
   - Used by: scan-workspace.ts

2. **scanFile** - Single file scan
   - Params: `{ root, file }`
   - Returns: `FileResult` with issues
   - Used by: file watcher updates

3. **scanContent** - In-memory scan
   - Params: `{ root, file, content, config? }`
   - Returns: `FileResult` with issues
   - Used by: file watcher for modified files

4. **getRulesMetadata** - Get available rules
   - Params: `{}`
   - Returns: `RuleMetadata[]`
   - Used by: manage-rules.ts

5. **clearCache** - Clear Rust-side cache
   - Params: `{}`
   - Returns: `undefined`
   - Used by: hard-scan.ts

6. **formatResults** - Format issues for clipboard
   - Params: `{ root, results, group_mode }`
   - Returns: `{ output, summary }`
   - Used by: copy.ts

### Performance Optimizations

**GZIP Compression:**
- Rust compresses large responses (>10KB typically)
- Marker-based protocol: `GZIP:{base64-data}`
- Client detects marker and decompresses

**Caching:**
- Rust maintains file cache with mtime + config hash
- Extension caches git diff results (30s TTL)
- Workspace state persists scan results

**Incremental Updates:**
- File watcher triggers single-file scans
- Results merged into existing dataset
- Branch mode filters updates by git diff

## Extension Activation

### Activation Flow

1. **Environment Detection**
   - Check for duplicate activation
   - Resolve workspace folder
   - Initialize logging

2. **State Restoration**
   - Load view mode from workspace state
   - Load group mode from workspace state
   - Load scan mode from workspace state
   - Load compare branch from workspace state
   - Load custom config directory from workspace state

3. **Component Initialization**
   - Create `IssuesPanelContent` (TreeDataProvider)
   - Create tree view with panel content
   - Create `IssuesPanelIcon` for badge
   - Create `StatusBarManager`
   - Register all commands

4. **File Watcher Setup**
   - Watch `**/*.{ts,tsx,js,jsx}`
   - Handle create/change/delete events
   - Filter by scan mode (branch mode checks git diff)
   - Update results incrementally

5. **Initial Scan**
   - Delayed 2 seconds after activation
   - Silent mode (no error toasts)
   - Uses cached results if available

### Context Keys

Set on activation for conditional menu visibility:

- `tscannerViewMode` - Current view mode (list/tree)
- `tscannerGroupMode` - Current group mode (default/rule)
- `tscannerScanMode` - Current scan mode (codebase/branch)
- `tscannerSearching` - True during active scan

### Tree Item Context Values

Used for context menu filtering:

- `TscannerNodeRuleGroup` - Rule group items (copy rule issues)
- `TscannerNodeFolder` - Folder items (copy folder issues)
- `TscannerNodeFile` - File items (copy file issues, copy path)
- `TscannerNodeIssue` - Issue items (not used in menus)

## Settings Menu System

### Menu Hierarchy

```
Main Menu
├── Manage Rules → manage-rules.ts
├── Manage Scan Mode → scan-mode.ts
├── Manage Config Location → config-location.ts
└── Open Config File → config-location.ts
```

### Manage Rules Flow

1. Start Rust server to get rules metadata
2. Load effective config (custom > local > global)
3. Build categorized quick pick items:
   - Custom rules section (if any)
   - Type Safety category
   - Variables category
   - Imports category
   - Code Quality category
   - Bug Prevention category
   - Style category
   - Performance category
4. Show multi-select picker
5. User toggles rules with space, confirms with enter
6. Update config:
   - Enabled rules: remove `enabled: false`
   - Disabled rules: add `enabled: false`
   - Unconfigured disabled: remove from config
7. Save to appropriate location:
   - If config exists: update in place
   - If no config: show location picker
8. Trigger rescan

### Scan Mode Flow

1. Show mode picker (Codebase vs Branch)
2. If Codebase selected:
   - Clear results
   - Update state
   - Invalidate cache
   - Trigger rescan
3. If Branch selected:
   - Show branch picker:
     - Keep current
     - Choose another (shows branch list)
   - Update compare branch
   - Clear results
   - Update state
   - Invalidate cache
   - Trigger rescan

### Config Location Flow

1. Detect current config location:
   - Custom (custom directory)
   - Local (.tscanner)
   - Global (extension storage)
2. Show location picker with current marked
3. If Custom selected:
   - Show folder picker UI
   - Navigate workspace tree
   - Select folder
4. If location changed and config exists:
   - Confirm migration
   - Load current config
   - Delete from old location
   - Save to new location
   - Clear results
   - Trigger rescan
5. If location changed and no config:
   - Just update preference
   - Show message to create config

## Git Integration Details

### Changed Files Detection

**VSCode Git API:**
```typescript
// Get VSCode Git extension
const gitExtension = vscode.extensions.getExtension('vscode.git');
const api = gitExtension.exports.getAPI(1);
const repo = api.repositories[0];

// Get uncommitted changes
const stagedChanges = await repo.diffWithHEAD();

// Get committed changes vs branch
const committedChanges = await repo.diffBetween(compareBranch, currentBranch);

// Merge both sets
const allChanges = [...stagedChanges, ...committedChanges];
```

**Caching Strategy:**
- Cache key: `${workspaceRoot}:${compareBranch}`
- TTL: 30 seconds
- Invalidated on mode change, hard scan, or manual invalidation

### Modified Line Ranges

**Git Diff Parsing:**
```bash
git diff "compareBranch...currentBranch" -- "filePath"
```

**Hunk Example:**
```diff
@@ -10,5 +10,7 @@
 unchanged line
-removed line
+added line 1
+added line 2
 unchanged line
```

**Parsing Logic:**
1. Find hunk header: `@@ -10,5 +10,7 @@`
2. Extract start line: `10`
3. Iterate through hunk:
   - Lines starting with `+`: added (track line number)
   - Lines starting with `-`: removed (don't increment)
   - Other lines: context (increment line number)
4. Build ranges: `{ startLine: 10, lineCount: 2 }`

**Range Filtering:**
```typescript
function isLineInRanges(line: number, ranges: ModifiedLineRange[]): boolean {
  return ranges.some(range => {
    const endLine = range.startLine + range.lineCount - 1;
    return line >= range.startLine && line <= endLine;
  });
}
```

## Scan Modes

### Workspace (Codebase) Mode

**Behavior:**
- Scans all files matching include patterns
- Respects exclude patterns
- No git filtering
- Shows all issues in codebase

**Use Cases:**
- Initial code audit
- Full project quality check
- Non-git projects

### Branch Mode

**Behavior:**
1. Run `git diff compareBranch...HEAD` to get changed files
2. Parse diff hunks to extract modified line ranges
3. Scan all files (uses cache for unchanged files)
4. Filter issues to only modified lines
5. Show only new/changed issues

**Use Cases:**
- PR validation
- Feature branch review
- Incremental development
- CI/CD integration

**Example:**
- Compare branch: `origin/main`
- Current branch: `feature/new-component`
- Changed files: 3
- Modified lines: 50-75 in file1.ts, 100-120 in file2.ts
- Total issues: 200 (full codebase)
- Filtered issues: 5 (in modified lines only)

## View Modes

### View Mode: List vs Tree

**List Mode:**
- Flat list of files
- No folder hierarchy
- Files shown directly under root or rule group
- Faster to browse for small result sets

**Tree Mode:**
- Folder hierarchy built from file paths
- Expandable folders
- Preserves project structure
- Better for large result sets

### Group Mode: Default (File) vs Rule

**Default (File) Mode:**
- Group by file path
- Each file shows its issues
- Good for fixing all issues in a file
- Copy issues per file

**Rule Mode:**
- Group by rule name
- Each rule shows affected files
- Good for fixing specific rule violations
- Copy issues per rule
- Shows rule distribution

### View State Matrix

| View Mode | Group Mode | Structure |
|-----------|-----------|-----------|
| List | Default | File → Issues |
| Tree | Default | Folder → File → Issues |
| List | Rule | Rule → Issues (flat) |
| Tree | Rule | Rule → Folder → File → Issues |

### Tree Expansion

**Challenge:** VSCode TreeView doesn't auto-expand folders

**Solution:** Force expansion after results load
```typescript
if (viewMode === ViewMode.Tree) {
  setTimeout(() => {
    const folders = panelContent.getAllFolderItems();
    for (const folder of folders) {
      treeView.reveal(folder, { expand: true, select: false, focus: false });
    }
  }, 100);
}
```

## File Watcher & Real-Time Updates

### Watcher Configuration

```typescript
const fileWatcher = vscode.workspace.createFileSystemWatcher(
  '**/*.{ts,tsx,js,jsx}'
);

fileWatcher.onDidChange(updateSingleFile);
fileWatcher.onDidCreate(updateSingleFile);
fileWatcher.onDidDelete(removeFile);
```

### Update Flow

**File Changed/Created:**
1. Check if search in progress → skip
2. Get workspace folder
3. If branch mode:
   - Invalidate git cache
   - Check if file in changed files set
   - Skip if not changed
4. Load file content
5. Load effective config
6. Scan content via RPC
7. If branch mode:
   - Get modified line ranges
   - Filter issues to modified lines
8. Remove old results for this file
9. Add new results
10. Update tree view
11. Update badge
12. Serialize to workspace state

**File Deleted:**
1. If branch mode: invalidate git cache
2. Filter out results for this file
3. Update tree view
4. Update badge
5. Serialize to workspace state

### Performance Considerations

- Single file scans are fast (<100ms typically)
- Results merged incrementally
- No full rescan needed
- Branch mode filters incrementally

## Issue Navigation

### F8/Shift+F8 Navigation

**State Management:**
```typescript
let currentIssueIndex = -1; // Module-level state

function goToNextIssue() {
  currentIssueIndex = (currentIssueIndex + 1) % results.length;
  navigateToIssue(results[currentIssueIndex]);
}

function goToPreviousIssue() {
  if (currentIssueIndex === -1) {
    currentIssueIndex = results.length - 1;
  } else {
    currentIssueIndex = (currentIssueIndex - 1 + results.length) % results.length;
  }
  navigateToIssue(results[currentIssueIndex]);
}

function resetIssueIndex() {
  currentIssueIndex = -1; // Reset on new scan
}
```

**Navigation Logic:**
```typescript
async function navigateToIssue(issue: IssueResult) {
  const doc = await vscode.workspace.openTextDocument(issue.uri);
  const editor = await vscode.window.showTextDocument(doc);

  const position = new vscode.Position(issue.line, issue.column);
  editor.selection = new vscode.Selection(position, position);
  editor.revealRange(
    new vscode.Range(position, position),
    vscode.TextEditorRevealType.InCenter
  );

  vscode.window.setStatusBarMessage(
    `Issue ${currentIssueIndex + 1}/${results.length}: ${issue.rule}`,
    3000
  );
}
```

**Keybindings:**
```json
{
  "command": "tscanner.goToNextIssue",
  "key": "f8",
  "when": "editorTextFocus"
},
{
  "command": "tscanner.goToPreviousIssue",
  "key": "shift+f8",
  "when": "editorTextFocus"
}
```

### Click-to-Navigate

Tree items include command in their definition:
```typescript
class LineResultItem extends vscode.TreeItem {
  constructor(result: IssueResult) {
    super(result.text, vscode.TreeItemCollapsibleState.None);

    this.command = {
      command: getCommandId(Command.OpenFile),
      title: 'Open File',
      arguments: [result.uri, result.line, result.column]
    };
  }
}
```

## Copy Issues Feature

### Copy by Rule

**Flow:**
1. User right-clicks rule group → Copy Rule Issues
2. Extract all issues for that rule
3. Convert to `ScanResult` format
4. Call Rust `formatResults` RPC method
5. Build header with scan mode and CLI command
6. Append formatted output
7. Append summary (issue counts)
8. Copy to clipboard

**Output Format:**
```
TScanner report searching for all the issues of the rule "no-any-type" in the branch mode

cli command: tscanner check --rule "no-any-type" --branch origin/main
found issues: 5 issues

Rules triggered:

  no-any-type: Detects usage of TypeScript 'any' type

Issues grouped by rule:

no-any-type (5 issues, 3 files)

  src/file1.ts (2 issues)
    ⚠ 10:15 -> const x: any = 123;
    ⚠ 20:8 -> function foo(x: any) {}

  ...

Issues: 5 (5 errors, 0 warnings)
Files: 3
Rules: 1
```

### Copy by File

Same as copy by rule, but filters to single file and uses `file` group mode.

### Copy by Folder

Recursively collects all issues in folder tree, then formats like copy by file.

### Use Case: AI-Assisted Fixes

Users can copy formatted issues and paste into AI chat:
```
Fix these issues:

[paste copied issues here]
```

AI can understand the structure and suggest fixes for all violations at once.

## Important Implementation Details

### Binary Selection

**Platform Detection:**
```typescript
const platform = process.platform; // linux, darwin, win32
const arch = process.arch; // x64, arm64

const PLATFORM_TARGET_MAP = {
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'linux-arm64': 'aarch64-unknown-linux-gnu',
  'darwin-x64': 'x86_64-apple-darwin',
  'darwin-arm64': 'aarch64-apple-darwin',
  'win32-x64': 'x86_64-pc-windows-msvc',
};

const target = PLATFORM_TARGET_MAP[`${platform}-${arch}`];
const binaryName = `tscanner-server-${target}${platform === 'win32' ? '.exe' : ''}`;
```

**Search Order:**
1. Bundled binary: `extension/out/binaries/{binaryName}`
2. Dev release: `packages/core/target/release/{binaryName}`
3. Dev debug: `packages/core/target/debug/{binaryName}`

### Workspace State Serialization

**Challenge:** VSCode workspace state doesn't support Uri objects

**Solution:** Convert Uri to string before saving
```typescript
const serializedResults = results.map(r => {
  const { uri, ...rest } = r;
  return {
    ...rest,
    uriString: uri.toString()
  };
});
setWorkspaceState(context, WorkspaceStateKey.CachedResults, serializedResults);
```

### Config Hash for Cache Invalidation

Rust generates hash from config to detect changes:
```rust
// Simplified
let hash = hash(rules + patterns + severity);
cache_key = format!("cache_{}.json", hash);
```

When config changes, hash changes, cache invalidated automatically.

### Logging

**Log Location:** `$TMPDIR/tscannerlogs.txt`

**Log Format:**
```
[2025-01-15T10:30:45.123-03:00] [vscode_extension] [INFO ] Extension activated
[2025-01-15T10:30:47.456-03:00] [vscode_extension] [DEBUG] Starting scan...
[2025-01-15T10:30:48.789-03:00] [vscode_extension] [INFO ] Scan completed: 42 issues
[2025-01-15T10:30:49.012-03:00] [vscode_extension] [ERROR] Failed to load config: ...
```

**Access:** Command Palette → "tscanner: Show Logs"

### GZIP Compression

**Threshold:** Responses >~10KB are compressed by Rust

**Detection:** Response starts with `GZIP:`

**Decompression:**
```typescript
if (line.startsWith('GZIP:')) {
  const base64Data = line.substring(5);
  const compressed = Buffer.from(base64Data, 'base64');
  const decompressed = zlib.gunzipSync(compressed);
  jsonString = decompressed.toString('utf8');
}
```

**Performance:** 10x reduction in data transfer for large scans

### Dev vs Production Mode

**Detection:**
```typescript
declare const __IS_DEV_BUILD__: boolean;
const IS_DEV = typeof __IS_DEV_BUILD__ !== 'undefined' && __IS_DEV_BUILD__;
```

**Set via esbuild:**
```typescript
define: {
  __IS_DEV_BUILD__: isDev ? 'true' : 'false',
}
```

**Effects:**
- Command IDs: `tscanner.command` vs `tscannerDev.command`
- Context keys: `tscannerViewMode` vs `tscannerViewModeDev`
- View ID: `tscannerExplorer` vs `tscannerExplorerDev`
- Extension ID: different marketplace listings

**Purpose:** Run prod and dev versions side-by-side for testing

## Build & Deployment

### Build Configuration

**esbuild.config.ts:**
```typescript
{
  entryPoints: ['src/extension.ts'],
  bundle: true,
  outfile: 'out/extension.js',
  external: ['vscode'],
  format: 'cjs',
  platform: 'node',
  target: 'node18',
  sourcemap: false,
  minify: false,
  define: {
    __IS_DEV_BUILD__: isDev ? 'true' : 'false',
  },
  alias: {
    zod: require.resolve('zod'),
  },
}
```

### Package Structure

```
dist-dev/ (or dist-prod/)
├── out/
│   ├── extension.js (bundled)
│   └── binaries/
│       ├── tscanner-server-x86_64-unknown-linux-gnu
│       ├── tscanner-server-aarch64-unknown-linux-gnu
│       ├── tscanner-server-x86_64-apple-darwin
│       ├── tscanner-server-aarch64-apple-darwin
│       └── tscanner-server-x86_64-pc-windows-msvc.exe
├── resources/
│   ├── icon.png
│   └── icon.svg
├── package.json
├── LICENSE
└── README.md
```

### Local Installation

**Scripts:**
```bash
pnpm run build           # Bundle extension
pnpm run vscode:package  # Create .vsix
pnpm run vscode:publish  # Publish to marketplace
```

**Manual Install:**
```bash
code --install-extension tscanner-vscode-0.0.20.vsix
```

## Common Operations

### Adding a New Command

1. Add to `Command` enum in `vscode-utils.ts`
2. Create command file in `commands/public` or `commands/internal`
3. Register in `commands/index.ts`
4. Add to `package.json` contributions
5. Add keybinding if needed

### Adding a New View Mode

1. Add to `ViewMode` or `GroupMode` enum
2. Update `VIEW_STATES` array in `view-mode.ts`
3. Add icon to `package.json` menus
4. Update tree building logic in `panel-content.ts`

### Adding a New Config Location

1. Update `ConfigLocation` enum
2. Add getters/setters in `config-manager.ts`
3. Update location picker in `config-location.ts`
4. Add migration logic

### Adding a New Tree Item Type

1. Create class in `tree-items.ts`
2. Add to `PanelContentItem` union
3. Update `getChildren` in `panel-content.ts`
4. Add context value to `TreeItemContextValue` enum
5. Add menu items in `package.json`

## Troubleshooting

### Extension Not Activating

Check:
- `activationEvents` in package.json
- Extension host log (Help → Toggle Developer Tools)
- Log file at `$TMPDIR/tscannerlogs.txt`

### Rust Binary Not Found

Check:
1. Binary exists in `out/binaries/`
2. Platform detection matches binary name
3. Binary has execute permissions (Unix)
4. Log shows attempted paths

### Tree View Not Updating

Check:
- `_onDidChangeTreeData.fire()` called
- Event emitter properly exposed
- Results actually changed

### Issues Not Showing

Check:
- Config has enabled rules
- Files match include patterns
- Files don't match exclude patterns
- Log shows scan completed
- Results not empty in panel content

### File Watcher Not Working

Check:
- Watcher glob pattern matches changed files
- Search not in progress (`isSearching === false`)
- Branch mode file in changed files set
- Log shows file change events

## Performance Metrics

Typical performance on medium project (1000 files, 100k LOC):

- **Initial Scan:** 2-5 seconds
- **Single File Update:** 50-150ms
- **Git Diff:** 100-300ms (cached: <10ms)
- **Tree Rebuild:** <50ms
- **Result Serialization:** <100ms

Bottlenecks:
1. Rust scanning (parallelized)
2. File I/O for line text loading
3. Git diff parsing
4. Tree view rendering (large result sets)
