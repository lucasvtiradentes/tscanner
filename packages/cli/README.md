<div align="center">
<h3>tscanner CLI</h3>
<p>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
</p>
</div>

---

## 🎺 Overview

Command-line interface for tscanner. Scan TypeScript/TSX files from terminal with fast Rust-powered analysis.

## 📦 Installation

```bash
npm install -g tscanner
pnpm add -g tscanner
yarn global add tscanner
```

**No installation:**
```bash
npx tscanner check
```

**Platforms:** Linux (x64/arm64), macOS (Intel/Apple Silicon), Windows (x64)

## 💡 Usage

### Initialize Config

```bash
tscanner init
tscanner init /path/to/project
```

Creates `.tscanner/rules.json` with default configuration.

### Scan Files

```bash
tscanner check
tscanner check /path/to/project
tscanner check --no-cache
```

**Exit codes:** 0 (no errors), 1 (errors found or config missing)

**Example output:**
```
Scanning...

src/index.ts
  ✖ 5:10 Found ': any' type annotation [no-any-type]
  ⚠ 10:7 'count' is never reassigned, use 'const' instead [prefer-const]

✖ 2 errors, 1 warnings
Scanned 2 files in 45ms
```

### List Rules

```bash
tscanner rules
tscanner rules /path/to/project
```

Shows enabled/disabled rules with descriptions.

## 📋 Commands

| Command | Description |
|---------|-------------|
| `init [path]` | Create `.tscanner/rules.json` |
| `check [path]` | Scan files and report issues |
| `check --no-cache` | Skip cache, force rescan |
| `rules [path]` | List configured rules |

## 🏗️ Architecture

**Multi-package distribution:**
- Main package: `tscanner` (wrapper + platform detection)
- Platform packages: `@tscanner/cli-linux-x64`, `@tscanner/cli-darwin-arm64`, etc.
- npm auto-installs only the binary for your platform

**Process flow:**
```
Node.js wrapper (main.js)
      ↓
Platform detection
      ↓
Spawn Rust binary
      ↓
Forward args & I/O
      ↓
Return exit code
```

## 📊 Performance

- **Caching:** `~/.cache/tscanner/cache_{hash}.json`
- **Parallel:** Multi-core file analysis via Rayon
- **Typical:** 100-500 files in <1s

## 🚀 Use Cases

### CI/CD Pipeline

```bash
#!/bin/bash
tscanner check || exit 1
```

### Pre-commit Hook

```bash
#!/bin/sh
if command -v tscanner &> /dev/null && [ -f .tscanner/rules.json ]; then
  tscanner check --no-cache
fi
```

### VS Code Task

```json
{
  "version": "2.0.0",
  "tasks": [{
    "label": "tscanner: Check",
    "type": "shell",
    "command": "tscanner check"
  }]
}
```

## 🔧 Development

```bash
pnpm install
pnpm run build
npm link
tscanner --version
```

## 📜 License

MIT License - see [LICENSE](../../LICENSE)
