# Development Guide

## Prerequisites

- **Node.js** 18+ and **pnpm** 8+
- **Rust** 1.70+ (install from https://rustup.rs/)
- **VSCode** 1.100+

## Initial Setup

```bash
./scripts/setup-dev.sh
```

This will:
1. Install Node.js dependencies
2. Build Rust workspace
3. Verify environment

## Development Workflow

### Option 1: Integrated Development

Run both Rust and TypeScript in watch mode:

**Terminal 1 - Rust auto-rebuild:**
```bash
cd packages/lino-core
cargo watch -x build
```

**Terminal 2 - Extension auto-rebuild:**
```bash
pnpm dev
```

**Terminal 3 - VSCode Extension Host:**
- Press `F5` in VSCode
- Or run "Debug: Start Debugging" from command palette

### Option 2: Manual Builds

**Build everything:**
```bash
./scripts/build-all.sh
```

**Build Rust only:**
```bash
cd packages/lino-core
cargo build
```

**Build extension only:**
```bash
pnpm build
```

## Project Structure

```
lino/
├── packages/
│   ├── lino-core/              # Rust workspace
│   │   ├── crates/
│   │   │   ├── lino_core/      # Core library
│   │   │   ├── lino_cli/       # CLI binary
│   │   │   └── lino_server/    # JSON-RPC server
│   │   └── target/             # Build artifacts
│   └── vscode-extension/       # TypeScript extension
│       ├── src/                # Source code
│       ├── out/                # Compiled output
│       └── binaries/           # Downloaded binaries (CI)
├── scripts/                    # Build and setup scripts
├── docs/                       # Documentation
└── plan/                       # Design documents
```

## Testing

### Rust Tests

```bash
cd packages/lino-core
cargo test
```

### Extension Tests

```bash
cd packages/vscode-extension
pnpm test
```

### Integration Tests

```bash
pnpm test
```

## Debugging

### Rust Core

```bash
cd packages/lino-core
RUST_LOG=debug cargo run --bin lino-server
```

### VSCode Extension

1. Set breakpoints in TypeScript files
2. Press `F5` to start debugging
3. Extension runs in separate Extension Development Host window

### Combined Debugging

1. Start Rust server with logging:
   ```bash
   RUST_LOG=trace cargo run --bin lino-server
   ```
2. Modify extension to use RUST_LOG environment variable
3. Press `F5` to debug extension

## Code Style

### Rust

- Run `cargo fmt` before committing
- Run `cargo clippy` to check for common issues
- Follow Rust API guidelines

### TypeScript

- Use existing ESLint configuration
- Run `pnpm format` before committing

## Common Tasks

### Add a new Rust dependency

```bash
cd packages/lino-core
cargo add <package-name>
```

### Add a new Node dependency

```bash
pnpm --filter vscode-extension add <package-name>
```

### Create a new Rust crate

```bash
cd packages/lino-core/crates
cargo new --lib <crate-name>
```

Then add to `packages/lino-core/Cargo.toml`:
```toml
[workspace]
members = [
  "crates/lino_core",
  "crates/lino_cli",
  "crates/lino_server",
  "crates/<crate-name>",
]
```

## Troubleshooting

### Duplicate commands or views in VSCode

If you see duplicate Lino commands/views, old extension versions are cached:

```bash
./scripts/clean-install.sh
```

Then reload VSCode: `Ctrl+Shift+P` → `Developer: Reload Window`

### Rust binary not found

```bash
source "$HOME/.cargo/env"
```

### VSCode extension not loading

1. Check `out/extension.js` exists
2. Rebuild: `pnpm build`
3. Restart Extension Host: Ctrl+Shift+F5

### Extension changes not taking effect

Use clean install to clear all caches:

```bash
./scripts/clean-install.sh
```

### Build errors after pulling

```bash
rm -rf node_modules packages/*/node_modules
pnpm install
cargo clean
cargo build
```

## Release Process

See `.github/workflows/release.yml` for automated release process.

Manual release:
```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions will:
1. Build Rust binaries for all platforms
2. Bundle extension with binaries
3. Create GitHub release
4. Publish to VSCode Marketplace
