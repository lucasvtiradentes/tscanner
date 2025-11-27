<a name="TOC"></a>

<div align="center">
  <img height="80" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-logo.png" alt="tscanner logo">
  <div><strong>TScanner - Core Engine</strong></div>
  <br />
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-architecture">Architecture</a> • <a href="#-configuration">Configuration</a> • <a href="#-rules">Rules</a> • <a href="#-json-rpc-protocol">JSON-RPC Protocol</a> • <a href="#-performance">Performance</a> • <a href="#-development">Development</a> • <a href="#-inspirations">Inspirations</a> • <a href="#-license">License</a>
</div>

<a href="#"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" /></a>

## 🎺 Overview<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

High-performance Rust engine powering [tscanner](https://github.com/lucasvtiradentes/tscanner). Provides blazing-fast TypeScript/TSX code analysis with parallel processing, AST-based validation, and intelligent caching.

The core engine serves as the foundation for multiple tscanner packages: the CLI tool, VS Code extension, and GitHub Action. It delivers consistent scanning results across all platforms with minimal overhead.

## ⭐ Features<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- **39 Built-in Rules** - AST-based TypeScript/TSX validation
- **Custom Rules** - Regex pattern matching with custom messages
- **Parallel Processing** - Rayon work-stealing thread pool for maximum throughput
- **Smart Caching** - DashMap concurrent cache with disk persistence
- **Auto-Registration** - Inventory-based rule discovery at compile time
- **JSON-RPC Server** - Line-delimited protocol with GZIP compression
- **File Watching** - Real-time change detection with notify
- **Glob Patterns** - Flexible include/exclude file filtering
- **Inline Disables** - Per-file and per-line disable comments
- **Zero Config** - Sensible defaults with optional customization

## 🏗️ Architecture<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

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

## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Create `.tscanner/config.jsonc`:

<!-- <DYNFIELD:DEFAULT_CONFIG> -->
```json
{
  "$schema": "https://unpkg.com/tscanner@0.0.20/schema.json",
  "builtinRules": {
    "no-any-type": {}
  },
  "customRules": {},
  "include": [
    "**/*.ts",
    "**/*.tsx"
  ],
  "exclude": [
    "**/node_modules/**",
    "**/dist/**",
    "**/build/**",
    "**/.git/**"
  ]
}
```
<!-- </DYNFIELD:DEFAULT_CONFIG> -->

<!-- <DYNFIELD:RULES> -->
## 📋 Rules<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

### Built-in Rules (39)

<details>
<summary><b>Type Safety (6)</b></summary>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>no-any-type</code></td>
    <td align="left">Detects usage of TypeScript 'any' type (<code>: any</code> and <code>as any</code>). Using 'any' defeats the purpose of TypeScript's type system.</td>
  </tr>
  <tr>
    <td align="left"><code>no-implicit-any</code></td>
    <td align="left">Detects function parameters without type annotations that implicitly have 'any' type.</td>
  </tr>
  <tr>
    <td align="left"><code>no-inferrable-types</code></td>
    <td align="left">Disallows explicit type annotations on variables initialized with literal values. TypeScript can infer these types automatically.</td>
  </tr>
  <tr>
    <td align="left"><code>no-non-null-assertion</code></td>
    <td align="left">Disallows the non-null assertion operator (!). Use proper null checks or optional chaining instead.</td>
  </tr>
  <tr>
    <td align="left"><code>no-single-or-array-union</code></td>
    <td align="left">Disallows union types that combine a type with its array form (e.g., <code>string | string[]</code>, <code>number | number[]</code>). Prefer using a consistent type to avoid handling multiple cases in function implementations.</td>
  </tr>
  <tr>
    <td align="left"><code>no-unnecessary-type-assertion</code></td>
    <td align="left">Disallows type assertions on values that are already of the asserted type (e.g., "hello" as string, 123 as number).</td>
  </tr>
</table>

</details>

<details>
<summary><b>Code Quality (14)</b></summary>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>max-function-length</code></td>
    <td align="left">Enforces a maximum number of statements in functions (default: 50). Long functions are harder to understand and maintain.</td>
  </tr>
  <tr>
    <td align="left"><code>max-params</code></td>
    <td align="left">Limits the number of parameters in a function. Functions with many parameters should use an options object instead.</td>
  </tr>
  <tr>
    <td align="left"><code>no-async-without-await</code></td>
    <td align="left">Disallows async functions that don't use await. The async keyword is unnecessary if await is never used.</td>
  </tr>
  <tr>
    <td align="left"><code>no-console-log</code></td>
    <td align="left">Finds console.log() statements in code. Console statements should be removed before committing to production.</td>
  </tr>
  <tr>
    <td align="left"><code>no-else-return</code></td>
    <td align="left">Disallows else blocks after return statements. The else is unnecessary since the function already returned.</td>
  </tr>
  <tr>
    <td align="left"><code>no-empty-class</code></td>
    <td align="left">Disallows empty classes without methods or properties.</td>
  </tr>
  <tr>
    <td align="left"><code>no-empty-function</code></td>
    <td align="left">Disallows empty functions and methods. Empty functions are often leftovers from incomplete code.</td>
  </tr>
  <tr>
    <td align="left"><code>no-empty-interface</code></td>
    <td align="left">Disallows empty interface declarations. Empty interfaces are equivalent to {} and usually indicate incomplete code.</td>
  </tr>
  <tr>
    <td align="left"><code>no-magic-numbers</code></td>
    <td align="left">Detects magic numbers in code (literals other than 0, 1, -1). Use named constants instead for better readability and maintainability.</td>
  </tr>
  <tr>
    <td align="left"><code>no-nested-ternary</code></td>
    <td align="left">Disallows nested ternary expressions. Nested ternaries are hard to read and should be replaced with if-else statements.</td>
  </tr>
  <tr>
    <td align="left"><code>no-return-await</code></td>
    <td align="left">Disallows redundant 'return await' in async functions. The await is unnecessary since the function already returns a Promise.</td>
  </tr>
  <tr>
    <td align="left"><code>no-todo-comments</code></td>
    <td align="left">Detects TODO, FIXME, and similar comment markers.</td>
  </tr>
  <tr>
    <td align="left"><code>no-unused-vars</code></td>
    <td align="left">Detects variables that are declared but never used in the code.</td>
  </tr>
  <tr>
    <td align="left"><code>no-useless-catch</code></td>
    <td align="left">Disallows catch blocks that only rethrow the caught error. Remove the try-catch or add meaningful error handling.</td>
  </tr>
</table>

</details>

<details>
<summary><b>Bug Prevention (4)</b></summary>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>consistent-return</code></td>
    <td align="left">Requires consistent return behavior in functions. Either all code paths return a value or none do.</td>
  </tr>
  <tr>
    <td align="left"><code>no-constant-condition</code></td>
    <td align="left">Disallows constant expressions in conditions (if/while/for/ternary). Likely a programming error.</td>
  </tr>
  <tr>
    <td align="left"><code>no-floating-promises</code></td>
    <td align="left">Disallows floating promises (promises used as statements without await, .then(), or .catch()). Unhandled promises can lead to silent failures.</td>
  </tr>
  <tr>
    <td align="left"><code>no-unreachable-code</code></td>
    <td align="left">Detects code after return, throw, break, or continue statements. This code will never execute.</td>
  </tr>
</table>

</details>

<details>
<summary><b>Variables (3)</b></summary>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>no-shadow</code></td>
    <td align="left">Disallows variable declarations that shadow variables in outer scopes. Shadowing can lead to confusing code and subtle bugs.</td>
  </tr>
  <tr>
    <td align="left"><code>no-var</code></td>
    <td align="left">Disallows the use of 'var' keyword. Use 'let' or 'const' instead for block-scoped variables.</td>
  </tr>
  <tr>
    <td align="left"><code>prefer-const</code></td>
    <td align="left">Suggests using 'const' instead of 'let' when variables are never reassigned.</td>
  </tr>
</table>

</details>

<details>
<summary><b>Imports (8)</b></summary>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>no-absolute-imports</code></td>
    <td align="left">Disallows absolute imports without alias. Prefer relative or aliased imports.</td>
  </tr>
  <tr>
    <td align="left"><code>no-alias-imports</code></td>
    <td align="left">Disallows aliased imports (starting with @). Prefer relative imports.</td>
  </tr>
  <tr>
    <td align="left"><code>no-default-export</code></td>
    <td align="left">Disallows default exports. Named exports are preferred for better refactoring support and explicit imports.</td>
  </tr>
  <tr>
    <td align="left"><code>no-duplicate-imports</code></td>
    <td align="left">Disallows multiple import statements from the same module. Merge them into a single import.</td>
  </tr>
  <tr>
    <td align="left"><code>no-dynamic-import</code></td>
    <td align="left">Disallows dynamic import() expressions. Dynamic imports make static analysis harder and can impact bundle optimization.</td>
  </tr>
  <tr>
    <td align="left"><code>no-forwarded-exports</code></td>
    <td align="left">Disallows re-exporting from other modules. This includes direct re-exports (export { X } from 'module'), star re-exports (export * from 'module'), and re-exporting imported values.</td>
  </tr>
  <tr>
    <td align="left"><code>no-nested-require</code></td>
    <td align="left">Disallows require() calls inside functions, blocks, or conditionals. Require statements should be at the top level for static analysis.</td>
  </tr>
  <tr>
    <td align="left"><code>no-relative-imports</code></td>
    <td align="left">Detects relative imports (starting with './' or '../'). Prefer absolute imports with @ prefix for better maintainability.</td>
  </tr>
</table>

</details>

<details>
<summary><b>Style (4)</b></summary>

<table>
  <tr>
    <th width="250">Rule</th>
    <th width="500">Description</th>
  </tr>
  <tr>
    <td align="left"><code>prefer-interface-over-type</code></td>
    <td align="left">Suggests using 'interface' keyword instead of 'type' for consistency.</td>
  </tr>
  <tr>
    <td align="left"><code>prefer-nullish-coalescing</code></td>
    <td align="left">Suggests using nullish coalescing (??) instead of logical OR (||) for default values. The || operator treats 0, "", and false as falsy, which may not be intended.</td>
  </tr>
  <tr>
    <td align="left"><code>prefer-optional-chain</code></td>
    <td align="left">Suggests using optional chaining (?.) instead of logical AND (&&) chains for null checks.</td>
  </tr>
  <tr>
    <td align="left"><code>prefer-type-over-interface</code></td>
    <td align="left">Suggests using 'type' keyword instead of 'interface' for consistency. Type aliases are more flexible and composable.</td>
  </tr>
</table>

</details>


<!-- </DYNFIELD:RULES> -->

## 🔌 JSON-RPC Protocol<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

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

## 📊 Performance<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

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

## 🔧 Development<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

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

## 💡 Inspirations<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- [Biome](https://github.com/biomejs/biome) - High-performance Rust-based linter and formatter for web projects
- [VSCode Bookmarks](https://github.com/alefragnani/vscode-bookmarks) - Bookmarks Extension for Visual Studio Code

## 📜 License<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

MIT License - see [LICENSE](../../LICENSE) file for details.

<a href="#"><img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" /></a>

<div align="center">
  <div>
    <a target="_blank" href="https://www.linkedin.com/in/lucasvtiradentes/"><img src="https://img.shields.io/badge/-linkedin-blue?logo=Linkedin&logoColor=white" alt="LinkedIn"></a>
    <a target="_blank" href="mailto:lucasvtiradentes@gmail.com"><img src="https://img.shields.io/badge/gmail-red?logo=gmail&logoColor=white" alt="Gmail"></a>
    <a target="_blank" href="https://x.com/lucasvtiradente"><img src="https://img.shields.io/badge/-X-black?logo=X&logoColor=white" alt="X"></a>
    <a target="_blank" href="https://github.com/lucasvtiradentes"><img src="https://img.shields.io/badge/-github-gray?logo=Github&logoColor=white" alt="Github"></a>
  </div>
</div>
