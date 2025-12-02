use crate::protocol::Response;
use std::path::Path;
use std::sync::Arc;
use tscanner_cache::FileCache;
use tscanner_config::{config_dir_name, config_file_name, TscannerConfig};
use tscanner_scanner::{load_config, Scanner};

pub fn error_response(id: u64, error: String) -> Response {
    Response::error(id, error)
}

pub fn success_response(id: u64, value: serde_json::Value) -> Response {
    Response {
        id,
        result: Some(value),
        error: None,
    }
}

pub fn load_config_with_fallback(
    config: Option<TscannerConfig>,
    root: &Path,
) -> Result<TscannerConfig, String> {
    if let Some(cfg) = config {
        Ok(cfg)
    } else {
        load_config(root, config_dir_name(), config_file_name()).map_err(|e| e.to_string())
    }
}

pub fn load_config_from_workspace(root: &Path) -> Result<TscannerConfig, String> {
    load_config(root, config_dir_name(), config_file_name()).map_err(|e| e.to_string())
}

pub fn create_scanner(
    config: TscannerConfig,
    cache: Arc<FileCache>,
    root: &Path,
) -> Result<Scanner, String> {
    Scanner::with_cache(config, cache, root.to_path_buf())
        .map_err(|e| format!("Failed to create scanner: {}", e))
}
