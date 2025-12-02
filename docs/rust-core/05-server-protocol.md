# JSON-RPC Server Protocol

## Server Overview

**Binary:** `tscanner-server`

**Communication:**
- Protocol: JSON-RPC over stdin/stdout
- Format: Line-delimited JSON
- Compression: All responses GZIP compressed (base64 encoded)

## Request Format

```json
{"id": 1, "method": "scan", "params": {...}}
```

## Response Format

```
GZIP:{base64-encoded-gzip-data}
```

Decompression flow:
```
stdout line → strip "GZIP:" prefix → base64 decode → gunzip → JSON parse
```

## RPC Methods

| Method | Params | Description |
|--------|--------|-------------|
| `scan` | `root`, `config?`, `branch?` | Full workspace scan with optional branch mode |
| `scanFile` | `root`, `file` | Single file scan for incremental updates |
| `scanContent` | `root`, `file`, `content`, `config?` | Scan in-memory content (unsaved changes) |
| `getRulesMetadata` | - | Get all available rules with metadata |
| `watch` | `root` | Start file system watcher |
| `clearCache` | - | Invalidate entire file cache |
| `formatResults` | `root`, `results`, `group_mode` | Format scan results for clipboard export |

### scan

```json
{
  "id": 1,
  "method": "scan",
  "params": {
    "root": "/path/to/workspace",
    "config": {...},
    "branch": "main"
  }
}
```

**Branch mode:** If `branch` provided, only scans files in `git diff --name-only <branch>`.

### scanFile

```json
{
  "id": 2,
  "method": "scanFile",
  "params": {
    "root": "/path/to/workspace",
    "file": "src/app.ts"
  }
}
```

### scanContent

```json
{
  "id": 3,
  "method": "scanContent",
  "params": {
    "root": "/path/to/workspace",
    "file": "src/app.ts",
    "content": "const x: any = 1;",
    "config": {...}
  }
}
```

### getRulesMetadata

```json
{
  "id": 4,
  "method": "getRulesMetadata",
  "params": {}
}
```

Response includes rule names, severity defaults, descriptions.

### watch

```json
{
  "id": 5,
  "method": "watch",
  "params": {
    "root": "/path/to/workspace"
  }
}
```

Starts file watcher for `.{ts,tsx,js,jsx}` files.

### formatResults

```json
{
  "id": 6,
  "method": "formatResults",
  "params": {
    "root": "/path/to/workspace",
    "results": [...],
    "group_mode": "rule"
  }
}
```

Formats scan results as markdown or plain text.

## Server State

```rust
struct ServerState {
    scanner: Option<Scanner>,
    watcher: Option<FileWatcher>,
    cache: Arc<FileCache>,
}
```

State persists across requests:
- Scanner reused for incremental scans
- Watcher sends notifications for file changes
- Cache shared across all operations

## File Watcher Notifications

After calling `watch`, server sends notifications on file changes:

```json
{
  "method": "file_updated",
  "params": {
    "file": "src/app.ts",
    "issues": [...]
  }
}
```

Notifications sent for:
- Modified files
- Created files
- Deleted files (empty issues array)

No `id` field (notification, not request).

## GZIP Compression Flow

```
Response object
  ↓ serde_json::to_string()
JSON string
  ↓ flate2::GzEncoder
GZIP bytes
  ↓ base64::encode()
Base64 string
  ↓ prepend "GZIP:"
"GZIP:{base64-data}"
  ↓ println!()
stdout
```

## Handler Implementation

**File structure:**
```
crates/tscanner_server/src/handlers/
├── scan.rs                 - Full workspace scan
├── scan_file.rs            - Single file scan
├── scan_content.rs         - In-memory content scan
├── get_rules_metadata.rs   - Rule catalog
├── watch.rs                - File watcher setup
├── clear_cache.rs          - Cache invalidation
└── format_results.rs       - Output formatting
```

Each handler:
1. Extracts params from JSON-RPC request
2. Calls core library functions
3. Returns result (automatically GZIP compressed)

## See Also

- [Scanner Flow](03-scanner-flow.md) - How scanning works
- [Caching](04-caching.md) - Cache behavior and invalidation
