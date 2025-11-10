# Lino

High-performance TypeScript linting platform with custom rule support.

## 🚀 Vision

Lino is evolving from a simple `any` type finder into a Biome-level performance linting platform with custom user-defined rules, powered by a hybrid Rust + TypeScript architecture.

## ✨ Current Features (Phase 0)

- **Find any types**: Scans workspace for `: any` and `as any` patterns
- **Tree/List view**: Toggle between hierarchical folder view or flat list
- **Sidebar integration**: Dedicated activity bar icon with issue count badge
- **Click to navigate**: Jump directly to any type usage in your code
- **Context actions**: Copy file paths (absolute/relative) from tree items
- **Performance**: Parallel file processing with caching
- **Logging**: Debug logs at `$TMPDIR/linologs.txt`

## 🎯 Roadmap

- **Phase 0 (✅ Complete)**: Monorepo structure with Rust workspace
- **Phase 1 (⏳ In Progress)**: Rust core with SWC parser + JSON-RPC server
- **Phase 2**: Sub-200ms scanning for 500+ files
- **Phase 3**: Extensible custom rule system
- **Phase 4**: Auto-fixes and advanced features

See [plan/lino-performance-roadmap.md](plan/lino-performance-roadmap.md) for details.

## 📦 Architecture

Hybrid architecture combining Rust performance with TypeScript integration:

```
VSCode Extension (TypeScript) ←→ Lino Core (Rust)
     UI/UX + Integration      ←→  Parsing + Analysis
```

## 🛠️ Development

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
│   │   │   ├── lino_core/      # Core library
│   │   │   ├── lino_cli/       # CLI binary
│   │   │   └── lino_server/    # JSON-RPC server
│   │   └── target/             # Build artifacts
│   └── vscode-extension/       # TypeScript extension
├── scripts/                    # Build and setup scripts
├── docs/                       # Documentation
└── plan/                       # Design documents
```

## 🔧 Tech Stack

**Current (Phase 0):**
- TypeScript + VSCode Extension API
- pnpm workspace
- esbuild

**Future (Phase 1+):**
- Rust + SWC (TypeScript parser)
- Rayon (parallelism)
- JSON-RPC (communication)

## 🎯 Performance Targets

| Codebase | Current | Target (Phase 2) |
|----------|---------|------------------|
| 100 files | ~2-3s | <500ms |
| 500 files | ~10s | <200ms |
| 2000 files | ~60s | <1s |

## 📝 License

MIT
