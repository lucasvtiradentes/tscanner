<div align="center">
<h3>tscanner Core</h3>
<p>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
</p>
</div>

---

## 🎺 Overview

High-performance Rust engine powering tscanner. Provides TypeScript/TSX scanning with parallel processing, AST analysis, and smart caching.

## 📦 Structure

Rust workspace with 3 crates:

```
packages/core/
├── crates/core/        Core library (Scanner, Parser, Rules, Cache)
├── crates/server/      JSON-RPC server binary
└── crates/cli/         CLI binary
```

## 🏗️ Architecture

**Core Modules:**
- **Scanner** - Parallel file processing via Rayon
- **Parser** - SWC-based TypeScript/TSX AST parsing
- **Rule Registry** - Inventory auto-registration (24 built-in rules)
- **Cache** - DashMap concurrent memory + disk persistence
- **Config** - `.tscanner/rules.json` with glob pattern matching

**JSON-RPC Protocol:**
```
Request:  {"id": 1, "method": "scan", "params": {...}}
Response: {"id": 1, "result": {...}}
```

**GZIP compression** for large results with `GZIP:{base64}` marker.

## 📋 Built-in Rules (24)

**Type Safety (6):**
- `no-any-type`, `no-implicit-any`, `prefer-type-over-interface`
- `prefer-interface-over-type`, `no-empty-class`, `no-unused-vars`

**Code Quality (10):**
- `no-console-log`, `no-var`, `prefer-const`, `no-magic-numbers`
- `consistent-return`, `no-empty-function`, `no-nested-ternary`
- `no-todo-comments`, `max-function-length`, `no-constant-condition`

**Imports (5):**
- `no-relative-imports`, `no-absolute-imports`, `no-alias-imports`
- `no-duplicate-imports`, `no-nested-require`

**Advanced (3):**
- `no-unreachable-code`, `no-dynamic-import`, `custom-regex`

## 🔌 JSON-RPC Methods

| Method | Description |
|--------|-------------|
| `scan` | Full workspace scan with caching |
| `scanFile` | Single file re-scan |
| `scanContent` | Analyze in-memory content |
| `watch` | Start file watcher |
| `getRulesMetadata` | Get all rule definitions |
| `clearCache` | Invalidate cache |

## 📊 Performance

- **Parallel:** Rayon work-stealing thread pool
- **Cache:** Mtime + config-hash validation (80-95% hit rate)
- **Optimization:** LTO enabled, stripped binaries
- **Typical:** 100-500 files in <1s

## 🔧 Development

```bash
cargo build
cargo build --release
cargo test
cargo watch -x build
```

**Run server:**
```bash
cargo run --bin tscanner-server
```

**Add new rule:**
1. Create `crates/core/src/rules/my_rule.rs`
2. Implement `Rule` trait
3. Add `inventory::submit!` for auto-registration
4. Add to `mod.rs`

## 📜 License

MIT License - see [LICENSE](../../LICENSE)
