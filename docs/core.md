# TScanner Core Package - Technical Documentation

## Overview

The core package is the Rust-powered engine behind TScanner, a high-performance TypeScript/TSX code quality scanner. It provides blazing-fast code analysis through parallel processing, AST-based validation, and intelligent caching. The core serves as the foundation for multiple TScanner packages: the CLI tool, VS Code extension, and GitHub Action.

**Key Features:**
- 39+ built-in AST-based rules for TypeScript/TSX validation
- Custom regex pattern matching rules
- Parallel file processing via Rayon work-stealing thread pool
- Smart caching with DashMap concurrent cache and disk persistence
- Automatic rule registration using inventory crate
- JSON-RPC server for IPC communication
- Real-time file watching with notify
- Inline disable directives support

## Package Structure

This is a Rust workspace containing 3 crates:

```
packages/core/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── core/               # Core library (Scanner, Parser, Rules, Cache)
│   │   ├── src/
│   │   │   ├── lib.rs      # Public API exports
│   │   │   ├── scanner.rs  # Parallel file scanning engine
│   │   │   ├── parser.rs   # SWC TypeScript/TSX AST parser
│   │   │   ├── registry.rs # Rule registry with auto-registration
│   │   │   ├── cache.rs    # DashMap concurrent cache with persistence
│   │   │   ├── config.rs   # Configuration loading and validation
│   │   │   ├── watcher.rs  # File system watcher
│   │   │   ├── formatter.rs # Output formatting
│   │   │   ├── types.rs    # Core data structures
│   │   │   ├── rules/      # 39+ built-in rules
│   │   │   ├── logger.rs   # File-based logging
│   │   │   ├── constants.rs # Application constants
│   │   │   ├── utils.rs    # Utility functions
│   │   │   ├── ast_utils.rs # AST helper functions
│   │   │   └── disable_comments.rs # Inline disable directive parser
│   │   └── Cargo.toml
│   ├── server/             # JSON-RPC server binary
│   │   ├── src/
│   │   │   ├── main.rs     # Server entry point
│   │   │   ├── protocol.rs # JSON-RPC protocol types
│   │   │   ├── state.rs    # Server state management
│   │   │   └── handlers/   # Request handlers
│   │   │       ├── scan.rs
│   │   │       ├── scan_file.rs
│   │   │       ├── scan_content.rs
│   │   │       ├── watch.rs
│   │   │       ├── get_rules_metadata.rs
│   │   │       ├── clear_cache.rs
│   │   │       └── format_results.rs
│   │   └── Cargo.toml
│   └── cli/                # CLI binary
│       ├── src/
│       │   ├── main.rs     # CLI entry point
│       │   ├── commands/   # CLI commands
│       │   │   ├── check.rs
│       │   │   ├── init.rs
│       │   │   └── rules.rs
│       │   └── config_loader.rs
│       └── Cargo.toml
```

## Core Dependencies

**Workspace Dependencies (Cargo.toml):**

```toml
swc_ecma_parser = "27"      # TypeScript/JSX parser
swc_ecma_ast = "18"         # Abstract Syntax Tree types
swc_ecma_visit = "18"       # AST visitor pattern
swc_common = "17"           # Common SWC utilities
rayon = "1.11"              # Parallel processing
dashmap = "6.1"             # Concurrent HashMap
walkdir = "2.5"             # Directory traversal
ignore = "0.4"              # Gitignore-style file filtering
globset = "0.4"             # Glob pattern matching
serde = "1.0"               # Serialization framework
serde_json = "1.0"          # JSON serialization
json_comments = "0.2"       # JSONC parser
schemars = "0.8"            # JSON Schema generation
notify = "8"                # File system watcher
anyhow = "1.0"              # Error handling
thiserror = "2.0"           # Error derive macros
regex = "1.11"              # Regular expressions
time = "0.3"                # Time utilities
pathdiff = "0.2"            # Path diffing
```

**Core-specific:**
- `inventory = "0.3"` - Compile-time rule registration
- `dirs = "5.0"` - System directory paths

**Server-specific:**
- `flate2 = "1.0"` - GZIP compression
- `base64 = "0.21"` - Base64 encoding

**CLI-specific:**
- `clap = "4.5"` - Command-line argument parsing
- `colored = "2.1"` - Terminal colors

## Module Architecture

### 1. Scanner (`scanner.rs`)

**Purpose:** Parallel file scanning with cache integration and git-aware filtering.

**Key Components:**

```rust
pub struct Scanner {
    registry: RuleRegistry,        // Rule registry with enabled rules
    config: TscannerConfig,        // Configuration
    cache: Arc<FileCache>,         // Thread-safe cache
}
```

**Key Methods:**

- `new(config)` - Create scanner with new cache
- `with_cache(config, cache)` - Create scanner with existing cache
- `scan(root, file_filter)` - Scan workspace with optional file filtering
- `scan_single(path)` - Re-scan single file (invalidates cache)
- `scan_content(path, content)` - Analyze in-memory content (no disk read)

**How It Works:**

1. **File Discovery:** Uses WalkBuilder from `ignore` crate with gitignore support
   - Filters for `.ts` and `.tsx` files
   - Excludes `node_modules`, `.git`, `dist` by default

2. **Parallel Processing:** Rayon's `par_iter()` processes files concurrently
   - Work-stealing thread pool automatically scales to CPU cores
   - Lock-free cache access via DashMap

3. **Cache Integration:**
   - Checks cache before parsing (mtime + config hash validation)
   - Updates cache after processing
   - Flushes cache to disk after scan

4. **Issue Collection:**
   - Runs all enabled rules against parsed AST
   - Filters issues by disable directives
   - Aggregates results into ScanResult

**Branch Mode Support:**
- Accepts `file_filter` parameter with HashSet of changed files
- Only scans files in the set
- Used by CLI and server for git diff integration

### 2. Parser (`parser.rs`)

**Purpose:** SWC-based TypeScript/TSX AST parsing with error recovery.

**Implementation:**

```rust
pub fn parse_file(path: &Path, source: &str) -> Result<Program>
```

**Key Features:**

- Uses SWC's Lexer and Parser for TypeScript/TSX
- Auto-detects TSX from `.tsx` extension
- Enables decorators and disables early errors for better recovery
- Returns swc_ecma_ast::Program (AST root)
- Propagates parse errors via anyhow::Result

**Configuration:**

```rust
TsSyntax {
    tsx: path.extension() == Some("tsx"),
    decorators: true,              // Support decorators
    dts: false,                    // Not declaration files
    no_early_errors: true,         // Better error recovery
    disallow_ambiguous_jsx_like: false,
}
```

### 3. Rule Registry (`registry.rs`)

**Purpose:** Dynamic rule loading with compile-time registration using inventory.

**Key Components:**

```rust
pub struct RuleRegistry {
    rules: HashMap<String, Arc<dyn Rule>>,              // All rules
    compiled_configs: HashMap<String, CompiledRuleConfig>, // Per-rule config
}
```

**Rule Trait:**

```rust
pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue>;
}
```

**How Auto-Registration Works:**

1. Each rule submits to inventory at compile time:
   ```rust
   inventory::submit!(RuleRegistration {
       name: "no-any-type",
       factory: || Arc::new(NoAnyTypeRule),
   });
   ```

2. Registry collects all submissions:
   ```rust
   for registration in inventory::iter::<RuleRegistration> {
       rules.insert(registration.name, (registration.factory)());
   }
   ```

3. No manual registration needed - just add rule file to `rules/mod.rs`

**Configuration Compilation:**

- Loads config from `.tscanner/config.jsonc`
- Compiles glob patterns for include/exclude
- Creates CompiledRuleConfig per rule with:
  - enabled flag
  - severity level
  - include/exclude GlobSets
  - custom message/pattern for regex rules

**Rule Filtering:**

`get_enabled_rules(file_path, config)` returns only rules that:
- Are enabled in config
- Match file path via include/exclude patterns

### 4. Cache (`cache.rs`)

**Purpose:** DashMap-based concurrent cache with disk persistence.

**Key Components:**

```rust
pub struct FileCache {
    entries: DashMap<PathBuf, CacheEntry>,  // Concurrent HashMap
    config_hash: u64,                       // Cache invalidation key
    cache_dir: Option<PathBuf>,             // ~/.cache/tscanner/
}

struct CacheEntry {
    mtime: SystemTime,      // File modification time
    config_hash: u64,       // Config hash at cache time
    issues: Vec<Issue>,     // Cached issues
}
```

**Caching Strategy:**

1. **Cache Key:** File path
2. **Validation:** Entry valid if:
   - mtime matches current file mtime
   - config_hash matches current config hash
3. **Storage:** `~/.cache/tscanner/cache_{config_hash}.json`
4. **Concurrency:** DashMap allows lock-free concurrent access

**Cache Lifecycle:**

- `load_from_disk()` - Load on creation if config hash matches
- `get(path)` - Check mtime + config hash, return if valid
- `insert(path, issues)` - Store entry with current mtime
- `invalidate(path)` - Remove single entry
- `clear()` - Remove all entries + disk file
- `flush()` - Save to disk (called after scan)

**Performance:**

- 80-95% cache hit rate in typical workflows
- Sub-millisecond cache lookups
- Parallel cache access without locking

### 5. Config (`config.rs`)

**Purpose:** Configuration loading, validation, and compilation.

**Main Type:**

```rust
pub struct TscannerConfig {
    pub schema: Option<String>,                     // JSON Schema URL
    pub builtin_rules: HashMap<String, BuiltinRuleConfig>,
    pub custom_rules: HashMap<String, CustomRuleConfig>,
    pub include: Vec<String>,                       // Default: ["**/*.{ts,tsx}"]
    pub exclude: Vec<String>,                       // Default: node_modules, dist, etc.
}
```

**Built-in Rule Config:**

```rust
pub struct BuiltinRuleConfig {
    pub enabled: Option<bool>,          // Default: true
    pub severity: Option<Severity>,     // Override default severity
    pub include: Vec<String>,           // Per-rule include patterns
    pub exclude: Vec<String>,           // Per-rule exclude patterns
}
```

**Custom Rule Config:**

```rust
pub struct CustomRuleConfig {
    pub rule_type: CustomRuleType,      // regex, script, ai
    pub pattern: Option<String>,        // Regex pattern (for regex type)
    pub script: Option<String>,         // Script path (for script type)
    pub prompt: Option<String>,         // Prompt path (for ai type)
    pub message: String,                // Error message
    pub severity: Severity,             // error or warning
    pub enabled: bool,                  // Default: true
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}
```

**Configuration Loading:**

1. `load_from_workspace(path)` - Loads from `.tscanner/config.jsonc`
2. Strips JSON comments using `json_comments` crate
3. Validates:
   - Regex patterns compile
   - Script/prompt files exist for custom rules
   - Conflicting rules (e.g., prefer-type-over-interface + prefer-interface-over-type)
4. Returns default config if not found

**Glob Compilation:**

- Compiles include/exclude patterns into GlobSet
- Per-rule patterns override global patterns
- Uses `globset` crate for efficient matching

**Config Hash:**

```rust
pub fn compute_hash(&self) -> u64
```

- Hashes all rule configurations (enabled state, severity, patterns)
- Used as cache invalidation key
- Any config change invalidates entire cache

### 6. Watcher (`watcher.rs`)

**Purpose:** Real-time file change detection using notify crate.

**Key Components:**

```rust
pub enum FileEvent {
    Modified(PathBuf),
    Created(PathBuf),
    Removed(PathBuf),
}

pub struct FileWatcher {
    _watcher: Box<dyn Watcher>,          // Notify watcher
    receiver: Receiver<FileEvent>,       // Event channel
}
```

**How It Works:**

1. Creates notify watcher in recursive mode
2. Filters events to only `.ts` and `.tsx` files
3. Excludes `node_modules`, `.git`, `dist`, `build`
4. Converts notify events to simplified FileEvent enum
5. Provides `try_recv()` and `recv_timeout()` for polling

**Integration:**

- Server creates watcher on `watch` RPC method
- Server polls `try_recv()` after each request
- On file change, server re-scans file and sends notification
- Extension receives notification and updates UI

### 7. Formatter (`formatter.rs`)

**Purpose:** Format scan results for console output.

**Modes:**

1. **By File:**
   ```
   Rules triggered:
     no-any-type       : Found ': any' type annotation
     no-console-log    : Found console.log statement

   Issues grouped by file:

   src/index.ts - 2 issues - 2 rules

     no-any-type (1 issue)
       ✖ 5:10 -> const data: any = {};

     no-console-log (1 issue)
       ✖ 7:1 -> console.log(data);
   ```

2. **By Rule:**
   ```
   Rules triggered:
     no-any-type    : Found ': any' type annotation

   Issues grouped by rule:

   no-any-type (2 issues, 2 files)

     src/index.ts (1 issue)
       ✖ 5:10 -> const data: any = {};

     src/utils.ts (1 issue)
       ✖ 12:5 -> function test(x: any) {}
   ```

**Features:**

- Shows rule descriptions at top
- Groups issues by file or rule
- Displays line text for context
- Uses severity icons (✖ for error, ⚠ for warning)
- Relative paths from project root

### 8. Types (`types.rs`)

**Core Data Structures:**

```rust
pub enum Severity {
    Error,      // Red in output, exits CLI with code 1
    Warning,    // Yellow in output, does not exit CLI
}

pub struct Issue {
    pub rule: String,           // Rule name
    pub file: PathBuf,          // File path
    pub line: usize,            // Line number (1-indexed)
    pub column: usize,          // Column number (1-indexed)
    pub message: String,        // Error message
    pub severity: Severity,     // Error or warning
    pub line_text: Option<String>, // Source line for context
}

pub struct FileResult {
    pub file: PathBuf,          // File path
    pub issues: Vec<Issue>,     // Issues in file
}

pub struct ScanResult {
    pub files: Vec<FileResult>, // Files with issues
    pub total_issues: usize,    // Total issue count
    pub duration_ms: u128,      // Scan duration
}
```

### 9. Disable Comments (`disable_comments.rs`)

**Purpose:** Parse inline disable directives to suppress rules.

**Directives:**

```typescript
// tscanner-disable-file
// Disables all rules for entire file

// tscanner-disable-line no-any-type, no-console-log
// Disables specific rules on this line

// tscanner-disable no-any-type
// Alias for disable-line

// tscanner-disable-next-line no-any-type
// Disables specific rules on next line
```

**Implementation:**

- Uses regex to extract directives
- Builds HashMap of line number → Set<rule_names>
- Scanner filters issues using `is_rule_disabled(line, rule)`

### 10. Logger (`logger.rs`)

**Purpose:** Thread-safe file logging to temp directory.

**Implementation:**

- Logs to `$TMPDIR/tscannerlogs.txt` (or `tscannerlogs-dev.txt` in dev mode)
- UTC-3 timestamps with millisecond precision
- Levels: INFO, ERROR, WARN, DEBUG
- Context prefix for identifying log source (e.g., "rust_server", "rust_cli")

**Example Log:**

```
[2025-01-27T14:30:45.123-03:00] [rust_server     ] [INFO ] TScanner server started
[2025-01-27T14:30:45.456-03:00] [rust_server     ] [INFO ] Scanning workspace: "/path/to/project"
```

## JSON-RPC Server

### Protocol

**Transport:** Line-delimited JSON over stdin/stdout

**Request Format:**

```json
{"id": 1, "method": "scan", "params": {...}}
```

**Response Format:**

```json
{"id": 1, "result": {...}, "error": null}
```

**Notification Format (server-initiated):**

```json
{"method": "file_updated", "params": {...}}
```

### GZIP Compression

**When:** Response > 10KB (automatically determined by serialization)

**Format:**

```
GZIP:{base64-encoded-gzip-data}\n
```

**Implementation:**

```rust
// Serialize → GZIP → Base64 → Write
let json = serde_json::to_string(&response)?;
let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
encoder.write_all(json.as_bytes())?;
let compressed = encoder.finish()?;
let encoded = base64::encode(&compressed);
stdout.write_all(b"GZIP:")?;
stdout.write_all(encoded.as_bytes())?;
stdout.write_all(b"\n")?;
```

**Client Decoding:**

1. Check if response starts with "GZIP:"
2. Extract base64 data after marker
3. Base64 decode
4. GZIP decompress
5. JSON parse

### Methods

#### `scan`

**Purpose:** Scan workspace with configuration and optional git diff filtering.

**Params:**

```rust
{
    "root": "/path/to/workspace",        // Required
    "config": {...},                     // Optional (uses .tscanner/config.jsonc if null)
    "branch": "origin/main"              // Optional (filters to changed files)
}
```

**Implementation:**

1. Load or use provided config
2. Create scanner with config hash
3. Get changed files if branch provided (git diff --name-only)
4. Get modified lines if branch provided (git diff with unified format)
5. Scan with file filter
6. Filter issues to only modified lines
7. Return ScanResult

**Response:**

```json
{
    "files": [
        {
            "file": "/path/to/file.ts",
            "issues": [...]
        }
    ],
    "total_issues": 5,
    "duration_ms": 123
}
```

#### `scanFile`

**Purpose:** Re-scan single file (invalidates cache).

**Params:**

```rust
{
    "root": "/path/to/workspace",
    "file": "/path/to/file.ts"
}
```

**Implementation:**

1. Load config from workspace
2. Create scanner with existing cache
3. Call `scanner.scan_single(file)` (invalidates cache entry)
4. Return FileResult or empty result

#### `scanContent`

**Purpose:** Analyze in-memory content (used for unsaved editor changes).

**Params:**

```rust
{
    "root": "/path/to/workspace",
    "file": "/path/to/file.ts",
    "content": "const x: any = 1;",
    "config": {...}  // Optional
}
```

**Implementation:**

1. Use provided config or load from workspace
2. Create scanner with cache
3. Call `scanner.scan_content(file, content)` (no disk read)
4. Return issues without caching

#### `watch`

**Purpose:** Start file watcher for real-time updates.

**Params:**

```rust
{
    "root": "/path/to/workspace"
}
```

**Implementation:**

1. Create FileWatcher for root
2. Store in server state
3. Poll `watcher.try_recv()` after each request
4. Send `file_updated` notification on changes

#### `getRulesMetadata`

**Purpose:** Get all available rules with metadata.

**Params:** None

**Response:**

```json
{
    "rules": [
        {
            "name": "no-any-type",
            "displayName": "No Any Type",
            "description": "Detects usage of TypeScript 'any' type",
            "ruleType": "ast",
            "defaultSeverity": "warning",
            "defaultEnabled": false,
            "category": "typesafety"
        }
    ]
}
```

#### `clearCache`

**Purpose:** Clear file cache (used when config changes).

**Params:** None

**Implementation:**

1. Call `state.cache.clear()`
2. Removes all entries and disk file

#### `formatResults`

**Purpose:** Format scan results for display.

**Params:**

```rust
{
    "root": "/path/to/workspace",
    "results": {...},  // ScanResult
    "group_mode": "file" | "rule"
}
```

**Response:**

```json
{
    "formatted": "string output"
}
```

### Server State

```rust
pub struct ServerState {
    pub scanner: Option<Scanner>,          // Current scanner instance
    pub watcher: Option<FileWatcher>,      // File watcher if active
    pub cache: Arc<FileCache>,             // Shared cache
}
```

**Lifecycle:**

- Created on server start
- Scanner recreated on each scan (config may change)
- Watcher persists until server shutdown
- Cache persists across scans (unless config hash changes)

### Event Processing

**File Events:**

```rust
fn process_file_events(state: &ServerState, stdout: &mut Stdout) {
    while let Some(event) = state.watcher.try_recv() {
        match event {
            FileEvent::Modified(path) | FileEvent::Created(path) => {
                let result = state.scanner.scan_single(&path);
                send_notification("file_updated", result);
            }
            FileEvent::Removed(path) => {
                state.cache.invalidate(&path);
            }
        }
    }
}
```

## CLI Commands

### `tscanner check [PATH]`

**Purpose:** Scan code for issues and display results.

**Options:**

```bash
--no-cache              # Skip cache, force full scan
--by-rule               # Group by rule instead of file
--json                  # Output JSON format
--pretty                # Pretty output with rule definitions
--branch <BRANCH>       # Only show issues in changed files vs branch
--file <PATTERN>        # Filter to specific file pattern (glob)
--rule <NAME>           # Filter to specific rule
--continue-on-error     # Don't exit with code 1 on errors
--config <PATH>         # Custom config directory
```

**Implementation Flow:**

1. Load config from workspace or custom path
2. Create cache (or skip if --no-cache)
3. Create scanner with cache
4. Get changed files if --branch (git diff)
5. Filter files if --file (glob pattern)
6. Scan with filters
7. Filter issues by --rule if provided
8. Format and display results
9. Exit with code 1 if errors found (unless --continue-on-error)

### `tscanner rules [PATH]`

**Purpose:** List all available rules and their metadata.

**Output:**

```
Available Rules (39):

Type Safety:
  no-any-type              Detects usage of TypeScript 'any' type
  no-implicit-any          Detects function parameters without types
  ...

Code Quality:
  no-console-log           Finds console.log() statements
  no-empty-function        Disallows empty functions
  ...
```

### `tscanner init [PATH]`

**Purpose:** Create default configuration file.

**Implementation:**

1. Create `.tscanner/` directory
2. Write default config to `.tscanner/config.jsonc`
3. Uses embedded default config from `assets/default-config.json`

## Rule Implementation

### AST-based Rules

**Pattern:**

```rust
use crate::rules::{Rule, RuleMetadata, RuleRegistration};
use crate::types::{Issue, Severity};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct MyRule;

// Register rule factory
inventory::submit!(RuleRegistration {
    name: "my-rule",
    factory: || Arc::new(MyRule),
});

// Register metadata
inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "my-rule",
        display_name: "My Rule",
        description: "Rule description",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
    }
});

// Implement Rule trait
impl Rule for MyRule {
    fn name(&self) -> &str { "my-rule" }

    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let mut visitor = MyVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

// Implement visitor
struct MyVisitor<'a> {
    issues: Vec<Issue>,
    path: PathBuf,
    source: &'a str,
}

impl<'a> Visit for MyVisitor<'a> {
    fn visit_expr(&mut self, n: &Expr) {
        // Check expression
        if matches_pattern(n) {
            let (line, column) = get_line_col(self.source, n.span().lo.0);
            self.issues.push(Issue {
                rule: "my-rule".to_string(),
                file: self.path.clone(),
                line,
                column,
                message: "Issue message".to_string(),
                severity: Severity::Warning,
                line_text: None,
            });
        }
        n.visit_children_with(self);
    }
}
```

**Key Points:**

- Use SWC's visitor pattern to traverse AST
- Call `visit_children_with(self)` to continue traversal
- Use `get_line_col()` utility to convert byte offset to line:column
- Severity is overridden by config, default only matters for metadata

### Regex Rules

**Pattern:**

```rust
pub struct RegexRule {
    name: String,
    pattern: Regex,
    message: String,
    severity: Severity,
}

impl Rule for RegexRule {
    fn check(&self, _program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        for (line_num, line) in source.lines().enumerate() {
            if let Some(mat) = self.pattern.find(line) {
                issues.push(Issue {
                    rule: self.name.clone(),
                    file: path.to_path_buf(),
                    line: line_num + 1,
                    column: mat.start() + 1,
                    message: self.message.clone(),
                    severity: self.severity,
                    line_text: None,
                });
            }
        }
        issues
    }
}
```

**Key Points:**

- Ignores AST (program parameter unused)
- Processes source text line by line
- Returns match position as line:column
- Created dynamically from custom rule config

## Communication with Other Packages

### VSCode Extension

**Protocol:** JSON-RPC over stdin/stdout

**Flow:**

1. Extension spawns server binary: `tscanner-server`
2. Extension sends JSON requests via stdin
3. Server responds via stdout (GZIP compressed if large)
4. Server sends notifications for file changes

**Example Extension → Server:**

```typescript
// Extension (TypeScript)
const request = {
    id: this.requestId++,
    method: 'scan',
    params: {
        root: this.workspaceRoot,
        config: this.config,  // From global storage
        branch: this.targetBranch
    }
};
this.serverProcess.stdin.write(JSON.stringify(request) + '\n');
```

**Example Server → Extension:**

```rust
// Server (Rust)
let response = Response {
    id: request.id,
    result: Some(serde_json::to_value(&scan_result)?),
    error: None,
};
send_response(&mut stdout, response);  // GZIP + base64 if large
```

### CLI

**Direct Usage:** CLI calls core library functions directly (no IPC)

**Flow:**

1. CLI command parsed with clap
2. Config loaded from workspace
3. Scanner created directly: `Scanner::new(config)`
4. Results formatted and printed to stdout
5. Exit code set based on error count

### GitHub Action

**Usage:** Runs CLI binary in GitHub Actions environment

**Flow:**

1. Action installs tscanner CLI
2. Runs `tscanner check --json --branch origin/main`
3. Parses JSON output
4. Creates annotations for issues
5. Fails workflow if errors found

## Performance Optimizations

### Parallel Processing

**Rayon Work-Stealing:**

```rust
let results: Vec<FileResult> = files
    .par_iter()  // Rayon parallel iterator
    .filter_map(|path| self.analyze_file(path))
    .collect();
```

- Automatically scales to available CPU cores
- Work-stealing prevents idle threads
- No manual thread management needed

### Caching

**Strategy:**

- **Memory:** DashMap for lock-free concurrent access
- **Disk:** JSON file at `~/.cache/tscanner/cache_{hash}.json`
- **Validation:** mtime + config hash
- **Invalidation:** Any config change invalidates entire cache

**Performance:**

- 80-95% cache hit rate in typical workflows
- Sub-millisecond cache lookups
- 10-100x speedup on cached scans

### Compilation Optimizations

**Release Profile:**

```toml
[profile.release]
opt-level = 3           # Maximum optimization
lto = true              # Link-time optimization
codegen-units = 1       # Single codegen unit for better inlining
strip = true            # Strip debug symbols for smaller binary
```

**Typical Performance:**

- 100-500 files: <1s
- 1000-2000 files: 1-3s
- 5000+ files: 5-10s (cached), 15-30s (cold)

## Important Implementation Details

### 1. Config Hash for Cache Invalidation

Any change to rule configuration invalidates cache:

```rust
pub fn compute_hash(&self) -> u64 {
    let mut hasher = DefaultHasher::new();

    // Hash all builtin rules
    for (name, config) in self.builtin_rules.sorted() {
        name.hash(&mut hasher);
        config.enabled.hash(&mut hasher);
        config.severity.hash(&mut hasher);
    }

    // Hash all custom rules
    for (name, config) in self.custom_rules.sorted() {
        name.hash(&mut hasher);
        config.enabled.hash(&mut hasher);
        config.pattern.hash(&mut hasher);
    }

    hasher.finish()
}
```

### 2. Git Diff Integration

**Changed Files:**

```rust
git diff --name-only origin/main
```

**Modified Lines:**

```rust
git diff origin/main
```

Parsed with custom diff parser that extracts:
- File path from `+++ b/path`
- Line numbers from hunk headers `@@ -a,b +c,d @@`
- Modified lines from `+` prefix

### 3. Inventory-based Rule Registration

**Compile-time Registration:**

```rust
inventory::submit!(RuleRegistration {
    name: "no-any-type",
    factory: || Arc::new(NoAnyTypeRule),
});
```

**Runtime Collection:**

```rust
for registration in inventory::iter::<RuleRegistration> {
    rules.insert(registration.name, (registration.factory)());
}
```

**Benefits:**

- No manual registration needed
- Rules discovered at compile time
- Zero runtime overhead
- Impossible to forget registration

### 4. GZIP Compression Protocol

**Why:** Large scan results (>10KB) compress 5-10x

**Implementation:**

- Server compresses large responses automatically
- Client detects "GZIP:" marker
- Base64 encoding for text-safe transport
- Fast compression (Compression::fast())

**Alternative:** Could use binary protocol (MessagePack, Bincode) but JSON is easier to debug

### 5. SWC Parser Configuration

**Critical Settings:**

```rust
TsSyntax {
    tsx: detect_from_extension,
    decorators: true,              // Support @decorator syntax
    no_early_errors: true,         // Better error recovery
    disallow_ambiguous_jsx_like: false,
}
```

**Why no_early_errors:**

- Allows parsing files with minor syntax errors
- Better for incremental analysis
- Issues still reported by rules, not parser

### 6. Disable Directive Parsing

**Regex Patterns:**

```rust
static DISABLE_FILE_RE: &str = r"//\s*tscanner-disable-file";
static DISABLE_LINE_RE: &str = r"//\s*tscanner-disable(?:-line)?\s+(.+)";
static DISABLE_NEXT_LINE_RE: &str = r"//\s*tscanner-disable-next-line\s+(.+)";
```

**Implementation:**

- Parse directives once per file
- Build HashMap of line → Set<rule_names>
- Filter issues using `is_rule_disabled(line, rule)`

### 7. File Watcher Filtering

**Filters:**

- Only `.ts` and `.tsx` extensions
- Excludes `node_modules`, `.git`, `dist`, `build`
- Applied at event level, not file scan level

**Why:** Reduces noise from non-TypeScript file changes

### 8. Logger Implementation

**Thread-Safe:**

```rust
static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
```

**Why file logging:**

- stdout/stderr used for JSON-RPC protocol
- Persistent across invocations
- Easier debugging of server issues

**Location:** `$TMPDIR/tscannerlogs.txt`

- Platform-specific temp directory
- Survives crashes
- Easy to tail during development

## Build and Distribution

### Building Binaries

**Development:**

```bash
cargo build
```

**Release:**

```bash
cargo build --release
```

**Cross-compilation:**

```bash
# Via scripts/build-binaries.ts (in vscode-extension)
# Builds for:
# - linux-x64
# - darwin-x64
# - darwin-arm64
# - win32-x64
```

### Binary Locations

**Server:** `target/release/tscanner-server`
**CLI:** `target/release/tscanner`

**Extension Distribution:**

Binaries copied to `packages/vscode-extension/binaries/`:

```
binaries/
├── tscanner-server-linux-x64
├── tscanner-server-darwin-x64
├── tscanner-server-darwin-arm64
└── tscanner-server-win32-x64.exe
```

Extension selects binary based on platform detection.

## Testing

**Unit Tests:**

```bash
cargo test -p core          # Core library tests
cargo test -p cli           # CLI tests
cargo test -p server        # Server tests
```

**Snapshot Tests:**

Uses `insta` crate for snapshot testing rules:

```rust
#[test]
fn test_no_any_type() {
    let source = r#"const x: any = 1;"#;
    let issues = scan_source(source);
    insta::assert_yaml_snapshot!(issues);
}
```

**Integration Tests:**

Located in `crates/core/tests/`:

```rust
// tests/spec_tests.rs
#[test]
fn test_scan_workspace() {
    let scanner = Scanner::new(TscannerConfig::default())?;
    let result = scanner.scan(Path::new("tests/fixtures"), None);
    assert!(!result.files.is_empty());
}
```

## Summary

The core package is a high-performance Rust engine that provides:

1. **Fast Scanning:** Parallel processing with Rayon + intelligent caching
2. **AST Analysis:** SWC-based TypeScript/TSX parsing with 39+ built-in rules
3. **Flexible Configuration:** JSONC config with glob patterns and per-rule settings
4. **IPC Protocol:** JSON-RPC server with GZIP compression for VSCode extension
5. **Direct API:** Library API for CLI and other Rust consumers
6. **Real-time Updates:** File watching with incremental re-scanning
7. **Git Integration:** Branch comparison with line-level filtering
8. **Extensibility:** Auto-registration system for adding new rules

The architecture balances performance (Rust + parallelism + caching) with usability (JSON config, inline directives, multiple output formats) to provide a production-ready code quality tool.
