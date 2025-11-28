# Development vs Production Workflows

## Local Testing

### Testing Action Logic Locally

Test the action TypeScript code without GitHub Actions infrastructure:

```bash
cd packages/github-action
pnpm run build
```

Set environment variables to simulate GitHub Actions context:

```bash
export GITHUB_WORKSPACE=/path/to/test/repo
export GITHUB_REPOSITORY=owner/repo
export GITHUB_TOKEN=ghp_your_token_here

node dist/index.js
```

### Mocking GitHub Context

Create a test script to simulate pull request events:

```typescript
process.env.GITHUB_WORKSPACE = '/tmp/test-repo';
process.env.GITHUB_REPOSITORY = 'owner/repo';
process.env.GITHUB_EVENT_NAME = 'pull_request';
process.env.GITHUB_EVENT_PATH = '/tmp/event.json';

await import('./dist/index.js');
```

Event payload (`/tmp/event.json`):

```json
{
  "pull_request": {
    "number": 123,
    "head": {
      "sha": "abc123def456"
    }
  }
}
```

### Testing with act

[act](https://github.com/nektos/act) runs GitHub Actions locally using Docker:

```bash
act pull_request -W .github/workflows/test.yml \
  -s GITHUB_TOKEN=ghp_token \
  --bind
```

Limited support for composite actions and monorepo structure.

## npx vs Local CLI

### Binary Resolution

Production mode uses `npx` to run published CLI:

```typescript
await execCommand('npx', ['tscanner@1.0.0', 'check', '--json']);
```

Dev mode uses local monorepo CLI:

```typescript
const cliPath = `${workspaceRoot}/packages/cli/dist/main.js`;
await execCommand('node', [cliPath, 'check', '--json']);
```

See `packages/github-action/src/core/cli-executor.ts`:

```typescript
export function createProdModeExecutor(version: string): CliExecutor {
  const packageSpec = `tscanner@${version}`;
  return {
    async execute(args: string[]): Promise<string> {
      await execCommand('npx', [packageSpec, ...args]);
    }
  };
}

export function createDevModeExecutor(): CliExecutor {
  const cliPath = `${workspaceRoot}/packages/cli/dist/main.js`;
  return {
    async execute(args: string[]): Promise<string> {
      await execCommand('node', [cliPath, ...args]);
    }
  };
}
```

### Version Pinning

Users control CLI version via `tscanner-version` input:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
  with:
    tscanner-version: '1.2.3'
```

Defaults to `latest` if not specified. Each action run pulls from npm cache or installs specified version.

### Dev Mode Testing

Test unreleased CLI changes with `dev-mode`:

```yaml
- uses: lucasvtiradentes/tscanner-action@main
  with:
    dev-mode: 'true'
```

Requires monorepo context with built CLI at `packages/cli/dist/main.js`.

## Build Process

### tsup Configuration

Single-file bundle for GitHub Actions (see `tsup.config.ts`):

```typescript
export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs'],
  outDir: 'dist',
  clean: true,
  minify: true,
  noExternal: [/.*/],
});
```

Key settings:
- `noExternal: [/.*/]` - Bundle all dependencies (required for GitHub Actions)
- `format: ['cjs']` - CommonJS for Node.js 20 runtime
- `minify: true` - Reduce file size

### Output to dist/

Build produces single file:

```bash
pnpm run build
```

Output:
```
packages/github-action/dist/
└── index.js  # Complete bundled action
```

### Why Bundled

GitHub Actions requires:
1. All code in single repository
2. No external npm installs during action execution
3. Fast cold starts

Bundling ensures all dependencies are included at build time.

## Release Workflow

### Versioning

GitHub Action uses semantic versioning (`v0.0.x` tags):

```json
{
  "name": "tscanner-github-action",
  "version": "0.0.17"
}
```

Version bump triggers publish to standalone repository.

### Standalone Repository Sync

Release process (see `.github/workflows/push-to-main.yml`):

1. **Detect version change** - Compare current vs previous commit
2. **Build action** - Run `pnpm build`
3. **Clone standalone repo** - `lucasvtiradentes/tscanner-action`
4. **Sync files**:
   ```bash
   cp action.yml dist/ README.md LICENSE tscanner-action/
   ```
5. **Create tag** - `git tag v0.0.17`
6. **Push to standalone** - Users install from `lucasvtiradentes/tscanner-action@v0.0.17`

Script: `scripts/release/publish-github-action.sh`

```bash
build_action() {
  cd packages/github-action
  pnpm build
}

sync_files_to_standalone() {
  cp action.yml dist/ README.md LICENSE /tmp/tscanner-action/
}

commit_and_push() {
  git tag v${CURRENT_VERSION}
  git push origin main
  git push origin v${CURRENT_VERSION}
}
```

### GitHub Marketplace

Published version appears at:
- Repository: `github.com/lucasvtiradentes/tscanner-action`
- Marketplace: `github.com/marketplace/actions/tscanner-action`

Users reference by tag:

```yaml
- uses: lucasvtiradentes/tscanner-action@v0.0.17
```

## Common Development Commands

### Full rebuild

```bash
cd packages/github-action
pnpm run build
```

### Type checking

```bash
pnpm run typecheck
```

### Linting

```bash
pnpm run lint
pnpm run lint:fix
```

### Formatting

```bash
pnpm run format
```

### Local testing with monorepo CLI

Build both packages:

```bash
cd packages/cli
pnpm run build

cd ../github-action
pnpm run build
```

Test with dev mode:

```bash
export GITHUB_WORKSPACE=/path/to/test/repo
export INPUT_GITHUB_TOKEN=ghp_token
export INPUT_DEV_MODE=true

node dist/index.js
```

### Validate action.yml syntax

```bash
cat action.yml | docker run --rm -i ghcr.io/rhysd/actionlint:latest -
```

### Test release script locally

```bash
export GITHUB_WORKSPACE=$(pwd)
export GH_PAT_SYNC_TSCANNER_GH_ACTION=ghp_token
bash scripts/release/publish-github-action.sh
```

Requires `tscanner-action` repository access.
