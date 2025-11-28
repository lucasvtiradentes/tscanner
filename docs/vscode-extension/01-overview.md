# VSCode Extension Overview

## What It Does

The TScanner VSCode extension provides real-time code quality scanning directly in the editor. Issues appear in a sidebar panel with one-click navigation to each problem location.

**Key capabilities:**
- Real-time scanning on file changes
- Sidebar TreeView with issue navigation
- Two scan modes: full codebase or branch-only
- Copy issues to clipboard for AI-assisted fixes
- Status bar with mode indicator

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        VSCode Extension                                  │
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐  │
│  │  extension   │  │  StatusBar   │  │  TreeView    │  │  Commands   │  │
│  │  (activate)  │  │  Manager     │  │  (Issues)    │  │  Registry   │  │
│  └──────┬───────┘  └──────────────┘  └──────────────┘  └─────────────┘  │
│         │                                                                │
│  ┌──────▼───────┐  ┌──────────────┐  ┌──────────────┐                   │
│  │   Scanner    │  │  Git Helper  │  │  Config      │                   │
│  │   (wrapper)  │  │  (diff)      │  │  Manager     │                   │
│  └──────┬───────┘  └──────────────┘  └──────────────┘                   │
│         │                                                                │
│  ┌──────▼───────┐                                                        │
│  │  RustClient  │◄────── JSON-RPC over stdin/stdout                     │
│  │  (spawn)     │                                                        │
│  └──────┬───────┘                                                        │
└─────────┼────────────────────────────────────────────────────────────────┘
          │
          ▼
   ┌─────────────┐
   │ tscanner-   │
   │ server      │  Rust binary (bundled or dev)
   └─────────────┘
```

## Activation Flow

1. **Trigger:** `onStartupFinished` (after VSCode fully loads)
2. **Workspace check:** Verify workspace folder exists
3. **State restoration:** Load persisted view mode, scan mode, compare branch from workspace state
4. **UI initialization:**
   - Create TreeView with `IssuesPanelContent` provider
   - Create StatusBarItem with mode indicator
   - Register all commands
5. **File watcher setup:** Watch `**/*.{ts,tsx,js,jsx}` for changes
6. **Initial scan:** After 2s delay, run first scan silently

## Key Features

| Feature | Description |
|---------|-------------|
| **Scan Modes** | Codebase (full) or Branch (changed files only) |
| **View Modes** | Group by file or rule, flat or tree layout |
| **Navigation** | F8/Shift+F8 to jump between issues |
| **Copy Issues** | Export issues to clipboard for AI tools |
| **Settings Menu** | Quick access to rule management, scan mode |
| **File Watcher** | Incremental updates on file save |

## Commands

| Command | Keybinding | Description |
|---------|------------|-------------|
| `tscanner.findIssue` | - | Scan workspace |
| `tscanner.hardScan` | - | Clear cache and rescan |
| `tscanner.goToNextIssue` | F8 | Navigate to next issue |
| `tscanner.goToPreviousIssue` | Shift+F8 | Navigate to previous issue |
| `tscanner.showLogs` | - | Open log file |

## File Structure

```
packages/vscode-extension/src/
├── extension.ts              # Entry point, activation
├── commands/                 # Command handlers
│   ├── public/               # User-facing commands
│   └── internal/             # Internal commands (navigation, copy)
├── common/
│   ├── lib/
│   │   ├── rust-client.ts    # JSON-RPC client
│   │   ├── scanner.ts        # Binary resolution, scan wrapper
│   │   ├── config-manager.ts # Config loading
│   │   └── vscode-utils.ts   # VSCode API helpers
│   └── utils/
│       ├── git-helper.ts     # Git diff integration
│       └── logger.ts         # Logging
├── issues-panel/             # TreeView provider
├── settings-menu/            # QuickPick settings UI
└── status-bar/               # StatusBar manager
```

## Related Documentation

- [VSCode Concepts](02-vscode-concepts.md) - UI components and APIs
- [Scan Modes](03-scan-modes.md) - Codebase vs Branch scanning
- [Rust Server Communication](04-rust-server-communication.md) - JSON-RPC protocol
- [Dev vs Prod](05-dev-vs-prod.md) - Development setup
