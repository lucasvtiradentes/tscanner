# Binary Resolution Strategy

## Overview

The tscanner CLI uses a 3-tier fallback strategy to locate the Rust binary across different environments (development, CI, production). This ensures the binary can be found whether you're developing locally, building in CI, or using the published npm package.

## Resolution Flow

```
getBinaryPath()
    |
    +---> 1. Development Path
    |     npm/cli-{platform}/tscanner[.exe]
    |     (local Cargo build output)
    |
    +---> 2. CI/Build Path
    |     npm/{platform}/tscanner[.exe]
    |     (CI artifact staging)
    |
    +---> 3. Production Path
          require.resolve('@tscanner/cli-{platform}/tscanner[.exe]')
          (npm optionalDependencies)
```

## Tier 1: Development Path

**Purpose**: Local development with Cargo builds

**Path**: `packages/cli/npm/cli-{platform}/tscanner[.exe]`

**When used**:
- Running `pnpm dev` or `cargo build` locally
- Binary built directly by Cargo into platform-specific npm package directories

**Example**:
```
packages/cli/npm/cli-linux-x64/tscanner
packages/cli/npm/cli-darwin-arm64/tscanner
packages/cli/npm/cli-win32-x64/tscanner.exe
```

## Tier 2: CI/Build Path

**Purpose**: CI pipeline artifact staging

**Path**: `packages/cli/npm/{platform}/tscanner[.exe]`

**When used**:
- CI builds binaries and stages them before packaging
- Intermediate location before creating platform packages

**Example**:
```
packages/cli/npm/linux-x64/tscanner
packages/cli/npm/darwin-arm64/tscanner
packages/cli/npm/win32-x64/tscanner.exe
```

## Tier 3: Production Path

**Purpose**: End-user installations via npm

**Path**: `node_modules/@tscanner/cli-{platform}/tscanner[.exe]`

**When used**:
- Production installations via `npm install tscanner`
- Binary resolved from optionalDependencies

**Resolution**:
```typescript
const packageName = PLATFORM_PACKAGE_MAP[platformKey];
const binaryPath = require.resolve(`${packageName}/${binaryName}`);
```

## Platform Detection

### Supported Platforms

| Platform Key | Node Platform | Node Arch | Package Name |
|--------------|---------------|-----------|--------------|
| `linux-x64` | `linux` | `x64` | `@tscanner/cli-linux-x64` |
| `linux-arm64` | `linux` | `arm64` | `@tscanner/cli-linux-arm64` |
| `darwin-x64` | `darwin` | `x64` | `@tscanner/cli-darwin-x64` |
| `darwin-arm64` | `darwin` | `arm64` | `@tscanner/cli-darwin-arm64` |
| `win32-x64` | `win32` | `x64` | `@tscanner/cli-win32-x64` |

### Detection Logic

```typescript
function getPlatformKey(): string {
  const platform = process.platform; // 'linux' | 'darwin' | 'win32'
  const arch = process.arch;         // 'x64' | 'arm64'

  if (platform === 'linux') {
    if (arch === 'x64') return 'linux-x64';
    if (arch === 'arm64') return 'linux-arm64';
  }

  if (platform === 'darwin') {
    if (arch === 'x64') return 'darwin-x64';
    if (arch === 'arm64') return 'darwin-arm64';
  }

  if (platform === 'win32') {
    if (arch === 'x64') return 'win32-x64';
  }

  throw new Error(`Unsupported platform: ${platform}-${arch}`);
}
```

## Binary Naming

Platform-specific binary names determined by OS:

| Platform | Binary Name |
|----------|-------------|
| Linux | `tscanner` |
| macOS | `tscanner` |
| Windows | `tscanner.exe` |

```typescript
function getBinaryName(): string {
  return `tscanner${process.platform === 'win32' ? '.exe' : ''}`;
}
```

## Postinstall Hook

The `postinstall.js` script runs after `npm install` to:

1. Detect platform and architecture
2. Locate the appropriate platform package
3. Make binary executable on Unix systems (chmod 755)
4. Provide user feedback

### Platform Package Resolution

```javascript
const platformKey = getPlatformKey();           // e.g., 'linux-x64'
const packageName = PLATFORM_MAP[platformKey];  // '@tscanner/cli-linux-x64'
const packagePath = require.resolve(packageName);
const binaryPath = join(packagePath, '..', binaryName);
```

### Unix Permissions

```javascript
if (process.platform !== 'win32') {
  chmodSync(binaryPath, 0o755); // rwxr-xr-x
}
```

### Workspace Detection

Suppresses output when running in pnpm workspace to reduce noise during monorepo installs:

```javascript
const isWorkspace = process.env.PNPM_SCRIPT_SRC_DIR !== undefined;
if (!isWorkspace) {
  logger.log('✅ tscanner binary installed successfully');
}
```

## Error Handling

### Unsupported Platform

Runtime error thrown during platform detection:

```
Error: Unsupported platform: freebsd-x64
tscanner is only available for:
  - Linux (x64, arm64)
  - macOS (x64, arm64)
  - Windows (x64)
```

### Binary Not Found

If all three tiers fail to locate binary:

```
Error: Failed to find TScanner binary for linux-x64
Please try reinstalling: npm install tscanner
Error: Cannot find module '@tscanner/cli-linux-x64'
```

### Postinstall Warnings

Non-fatal warnings during postinstall for edge cases:

```
Warning: Failed to install tscanner binary for linux-x64
Expected package: @tscanner/cli-linux-x64
This might happen if optional dependencies were not installed.
```

## Optional Dependencies

Platform packages declared as `optionalDependencies` in `package.json`:

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

npm installs only the package matching the current platform, reducing installation size.

## Execution Flow

```
User runs: tscanner scan
    |
    +---> main.ts: getBinaryPath()
    |       |
    |       +---> binary-resolver.ts
    |       |       |
    |       |       +---> Check Tier 1 (dev path)
    |       |       +---> Check Tier 2 (CI path)
    |       |       +---> Check Tier 3 (npm packages)
    |       |
    |       +---> Returns: /path/to/tscanner[.exe]
    |
    +---> spawn(binaryPath, args, { stdio: 'inherit' })
    |
    +---> Forward stdin/stdout/stderr
    +---> Proxy exit codes and signals
```

## Development Workflow

1. **Cargo Build**: `cargo build --release` outputs to `npm/cli-{platform}/`
2. **Local Testing**: CLI resolves binary from Tier 1 (dev path)
3. **CI Build**: Artifacts staged to Tier 2 (CI path)
4. **Package**: Platform packages created with binaries
5. **Publish**: Users install via npm, resolve from Tier 3

## CI/CD Integration

CI builds all platform binaries in parallel using cross-compilation:

```bash
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

Binaries moved to `npm/{platform}/` for packaging into platform-specific npm packages.
