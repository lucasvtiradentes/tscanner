# Rust Server Communication

How the VSCode extension communicates with the Rust scanning engine.

## Protocol Overview

**Transport:** Line-delimited JSON-RPC over stdin/stdout

```
┌─────────────────┐         stdin          ┌─────────────────┐
│   RustClient    │ ──────────────────────►│  tscanner-      │
│   (TypeScript)  │                        │  server (Rust)  │
│                 │ ◄──────────────────────│                 │
└─────────────────┘        stdout          └─────────────────┘
                     (JSON or GZIP:base64)
```

## JSON-RPC Format

**Request:**

```json
{"id": 1, "method": "scan", "params": {"root": "/workspace", "config": {...}}}
```

**Response (small):**

```json
{"id": 1, "result": {"files": [...], "total_issues": 5, "duration_ms": 150}}
```

**Response (large, >10KB):**

```
GZIP:H4sIAAAA...base64-encoded-gzip-data...
```

## Available Methods

| Method | Purpose | Params |
|--------|---------|--------|
| `scan` | Full workspace scan | `root`, `config?`, `branch?` |
| `scanFile` | Single file scan | `root`, `file` |
| `scanContent` | In-memory content scan | `root`, `file`, `content`, `config?` |
| `getRulesMetadata` | Get all rule definitions | `{}` |
| `clearCache` | Invalidate scan cache | `{}` |
| `formatResults` | Format for clipboard | `root`, `results`, `group_mode` |

## RustClient Implementation

### Spawning the Server

```typescript
export class RustClient {
  private process: ChildProcess | null = null;

  async start(): Promise<void> {
    this.process = spawn(this.binaryPath, [], {
      stdio: ['pipe', 'pipe', 'pipe'],
      env: {
        ...process.env,
        NO_COLOR: '1',
        RUST_LOG: 'core=warn,server=info',
      },
    });

    this.process.stdout?.on('data', this.handleResponse);
    this.process.stderr?.on('data', this.handleLog);
  }
}
```

### Sending Requests

```typescript
private async sendRequest<M extends RpcMethod>(
  method: M,
  params: RpcRequestMap[M]
): Promise<RpcResponseMap[M]> {
  if (!this.process) {
    await this.start();
  }

  const id = ++this.requestId;
  const request = { id, method, params };

  return new Promise((resolve, reject) => {
    this.pendingRequests.set(id, { resolve, reject });
    this.process?.stdin?.write(`${JSON.stringify(request)}\n`);
  });
}
```

### Handling Responses

```typescript
this.process.stdout?.on('data', (data: Buffer) => {
  this.buffer += data.toString();
  const lines = this.buffer.split('\n');
  this.buffer = lines.pop() || '';

  for (const line of lines) {
    if (!line.trim()) continue;

    let jsonString = line;

    if (line.startsWith('GZIP:')) {
      const base64Data = line.substring(5);
      const compressed = Buffer.from(base64Data, 'base64');
      const decompressed = zlib.gunzipSync(compressed);
      jsonString = decompressed.toString('utf8');
    }

    const response = JSON.parse(jsonString);
    const pending = this.pendingRequests.get(response.id);

    if (pending) {
      this.pendingRequests.delete(response.id);
      if (response.error) {
        pending.reject(new Error(response.error));
      } else {
        pending.resolve(response.result);
      }
    }
  }
});
```

## GZIP Compression

Responses >10KB are GZIP compressed to reduce IPC overhead:

```
Server detects response size > 10KB
       │
       ▼
GZIP compress JSON
       │
       ▼
Base64 encode compressed bytes
       │
       ▼
Prefix with "GZIP:"
       │
       ▼
Write to stdout: "GZIP:H4sIAAAA..."
```

**Client decompression:**

```typescript
if (line.startsWith('GZIP:')) {
  const base64Data = line.substring(5);
  const compressed = Buffer.from(base64Data, 'base64');
  const decompressed = zlib.gunzipSync(compressed);
  jsonString = decompressed.toString('utf8');
}
```

**Compression ratio:** Typically 5-10x reduction for large scan results.

## Process Lifecycle

### Startup

1. Extension activates
2. First scan request triggers `RustClient.start()`
3. Server process spawns
4. Process kept alive for subsequent requests

### Keeping Alive

```typescript
let rustClient: RustClient | null = null;

export async function scanWorkspace() {
  if (!rustClient) {
    rustClient = new RustClient(binaryPath);
    await rustClient.start();
  }

  return rustClient.scan(workspaceRoot, ...);
}
```

### Shutdown

```typescript
export function deactivate() {
  disposeScanner();
}

export function dispose() {
  if (rustClient) {
    rustClient.stop();
    rustClient = null;
  }
}

async stop(): Promise<void> {
  if (this.process) {
    this.process.kill();
    this.process = null;
  }
}
```

## Error Handling

### Process Errors

```typescript
this.process.on('error', (err) => {
  logger.error(`Rust process error: ${err}`);
});

this.process.on('exit', (code) => {
  logger.info(`Rust process exited with code: ${code}`);
  this.process = null;
});
```

### Binary Not Found

```typescript
if (!binaryPath) {
  vscode.window.showErrorMessage(
    'TScanner: Rust binary not found. Build with: cd packages/rust-core && cargo build --release'
  );
  throw new Error('Rust binary not found');
}
```

## Type Definitions

```typescript
type ScanParams = {
  root: string;
  config?: TscannerConfig;
  branch?: string;
};

type ScanResult = {
  files: FileResult[];
  total_issues: number;
  duration_ms: number;
};

type FileResult = {
  file: string;
  issues: Issue[];
};

type Issue = {
  rule: string;
  message: string;
  severity: 'Error' | 'Warning';
  line: number;
  column: number;
  line_text?: string;
};
```
