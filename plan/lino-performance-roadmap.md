# Lino Performance Roadmap: From MVP to Biome-Level Performance

## Executive Summary

This document outlines a strategic plan to evolve Lino from its current MVP (simple regex-based TypeScript `any` finder) into a high-performance, extensible linting platform capable of matching Biome's performance characteristics while supporting custom user-defined rules.

**Current State:**
- VSCode extension in TypeScript
- Simple regex pattern matching (`: any`, `as any`)
- Chunk-based parallel processing (10 files at a time)
- No incremental updates
- No proper AST analysis

**Target State:**
- Sub-200ms analysis for 500+ file codebases
- Custom rule engine with regex and AST-based rules
- Incremental updates during editing
- Proper TypeScript AST integration
- Multi-threaded processing
- Smart caching and file watching

## Strategic Architecture Options

### Option 1: Pure TypeScript Optimization (Evolutionary)

**Pros:**
- Maintains current codebase
- Easier for contributors
- VSCode API integration remains simple
- Incremental improvements

**Cons:**
- Fundamental Node.js performance ceiling (~4-7x slower than Rust)
- No true multi-threading (workers are expensive)
- GC pauses inevitable
- Hard to achieve <200ms on large codebases

**Verdict:** Only viable for small-to-medium codebases (<100 files)

### Option 2: Hybrid Architecture (Recommended)

**Core Concept:** Rust binary for heavy lifting + TypeScript extension for VSCode integration

**Architecture:**
```
┌─────────────────────────────────────────┐
│  VSCode Extension (TypeScript)           │
│  - UI/UX integration                     │
│  - Configuration management              │
│  - Tree view rendering                   │
│  - Command handling                      │
└─────────────┬───────────────────────────┘
              │ stdin/stdout + JSON-RPC
┌─────────────▼───────────────────────────┐
│  Lino Core (Rust Binary)                 │
│  - File traversal & watching             │
│  - TypeScript parsing (SWC/tree-sitter)  │
│  - Rule engine                           │
│  - Multi-threaded analysis               │
│  - Incremental caching                   │
└──────────────────────────────────────────┘
```

**Pros:**
- Best of both worlds: Rust performance + TypeScript integration
- Achievable performance parity with Biome
- Can evolve core independently
- Enables CLI usage outside VSCode

**Cons:**
- More complex build system
- Need to maintain two codebases
- Cross-platform binary distribution

**Verdict:** Best long-term architecture for serious performance

### Option 3: Full Rust Rewrite (Revolutionary)

**Pros:**
- Maximum performance potential
- Clean slate architecture
- Native LSP integration possible

**Cons:**
- Complete rewrite required
- Longer development time
- VSCode extension still needs TypeScript glue
- May be overkill for project scope

**Verdict:** Consider if Lino becomes a general-purpose linter platform

## Recommended Approach: Hybrid Architecture

### Phase 0: Monorepo Setup

**Goal:** Transform current single-repo into a pnpm-managed monorepo structure

#### Why Monorepo?

**Critical Decision:** The hybrid architecture MUST be implemented as a monorepo. This is non-negotiable for the following reasons:

1. **Synchronized Versioning**
   - Single source of truth for version numbers
   - Coordinated releases (Rust binary + VSCode extension released together)
   - No version mismatch issues between components
   - Atomic protocol changes (JSON-RPC updates in single commit)

2. **Simplified Development Workflow**
   - Clone once, work on everything
   - Test integration immediately
   - Single CI/CD pipeline
   - Unified build and release process

3. **Protocol Changes Are Atomic**
   ```
   Single commit touches both sides:
   packages/lino-core/src/protocol.rs    # Rust side
   packages/vscode-extension/src/rpc.ts  # TypeScript side
   ```

4. **Real-World Validation**
   - Biome itself is a monorepo (90+ Rust crates + NPM packages)
   - Rome (predecessor) was a monorepo
   - OXC (Rust linter) is a monorepo
   - SWC (Rust transpiler) is a monorepo
   - Deno (Rust + TypeScript) is a monorepo

#### 0.1 Monorepo Structure

```
lino/                           # Root
├── .github/
│   └── workflows/
│       ├── rust.yml            # Rust CI
│       ├── extension.yml       # VSCode extension CI
│       └── release.yml         # Build all platforms + publish
├── packages/
│   ├── lino-core/              # Rust workspace
│   │   ├── Cargo.toml          # Workspace manifest
│   │   ├── crates/
│   │   │   ├── lino_core/      # Core library
│   │   │   ├── lino_cli/       # CLI binary
│   │   │   └── lino_server/    # JSON-RPC server
│   │   └── tests/              # Integration tests
│   └── vscode-extension/       # TypeScript extension
│       ├── package.json
│       ├── binaries/           # Downloaded/built binaries
│       ├── src/
│       ├── scripts/
│       │   └── postinstall.js  # Binary download
│       └── tests/
├── scripts/
│   ├── setup-dev.sh            # One-command dev setup
│   ├── build-all.sh            # Build both Rust + extension
│   ├── build-binaries.sh       # Cross-compile Rust for all platforms
│   └── release.sh              # Coordinated release
├── docs/
│   ├── architecture.md         # System architecture
│   ├── protocol.md             # JSON-RPC protocol spec
│   ├── contributing.md         # Contribution guide
│   └── development.md          # Dev environment setup
├── plan/                       # Keep existing plans
│   ├── biome-architecture-analysis.md
│   └── lino-performance-roadmap.md
├── pnpm-workspace.yaml         # pnpm workspace config
├── package.json                # Root package.json (scripts only)
└── README.md                   # Single entry point
```

#### 0.2 pnpm Workspace Configuration

**`pnpm-workspace.yaml`:**
```yaml
packages:
  - 'packages/*'
```

**Root `package.json`:**
```json
{
  "name": "lino",
  "private": true,
  "version": "1.0.0",
  "scripts": {
    "dev": "pnpm --filter vscode-extension dev",
    "build": "pnpm --filter vscode-extension build",
    "build:rust": "./scripts/build-binaries.sh",
    "build:all": "pnpm build:rust && pnpm build",
    "test": "pnpm --recursive test",
    "clean": "pnpm --recursive clean"
  },
  "devDependencies": {
    "typescript": "^5.7.3"
  }
}
```

**`packages/vscode-extension/package.json`:**
```json
{
  "name": "lino-vscode",
  "version": "1.0.0",
  "scripts": {
    "dev": "esbuild src/extension.ts --bundle --outfile=dist/extension.js --watch",
    "build": "node scripts/postinstall.js && esbuild src/extension.ts --bundle --outfile=dist/extension.js --minify",
    "postinstall": "node scripts/postinstall.js"
  }
}
```

#### 0.3 Migration Steps

**Step 1: Initialize monorepo structure**
```bash
# Create directory structure
mkdir -p packages/{lino-core,vscode-extension}
mkdir -p scripts docs

# Move existing extension code
mv src packages/vscode-extension/
mv package.json packages/vscode-extension/
mv tsconfig.json packages/vscode-extension/

# Create workspace files
cat > pnpm-workspace.yaml <<EOF
packages:
  - 'packages/*'
EOF
```

**Step 2: Setup Rust workspace**
```bash
cd packages/lino-core
cargo init --name lino_core crates/lino_core
cargo init --name lino_cli --bin crates/lino_cli
cargo init --name lino_server --bin crates/lino_server
```

**Step 3: Create helper scripts**
```bash
# scripts/setup-dev.sh
#!/bin/bash
pnpm install
cd packages/lino-core && cargo build

# scripts/build-all.sh
#!/bin/bash
./scripts/build-binaries.sh
pnpm --filter vscode-extension build
```

#### 0.4 Development Workflow

**Initial setup:**
```bash
./scripts/setup-dev.sh
```

**Development (watch mode):**
```bash
# Terminal 1: Rust auto-rebuild
cd packages/lino-core
cargo watch -x build

# Terminal 2: Extension auto-rebuild
cd packages/vscode-extension
pnpm dev
```

**Testing integration:**
```bash
# Extension tests automatically use local Rust binary
cd packages/vscode-extension
pnpm test  # Uses packages/lino-core/target/debug/lino-server
```

**Release:**
```bash
./scripts/release.sh v1.0.0
# → Builds all platform binaries
# → Bundles into VSIX with binaries
# → Creates GitHub release
# → Publishes to VSCode marketplace
```

#### 0.5 Benefits Summary

| Aspect | Separate Repos | Monorepo |
|--------|----------------|----------|
| Setup | Clone 2 repos | Clone once |
| Version sync | Manual coordination | Automatic |
| Protocol changes | 2 PRs, risk of mismatch | 1 PR, atomic |
| CI/CD | 2 separate pipelines | Unified pipeline |
| Testing integration | Complex, version conflicts | Direct, local binaries |
| Release | Coordinate 2 releases | Single coordinated release |

**Decision: Monorepo is mandatory for hybrid architecture success.**

### Phase 1: Foundation (MVP → Hybrid)

**Goal:** Establish Rust core with basic parity to current functionality

#### 1.1 Create Rust Binary (`lino-core`)

**Crate Structure:**
```
lino-core/
├── lino_core/          # Core library
│   ├── parser/         # TypeScript parsing
│   ├── rules/          # Rule engine
│   ├── scanner/        # File system operations
│   └── cache/          # Incremental caching
├── lino_cli/           # CLI binary
└── lino_server/        # JSON-RPC server for VSCode
```

**Key Dependencies:**
```toml
[dependencies]
swc_ecma_parser = "0.147"     # Fast TypeScript parser
swc_ecma_ast = "0.116"         # AST types
rayon = "1.11"                 # Parallelism
dashmap = "6.1"                # Concurrent HashMap
walkdir = "2.5"                # Directory traversal
ignore = "0.4"                 # .gitignore handling
globset = "0.4"                # Pattern matching
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
notify = "7.0"                 # File watching
```

**Why SWC?**
- Written in Rust (no FFI overhead)
- Extremely fast (used by Next.js, Deno)
- Full TypeScript support
- AST-based analysis capability
- Battle-tested on large codebases

#### 1.2 Initial Feature Set

**Must Have:**
1. **File Scanner**
   - Parallel directory traversal
   - .gitignore integration
   - Configurable ignore patterns
   - Glob pattern support

2. **TypeScript Parser**
   - SWC-based parsing
   - Error-tolerant (partial parse on syntax errors)
   - Source map generation for accurate positions

3. **Basic Rule Engine**
   - Pattern-based rules (regex)
   - AST-based rules (visitor pattern)
   - Rule configuration format

4. **JSON-RPC Server**
   - Stdin/stdout communication
   - Request/response protocol
   - Notification support (file changes, progress)

5. **Result Format**
   - Standardized issue format
   - File location with line/column
   - Rule metadata

#### 1.3 VSCode Extension Bridge

**Update current extension:**
```typescript
// src/linoCore.ts
import { spawn } from 'child_process';

class LinoCoreClient {
  private process: ChildProcess;

  async scan(workspaceRoot: string, config: Config): Promise<Issue[]> {
    // Send JSON-RPC request to Rust binary
    const request = {
      method: 'scan',
      params: { root: workspaceRoot, config }
    };

    return this.sendRequest(request);
  }

  async watchFiles(paths: string[]): Promise<void> {
    // Enable incremental mode
  }
}
```

**Benefits:**
- Minimal changes to existing extension
- Can test Rust core independently
- Fallback to TypeScript if binary unavailable

### Phase 2: Performance Optimization

**Goal:** Achieve <200ms for 500 files

#### 2.1 Implement Parallel Processing

**Scanner Architecture:**
```rust
use rayon::prelude::*;

pub fn scan_workspace(root: &Path, config: &Config) -> Vec<Issue> {
    let files = discover_files(root, config);

    // Parallel processing with Rayon
    files.par_iter()
        .filter_map(|path| parse_and_analyze(path, config))
        .flatten()
        .collect()
}

fn parse_and_analyze(path: &Path, config: &Config) -> Option<Vec<Issue>> {
    let source = fs::read_to_string(path).ok()?;
    let ast = parse_typescript(&source)?;

    // Run all rules on this AST
    config.rules.par_iter()
        .flat_map(|rule| rule.check(&ast, path))
        .collect()
}
```

**Key Optimizations:**
- File I/O in parallel
- Parsing in parallel
- Rule execution in parallel
- Lock-free result aggregation

#### 2.2 Incremental Caching

**Cache Strategy:**
```rust
use dashmap::DashMap;
use std::time::SystemTime;

pub struct FileCache {
    // path → (mtime, issues)
    entries: DashMap<PathBuf, (SystemTime, Vec<Issue>)>,
}

impl FileCache {
    pub fn get_or_analyze(&self, path: &Path, config: &Config) -> Vec<Issue> {
        let mtime = fs::metadata(path)
            .and_then(|m| m.modified())
            .unwrap();

        // Check cache
        if let Some(entry) = self.entries.get(path) {
            if entry.0 == mtime {
                return entry.1.clone();
            }
        }

        // Cache miss - analyze
        let issues = analyze_file(path, config);
        self.entries.insert(path.to_path_buf(), (mtime, issues.clone()));
        issues
    }
}
```

**Cache Storage:**
- In-memory for active session (DashMap)
- Persistent cache in `~/.cache/lino/` (serde + bincode)
- Invalidation on file change or config change

#### 2.3 Smart File Watching

**Watch Strategy:**
```rust
use notify::{Watcher, RecursiveMode, Event};

pub fn watch_workspace(root: &Path, tx: Sender<FileEvent>) {
    let (watch_tx, watch_rx) = channel();

    let mut watcher = notify::recommended_watcher(watch_tx)?;
    watcher.watch(root, RecursiveMode::Recursive)?;

    for event in watch_rx {
        match event {
            Event::Modify(path) => {
                tx.send(FileEvent::Modified(path))?;
            }
            Event::Create(path) => {
                tx.send(FileEvent::Created(path))?;
            }
            Event::Remove(path) => {
                tx.send(FileEvent::Deleted(path))?;
            }
        }
    }
}
```

**Benefits:**
- Only reanalyze changed files
- Instant feedback in editor
- Minimal CPU usage when idle

### Phase 3: Extensible Rule System

**Goal:** Support custom user-defined rules

#### 3.1 Rule Configuration Format

**`.lino/rules.json`:**
```json
{
  "rules": {
    "no-relative-imports": {
      "enabled": true,
      "type": "ast",
      "severity": "error",
      "include": ["src/**/*.ts"],
      "exclude": ["src/**/*.test.ts"],
      "message": "Use absolute imports with @ prefix"
    },
    "prefer-type-over-interface": {
      "enabled": true,
      "type": "ast",
      "severity": "warning",
      "include": ["src/**/*.ts"]
    },
    "no-console-log": {
      "enabled": true,
      "type": "regex",
      "severity": "warning",
      "pattern": "console\\.log\\(",
      "include": ["src/**/*.ts"],
      "exclude": ["src/debug/**"]
    }
  },
  "include": ["src/**/*.{ts,tsx}"],
  "exclude": ["node_modules/**", "dist/**"]
}
```

#### 3.2 Rule Engine Architecture

**Built-in Rules (Rust):**
```rust
pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self, ast: &Program, path: &Path) -> Vec<Issue>;
}

pub struct NoRelativeImportsRule;

impl Rule for NoRelativeImportsRule {
    fn name(&self) -> &str { "no-relative-imports" }

    fn check(&self, ast: &Program, path: &Path) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Visit all import declarations
        for import in ast.imports() {
            if import.source.starts_with('.') {
                issues.push(Issue {
                    rule: self.name(),
                    location: import.span.into(),
                    message: "Use absolute imports with @".into(),
                });
            }
        }

        issues
    }
}
```

**Custom Rules (Future: WASM):**
```rust
// Future: Load user rules as WASM plugins
pub struct WasmRule {
    module: wasmtime::Module,
}

impl Rule for WasmRule {
    fn check(&self, ast: &Program, path: &Path) -> Vec<Issue> {
        // Serialize AST, call WASM function, deserialize results
    }
}
```

#### 3.3 Rule Registry

**Dynamic Rule Loading:**
```rust
pub struct RuleRegistry {
    rules: HashMap<String, Box<dyn Rule>>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        let mut rules = HashMap::new();

        // Register built-in rules
        rules.insert("no-relative-imports", Box::new(NoRelativeImportsRule));
        rules.insert("prefer-type-over-interface", Box::new(PreferTypeRule));
        rules.insert("no-any-type", Box::new(NoAnyTypeRule));

        Self { rules }
    }

    pub fn load_custom_rules(&mut self, path: &Path) {
        // Load WASM plugins from .lino/custom-rules/
    }
}
```

### Phase 4: Advanced Features

#### 4.1 Fix Suggestions (Code Actions)

**Auto-fix Support:**
```rust
pub struct Fix {
    pub range: TextRange,
    pub new_text: String,
    pub description: String,
}

pub trait Rule {
    fn check(&self, ast: &Program, path: &Path) -> Vec<Issue>;
    fn fix(&self, issue: &Issue, source: &str) -> Option<Fix>;
}
```

**VSCode Integration:**
```typescript
class LinoCodeActionProvider implements vscode.CodeActionProvider {
  async provideCodeActions(document: vscode.TextDocument, range: vscode.Range) {
    const issue = this.findIssueAtRange(document, range);
    if (!issue) return;

    // Request fix from Rust core
    const fix = await linoCoreClient.getFix(issue.id);

    const action = new vscode.CodeAction(
      fix.description,
      vscode.CodeActionKind.QuickFix
    );
    action.edit = new vscode.WorkspaceEdit();
    action.edit.replace(document.uri, fix.range, fix.newText);

    return [action];
  }
}
```

#### 4.2 Configuration UI

**VSCode Webview for `.lino/rules.json`:**
- Rule browser with search
- Enable/disable toggles
- Severity adjustments
- File pattern configurator
- Live preview of affected files

#### 4.3 Rule Marketplace (Long-term)

**Shareable Rule Packages:**
```
.lino/
├── rules.json              # Active configuration
├── custom-rules/           # User's custom rules
│   ├── company-style.wasm
│   └── security-checks.wasm
└── installed-packs/        # Downloaded rule packs
    ├── react-best-practices/
    └── angular-guidelines/
```

## Performance Targets & Benchmarks

### Phase 1 Targets (MVP Parity)

**Codebase Size: 100 files (~50K LOC)**
- Full scan: <500ms (vs current ~2-3s)
- Incremental update: <50ms
- Memory usage: <100MB

### Phase 2 Targets (Biome-level)

**Codebase Size: 500 files (~250K LOC)**
- Full scan: <200ms
- Incremental update: <10ms
- Memory usage: <200MB

**Codebase Size: 2000 files (~1M LOC)**
- Full scan: <1s
- Incremental update: <20ms
- Memory usage: <500MB

### Performance Comparison

| Tool        | 500 Files | 2000 Files | Notes                    |
|-------------|-----------|------------|--------------------------|
| ESLint      | 3-5s      | 15-20s     | Single-threaded          |
| Biome       | ~200ms    | ~800ms     | Multi-threaded Rust      |
| **Lino (Target)** | <200ms | <1s    | Multi-threaded Rust      |
| Lino (Current) | ~10s   | ~60s      | TypeScript, simple regex |

## Technical Challenges & Solutions

### Challenge 1: Cross-Platform Binary Distribution

**Problem:** Need to ship Rust binary with VSCode extension

**Solution:**
```json
{
  "postinstall": "node scripts/download-binary.js"
}
```

**Binary Strategy:**
- Compile for major platforms: `{linux,darwin,win32}-{x64,arm64}`
- Store in GitHub releases
- Download on install based on `process.platform` + `process.arch`
- Include prebuilt binaries in VSIX (increases size ~20MB)

### Challenge 2: TypeScript AST Complexity

**Problem:** SWC AST is complex, many node types

**Solution:**
- Use visitor pattern with trait defaults
- Provide AST helper utilities
- Pre-built common traversals
- Document AST structure well

**Example Helper:**
```rust
pub trait Visitor {
    fn visit_import(&mut self, _import: &ImportDecl) {}
    fn visit_type_alias(&mut self, _alias: &TypeAlias) {}
    fn visit_interface(&mut self, _interface: &Interface) {}
    // ... defaults for all node types
}

pub fn walk_ast<V: Visitor>(ast: &Program, visitor: &mut V) {
    // Traverses and calls appropriate visit_* methods
}
```

### Challenge 3: Configuration Complexity

**Problem:** Supporting complex include/exclude patterns per rule

**Solution:**
```rust
use globset::{Glob, GlobSetBuilder};

pub struct RuleConfig {
    pub include: GlobSet,
    pub exclude: GlobSet,
    pub severity: Severity,
}

impl RuleConfig {
    pub fn matches(&self, path: &Path) -> bool {
        self.include.is_match(path) && !self.exclude.is_match(path)
    }
}
```

**Benefits:**
- Fast pattern matching (compiled once)
- Intuitive glob syntax
- Per-rule file filtering

### Challenge 4: Debugging Experience

**Problem:** Rust errors harder to debug than TypeScript

**Solution:**
- Comprehensive logging with `tracing` crate
- Debug mode with verbose output
- JSON-RPC request/response logging
- Clear error messages with context

**Implementation:**
```rust
use tracing::{info, debug, error};

#[instrument]
pub fn analyze_file(path: &Path) -> Result<Vec<Issue>> {
    debug!("Parsing file: {:?}", path);
    let ast = parse(path)?;

    debug!("Running rules on AST");
    let issues = run_rules(&ast);

    info!("Found {} issues in {:?}", issues.len(), path);
    Ok(issues)
}
```

## Migration Strategy

### Step 0: Monorepo Setup (Week 0)

- Transform repo into pnpm monorepo structure
- Create `packages/lino-core` and `packages/vscode-extension`
- Setup helper scripts (`setup-dev.sh`, `build-all.sh`)
- Configure CI/CD for monorepo
- Test build and development workflow

**Deliverable:** Functional monorepo with both packages

### Step 1: Parallel Development (Weeks 1-4)

- Create `lino-core` Rust workspace in `packages/lino-core`
- Implement basic scanner + parser
- Build JSON-RPC server
- Test with simple rules

**Deliverable:** Working Rust binary that can find `: any` patterns

### Step 2: Extension Integration (Weeks 5-6)

- Add Rust binary spawning to extension
- Implement request/response protocol
- Maintain fallback to TypeScript
- Test on real codebases

**Deliverable:** Hybrid extension with Rust backend

### Step 3: Performance Tuning (Weeks 7-8)

- Profile and optimize hot paths
- Implement caching
- Add file watching
- Benchmark against targets

**Deliverable:** <200ms scan for 500 files

### Step 4: Rule System (Weeks 9-12)

- Implement rule configuration parser
- Build rule registry
- Add built-in rules
- Document rule authoring

**Deliverable:** Extensible rule system

### Step 5: Polish & Release (Weeks 13-14)

- Comprehensive testing
- Documentation
- Error handling improvements
- Publishing

**Deliverable:** 1.0 release

## Build & Distribution

### Development Setup

```bash
# Root structure
lino/
├── packages/
│   ├── lino-core/          # Rust workspace
│   │   ├── lino_core/      # Core library
│   │   ├── lino_cli/       # CLI binary
│   │   └── lino_server/    # JSON-RPC server
│   └── vscode-extension/   # TypeScript extension
├── scripts/
│   ├── build-binaries.sh   # Cross-compile for all platforms
│   └── download-binary.js  # Post-install script
└── .github/
    └── workflows/
        └── release.yml     # Auto-build on tag
```

### Build Process

```bash
# Development
cd packages/lino-core
cargo build --release

# Cross-compilation (using cross)
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-pc-windows-gnu
```

### CI/CD

**GitHub Actions:**
```yaml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  build-binaries:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release --target ${{ matrix.target }}
      - uses: actions/upload-artifact@v4
        with:
          name: lino-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/lino-server*

  publish-extension:
    needs: build-binaries
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4
      - run: npm run package
      - run: vsce publish
```

## Risk Mitigation

### Risk 1: Rust Learning Curve

**Mitigation:**
- Start with simple components (scanner, file I/O)
- Leverage existing libraries (SWC, Rayon)
- Comprehensive documentation
- Reference Biome codebase

### Risk 2: Binary Size

**Concern:** Rust binaries can be large (~10-20MB)

**Mitigation:**
- Strip symbols in release builds
- Use `opt-level = 'z'` for size optimization
- Consider UPX compression
- Acceptable trade-off for performance

### Risk 3: Platform-Specific Issues

**Mitigation:**
- Extensive CI testing on all platforms
- Fallback to TypeScript implementation
- Clear error messages for unsupported platforms
- Community testing before release

### Risk 4: Maintenance Burden

**Mitigation:**
- Keep TypeScript extension thin (mostly UI)
- Comprehensive test coverage for Rust core
- Clear separation of concerns
- Good documentation for contributors

## Success Metrics

### Performance Metrics
- [ ] Sub-200ms full scan for 500 files
- [ ] Sub-10ms incremental updates
- [ ] Memory usage under 200MB for 500 files
- [ ] 99th percentile response time <100ms

### Usability Metrics
- [ ] Configuration in <5 minutes
- [ ] Custom rule authoring documented
- [ ] Zero-config for basic usage
- [ ] Auto-fixes for common issues

### Adoption Metrics
- [ ] 1000+ active installations
- [ ] 50+ custom rules shared
- [ ] 90%+ positive user feedback
- [ ] Used in large codebases (10K+ files)

## Conclusion

The path from Lino's current MVP to Biome-level performance is achievable through a hybrid architecture combining Rust's performance with TypeScript's VSCode integration.

**Key Success Factors:**

1. **Incremental Approach**: Phase-by-phase implementation reduces risk
2. **Proven Technologies**: SWC, Rayon, and other battle-tested libraries
3. **Clear Architecture**: Separation of concerns enables parallel development
4. **Performance Focus**: Benchmarking and profiling throughout development
5. **Extensibility**: Rule system designed for user customization

**Timeline Estimate:** 3-4 months for feature-complete 1.0 release (includes Week 0 monorepo setup)

**Effort:** ~500-600 hours of focused development

**Prerequisites:**
- **Monorepo setup is Phase 0** - must be completed before Rust development begins
- Use pnpm workspaces for package management
- All development happens within monorepo structure

**Outcome:** A high-performance, extensible TypeScript linting platform that scales to large codebases while maintaining sub-second response times.

The investment in Rust infrastructure will pay dividends in performance, scalability, and long-term maintainability. This positions Lino not just as a simple `any` finder, but as a serious platform for custom code quality enforcement in large TypeScript projects.
