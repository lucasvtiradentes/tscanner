# Stage 1: Extend LSP Server with Custom Requests (Rust)

## Refactor Context

This is part of a multi-stage refactor to merge RPC and LSP servers into a single LSP server.

### All Stages

| Stage | Description | Status |
|-------|-------------|--------|
| **1** | **Extend LSP server with custom requests (Rust)** | **CURRENT** |
| 2 | Migrate VSCode to single LSP client | Pending |

See [00-merge-servers-overview.md](./00-merge-servers-overview.md) for full plan.

---

## Goal

Add custom LSP request handlers to `tscanner_lsp` that replicate all functionality from `tscanner_rpc`.

## Custom Requests to Add

| Method | Params | Result |
|--------|--------|--------|
| `tscanner/scan` | `ScanParams` | `ScanResult` |
| `tscanner/scanFile` | `ScanFileParams` | `FileResult` |
| `tscanner/scanContent` | `ScanContentParams` | `ContentScanResult` |
| `tscanner/clearCache` | `()` | `()` |
| `tscanner/getRulesMetadata` | `()` | `Vec<RuleMetadata>` |
| `tscanner/formatResults` | `FormatResultsParams` | `FormatPrettyResult` |

## New Crate Structure

```
crates/tscanner_lsp/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── capabilities.rs
    ├── server.rs                    # MODIFY - add custom request routing
    ├── session.rs                   # MODIFY - add scanner state
    ├── custom_requests.rs           # CREATE - LSP request type definitions
    └── handlers/
        ├── mod.rs                   # MODIFY - export new handlers
        ├── diagnostics.rs
        ├── code_action.rs
        └── custom/                  # CREATE - folder for custom handlers
            ├── mod.rs
            ├── scan.rs
            ├── scan_file.rs
            ├── scan_content.rs
            ├── clear_cache.rs
            ├── get_rules_metadata.rs
            └── format_results.rs
```

## Steps

### 1. Add dependencies to `tscanner_lsp/Cargo.toml`

```toml
[dependencies]
# ... existing deps ...
tscanner_scanner = { path = "../tscanner_scanner" }
tscanner_cache = { path = "../tscanner_cache" }
tscanner_config = { path = "../tscanner_config" }
```

### 2. Create `tscanner_lsp/src/custom_requests.rs`

```rust
use lsp_types::request::Request;
use serde::{Deserialize, Serialize};
use tscanner_config::TscannerConfig;
use tscanner_diagnostics::{ContentScanResult, FileResult, ScanResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanParams {
    pub root: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<TscannerConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

pub enum ScanRequest {}
impl Request for ScanRequest {
    type Params = ScanParams;
    type Result = ScanResult;
    const METHOD: &'static str = "tscanner/scan";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanFileParams {
    pub root: String,
    pub file: String,
}

pub enum ScanFileRequest {}
impl Request for ScanFileRequest {
    type Params = ScanFileParams;
    type Result = FileResult;
    const METHOD: &'static str = "tscanner/scanFile";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanContentParams {
    pub root: String,
    pub file: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<TscannerConfig>,
}

pub enum ScanContentRequest {}
impl Request for ScanContentRequest {
    type Params = ScanContentParams;
    type Result = ContentScanResult;
    const METHOD: &'static str = "tscanner/scanContent";
}

pub enum ClearCacheRequest {}
impl Request for ClearCacheRequest {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "tscanner/clearCache";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleMetadata {
    pub name: String,
    pub description: String,
    pub category: String,
    pub severity: String,
}

pub enum GetRulesMetadataRequest {}
impl Request for GetRulesMetadataRequest {
    type Params = ();
    type Result = Vec<RuleMetadata>;
    const METHOD: &'static str = "tscanner/getRulesMetadata";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatResultsParams {
    pub root: String,
    pub results: ScanResult,
    pub group_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatPrettyResult {
    pub output: String,
    pub summary: FormatSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatSummary {
    pub total_issues: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub file_count: usize,
    pub rule_count: usize,
}

pub enum FormatResultsRequest {}
impl Request for FormatResultsRequest {
    type Params = FormatResultsParams;
    type Result = FormatPrettyResult;
    const METHOD: &'static str = "tscanner/formatResults";
}
```

### 3. Update `tscanner_lsp/src/session.rs`

Add scanner state (copy from `tscanner_rpc/src/state.rs`):

```rust
use std::path::PathBuf;
use std::sync::Arc;
use tscanner_cache::FileCache;
use tscanner_scanner::Scanner;

pub struct Session {
    root: Option<PathBuf>,
    pub scanner: Option<Scanner>,
    pub cache: Arc<FileCache>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            root: None,
            scanner: None,
            cache: Arc::new(FileCache::new()),
        }
    }

    pub fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root);
    }

    pub fn root(&self) -> Option<&PathBuf> {
        self.root.as_ref()
    }
}
```

### 4. Create `tscanner_lsp/src/handlers/custom/mod.rs`

```rust
mod clear_cache;
mod format_results;
mod get_rules_metadata;
mod scan;
mod scan_content;
mod scan_file;

pub use clear_cache::handle_clear_cache;
pub use format_results::handle_format_results;
pub use get_rules_metadata::handle_get_rules_metadata;
pub use scan::handle_scan;
pub use scan_content::handle_scan_content;
pub use scan_file::handle_scan_file;
```

### 5. Create handler files

Copy logic from `tscanner_rpc/src/handlers/*.rs` to corresponding files in `tscanner_lsp/src/handlers/custom/`.

Example `scan.rs`:

```rust
use crate::custom_requests::ScanParams;
use crate::session::Session;
use lsp_server::{Message, Request, Response};
use lsp_types::request::Request as LspRequest;
use std::path::PathBuf;
use std::sync::Arc;
use tscanner_cache::FileCache;
use tscanner_config::{config_dir_name, config_file_name};
use tscanner_diagnostics::ScanResult;
use tscanner_fs::{get_changed_files, get_modified_lines};
use tscanner_scanner::{load_config, ConfigExt, Scanner};

pub fn handle_scan(
    connection: &lsp_server::Connection,
    req: Request,
    session: &mut Session,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let params: ScanParams = serde_json::from_value(req.params)?;

    let config = if let Some(cfg) = params.config {
        cfg
    } else {
        load_config(&params.root, config_dir_name(), config_file_name())?
    };

    let config_hash = config.compute_hash();
    let cache = Arc::new(FileCache::with_config_hash(config_hash));
    session.cache = cache.clone();

    let root = PathBuf::from(&params.root);
    let scanner = Scanner::with_cache(config, cache, root.clone())?;

    let (changed_files, modified_lines) = if let Some(ref branch_name) = params.branch {
        match (
            get_changed_files(&params.root, branch_name),
            get_modified_lines(&params.root, branch_name),
        ) {
            (Ok(files), Ok(lines)) => (Some(files), Some(lines)),
            (Err(e), _) | (_, Err(e)) => {
                let response = Response::new_err(
                    req.id,
                    lsp_server::ErrorCode::InternalError as i32,
                    format!("Failed to get changed files: {}", e),
                );
                connection.sender.send(Message::Response(response))?;
                return Ok(());
            }
        }
    } else {
        (None, None)
    };

    let mut result = scanner.scan_codebase_with_filter(
        std::slice::from_ref(&root),
        changed_files.as_ref(),
    );

    if let Some(ref line_filter) = modified_lines {
        result.filter_by_modified_lines(line_filter);
    }

    session.scanner = Some(scanner);

    let response = Response::new_ok(req.id, serde_json::to_value(&result)?);
    connection.sender.send(Message::Response(response))?;

    Ok(())
}
```

### 6. Update `tscanner_lsp/src/server.rs`

Add routing for custom requests in `main_loop`:

```rust
use crate::handlers::custom::{
    handle_clear_cache, handle_format_results, handle_get_rules_metadata,
    handle_scan, handle_scan_content, handle_scan_file,
};

fn main_loop(connection: &Connection, session: &mut Session) -> Result<(), LspError> {
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }

                match req.method.as_str() {
                    "textDocument/codeAction" => {
                        handle_code_action_request(connection, req, session)?;
                    }
                    "tscanner/scan" => {
                        handle_scan(connection, req, session)?;
                    }
                    "tscanner/scanFile" => {
                        handle_scan_file(connection, req, session)?;
                    }
                    "tscanner/scanContent" => {
                        handle_scan_content(connection, req, session)?;
                    }
                    "tscanner/clearCache" => {
                        handle_clear_cache(connection, req, session)?;
                    }
                    "tscanner/getRulesMetadata" => {
                        handle_get_rules_metadata(connection, req, session)?;
                    }
                    "tscanner/formatResults" => {
                        handle_format_results(connection, req, session)?;
                    }
                    _ => {}
                }
            }
            Message::Notification(notif) => {
                handle_notification(connection, notif, session)?;
            }
            Message::Response(_) => {}
        }
    }
    Ok(())
}
```

### 7. Update `tscanner_lsp/src/lib.rs`

```rust
mod capabilities;
mod custom_requests;
mod handlers;
mod server;
mod session;

pub use server::run_lsp_server;
```

## Files Summary

| Action | File |
|--------|------|
| CREATE | `tscanner_lsp/src/custom_requests.rs` |
| CREATE | `tscanner_lsp/src/handlers/custom/mod.rs` |
| CREATE | `tscanner_lsp/src/handlers/custom/scan.rs` |
| CREATE | `tscanner_lsp/src/handlers/custom/scan_file.rs` |
| CREATE | `tscanner_lsp/src/handlers/custom/scan_content.rs` |
| CREATE | `tscanner_lsp/src/handlers/custom/clear_cache.rs` |
| CREATE | `tscanner_lsp/src/handlers/custom/get_rules_metadata.rs` |
| CREATE | `tscanner_lsp/src/handlers/custom/format_results.rs` |
| MODIFY | `tscanner_lsp/Cargo.toml` |
| MODIFY | `tscanner_lsp/src/lib.rs` |
| MODIFY | `tscanner_lsp/src/server.rs` |
| MODIFY | `tscanner_lsp/src/session.rs` |
| MODIFY | `tscanner_lsp/src/handlers/mod.rs` |

## Verification

```bash
cd packages/rust-core
cargo build
cargo test

# Test custom requests manually (optional)
# The RPC server should still work for backwards compatibility
```

## Notes

- Handler logic is copied from `tscanner_rpc/src/handlers/` with minimal changes
- Session now holds scanner state like RPC's `ServerState`
- RPC server still works - Stage 2 will migrate VSCode and then RPC can be deleted
- Custom LSP requests use standard request/response pattern (not notifications)
