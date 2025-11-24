<a name="TOC"></a>

<div align="center">
<img width="128" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/logo.png" alt="tscanner Core logo">
<h4>tscanner - Core Engine</h4>
<p>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <br>
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-architecture">Architecture</a> • <a href="#-built-in-rules">Built-in Rules</a> • <a href="#-json-rpc-protocol">JSON-RPC Protocol</a> • <a href="#-performance">Performance</a> • <a href="#-development">Development</a> • <a href="#-license">License</a>
</p>

</div>

<a href="#"><img src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/divider.png" /></a>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

High-performance Rust engine powering [tscanner](https://github.com/lucasvtiradentes/tscanner). Provides blazing-fast TypeScript/TSX code analysis with parallel processing, AST-based validation, and intelligent caching.

The core engine serves as the foundation for multiple tscanner packages: the CLI tool, VS Code extension, and GitHub Action. It delivers consistent scanning results across all platforms with minimal overhead.

## ⭐ Features<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

- **23+ Built-in Rules** - AST-based TypeScript/TSX validation
- **Custom Rules** - Regex pattern matching with custom messages
- **Parallel Processing** - Rayon work-stealing thread pool for maximum throughput
- **Smart Caching** - DashMap concurrent cache with disk persistence
- **Auto-Registration** - Inventory-based rule discovery at compile time
- **JSON-RPC Server** - Line-delimited protocol with GZIP compression
- **File Watching** - Real-time change detection with notify
- **Glob Patterns** - Flexible include/exclude file filtering
- **Inline Disables** - Per-file and per-line disable comments
- **Zero Config** - Sensible defaults with optional customization

## 🏗️ Architecture<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

### Workspace Structure

Rust workspace with 3 crates:

```
packages/core/
├── crates/core/        Core library (Scanner, Parser, Rules, Cache)
├── crates/server/      JSON-RPC server binary
└── crates/cli/         CLI binary
```

<details>
<summary><b>Core Modules</b></summary>

**Scanner (`scanner.rs`)**
- Parallel file processing via Rayon
- Cache integration with automatic invalidation
- Glob pattern matching for file filtering
- Git-aware scanning (branch mode support)

**Parser (`parser.rs`)**
- SWC-based TypeScript/TSX AST parsing
- Source map generation for error reporting
- Syntax error recovery and reporting

**Rule Registry (`registry.rs`)**
- Inventory auto-registration at compile time
- Dynamic rule loading from configuration
- Per-file rule filtering based on glob patterns
- Severity level management (error/warning)

**Cache (`cache.rs`)**
- DashMap concurrent memory cache
- Disk persistence to `~/.cache/tscanner/`
- Mtime + config-hash validation
- Atomic cache updates during scans

**Config (`config.rs`)**
- `.tscanner/config.jsonc` loader
- Built-in and custom rule configuration
- Glob pattern compilation with globset
- Config hash generation for cache invalidation

**File Watcher (`watcher.rs`)**
- Real-time file change detection
- Debounced event handling
- Integration with scanner for incremental updates

**Formatter (`formatter.rs`)**
- Multiple output formats (JSON, pretty, standard)
- Grouping by file or rule
- Relative path conversion
- Color-coded severity levels

</details>

<details>
<summary><b>Communication Flow</b></summary>

```
Extension/CLI          JSON-RPC Protocol       Core Engine
     │                        │                      │
     ├─────── scan() ────────>│                      │
     │                        ├────── Scanner ──────>│
     │                        │                      ├─ Load Config
     │                        │                      ├─ Check Cache
     │                        │                      ├─ Parse Files (SWC)
     │                        │                      ├─ Run Rules (Rayon)
     │                        │                      └─ Update Cache
     │                        │<──── ScanResult ─────┤
     │<──── GZIP:{base64} ────┤                      │
     │                        │                      │
```

</details>

## 📋 Built-in Rules<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

<details>
<summary><b>Type Safety (6)</b></summary>

| Rule | Description |
|------|-------------|
| `no-any-type` | Disallow explicit `: any` type annotations |
| `no-implicit-any` | Disallow implicit `any` from missing types |
| `prefer-type-over-interface` | Prefer type aliases over interfaces |
| `prefer-interface-over-type` | Prefer interfaces over type aliases |
| `no-empty-class` | Disallow empty class declarations |
| `no-unused-vars` | Detect unused variables and imports |

</details>

<details>
<summary><b>Code Quality (10)</b></summary>

| Rule | Description |
|------|-------------|
| `no-console-log` | Disallow `console.log` statements |
| `no-var` | Disallow `var` keyword (prefer let/const) |
| `prefer-const` | Prefer `const` over `let` when not reassigned |
| `no-magic-numbers` | Disallow magic numbers (require named constants) |
| `consistent-return` | Enforce consistent return statements |
| `no-empty-function` | Disallow empty function bodies |
| `no-nested-ternary` | Disallow nested ternary expressions |
| `no-todo-comments` | Detect TODO/FIXME comments |
| `max-function-length` | Enforce maximum function length |
| `no-constant-condition` | Disallow constant conditions in loops |

</details>

<details>
<summary><b>Imports (5)</b></summary>

| Rule | Description |
|------|-------------|
| `no-relative-imports` | Disallow relative imports (`./`, `../`) |
| `no-absolute-imports` | Disallow absolute imports from root |
| `no-alias-imports` | Disallow aliased imports (`@/`, `~/`) |
| `no-duplicate-imports` | Disallow duplicate imports from same module |
| `no-nested-require` | Disallow nested require() calls |

</details>

<details>
<summary><b>Advanced (2)</b></summary>

| Rule | Description |
|------|-------------|
| `no-unreachable-code` | Detect unreachable code after return/throw |
| `no-dynamic-import` | Disallow dynamic `import()` expressions |

**Note:** Custom regex rules can be defined in configuration for additional validation.

</details>

## 🔌 JSON-RPC Protocol<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

<details>
<summary><b>Transport & Methods</b></summary>

Line-delimited JSON over stdin/stdout with optional GZIP compression:

```
Request:  {"id": 1, "method": "scan", "params": {...}}
Response: {"id": 1, "result": {...}}
```

**Compression:**
- Automatic GZIP for results > 10KB
- Base64 encoding for transport
- Marker format: `GZIP:{base64-encoded-data}`

**Methods:**

| Method | Description | Params |
|--------|-------------|--------|
| `scan` | Scan workspace with config | `root`, `config?`, `branch?` |
| `scanFile` | Re-scan single file | `root`, `file` |
| `scanContent` | Analyze in-memory content | `root`, `file`, `content`, `config?` |
| `watch` | Start file watcher | `root` |
| `getRulesMetadata` | Get all rule definitions | - |
| `clearCache` | Invalidate cache | - |
| `formatResults` | Format scan results | `root`, `results`, `group_mode` |

</details>

<details>
<summary><b>Response Types</b></summary>

**Scan Result:**
```json
{
  "files": [
    {
      "path": "src/index.ts",
      "issues": [
        {
          "rule": "no-any-type",
          "message": "Found ': any' type annotation",
          "severity": "error",
          "line": 5,
          "column": 10,
          "endLine": 5,
          "endColumn": 13
        }
      ]
    }
  ],
  "summary": {
    "totalFiles": 1,
    "totalIssues": 1,
    "errorCount": 1,
    "warningCount": 0
  }
}
```

**Rule Metadata:**
```json
{
  "rules": [
    {
      "name": "no-any-type",
      "description": "Disallow explicit ': any' type annotations",
      "category": "TypeSafety",
      "severity": "error",
      "type": "ast"
    }
  ]
}
```

</details>

## 📊 Performance<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

<details>
<summary><b>Optimization Details</b></summary>

**Parallelism:**
- Rayon work-stealing thread pool
- CPU core detection and utilization
- Lock-free data structures (DashMap)

**Caching:**
- Mtime-based invalidation
- Config hash tracking
- 80-95% cache hit rate in typical workflows
- Sub-millisecond cache lookups

**Optimization:**
- LTO (Link-Time Optimization) enabled
- Stripped binaries for minimal size
- Release profile: `opt-level = 3`, `codegen-units = 1`

**Typical Performance:**
- 100-500 files: <1s
- 1000-2000 files: 1-3s
- 5000+ files: 5-10s (cached), 15-30s (cold)

</details>

## 🔧 Development<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

<details>
<summary><b>Build Commands</b></summary>

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Watch mode (requires cargo-watch)
cargo watch -x build

# Generate config schema
cargo run --bin generate_schema
```

**Run Binaries:**

```bash
# Server
cargo run --bin tscanner-server

# CLI
cargo run --bin tscanner -- check /path/to/project
```

</details>

<details>
<summary><b>Adding New Rules</b></summary>

1. Create `crates/core/src/rules/my_rule.rs`:

```rust
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleRegistration};
use inventory;

pub struct MyRule;

impl Rule for MyRule {
    fn name(&self) -> &str { "my-rule" }
    fn description(&self) -> &str { "Rule description" }
    fn category(&self) -> RuleCategory { RuleCategory::CodeQuality }
    fn check(&self, ctx: &RuleContext) -> Vec<Issue> { /* ... */ }
}

inventory::submit!(RuleRegistration {
    name: "my-rule",
    factory: || Arc::new(MyRule),
});
```

2. Add to `crates/core/src/rules/mod.rs`:

```rust
mod my_rule;
```

3. Rebuild - rule auto-registers via inventory

</details>

<details>
<summary><b>Testing</b></summary>

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p core
cargo test -p cli
cargo test -p server

# Run with output
cargo test -- --nocapture
```

</details>

<details>
<summary><b>Configuration Schema</b></summary>

The `generate_schema` binary creates JSON Schema for `.tscanner/config.jsonc`:

```bash
cargo run --bin generate_schema > schema.json
```

Used by VS Code extension for autocomplete and validation.

</details>

## 📜 License<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

MIT License - see [LICENSE](../../LICENSE) file for details.
