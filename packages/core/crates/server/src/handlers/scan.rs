use super::common::{create_scanner, error_response, success_response};
use crate::protocol::{Response, ScanParams};
use crate::state::ServerState;
use core::{config_dir_name, get_changed_files, get_modified_lines, FileCache, TscannerConfig};
use std::sync::Arc;

pub fn handle_scan(request_id: u64, params: ScanParams, state: &mut ServerState) -> Response {
    core::log_info(&format!("Scanning workspace: {:?}", params.root));

    let config = if let Some(cfg) = params.config {
        core::log_info("Using config from request params (global storage)");
        cfg
    } else {
        match TscannerConfig::load_from_workspace(&params.root) {
            Ok(c) => {
                core::log_info(&format!(
                    "Loaded configuration from workspace ({})",
                    config_dir_name()
                ));
                c
            }
            Err(e) => return error_response(request_id, e.to_string()),
        }
    };

    let config_hash = config.compute_hash();
    core::log_debug(&format!("Config hash: {}", config_hash));

    let cache = Arc::new(FileCache::with_config_hash(config_hash));
    state.cache = cache.clone();

    let scanner = match create_scanner(config, cache, &params.root) {
        Ok(s) => s,
        Err(e) => return error_response(request_id, e),
    };

    let (changed_files, modified_lines) = if let Some(ref branch_name) = params.branch {
        match (
            get_changed_files(&params.root, branch_name),
            get_modified_lines(&params.root, branch_name),
        ) {
            (Ok(files), Ok(lines)) => {
                core::log_info(&format!(
                    "Found {} changed files vs {}",
                    files.len(),
                    branch_name
                ));
                (Some(files), Some(lines))
            }
            (Err(e), _) | (_, Err(e)) => {
                return error_response(request_id, format!("Failed to get changed files: {}", e))
            }
        }
    } else {
        (None, None)
    };

    let mut result = scanner.scan(&params.root, changed_files.as_ref());

    if let Some(ref line_filter) = modified_lines {
        result.filter_by_modified_lines(line_filter);
    }

    state.scanner = Some(scanner);

    match serde_json::to_value(&result) {
        Ok(value) => success_response(request_id, value),
        Err(e) => error_response(
            request_id,
            format!("Failed to serialize scan results: {}", e),
        ),
    }
}
