use super::common::{create_scanner, error_response, success_response};
use crate::protocol::{Response, ScanParams};
use crate::state::ServerState;
use std::sync::Arc;
use tscanner_cache::FileCache;
use tscanner_config::{config_dir_name, config_file_name};
use tscanner_fs::{get_changed_files, get_modified_lines};
use tscanner_scanner::{load_config, ConfigExt};

pub fn handle_scan(request_id: u64, params: ScanParams, state: &mut ServerState) -> Response {
    let config = if let Some(cfg) = params.config {
        cfg
    } else {
        match load_config(&params.root, config_dir_name(), config_file_name()) {
            Ok(c) => c,
            Err(e) => return error_response(request_id, e.to_string()),
        }
    };

    let config_hash = config.compute_hash();
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
            (Ok(files), Ok(lines)) => (Some(files), Some(lines)),
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
