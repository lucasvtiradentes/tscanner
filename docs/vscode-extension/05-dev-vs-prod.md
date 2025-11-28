# Dev vs Prod

Development setup and production build process for the VSCode extension.

## Binary Resolution

The extension looks for the Rust server binary in this order:

```
1. Bundled binary (production)
   └─► out/binaries/tscanner-server-{target}

2. Dev release binary
   └─► ../../core/target/release/tscanner-server

3. Dev debug binary
   └─► ../../core/target/debug/tscanner-server
```

**Resolution code:**

```typescript
export function getRustBinaryPath(): string | null {
  const extensionPath = getExtensionPath();
  const platform = process.platform;
  const arch = process.arch;
  const target = PLATFORM_TARGET_MAP[`${platform}-${arch}`];

  // 1. Check bundled binary
  const bundledBinary = join(extensionPath, 'out', 'binaries', binaryName);
  if (existsSync(bundledBinary)) return bundledBinary;

  // 2. Check dev release binary
  const devRelease = join(extensionPath, '..', '..', 'core', 'target', 'release', binaryName);
  if (existsSync(devRelease)) return devRelease;

  // 3. Check dev debug binary
  const devDebug = join(extensionPath, '..', '..', 'core', 'target', 'debug', binaryName);
  if (existsSync(devDebug)) return devDebug;

  return null;
}
```

**Platform target mapping:**

| Platform-Arch | Rust Target |
|---------------|-------------|
| `linux-x64` | `x86_64-unknown-linux-gnu` |
| `linux-arm64` | `aarch64-unknown-linux-gnu` |
| `darwin-x64` | `x86_64-apple-darwin` |
| `darwin-arm64` | `aarch64-apple-darwin` |
| `win32-x64` | `x86_64-pc-windows-msvc` |

## Local Development

### Prerequisites

```bash
# Build Rust binary first
cd packages/core
cargo build --release
```

### Running in Debug Mode

1. Open monorepo in VSCode
2. Press F5 (launches Extension Development Host)
3. New VSCode window opens with extension loaded
4. Extension uses dev binary from `packages/core/target/release/`

### Dev Build Flag

esbuild injects `__IS_DEV_BUILD__` at build time:

```typescript
// esbuild.config.ts
const isDev = !process.env.CI;

const extensionBuildOptions = {
  define: {
    __IS_DEV_BUILD__: isDev ? 'true' : 'false',
  },
};
```

**Usage in code:**

```typescript
declare const __IS_DEV_BUILD__: boolean;
const IS_DEV = typeof __IS_DEV_BUILD__ !== 'undefined' && __IS_DEV_BUILD__;

export function getCommandId(command: string): string {
  return IS_DEV ? `tscannerDev.${command}` : `tscanner.${command}`;
}
```

This allows running dev and prod extensions side-by-side.

## Building for Production

### Extension Bundle

```bash
cd packages/vscode-extension
pnpm run build
```

**esbuild configuration:**

```typescript
const extensionBuildOptions: BuildOptions = {
  entryPoints: ['src/extension.ts'],
  bundle: true,
  outfile: 'out/extension.js',
  external: ['vscode'],
  format: 'cjs',
  platform: 'node',
  target: 'node18',
};
```

### Output Structure

```
out/
├── extension.js           # Bundled extension code
└── binaries/              # Platform-specific Rust binaries
    ├── tscanner-server-x86_64-unknown-linux-gnu
    ├── tscanner-server-aarch64-unknown-linux-gnu
    ├── tscanner-server-x86_64-apple-darwin
    ├── tscanner-server-aarch64-apple-darwin
    └── tscanner-server-x86_64-pc-windows-msvc.exe
```

### Building Rust Binaries

Cross-compile for all platforms:

```bash
cd packages/core

# Linux x64
cargo build --release --target x86_64-unknown-linux-gnu

# Linux ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# macOS x64
cargo build --release --target x86_64-apple-darwin

# macOS ARM64
cargo build --release --target aarch64-apple-darwin

# Windows x64
cargo build --release --target x86_64-pc-windows-msvc
```

## Packaging

### VSIX Package

```bash
cd packages/vscode-extension
pnpm run vscode:package
# Outputs: tscanner-vscode-0.0.x.vsix
```

**Package command:**

```json
{
  "scripts": {
    "vscode:package": "vsce package --no-dependencies"
  }
}
```

`--no-dependencies` because all dependencies are bundled.

### Publishing

**VS Code Marketplace:**

```bash
vsce publish --no-dependencies
```

**Open VSX Registry:**

```bash
ovsx publish tscanner-vscode-0.0.x.vsix
```

## Common Development Commands

| Command | Description |
|---------|-------------|
| `pnpm run build` | Build extension bundle |
| `pnpm run typecheck` | TypeScript type check |
| `pnpm run lint` | Run Biome linter |
| `pnpm run lint:fix` | Auto-fix lint issues |
| `pnpm run vscode:package` | Create VSIX package |
| `pnpm run vscode:publish` | Publish to Marketplace |

## Testing Locally

### Manual Testing

1. Build extension: `pnpm run build`
2. Press F5 to launch Extension Development Host
3. Open a TypeScript project
4. Verify scanning works

### Testing Rust Changes

1. Make changes in `packages/core`
2. Rebuild: `cargo build --release`
3. Reload Extension Development Host (Ctrl+R in debug window)
4. Extension automatically uses updated binary

### Testing Package

```bash
# Package
pnpm run vscode:package

# Install in VSCode
code --install-extension tscanner-vscode-0.0.x.vsix

# Or via UI: Extensions → ... → Install from VSIX
```
