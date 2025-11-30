use crate::protocol::{Response, ScanParams};
use crate::state::ServerState;
use core::{get_changed_files, get_modified_lines, FileCache, Scanner, TscannerConfig};
use std::sync::Arc;

pub fn handle_scan(request_id: u64, params: ScanParams, state: &mut ServerState) -> Response {
    core::log_info(&format!("Scanning workspace: {:?}", params.root));

    let config = if let Some(cfg) = params.config {
        core::log_info("Using config from request params (global storage)");
        cfg
    } else {
        match TscannerConfig::load_from_workspace(&params.root) {
            Ok(c) => {
                core::log_info("Loaded configuration from workspace (.tscanner)");
                c
            }
            Err(e) => {
                return Response {
                    id: request_id,
                    result: None,
                    error: Some(e.to_string()),
                };
            }
        }
    };

    let config_hash = config.compute_hash();
    core::log_debug(&format!("Config hash: {}", config_hash));

    let cache = Arc::new(FileCache::with_config_hash(config_hash));
    state.cache = cache.clone();

    let scanner = match Scanner::with_cache(config, cache, params.root.clone()) {
        Ok(s) => s,
        Err(e) => {
            return Response {
                id: request_id,
                result: None,
                error: Some(format!("Failed to create scanner: {}", e)),
            }
        }
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
                return Response {
                    id: request_id,
                    result: None,
                    error: Some(format!("Failed to get changed files: {}", e)),
                }
            }
        }
    } else {
        (None, None)
    };

    let mut result = scanner.scan(&params.root, changed_files.as_ref());

    if let Some(ref line_filter) = modified_lines {
        let original_count = result.files.iter().map(|f| f.issues.len()).sum::<usize>();

        result.files = result
            .files
            .into_iter()
            .filter_map(|mut file_result| {
                if let Some(modified_lines_in_file) = line_filter.get(&file_result.file) {
                    file_result
                        .issues
                        .retain(|issue| modified_lines_in_file.contains(&issue.line));
                    if !file_result.issues.is_empty() {
                        Some(file_result)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let filtered_count = result.files.iter().map(|f| f.issues.len()).sum::<usize>();
        result.total_issues = filtered_count;

        core::log_info(&format!(
            "Filtered {} → {} issues (only modified lines)",
            original_count, filtered_count
        ));
    }

    state.scanner = Some(scanner);

    Response {
        id: request_id,
        result: Some(serde_json::to_value(&result).unwrap()),
        error: None,
    }
}
