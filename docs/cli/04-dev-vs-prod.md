# Development vs Production Workflows

## Local Development Setup

### Building Rust Binary Locally

Build the CLI binary for your platform:

```bash
cd packages/rust-core
cargo build --release
```

The binary is created at `packages/rust-core/target/release/tscanner` (or `tscanner.exe` on Windows).

### How CLI Finds the Binary

Binary resolution follows this priority order (see `packages/cli/src/binary-resolver.ts`):

1. **Dev path**: `packages/cli/npm/cli-{platform}/tscanner`
   - Created by `scripts/instal-local/install-local-cli.ts`
   - Used during local development

2. **CI path**: `packages/cli/npm/{platform}/tscanner`
   - Alternative location for CI builds

3. **Installed package**: `@tscanner/cli-{platform}/tscanner`
   - Resolved via `require.resolve()`
   - Used in production after npm install

### Running CLI in Dev Mode

After building Rust binary, the install script runs automatically:

```bash
pnpm run build
```

This executes:
1. Rust build (`cargo build --release`)
2. TypeScript build (`tsup`)
3. Local install script (copies binary to `packages/cli/npm/cli-{platform}/`)

Then run the CLI:

```bash
cd packages/cli
pnpm run dev scan /path/to/project
```

Or test the built version:

```bash
pnpm run start scan /path/to/project
```

## Building for Production

### Cross-Compilation Targets

The project supports 5 platform targets:

| Platform | Rust Target | npm Package |
|----------|-------------|-------------|
| Linux x64 | `x86_64-unknown-linux-gnu` | `@tscanner/cli-linux-x64` |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | `@tscanner/cli-linux-arm64` |
| macOS x64 | `x86_64-apple-darwin` | `@tscanner/cli-darwin-x64` |
| macOS ARM64 | `aarch64-apple-darwin` | `@tscanner/cli-darwin-arm64` |
| Windows x64 | `x86_64-pc-windows-msvc` | `@tscanner/cli-win32-x64` |

Cross-compilation workflow (Linux ARM64 example):

```bash
rustup target add aarch64-unknown-linux-gnu
sudo apt-get install gcc-aarch64-linux-gnu

cat > .cargo/config.toml << EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF

cd packages/rust-core
cargo build --release --target aarch64-unknown-linux-gnu
```

### Creating Platform Packages

After building all platform binaries, generate scoped npm packages:

```bash
pnpm tsx scripts/release/generate-cli-packages.ts
```

This creates packages in `packages/cli/npm/cli-{platform}/`:
- `package.json` with `@tscanner/cli-{platform}` name
- `tscanner` or `tscanner.exe` binary
- OS/CPU constraints for optional dependency installation

### npm Publish Workflow

Production publishing follows these steps:

1. **Build all platforms** (GitHub Actions matrix)
   - Runs on ubuntu-latest, macos-latest, windows-latest
   - Uses `.github/actions/build-rust-binary/action.yml`
   - Uploads artifacts: `binary-cli-{target}`

2. **Organize binaries**
   - Downloads all artifacts
   - Copies to `packages/cli/npm/cli-{platform}/`

3. **Generate packages**
   - Runs `scripts/release/generate-cli-packages.ts`
   - Creates `package.json` for each platform
   - Updates main CLI package `optionalDependencies`

4. **Publish to npm**
   - Publishes `@tscanner/cli-{platform}` packages (5x)
   - Publishes main `tscanner` package
   - Users install main package, npm installs platform-specific binary as optional dependency

## Testing Locally

### Using npm link

Test CLI as if it were installed globally:

```bash
cd packages/cli
pnpm run build
npm link
```

Then use anywhere:

```bash
tscanner scan ~/my-project
tscanner rules --list
```

Unlink when done:

```bash
npm unlink -g tscanner
```

### Integration Testing

Create a test project and use local CLI:

```bash
mkdir /tmp/test-project
cd /tmp/test-project
npm init -y

cd /path/to/tscanner/packages/cli
pnpm run dev scan /tmp/test-project
```

### Testing Platform-Specific Binary

Manually specify a different platform binary:

```bash
export TSCANNER_BINARY_PATH=/path/to/tscanner/packages/cli/npm/cli-linux-arm64/tscanner

node packages/cli/dist/main.js scan ~/project
```

This bypasses the automatic platform detection.

## Common Development Commands

### Full rebuild

```bash
pnpm run clean
pnpm install
pnpm run build
```

### Rust only

```bash
cd packages/rust-core
cargo build --release
```

### CLI only (after Rust build)

```bash
cd packages/cli
pnpm run build
```

### Type checking

```bash
cd packages/cli
pnpm run typecheck
```

### Linting

```bash
cd packages/cli
pnpm run lint
pnpm run lint:fix
```

### Testing postinstall behavior

```bash
cd packages/cli
node postinstall.js
```

This verifies binary installation and platform detection.

### Release build (all platforms)

Full cross-platform build requires CI/CD (GitHub Actions). For local testing of release scripts:

```bash
export CI=true
pnpm tsx scripts/release/generate-cli-packages.ts
```

Requires pre-existing binaries in `packages/cli/npm/cli-{platform}/`.
