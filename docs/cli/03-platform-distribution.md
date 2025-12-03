# CLI Platform Distribution

## Overview

The TScanner CLI uses npm's optional dependencies mechanism to distribute platform-specific Rust binaries. When `npm install tscanner` runs, npm automatically installs only the binary package matching the user's platform, avoiding unnecessary downloads.

## How Optional Dependencies Work

**Main Package (`tscanner`):**
```json
{
  "optionalDependencies": {
    "@tscanner/cli-darwin-arm64": "0.0.20",
    "@tscanner/cli-darwin-x64": "0.0.20",
    "@tscanner/cli-linux-arm64": "0.0.20",
    "@tscanner/cli-linux-x64": "0.0.20",
    "@tscanner/cli-win32-x64": "0.0.20"
  }
}
```

npm will attempt to install all optional dependencies but won't fail if some are incompatible with the current platform. Each platform package has `os` and `cpu` constraints that prevent installation on non-matching platforms.

**Platform Package Example (`@tscanner/cli-linux-x64`):**
```json
{
  "name": "@tscanner/cli-linux-x64",
  "os": ["linux"],
  "cpu": ["x64"],
  "files": ["tscanner"]
}
```

The `os` and `cpu` fields ensure npm only installs this package on Linux x64 systems.

## Supported Platforms

| Platform | Package Name | Rust Target Triple |
|----------|-------------|-------------------|
| Linux x64 | `@tscanner/cli-linux-x64` | `x86_64-unknown-linux-gnu` |
| Linux ARM64 | `@tscanner/cli-linux-arm64` | `aarch64-unknown-linux-gnu` |
| macOS x64 | `@tscanner/cli-darwin-x64` | `x86_64-apple-darwin` |
| macOS ARM64 | `@tscanner/cli-darwin-arm64` | `aarch64-apple-darwin` |
| Windows x64 | `@tscanner/cli-win32-x64` | `x86_64-pc-windows-msvc` |

## Package Structure

Each platform package follows a minimal structure:

```
@tscanner/cli-linux-x64/
├── package.json
├── CHANGELOG.md
└── tscanner           # Rust binary (or tscanner.exe on Windows)
```

**Contents:**
- `package.json` - Package metadata with `os`/`cpu` constraints
- `tscanner` - Compiled Rust CLI binary for target platform
- `CHANGELOG.md` - Version history (optional)

The binary is the standalone executable compiled from `packages/rust-core/crates/tscanner_cli/`.

## Binary Resolution

The main `tscanner` package uses a postinstall script to locate and prepare the platform binary:

```javascript
// postinstall.js
const PLATFORM_MAP = {
  'linux-x64': '@tscanner/cli-linux-x64',
  'linux-arm64': '@tscanner/cli-linux-arm64',
  'darwin-x64': '@tscanner/cli-darwin-x64',
  'darwin-arm64': '@tscanner/cli-darwin-arm64',
  'win32-x64': '@tscanner/cli-win32-x64',
};

function getPlatformKey() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === 'linux' && arch === 'x64') return 'linux-x64';
  if (platform === 'darwin' && arch === 'arm64') return 'darwin-arm64';
  // ... other platforms
}
```

**Resolution Flow:**
```
1. User installs tscanner
2. npm installs matching @tscanner/cli-{platform}-{arch}
3. postinstall.js runs
4. Detects platform (process.platform + process.arch)
5. Resolves path to platform binary via require.resolve()
6. Makes binary executable (chmod 0o755 on Unix)
7. Ready for use via bin/tscanner
```

## Rust Target to npm Mapping

| Rust Target Triple | npm Platform | npm CPU | Package Suffix |
|-------------------|-------------|---------|---------------|
| `x86_64-unknown-linux-gnu` | `linux` | `x64` | `linux-x64` |
| `aarch64-unknown-linux-gnu` | `linux` | `arm64` | `linux-arm64` |
| `x86_64-apple-darwin` | `darwin` | `x64` | `darwin-x64` |
| `aarch64-apple-darwin` | `darwin` | `arm64` | `darwin-arm64` |
| `x86_64-pc-windows-msvc` | `win32` | `x64` | `win32-x64` |

**Target Components:**
- **Architecture**: `x86_64` (Intel/AMD 64-bit) or `aarch64` (ARM 64-bit)
- **Vendor**: `unknown`, `apple`, or `pc`
- **OS**: `linux-gnu`, `darwin`, or `windows-msvc`

## Installation Example

**User runs:**
```bash
npm install tscanner
```

**On macOS ARM64:**
```
├─ tscanner@0.0.20
│  ├─ @tscanner/cli-darwin-arm64@0.0.20 ✓ (installed)
│  ├─ @tscanner/cli-darwin-x64@0.0.20 ✗ (skipped: cpu mismatch)
│  ├─ @tscanner/cli-linux-arm64@0.0.20 ✗ (skipped: os mismatch)
│  ├─ @tscanner/cli-linux-x64@0.0.20 ✗ (skipped: os mismatch)
│  └─ @tscanner/cli-win32-x64@0.0.20 ✗ (skipped: os mismatch)
└─ postinstall.js runs → finds binary at:
   node_modules/@tscanner/cli-darwin-arm64/tscanner
```

Only the `darwin-arm64` package is installed. The user gets a ~5MB download instead of ~25MB (all platforms).

## Build Process

Platform packages are generated during CI release workflow:

```
1. Build Rust binary for each target:
   cargo build --release --target {rust-target}

2. Create platform package directory:
   packages/cli/npm/cli-{platform}-{arch}/

3. Copy binary:
   cp target/{rust-target}/release/tscanner → npm/cli-{platform}-{arch}/

4. Generate package.json with os/cpu constraints

5. Publish to npm:
   npm publish packages/cli/npm/cli-{platform}-{arch}/
```

See `.github/actions/build-rust-binary/action.yml` for implementation details.

## Advantages

**Bandwidth Savings:**
- User downloads only 1 binary (~5MB) instead of all 5 platforms (~25MB)
- Reduces npm registry bandwidth by 80%

**Storage Savings:**
- `node_modules` contains only 1 platform package
- Avoids storing unused binaries in CI caches

**Reliability:**
- npm's dependency resolution is battle-tested
- No custom download logic or fallback URLs needed
- Works with corporate proxies and private registries

**Compatibility:**
- Works with npm, yarn, pnpm, and bun
- Supports `npm ci` (lockfile-based installs)
- No runtime platform detection needed

## Comparison to VSCode Extension

| Aspect | CLI | VSCode Extension |
|--------|-----|-----------------|
| Binary Type | `tscanner` | `tscanner-server` |
| Distribution | npm optional dependencies | Bundled in `.vsix` |
| Selection | npm install-time | Runtime platform detection |
| Binaries Shipped | 1 (user's platform) | 5 (all platforms) |
| Update Mechanism | `npm update` | Extension update |
| Binary Location | `node_modules/@tscanner/cli-*/` | `out/binaries/` |

The VSCode extension bundles all platform binaries because users download the extension once and may use it across multiple machines via Settings Sync.
