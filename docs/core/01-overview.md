# Rust Core Architecture Overview

## Workspace Structure

The Rust core is organized as a Cargo workspace with three crates:

```
packages/core/
├── crates/
│   ├── core/      # Core library (scanner, parser, rules, cache)
│   ├── server/    # JSON-RPC server for VSCode extension
│   └── cli/       # CLI binary (planned)
├── Cargo.toml     # Workspace configuration
└── Cargo.lock
```

### Binary Outputs

- `tscanner` - Standalone CLI binary (from cli)
- `tscanner-server` - JSON-RPC server for VSCode extension (from server)

## Key Dependencies

### Core Dependencies (core)

| Dependency | Purpose |
|------------|---------|
| `swc_ecma_parser` | Parse TypeScript/JavaScript source into AST |
| `swc_ecma_ast` | AST node types and visitor pattern |
| `swc_ecma_visit` | AST traversal and pattern matching |
| `rayon` | Parallel iterator for multi-threaded file scanning |
| `dashmap` | Concurrent HashMap for thread-safe caching |
| `inventory` | Compile-time rule registration macro |
| `notify` | File system event monitoring |
| `globset` | Fast glob pattern matching |
| `serde` / `serde_json` | Serialization for config and cache |

### Server Dependencies (server)

| Dependency | Purpose |
|------------|---------|
| `flate2` | GZIP compression for large scan results |
| `base64` | Encode compressed data for JSON transport |

### CLI Dependencies (cli)

| Dependency | Purpose |
|------------|---------|
| `clap` | Command-line argument parsing |

### Testing Dependencies

| Dependency | Purpose |
|------------|---------|
| `insta` | Snapshot testing for AST rules |

## Module Responsibilities

### core

**scanner.rs**
Orchestrates parallel file scanning using Rayon. Loads configuration, matches files against include/exclude patterns, checks cache for valid entries, parses files with SWC, and runs rules. Returns aggregated issues.

**parser.rs**
SWC-based TypeScript/TSX parser. Converts source text to AST with error recovery. Supports TypeScript, JSX, decorators, and latest ECMAScript features.

**registry.rs**
Global rule registry using `inventory` for compile-time registration. Rules implement `RuleImpl` trait and register via `register_rule!` macro. Provides `get_all_rules()` and `get_enabled_rules()`.

**cache.rs**
DashMap-based concurrent cache with disk persistence. Stores (mtime, config_hash, issues) per file. Loads from disk on startup, flushes after each scan. Cache invalidation on file change or config change.

**config.rs**
Loads and parses `.tscanner/rules.jsonc`. Computes config hash for cache invalidation. Validates rule configuration (enabled, severity, include/exclude patterns).

**watcher.rs**
File system watcher using `notify`. Monitors workspace for file create/modify/delete events. Filters events by include/exclude patterns. Sends invalidation events to extension.

**rules/**
13+ built-in rules implementing AST analysis and regex patterns. Each rule:
- Implements `RuleImpl` trait
- Registers via `register_rule!`
- Provides metadata (name, severity, message)
- Implements `check()` method using SWC visitors

See [Rule System](02-rule-system.md) for details on rule implementation.

### server

**main.rs**
JSON-RPC server with line-delimited protocol. Methods:
- `scan` - Full workspace scan with config loading
- `watch` - Start file watcher
- `scanFile` - Single file scan
- `getRulesMetadata` - List all rules

GZIP compresses responses >1KB using marker protocol: `GZIP:{base64-data}`.

See [Server Protocol](05-server-protocol.md) for JSON-RPC specification.

### cli

**main.rs**
CLI interface using `clap`. Commands:
- `scan` - Scan workspace
- `watch` - Watch and re-scan on changes
- `rules` - List available rules
- `config` - Initialize/validate configuration

## Data Flow

```
1. VSCode Extension → JSON-RPC → server
2. server → config.rs → Load .tscanner/config.jsonc
3. server → scanner.rs → Parallel scan
4. scanner.rs → cache.rs → Check cached results
5. scanner.rs → parser.rs → Parse uncached files (SWC)
6. parser.rs → rules/* → Run enabled rules on AST
7. scanner.rs → Aggregate issues → cache.rs
8. cache.rs → Flush to disk
9. server → GZIP compress → JSON-RPC → Extension
```

See [Scanner Flow](03-scanner-flow.md) for detailed execution flow.

## Performance Characteristics

- **Parallel Scanning**: Rayon thread pool (default: CPU count)
- **Caching**: O(1) lookup via DashMap, skips unchanged files
- **Compression**: 70-90% size reduction for large results
- **Incremental Updates**: File watcher enables delta scans

See [Caching](04-caching.md) for cache implementation details.

## Testing Strategy

- **Unit Tests**: Per-module tests for scanner, parser, cache, config
- **Integration Tests**: Full scan workflow with fixture files
- **Snapshot Tests**: AST rule outputs using `insta`
- **Benchmarks**: Scanner performance on large codebases

See [Testing](06-testing.md) for test organization and best practices.
