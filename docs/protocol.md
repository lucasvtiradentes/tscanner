# JSON-RPC Protocol

Communication protocol between VSCode extension and Rust core.

## Transport

- **Bidirectional:** stdin/stdout
- **Format:** Newline-delimited JSON
- **Encoding:** UTF-8

## Message Types

### Request

```json
{
  "id": 1,
  "method": "methodName",
  "params": {...}
}
```

### Response

```json
{
  "id": 1,
  "result": {...}
}
```

### Error Response

```json
{
  "id": 1,
  "error": {
    "code": -32600,
    "message": "Invalid request"
  }
}
```

### Notification (no response expected)

```json
{
  "method": "notification",
  "params": {...}
}
```

## Methods

### `initialize`

Initialize the server with workspace configuration.

**Request:**
```json
{
  "id": 1,
  "method": "initialize",
  "params": {
    "workspaceRoot": "/path/to/workspace",
    "config": {
      "include": ["**/*.{ts,tsx}"],
      "exclude": ["node_modules/**", "dist/**"]
    }
  }
}
```

**Response:**
```json
{
  "id": 1,
  "result": {
    "capabilities": {
      "incrementalSync": true,
      "watchFiles": true,
      "fixProvider": false
    }
  }
}
```

### `scan`

Scan workspace for issues.

**Request:**
```json
{
  "id": 2,
  "method": "scan",
  "params": {
    "full": true
  }
}
```

**Response:**
```json
{
  "id": 2,
  "result": {
    "issues": [
      {
        "filePath": "/path/to/file.ts",
        "line": 10,
        "column": 5,
        "rule": "no-any-type",
        "severity": "error",
        "message": "Avoid using 'any' type"
      }
    ]
  }
}
```

### `watchFiles`

Enable file watching for incremental updates.

**Request:**
```json
{
  "id": 3,
  "method": "watchFiles",
  "params": {
    "paths": ["src/**/*.ts"]
  }
}
```

**Response:**
```json
{
  "id": 3,
  "result": {
    "watching": true
  }
}
```

## Notifications (Server → Extension)

### `fileChanged`

Sent when a watched file changes.

```json
{
  "method": "fileChanged",
  "params": {
    "path": "/path/to/file.ts",
    "type": "modified"
  }
}
```

### `progress`

Sent during long-running operations.

```json
{
  "method": "progress",
  "params": {
    "operation": "scan",
    "current": 50,
    "total": 100,
    "message": "Scanning files..."
  }
}
```

## Error Codes

| Code | Message | Description |
|------|---------|-------------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid Request | Missing required fields |
| -32601 | Method not found | Unknown method |
| -32602 | Invalid params | Invalid method parameters |
| -32603 | Internal error | Server internal error |

## Implementation Status

⏳ Protocol defined (Phase 0)
⏳ Server implementation (Phase 1)
⏳ Extension client (Phase 1)
