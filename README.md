# Lino

High-performance TypeScript linting platform with configurable rules and Git integration.

## 🚀 Vision

Lino is a Biome-level performance linting platform with custom user-defined rules, powered by a hybrid Rust + TypeScript architecture.

## ✨ Current Features

**Core Linting**
- **13+ built-in rules**: Type safety, code quality, and best practices
- **Custom regex rules**: Define your own patterns in `.lino/rules.json`
- **AST-based analysis**: SWC-powered TypeScript/TSX parsing
- **Configurable severity**: Error or warning levels per rule

**VSCode Integration**
- **Tree/List views**: Hierarchical folders or flat file listing
- **Group by rule**: Organize issues by rule type or file
- **Sidebar integration**: Activity bar icon with live issue count
- **Click to navigate**: Jump directly to any issue in your code
- **Context actions**: Copy file paths (absolute/relative) from tree items

**Git Integration**
- **Branch mode**: Scan only changed files vs target branch
- **Workspace mode**: Scan entire codebase
- **Live updates**: Incremental re-scan on file changes

**Performance**
- **Parallel processing**: Rayon-powered concurrent file analysis
- **Smart caching**: File + config hash-based cache with disk persistence
- **GZIP compression**: Compressed JSON-RPC responses for large datasets

## 🎯 Roadmap

- **Phase 0 (✅ Complete)**: Monorepo structure with Rust workspace
- **Phase 1 (✅ ~90% Complete)**: Rust core + SWC parser + JSON-RPC server + 13 rules
- **Phase 2 (⏳ Next)**: Performance optimization for <200ms on 500+ files
- **Phase 3**: Auto-fixes and advanced rule options
- **Phase 4**: Language server protocol (LSP) support

See [plan/lino-performance-roadmap.md](plan/lino-performance-roadmap.md) for details.

## 📦 Architecture

Hybrid architecture with JSON-RPC communication:

```
VSCode Extension (TypeScript)     lino-server (Rust)
    ├─ UI/Settings/Commands  ←→  ├─ JSON-RPC Interface
    ├─ Git Integration            ├─ Scanner (Rayon)
    ├─ Tree View Provider         ├─ SWC Parser
    └─ Status Bar                 ├─ Rule Registry (13+ rules)
                                  └─ File Cache (DashMap)
```

## 🛠️ Development

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/) or run `source "$HOME/.cargo/env"`
- **pnpm**: `npm install -g pnpm`

### Quick Start

```bash
./scripts/setup-dev.sh
```

### Development Workflow

**Terminal 1 - Rust auto-rebuild:**
```bash
cd packages/lino-core
cargo watch -x build
```

**Terminal 2 - Extension auto-rebuild:**
```bash
pnpm dev
```

**VSCode - Debug Extension:**
Press `F5` to launch Extension Development Host

### Build Everything

```bash
./scripts/build-all.sh
```

### Build Cross-Platform Binaries

```bash
./scripts/build-binaries.sh
```

Targets: `x86_64/aarch64-unknown-linux-gnu`, `x86_64/aarch64-apple-darwin`, `x86_64-pc-windows-msvc`

### Clean Install (if issues)

If you see duplicate commands/views or extension not updating:

```bash
./scripts/clean-install.sh
```

Then reload VSCode: `Ctrl+Shift+P` → `Developer: Reload Window`

## 📚 Documentation

- [Architecture](docs/architecture.md) - System design and component overview
- [Protocol](docs/protocol.md) - JSON-RPC communication protocol
- [Development](docs/development.md) - Developer guide and workflows
- [Roadmap](plan/lino-performance-roadmap.md) - Performance evolution plan

## 🏗️ Monorepo Structure

```
lino/
├── packages/
│   ├── lino-core/              # Rust workspace
│   │   ├── crates/
│   │   │   ├── lino_core/      # Core library (Scanner, Parser, Rules, Cache)
│   │   │   ├── lino_cli/       # CLI binary (planned)
│   │   │   └── lino_server/    # JSON-RPC server (main entry point)
│   │   └── target/             # Build artifacts
│   └── vscode-extension/       # TypeScript extension
│       ├── src/                # Extension source
│       │   ├── extension.ts    # Main activation + commands
│       │   ├── rustClient.ts   # JSON-RPC client
│       │   ├── issueScanner.ts # Scan orchestration
│       │   ├── searchProvider.ts # Tree view provider
│       │   ├── treeBuilder.ts  # Folder hierarchy
│       │   ├── gitHelper.ts    # Git integration
│       │   └── logger.ts       # File logging
│       ├── binaries/           # Bundled Rust binaries per platform
│       └── out/                # Compiled extension
├── scripts/                    # Build and setup scripts
├── docs/                       # Documentation
└── plan/                       # Design documents
```

## 🔧 Tech Stack

**Rust Core (lino-core):**
- **swc_ecma_parser**: TypeScript/TSX AST parsing
- **Rayon**: Parallel file processing
- **DashMap**: Concurrent cache storage
- **ignore**: .gitignore support
- **globset**: File pattern matching
- **notify**: File system watching
- **serde_json**: JSON-RPC serialization
- **flate2 + base64**: Response compression

**VSCode Extension:**
- **TypeScript**: Extension logic
- **esbuild**: Fast bundling
- **VSCode Extension API**: UI/commands/views
- **JSON-RPC**: Communication with Rust server

**Build System:**
- **pnpm**: Package manager
- **Cargo**: Rust build tool
- **Cross-compilation**: Multi-platform binaries

## 🎯 Performance Status

| Codebase | Phase 0 (TS) | Phase 1 (Rust) | Target (Phase 2) |
|----------|--------------|----------------|------------------|
| 100 files | ~2-3s | ~800ms | <500ms |
| 500 files | ~10s | ~3s | <200ms |
| 2000 files | ~60s | ~15s | <1s |

**Performance improvements in Phase 1:**
- ✅ Rayon parallel processing
- ✅ File + config hash caching
- ✅ GZIP compression for large results
- ⏳ Further optimization needed for Phase 2 targets

## 📝 License

MIT
