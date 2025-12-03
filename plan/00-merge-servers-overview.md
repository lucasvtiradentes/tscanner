# Merge RPC into Single LSP Server Plan

## Current State

```
VSCode Extension spawns TWO server processes:

┌─────────────────────────────────────────────────────────────┐
│                    VSCode Extension                         │
├─────────────────────────────────────────────────────────────┤
│  RustClient (rust-client.ts)    TscannerLspClient           │
│         │                              │                    │
│    spawn(binary)                 spawn(binary, ['--lsp'])   │
│         │                              │                    │
│         ▼                              ▼                    │
│  ┌─────────────┐                ┌─────────────┐            │
│  │ RPC Server  │                │ LSP Server  │            │
│  │ (JSON-RPC)  │                │ (LSP)       │            │
│  └─────────────┘                └─────────────┘            │
└─────────────────────────────────────────────────────────────┘

Crates involved:
├── tscanner_rpc/      # JSON-RPC handlers
├── tscanner_lsp/      # LSP handlers
└── tscanner_server/   # Entry point (routes based on --lsp flag)
```

## Target State

```
VSCode Extension spawns ONE server process:

┌─────────────────────────────────────────────────────────────┐
│                    VSCode Extension                         │
├─────────────────────────────────────────────────────────────┤
│              TscannerLspClient (lsp-client.ts)              │
│                          │                                  │
│                    spawn(binary)                            │
│                          │                                  │
│                          ▼                                  │
│  ┌───────────────────────────────────────────────────────┐ │
│  │                   LSP Server                           │ │
│  │  ┌─────────────────┐  ┌─────────────────────────────┐ │ │
│  │  │ Standard LSP    │  │ Custom Requests (tscanner/*) │ │ │
│  │  │ - diagnostics   │  │ - tscanner/scan             │ │ │
│  │  │ - codeAction    │  │ - tscanner/scanFile         │ │ │
│  │  │ - didOpen/Save  │  │ - tscanner/scanContent      │ │ │
│  │  └─────────────────┘  │ - tscanner/clearCache       │ │ │
│  │                       │ - tscanner/getRulesMetadata │ │ │
│  │                       │ - tscanner/formatResults    │ │ │
│  │                       └─────────────────────────────┘ │ │
│  └───────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘

Crates:
├── tscanner_lsp/      # All handlers (standard + custom)
└── tscanner_server/   # Entry point (always LSP mode)

Deleted:
└── tscanner_rpc/      # REMOVED
```

## Refactor Stages

| Stage | Description | Files |
|-------|-------------|-------|
| 1 | Extend LSP server with custom requests (Rust) | [01-extend-lsp-rust.md](./01-extend-lsp-rust.md) |
| 2 | Migrate VSCode to single LSP client | [02-migrate-vscode.md](./02-migrate-vscode.md) |

## Session Recommendations

| Session | Stages | Estimated Complexity |
|---------|--------|---------------------|
| Session 1 | Stage 1 | Medium |
| Session 2 | Stage 2 | Low |

## Key Principles

1. **LSP supports custom requests** - use `tscanner/*` method prefix
2. **No breaking changes** - RPC still works until Stage 2 complete
3. **Single process** - one binary, one spawn, one connection
4. **Tests must pass after each stage**

## Validation Command

```bash
(npm run format 2>/dev/null || ! grep -q "\"format\":" package.json 2>/dev/null) && (npm run lint 2>/dev/null || ! grep -q "\"lint\":" package.json 2>/dev/null) && (npm run typecheck 2>/dev/null || ! grep -q "\"typecheck\":" package.json 2>/dev/null) && (npm run build 2>/dev/null || ! grep -q "\"build\":" package.json 2>/dev/null)
```
