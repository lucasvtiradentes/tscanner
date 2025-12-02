# Contributing

## Prerequisites

- **Node.js** ≥ 18
- **pnpm** ≥ 8 (`npm install -g pnpm`)
- **Rust** ≥ 1.70 ([rustup.rs](https://rustup.rs))

## Setup

```bash
git clone https://github.com/lucasvtiradentes/tscanner.git
cd tscanner
pnpm install
pnpm run build
```

## Project Structure

```
packages/
├── rust-core/          # Rust scanner engine (SWC parser, 39+ rules)
├── cli/                # TypeScript CLI wrapper + platform binaries
├── vscode-extension/   # VSCode/Cursor/VSCodium extension
└── github-action/      # CI/CD integration

shared/
└── tscanner-common/    # Shared types and schemas

scripts/
├── instal-local/       # Local dev installation scripts
├── update-docs/        # Auto-generate documentation
└── release/            # Publishing automation
```

## Development

### Commands

| Command | Description |
|---------|-------------|
| `pnpm run build` | Build all packages (Rust → TS → extension) |
| `pnpm run dev` | Run CLI in dev mode |
| `pnpm run typecheck` | Check TypeScript types |
| `pnpm run lint` | Run biome linter |
| `pnpm run lint:fix` | Auto-fix lint issues |
| `pnpm run format` | Format code |
| `pnpm run test` | Run all tests |
| `pnpm run clean` | Remove build artifacts |

### Local Installation

After `pnpm run build`, these scripts run automatically:

1. **`scripts/instal-local/install-local-cli.ts`** - Copies Rust binary to CLI package
2. **`scripts/instal-local/install-local-vscode.ts`** - Installs dev extension to editors

The VSCode extension installs with "(Dev)" suffix to avoid conflicts with production version.

### Testing VSCode Extension

1. Run `pnpm run build`
2. Reload VSCode (`Ctrl+Shift+P` → "Reload Window")
3. Extension appears as "TScanner (Dev)" in sidebar

## Package Workflow

```
core (Rust)
    ↓ builds binaries
cli, vscode-extension
    ↓ uses binaries
github-action
```

All packages share types via `tscanner-common`.

## Code Style

- **Formatter**: Biome (2 spaces, single quotes, trailing commas)
- **Linting**: Biome with strict unused imports/variables
- **Commits**: Conventional commits (`feat:`, `fix:`, `chore:`, etc.)

## Adding Rules

Rules live in `packages/rust-core/crates/tscanner_rules/src/`. Each rule:

1. Implements the `Rule` trait
2. Registers via `inventory::submit!`
3. Gets auto-discovered by the registry

See existing rules for examples.

## Pull Requests

1. Fork and create a feature branch
2. Make changes with tests if applicable
3. Run `pnpm run typecheck && pnpm run lint && pnpm run build`
4. Submit PR against `main`

## Troubleshooting

**Rust build fails**: Ensure `cargo` is in PATH and run `rustup update`

**Extension not updating**: Run `pnpm run clean && pnpm install && pnpm run build`, then reload VSCode

**Binary not found**: Check `packages/rust-core/target/release/` for compiled binaries
