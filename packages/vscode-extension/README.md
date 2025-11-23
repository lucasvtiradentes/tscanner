<div align="center">
<img width="128" src="https://raw.githubusercontent.com/lucasvtiradentes/tscanner/main/packages/vscode-extension/resources/icon.png" alt="tscanner Extension">
<h3>tscanner - VSCode Extension</h3>
<p>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
</p>
</div>

---

## 🎺 Overview

Real-time TypeScript/TSX code quality scanner for VSCode. Enforces patterns, detects anti-patterns, and validates conventions with sidebar integration and Git-aware scanning.

## ⭐ Features

**View Modes**
- Tree/list sidebar views with issue badges
- Group by file path or rule name
- 4 combinations for flexible organization

**Scan Modes**
- **Workspace:** Full codebase analysis
- **Branch:** Scan only changed files vs target branch (git diff)
- Line-level filtering: show only issues in modified lines

**Navigation**
- Click issues to jump to exact line/column
- F8/Shift+F8 keyboard navigation
- Right-click context menus (copy paths)
- Status bar shows scan mode and branch

**Code Quality**
- 23+ built-in rules (AST + regex)
- Custom patterns for project conventions
- Configurable severity (error/warning)
- Inline disable directives

**Performance**
- Incremental file watching
- Smart caching with config-hash invalidation
- GZIP compression for large results
- Parallel processing via Rust backend

## 💡 Usage

### Installation

1. Install from VSCode marketplace
2. Open TypeScript/TSX workspace
3. Click tscanner icon in activity bar

### Commands

| Command | Shortcut | Description |
|---------|----------|-------------|
| Scan Workspace | - | Run full scan |
| Hard Scan | - | Clear cache + rescan |
| Open Settings | - | Configure rules & modes |
| Next Issue | F8 | Jump to next issue |
| Previous Issue | Shift+F8 | Jump to previous issue |
| Show Logs | - | View log file |

### Settings Menu

**Manage Rules:**
- Multi-select UI for 23+ built-in rules
- Add custom regex patterns
- Save to local or global config

**Manage Scan Settings:**
- Workspace or branch mode
- Select target branch for comparison

**Config Files:**
- Edit `.tscanner/rules.json`
- Create from template

### View Modes

```
List + Default    → Flat files with nested issues
Tree + Default    → Folder hierarchy with issues
List + By Rule    → Rules with nested file/issues
Tree + By Rule    → Rules → folders → files → issues
```

### Configuration

`.tscanner/rules.json`:

```json
{
  "rules": {
    "no-any-type": { "enabled": true, "type": "ast", "severity": "error" },
    "custom-pattern": { "enabled": true, "type": "regex", "severity": "warning", "pattern": "TODO:" }
  },
  "include": ["**/*.ts", "**/*.tsx"],
  "exclude": ["**/node_modules/**", "**/dist/**"]
}
```

**Config locations:**
- Local: `.tscanner/rules.json` (recommended)
- Global: `~/.vscode/extensions/.tscanner-config-{hash}.json`

### Disable Directives

```typescript
// tscanner-disable-file
// tscanner-disable rule1, rule2
// tscanner-disable-line rule1
// tscanner-disable-next-line rule1
```

## 🏗️ Architecture

```
Extension (TypeScript)        Rust Server
├─ JSON-RPC client      ←→    ├─ Scanner
├─ Tree provider              ├─ Parser (SWC)
├─ Git helper                 ├─ Rules (23+)
├─ File watcher               ├─ Cache
└─ Status bar                 └─ Config
```

**Communication:** Line-delimited JSON-RPC over stdin/stdout with GZIP compression

**Branch Mode:**
1. Run `git diff {branch}...HEAD --name-only` for changed files
2. Parse hunks to extract modified line ranges
3. Scan all files but filter issues to modified lines only

## 🔧 Development

```bash
pnpm install
pnpm run compile     # TypeScript compilation
pnpm run bundle      # esbuild minified bundle
pnpm run build       # Bundle + install locally
pnpm run dev         # Watch mode
```

**Debug:** Press F5 to launch Extension Development Host

## 📜 License

MIT License - see [LICENSE](../../LICENSE)
