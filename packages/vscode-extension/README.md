<a name="TOC"></a>

<div align="center">
<img width="128" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/logo.png" alt="tscanner Extension logo">
<h4>Tscanner - VS Code Extension</h4>
<p>
  <a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/visual-studio-marketplace/v/lucasvtiradentes.tscanner-vscode.svg" alt="vscode version"></a>
  <a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/visual-studio-marketplace/i/lucasvtiradentes.tscanner-vscode.svg" alt="installs"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <br>
  <a href="#-overview">Overview</a> • <a href="#-features">Features</a> • <a href="#-installation">Installation</a> • <a href="#-usage">Usage</a> • <a href="#-architecture">Architecture</a> • <a href="#-license">License</a>
</p>

</div>

<a href="#"><img src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/divider.png" /></a>

## 🎺 Overview

Real-time TypeScript code quality scanner with sidebar integration and Git-aware scanning. Catch issues as you type with instant visual feedback.

<img src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/vscode-demo.png" alt="VS Code Extension Screenshot" width="100%">

## ⭐ Features<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

- **Real-time Scanning** - File system watching detects changes instantly
- **Multiple scanning modes** - Scan full codebase or only branch changes
- **Flexible Views** - Tree/list layouts with file or rule grouping
- **Quick Navigation** - F8/Shift+F8 keyboard shortcuts to jump between issues
- **Settings Menu** - Interactive rule management and configuration
- **Status Bar Integration** - See useful info at a glance
- **13+ Built-in Rules** - AST-based validation for TypeScript/TSX
- **Custom Rules** - Regex patterns, JavaScript scripts, or AI-powered validation

## 🚀 Installation<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

**From VS Code Marketplace:**

1. Open VS Code
2. Go to Extensions (Ctrl/Cmd + Shift + X)
3. Search for "tscanner"
4. Click Install

**From Command Line:**

```bash
code --install-extension lucasvtiradentes.tscanner-vscode
```

## 💡 Usage<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

### Getting Started

1. Open a TypeScript/TSX workspace in VS Code
2. Click the tscanner icon in the activity bar
3. Issues appear automatically in the sidebar
4. Click any issue to jump to its location

### Scan Modes

Switch between scanning strategies:

- **Codebase**: Analyze all files in the codebase
- **Branch**: Scan only files changed compared to target branch (git diff)

Change via Settings Menu or status bar click.

### View Modes

Organize results with 4 combinations:

```
List + Default    → Flat files with nested issues
Tree + Default    → Folder hierarchy with issues
List + By Rule    → Rules with nested file/issues
Tree + By Rule    → Rules → folders → files → issues
```

Toggle via toolbar icons or commands.

### Commands

Access via Command Palette (Ctrl/Cmd + Shift + P):

| Command | Shortcut | Description |
|---------|----------|-------------|
| `tscanner: Scan Workspace` | - | Run the selected scanning mode scan (Codebase or Branch) |
| `tscanner: Hard Scan` | - | Clear cache and rescan |
| `tscanner: Refresh` | F5 | Reload current results |
| `tscanner: Open Settings` | - | Configure rules and modes |
| `tscanner: Next Issue` | F8 | Jump to next issue |
| `tscanner: Previous Issue` | Shift+F8 | Jump to previous issue |
| `tscanner: Show Logs` | - | View extension logs |


### Other details 

<details>
<summary><b>Settings Menu</b></summary>

Interactive configuration panel:

- **Manage Rules**: Multi-select UI for 13+ built-in rules with enable/disable toggles
- **Scan Settings**: Choose workspace or branch mode, select target branch
- **Config Files**: Edit `.tscanner/rules.json` or create from template

</details>

<details>
<summary><b>Issue Navigation</b></summary>

Navigate efficiently:

- **Click to Jump**: Click any issue to open file at exact line/column
- **Keyboard**: F8 (next issue), Shift+F8 (previous issue)
- **Context Menu**: Right-click for copy path options
- **Badge Count**: Sidebar shows total issue count

</details>

<details>
<summary><b>Configuration</b></summary>

Create `.tscanner/rules.json` in your workspace root:

```json
{
  "builtinRules": {
    "no-any-type": {
      "enabled": true,
      "severity": "error"
    },
    "no-console-log": {
      "enabled": true,
      "severity": "warning"
    }
  },
  "customRules": {
    "no-todos": {
      "type": "regex",
      "pattern": "TODO:|FIXME:",
      "message": "Remove TODO comments",
      "severity": "warning"
    }
  },
  "include": ["**/*.{ts,tsx}"],
  "exclude": ["node_modules/**", "dist/**"]
}
```

**Config locations:**
- Local: `.tscanner/rules.json` (workspace-specific, recommended)
- Global: Managed via Settings Menu for all workspaces

</details>

<details>
<summary><b>Status Bar</b></summary>

Quick access info:

- **Scan Mode**: Shows "Codebase" or "Branch: {name}"
- **Click**: Opens Settings Menu
- **Config Status**: Green checkmark if `.tscanner/rules.json` exists

</details>

<details>
<summary><b>Branch Mode</b></summary>

When scanning branch changes:

1. Extension runs `git diff {branch}...HEAD` to detect changed files
2. Parses hunks to extract modified line ranges
3. Scans all files but filters issues to modified lines only

Perfect for PR validation - see only issues you introduced.

</details>

## 🏗️ Architecture<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

```
Extension (TypeScript)        Rust Server
├─ JSON-RPC client      ←→    ├─ Scanner (Rayon)
├─ Tree provider              ├─ Parser (SWC)
├─ Git helper                 ├─ Rules (13+)
├─ File watcher               ├─ Cache (DashMap)
└─ Status bar                 └─ Config loader
```

**Communication:** Line-delimited JSON-RPC over stdin/stdout with GZIP compression for large result sets.

## 📜 License<a href="#TOC"><img align="right" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/.github/image/up_arrow.png" width="22"></a>

MIT License - see [LICENSE](../../LICENSE) file for details.
