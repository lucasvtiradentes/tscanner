use crate::protocol::Response;
use core::{config_dir_name, FileCache, Scanner, TscannerConfig};
use std::path::Path;
use std::sync::Arc;

pub fn error_response(id: u64, error: String) -> Response {
    Response {
        id,
        result: None,
        error: Some(error),
    }
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
        core::log_debug("Using config from request params (global storage)");
        Ok(cfg)
    } else {
        match TscannerConfig::load_from_workspace(root) {
            Ok(c) => {
                core::log_debug(&format!(
                    "Loaded configuration from workspace ({})",
                    config_dir_name()
                ));
                Ok(c)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn load_config_from_workspace(root: &Path) -> Result<TscannerConfig, String> {
    TscannerConfig::load_from_workspace(root).map_err(|e| e.to_string())
}

pub fn create_scanner(
    config: TscannerConfig,
    cache: Arc<FileCache>,
    root: &Path,
) -> Result<Scanner, String> {
    Scanner::with_cache(config, cache, root.to_path_buf())
        .map_err(|e| format!("Failed to create scanner: {}", e))
}
