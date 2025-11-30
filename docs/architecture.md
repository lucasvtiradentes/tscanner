# TScanner Architecture

## System Overview

```
                                    ┌─────────────────────────────────────┐
                                    │          Rust Core (packages/core)  │
                                    │  ┌─────────────────────────────────┐│
┌──────────────────────┐            │  │ Scanner    - Rayon parallel     ││
│   VSCode Extension   │  JSON-RPC  │  │ Parser     - SWC TypeScript AST ││
│  ├─ TreeDataProvider │◄──────────►│  │ Registry   - 39+ rules          ││
│  ├─ Git Integration  │   stdio    │  │ Cache      - DashMap + disk     ││
│  └─ File Watcher     │            │  │ Watcher    - notify crate       ││
└──────────────────────┘            │  │ Config     - .tscanner/config   ││
                                    │  └─────────────────────────────────┘│
┌──────────────────────┐            │                                     │
│        CLI           │   spawn    │  ┌─────────────────────────────────┐│
│  ├─ Platform detect  │───────────►│  │ CLI Binary  (tscanner)          ││
│  └─ Binary resolver  │  inherit   │  │ ├─ clap argument parsing        ││
└──────────────────────┘   stdio    │  │ └─ Commands: check, init, rules ││
                                    │  └─────────────────────────────────┘│
┌──────────────────────┐            │                                     │
│   GitHub Action      │   npx      │  ┌─────────────────────────────────┐│
│  ├─ Input validation │───────────►│  │ Server Binary (tscanner-server) ││
│  ├─ PR comments      │   stdout   │  │ ├─ JSON-RPC methods             ││
│  └─ Scan orchestrate │◄───────────│  │ └─ GZIP compression             ││
└──────────────────────┘    JSON    │  └─────────────────────────────────┘│
                                    └─────────────────────────────────────┘
```

## Communication Protocols

### VSCode Extension ↔ Rust Server

**Protocol:** Line-delimited JSON-RPC over stdin/stdout

**Request:**
```json
{"id": 1, "method": "scan", "params": {"root": "/workspace", "branch": "main"}}
```

**Response (small):**
```json
{"id": 1, "result": {"files": [...], "total_issues": 5}}
```

**Response (large, >10KB):**
```
GZIP:H4sIAAAA...base64-encoded-gzip-data...
```

**JSON-RPC Methods:**
| Method | Purpose | Used By |
|--------|---------|---------|
| `scan` | Full workspace scan | Extension, on activation |
| `scanFile` | Single file scan | File watcher updates |
| `scanContent` | In-memory content | Unsaved file changes |
| `getRulesMetadata` | Rule catalog | Settings menu |
| `clearCache` | Invalidate cache | Hard scan command |
| `formatResults` | Format for clipboard | Copy issues feature |

### CLI ↔ Rust Binary

**Protocol:** Direct process spawn with stdio inheritance

```
Node.js CLI                     Rust Binary
     │                              │
     ├── spawn(binary, args) ──────►│
     │   stdio: 'inherit'           │
     │                              ├── Parse args (clap)
     │                              ├── Load config
     │                              ├── Run scan
     │                              ├── Output to stdout
     │   ◄────── stdout ────────────┤
     │   ◄────── exit code ─────────┤
     │                              │
```

**No JSON-RPC:** CLI uses direct stdio inheritance for simplicity and zero overhead.

### GitHub Action ↔ CLI

**Protocol:** Shell execution with JSON stdout capture

```bash
npx tscanner@latest check --json --branch origin/main
```

**Output Parsing:**
1. Action executes CLI with `--json` flag
2. CLI outputs JSON to stdout
3. Action parses JSON into TypeScript types
4. Action generates markdown PR comment

## Data Flow: Full Scan

```
┌─────────────────────────────────────────────────────────────────────────┐
│ 1. USER ACTION                                                          │
│    VSCode: Click scan / GitHub: PR opened / CLI: tscanner check         │
└─────────────────────────────────────┬───────────────────────────────────┘
                                      │
┌─────────────────────────────────────▼───────────────────────────────────┐
│ 2. CONFIGURATION LOADING                                                │
│    Load .tscanner/config.jsonc → Validate rules → Compile glob patterns │
│    Generate config hash for cache invalidation                          │
└─────────────────────────────────────┬───────────────────────────────────┘
                                      │
┌─────────────────────────────────────▼───────────────────────────────────┐
│ 3. FILE DISCOVERY                                                       │
│    WalkBuilder with gitignore → Filter .ts/.tsx → Apply include/exclude │
│    Branch mode: Filter to git diff changed files                        │
└─────────────────────────────────────┬───────────────────────────────────┘
                                      │
┌─────────────────────────────────────▼───────────────────────────────────┐
│ 4. PARALLEL PROCESSING (Rayon)                                          │
│    For each file in parallel:                                           │
│    ├─ Check cache (mtime + config hash)                                 │
│    ├─ If cache miss: Parse with SWC → Run enabled rules → Cache result  │
│    └─ Collect issues                                                    │
└─────────────────────────────────────┬───────────────────────────────────┘
                                      │
┌─────────────────────────────────────▼───────────────────────────────────┐
│ 5. POST-PROCESSING                                                      │
│    Branch mode: Filter issues to modified line ranges (git diff hunks)  │
│    Apply disable directives (// tscanner-disable-line)                  │
│    Add line text for context                                            │
└─────────────────────────────────────┬───────────────────────────────────┘
                                      │
┌─────────────────────────────────────▼───────────────────────────────────┐
│ 6. RESPONSE                                                             │
│    VSCode: JSON-RPC → GZIP if large → TreeView update                   │
│    CLI: Format output → Print to stdout → Exit code (0=clean, 1=errors) │
│    Action: Parse JSON → Generate markdown → Post PR comment             │
└─────────────────────────────────────────────────────────────────────────┘
```

## Caching Strategy

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           MEMORY CACHE                                  │
│                    DashMap<PathBuf, CacheEntry>                         │
│                                                                         │
│  CacheEntry {                                                           │
│    mtime: SystemTime,     // File modification time                     │
│    config_hash: u64,      // Config hash at cache time                  │
│    issues: Vec<Issue>,    // Cached issues                              │
│  }                                                                      │
│                                                                         │
│  Validation: mtime matches AND config_hash matches                      │
└─────────────────────────────────────┬───────────────────────────────────┘
                                      │ flush()
                                      ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                            DISK CACHE                                   │
│               ~/.cache/tscanner/cache_{config_hash}.json                │
│                                                                         │
│  Loaded on server start if config hash matches                          │
│  Written after each scan completes                                      │
└─────────────────────────────────────────────────────────────────────────┘

Cache Invalidation:
  • File change: mtime differs from cached entry
  • Config change: config_hash differs (all entries invalid)
  • Hard scan: Manual clear via clearCache RPC
  • Git operations: Extension invalidates git diff cache (30s TTL)
```

## Platform Binary Distribution

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          BINARY SELECTION                               │
└─────────────────────────────────────────────────────────────────────────┘

CLI Package (npm optional dependencies):
┌─────────────────────┬────────────────────────────────────────────────────┐
│ Platform            │ Package                                            │
├─────────────────────┼────────────────────────────────────────────────────┤
│ linux-x64           │ @tscanner/cli-linux-x64                            │
│ linux-arm64         │ @tscanner/cli-linux-arm64                          │
│ darwin-x64          │ @tscanner/cli-darwin-x64                           │
│ darwin-arm64        │ @tscanner/cli-darwin-arm64                         │
│ win32-x64           │ @tscanner/cli-win32-x64                            │
└─────────────────────┴────────────────────────────────────────────────────┘
npm installs only the matching platform package automatically.

VSCode Extension (bundled binaries):
┌─────────────────────────────────────────────────────────────────────────┐
│ out/binaries/                                                           │
│   ├─ tscanner-server-x86_64-unknown-linux-gnu                           │
│   ├─ tscanner-server-aarch64-unknown-linux-gnu                          │
│   ├─ tscanner-server-x86_64-apple-darwin                                │
│   ├─ tscanner-server-aarch64-apple-darwin                               │
│   └─ tscanner-server-x86_64-pc-windows-msvc.exe                         │
└─────────────────────────────────────────────────────────────────────────┘
Extension detects platform and selects appropriate binary at runtime.
```

## Git Integration (Branch Mode)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        BRANCH MODE FLOW                                 │
└─────────────────────────────────────────────────────────────────────────┘

1. Get changed files:
   git diff --name-only origin/main...HEAD
   → ["src/file1.ts", "src/file2.ts", "src/utils/helper.ts"]

2. Get modified line ranges (per file):
   git diff origin/main...HEAD -- src/file1.ts

   @@ -10,5 +10,7 @@         ← Hunk header: start line 10, 7 lines
    unchanged line
   -removed line              ← Removed (don't track)
   +added line 1              ← Line 10 (track)
   +added line 2              ← Line 11 (track)
    unchanged line

   → ModifiedLineRange { startLine: 10, lineCount: 2 }

3. Filter issues:
   Full scan: 200 issues
   Filter to modified ranges: 5 issues (only in changed code)

VSCode Extension: Uses VSCode Git API for diff
CLI/Action: Executes git commands directly
```

## Rule System

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      RULE REGISTRATION                                  │
│                                                                         │
│  // Compile-time registration with inventory crate                      │
│  inventory::submit!(RuleRegistration {                                  │
│      name: "no-explicit-any",                                               │
│      factory: || Arc::new(NoAnyTypeRule),                               │
│  });                                                                    │
│                                                                         │
│  // Runtime collection (no manual registration needed)                  │
│  for registration in inventory::iter::<RuleRegistration> {              │
│      rules.insert(name, factory());                                     │
│  }                                                                      │
└─────────────────────────────────────────────────────────────────────────┘

Rule Types:
┌─────────────┬────────────────────────────────────────────────────────────┐
│ AST Rules   │ Use SWC visitor pattern to analyze TypeScript AST          │
│             │ Examples: no-explicit-any, prefer-const, no-var                │
├─────────────┼────────────────────────────────────────────────────────────┤
│ Regex Rules │ Pattern matching on source text (custom rules)             │
│             │ Defined in config with pattern + message                   │
└─────────────┴────────────────────────────────────────────────────────────┘

Disable Directives:
  // tscanner-disable-file           → Disable all rules for file
  // tscanner-disable-line rule-name → Disable specific rule on this line
  // tscanner-disable-next-line rule → Disable specific rule on next line
```

## Package Dependencies

```
┌──────────────────────┐
│    tscanner-common   │ ← Shared types, constants, utilities
└──────────┬───────────┘
           │
     ┌─────┼─────┬─────────────┐
     │     │     │             │
     ▼     ▼     ▼             ▼
┌────────┐ ┌────┐ ┌──────────────┐ ┌─────────────────┐
│  CLI   │ │Core│ │GitHub Action │ │VSCode Extension │
│        │ │    │ │              │ │                 │
│ Node.js│ │Rust│ │  TypeScript  │ │   TypeScript    │
│ wrapper│ │    │ │   Actions    │ │   VSCode API    │
└───┬────┘ └──┬─┘ └──────┬───────┘ └────────┬────────┘
    │         │          │                  │
    │    ┌────┴────┐     │                  │
    │    │         │     │                  │
    └───►│  Rust   │◄────┘                  │
         │ Binary  │◄───────────────────────┘
         └─────────┘
```

## Performance Characteristics

| Operation | Typical Time | Notes |
|-----------|--------------|-------|
| Cold scan (1000 files) | 2-5s | Parallel with Rayon |
| Cached scan (1000 files) | <500ms | 80-95% cache hit rate |
| Single file update | 50-150ms | Incremental via file watcher |
| JSON-RPC round trip | 10-50ms | Plus scan time |
| GZIP compression | 5-10x reduction | For responses >10KB |
| Git diff (cached) | <10ms | 30s TTL |

## Security Considerations

- **Input Validation:** Zod schemas for all user inputs
- **HTML Escaping:** Code snippets in PR comments escaped
- **No Shell Interpolation:** Command args passed as arrays
- **Token Handling:** GitHub tokens never logged
- **Disable Directives:** Parsed per-file, cannot disable globally
