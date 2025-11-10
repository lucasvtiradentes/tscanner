# Lino Architecture

## Overview

Lino uses a hybrid architecture combining Rust performance with TypeScript VSCode integration.

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
│  - TypeScript parsing (SWC)              │
│  - Rule engine                           │
│  - Multi-threaded analysis               │
│  - Incremental caching                   │
└──────────────────────────────────────────┘
```

## Packages

### `lino-core` (Rust)

Core analysis engine built in Rust for maximum performance.

**Crates:**
- `lino_core` - Core library with parsing, scanning, and rule engine
- `lino_cli` - CLI binary for standalone usage
- `lino_server` - JSON-RPC server for VSCode extension communication

**Key Dependencies:**
- `swc_ecma_parser` - Fast TypeScript/JavaScript parser
- `rayon` - Data parallelism library
- `dashmap` - Concurrent HashMap for caching
- `notify` - File system watcher

### `vscode-extension` (TypeScript)

VSCode extension providing user interface and editor integration.

**Key Files:**
- `src/extension.ts` - Main activation and commands
- `src/searchProvider.ts` - Tree/list view data provider
- `src/anyFinder.ts` - File scanning coordinator
- `src/treeBuilder.ts` - Folder hierarchy builder
- `src/logger.ts` - Logging utility

## Communication Protocol

The extension and Rust core communicate via JSON-RPC over stdin/stdout:

```typescript
{
  method: 'scan',
  params: {
    root: '/path/to/workspace',
    config: { rules: {...}, include: [...], exclude: [...] }
  }
}
```

## Current State (Phase 0)

✅ Monorepo structure established
✅ Rust workspace configured
✅ TypeScript extension in `packages/vscode-extension`
✅ Helper scripts for development
⏳ Rust core implementation (Phase 1)
⏳ JSON-RPC server (Phase 1)
⏳ Extension integration (Phase 1)

## Next Steps

See `/plan/lino-performance-roadmap.md` for the full implementation roadmap.
