# Stage 2: Migrate VSCode to Single LSP Client

## Refactor Context

This is part of a multi-stage refactor to merge RPC and LSP servers into a single LSP server.

### All Stages

| Stage | Description | Status |
|-------|-------------|--------|
| 1 | Extend LSP server with custom requests (Rust) | Done |
| **2** | **Migrate VSCode to single LSP client** | **CURRENT** |

See [00-merge-servers-overview.md](./00-merge-servers-overview.md) for full plan.

---

## Goal

Replace dual-client architecture (RustClient + TscannerLspClient) with single LSP client that handles both standard LSP features and custom scan requests.

## Current State

```
scanner.ts
├── rustClient: RustClient        # JSON-RPC client
└── lspClient: TscannerLspClient  # LSP client

rust-client.ts   → spawn(binary)        → RPC Server
lsp-client.ts    → spawn(binary --lsp)  → LSP Server
```

## Target State

```
scanner.ts
└── lspClient: TscannerLspClient  # Single LSP client (extended)

lsp-client.ts → spawn(binary) → LSP Server (handles everything)
```

## Steps

### 1. Extend `lsp-client.ts` with custom request support

```typescript
import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  RequestType,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';
import {
  ContentScanResult,
  FileResult,
  FormatPrettyResult,
  RuleMetadata,
  ScanContentParams,
  ScanFileParams,
  ScanParams,
  ScanResult,
  TscannerConfig,
} from '../types';
import { CONFIG_DIR_NAME, CONFIG_FILE_NAME } from '../constants';
import { logger } from '../utils/logger';

const ScanRequestType = new RequestType<ScanParams, ScanResult, void>('tscanner/scan');
const ScanFileRequestType = new RequestType<ScanFileParams, FileResult, void>('tscanner/scanFile');
const ScanContentRequestType = new RequestType<ScanContentParams, ContentScanResult, void>('tscanner/scanContent');
const ClearCacheRequestType = new RequestType<void, void, void>('tscanner/clearCache');
const GetRulesMetadataRequestType = new RequestType<void, RuleMetadata[], void>('tscanner/getRulesMetadata');

type FormatResultsParams = {
  root: string;
  results: ScanResult;
  group_mode: string;
};

const FormatResultsRequestType = new RequestType<FormatResultsParams, FormatPrettyResult, void>('tscanner/formatResults');

export class TscannerLspClient {
  private client: LanguageClient | null = null;

  constructor(private binaryPath: string) {}

  async start(workspaceRoot: string): Promise<void> {
    if (this.client) {
      logger.info('LSP client already running');
      return;
    }

    logger.info(`Starting LSP client: ${this.binaryPath}`);

    const serverOptions: ServerOptions = {
      command: this.binaryPath,
      args: [],  // No --lsp flag needed anymore
      transport: TransportKind.stdio,
    };

    const clientOptions: LanguageClientOptions = {
      documentSelector: [
        { scheme: 'file', language: 'typescript' },
        { scheme: 'file', language: 'typescriptreact' },
        { scheme: 'file', language: 'javascript' },
        { scheme: 'file', language: 'javascriptreact' },
      ],
      workspaceFolder: vscode.workspace.getWorkspaceFolder(vscode.Uri.file(workspaceRoot)),
      synchronize: {
        fileEvents: [
          vscode.workspace.createFileSystemWatcher('**/*.{ts,tsx,js,jsx}'),
          vscode.workspace.createFileSystemWatcher(`**/${CONFIG_DIR_NAME}/${CONFIG_FILE_NAME}`),
        ],
      },
    };

    this.client = new LanguageClient('tscanner', 'TScanner LSP', serverOptions, clientOptions);

    try {
      await this.client.start();
      logger.info('LSP client started successfully');
    } catch (error) {
      logger.error(`Failed to start LSP client: ${error}`);
      throw error;
    }
  }

  async stop(): Promise<void> {
    if (this.client) {
      logger.info('Stopping LSP client');
      await this.client.stop();
      this.client = null;
      logger.info('LSP client stopped');
    }
  }

  isRunning(): boolean {
    return this.client !== null;
  }

  async scan(params: ScanParams): Promise<ScanResult> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(ScanRequestType, params);
  }

  async scanFile(params: ScanFileParams): Promise<FileResult> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(ScanFileRequestType, params);
  }

  async scanContent(params: ScanContentParams): Promise<ContentScanResult> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(ScanContentRequestType, params);
  }

  async clearCache(): Promise<void> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(ClearCacheRequestType);
  }

  async getRulesMetadata(): Promise<RuleMetadata[]> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(GetRulesMetadataRequestType);
  }

  async formatResults(root: string, results: ScanResult, groupMode: string): Promise<FormatPrettyResult> {
    if (!this.client) throw new Error('LSP client not started');
    return this.client.sendRequest(FormatResultsRequestType, {
      root,
      results,
      group_mode: groupMode,
    });
  }
}
```

### 2. Simplify `scanner.ts`

Replace dual-client with single LSP client:

```typescript
import * as vscode from 'vscode';
import { BINARY_BASE_NAME, PLATFORM_TARGET_MAP, getServerBinaryName } from '../constants';
import type { IssueResult, TscannerConfig } from '../types';
import { getExtensionPath } from '../utils/extension-helper';
import { LOG_FILE_PATH, logger } from '../utils/logger';
import { TscannerLspClient } from './lsp-client';
import { getCurrentWorkspaceFolder, openTextDocument } from './vscode-utils';

let lspClient: TscannerLspClient | null = null;

export function getRustBinaryPath(): string | null {
  // ... keep existing implementation ...
}

async function ensureLspClient(): Promise<TscannerLspClient> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    throw new Error('No workspace folder found');
  }

  if (!lspClient) {
    const binaryPath = getRustBinaryPath();
    if (!binaryPath) {
      throw new Error('Rust binary not found');
    }

    lspClient = new TscannerLspClient(binaryPath);
    await lspClient.start(workspaceFolder.uri.fsPath);
  }

  return lspClient;
}

function mapIssueToResult(uri: vscode.Uri, issue: any, lineText?: string): IssueResult {
  return {
    uri,
    line: issue.line - 1,
    column: issue.column - 1,
    endColumn: issue.end_column - 1,
    text: (lineText ?? issue.line_text ?? '').trim(),
    rule: issue.rule,
    severity: issue.severity,
    message: issue.message,
  };
}

export async function scanWorkspace(
  fileFilter?: Set<string>,
  config?: TscannerConfig,
  branch?: string,
): Promise<IssueResult[]> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) return [];

  try {
    const client = await ensureLspClient();
    const result = await client.scan({
      root: workspaceFolder.uri.fsPath,
      config,
      branch,
    });

    logger.info(`Scan completed: ${result.total_issues} issues in ${result.duration_ms}ms`);

    const results: IssueResult[] = [];
    for (const fileResult of result.files) {
      const uri = vscode.Uri.file(fileResult.file);
      for (const issue of fileResult.issues) {
        results.push(mapIssueToResult(uri, issue, issue.line_text));
      }
    }

    return results;
  } catch (error) {
    logger.error(`Scan failed: ${error}`);
    throw error;
  }
}

export async function scanFile(filePath: string): Promise<IssueResult[]> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) return [];

  try {
    const client = await ensureLspClient();
    const result = await client.scanFile({
      root: workspaceFolder.uri.fsPath,
      file: filePath,
    });

    const uri = vscode.Uri.file(result.file);
    return result.issues.map((issue) => mapIssueToResult(uri, issue));
  } catch (error) {
    logger.error(`Failed to scan file ${filePath}: ${error}`);
    throw error;
  }
}

export type ScanContentResult = {
  issues: IssueResult[];
  relatedFiles: string[];
};

export async function scanContent(
  filePath: string,
  content: string,
  config?: TscannerConfig,
): Promise<ScanContentResult> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) return { issues: [], relatedFiles: [] };

  try {
    const client = await ensureLspClient();
    const result = await client.scanContent({
      root: workspaceFolder.uri.fsPath,
      file: filePath,
      content,
      config,
    });

    const issues = result.issues.map((issue) => {
      const issueFile = issue.file || result.file;
      const uri = vscode.Uri.file(issueFile);
      return mapIssueToResult(uri, issue);
    });

    return {
      issues,
      relatedFiles: result.related_files ?? [],
    };
  } catch (error) {
    logger.error(`Failed to scan content for ${filePath}: ${error}`);
    throw error;
  }
}

export async function clearCache(): Promise<void> {
  const client = await ensureLspClient();
  await client.clearCache();
  logger.info('Cache cleared via LSP');
}

export function getLspClient(): TscannerLspClient | null {
  return lspClient;
}

export async function startLspClient(): Promise<void> {
  await ensureLspClient();
}

export function dispose() {
  if (lspClient) {
    lspClient.stop();
    lspClient = null;
  }
}
```

### 3. Delete `rust-client.ts`

Remove the file entirely:

```bash
rm packages/vscode-extension/src/common/lib/rust-client.ts
```

### 4. Update `extension.ts`

Remove `getRustClient` from command context:

```typescript
const commandContext: CommandContext = {
  context,
  treeView,
  stateRefs,
  updateBadge,
  updateStatusBar,
  getLspClient,  // Replace getRustClient with getLspClient
};
```

### 5. Update `extension-state.ts`

Update `CommandContext` type:

```typescript
import { TscannerLspClient } from './lsp-client';

export type CommandContext = {
  context: vscode.ExtensionContext;
  treeView: vscode.TreeView<any>;
  stateRefs: ExtensionStateRefs;
  updateBadge: () => void;
  updateStatusBar: () => Promise<void>;
  getLspClient: () => TscannerLspClient | null;
};
```

### 6. Update Rust server entry point

Modify `tscanner_server/src/main.rs` to always use LSP:

```rust
fn main() {
    tscanner_service::init_logger("tscanner_server");
    tscanner_service::log_info("TScanner server started");

    if let Err(e) = tscanner_lsp::run_lsp_server() {
        tscanner_service::log_error(&format!("Server error: {}", e));
        std::process::exit(1);
    }
}
```

### 7. Delete `tscanner_rpc` crate

```bash
rm -rf packages/rust-core/crates/tscanner_rpc
```

Update `packages/rust-core/Cargo.toml`:

```toml
[workspace]
members = [
    "crates/tscanner_cache",
    "crates/tscanner_cli",
    "crates/tscanner_config",
    "crates/tscanner_diagnostics",
    "crates/tscanner_fs",
    "crates/tscanner_lsp",
    # "crates/tscanner_rpc",  # REMOVED
    "crates/tscanner_rules",
    "crates/tscanner_scanner",
    "crates/tscanner_server",
    "crates/tscanner_service",
]
```

Update `tscanner_server/Cargo.toml`:

```toml
[dependencies]
tscanner_lsp = { path = "../tscanner_lsp" }
tscanner_service = { path = "../tscanner_service" }
# tscanner_rpc = { path = "../tscanner_rpc" }  # REMOVED
```

## Files Summary

| Action | File |
|--------|------|
| MODIFY | `vscode-extension/src/common/lib/lsp-client.ts` |
| MODIFY | `vscode-extension/src/common/lib/scanner.ts` |
| MODIFY | `vscode-extension/src/common/lib/extension-state.ts` |
| MODIFY | `vscode-extension/src/extension.ts` |
| DELETE | `vscode-extension/src/common/lib/rust-client.ts` |
| MODIFY | `rust-core/crates/tscanner_server/src/main.rs` |
| MODIFY | `rust-core/crates/tscanner_server/Cargo.toml` |
| MODIFY | `rust-core/Cargo.toml` |
| DELETE | `rust-core/crates/tscanner_rpc/` (entire folder) |

## Verification

```bash
# Rust side
cd packages/rust-core
cargo build
cargo test

# VSCode side
cd packages/vscode-extension
npm run build
npm run typecheck

# Full validation
(npm run format 2>/dev/null || ! grep -q "\"format\":" package.json 2>/dev/null) && \
(npm run lint 2>/dev/null || ! grep -q "\"lint\":" package.json 2>/dev/null) && \
(npm run typecheck 2>/dev/null || ! grep -q "\"typecheck\":" package.json 2>/dev/null) && \
(npm run build 2>/dev/null || ! grep -q "\"build\":" package.json 2>/dev/null)

# Manual testing
# 1. Install extension in VSCode
# 2. Open a TypeScript project
# 3. Verify scan works
# 4. Check only ONE server process: ps aux | grep tscanner
```

## Notes

- The `--lsp` flag is no longer needed since server always runs in LSP mode
- `getLspClient` replaces `getRustClient` in command context
- All scan functionality now goes through LSP custom requests
- Single process means less memory usage and simpler debugging
