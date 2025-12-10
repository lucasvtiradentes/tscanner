use lsp_server::{Connection, Message, RequestId, Response};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tscanner_cache::FileCache;
use tscanner_config::{config_dir_name, config_file_name, TscannerConfig};
use tscanner_scanner::{load_config, Scanner};

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn load_config_or_respond(
    connection: &Connection,
    req_id: &RequestId,
    root: &Path,
    provided_config: Option<TscannerConfig>,
) -> Result<Option<TscannerConfig>, LspError> {
    if let Some(cfg) = provided_config {
        return Ok(Some(cfg));
    }

    match load_config(root, config_dir_name(), config_file_name()) {
        Ok(c) => Ok(Some(c)),
        Err(e) => {
            let response = Response::new_err(
                req_id.clone(),
                lsp_server::ErrorCode::InternalError as i32,
                e.to_string(),
            );
            connection.sender.send(Message::Response(response))?;
            Ok(None)
        }
    }
}

pub fn create_scanner_or_respond(
    connection: &Connection,
    req_id: &RequestId,
    config: TscannerConfig,
    cache: Arc<FileCache>,
    root: PathBuf,
) -> Result<Option<Scanner>, LspError> {
    match Scanner::with_cache(config, cache, root) {
        Ok(s) => Ok(Some(s)),
        Err(e) => {
            let response = Response::new_err(
                req_id.clone(),
                lsp_server::ErrorCode::InternalError as i32,
                format!("Failed to create scanner: {}", e),
            );
            connection.sender.send(Message::Response(response))?;
            Ok(None)
        }
    }
}
