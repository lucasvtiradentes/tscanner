use crate::protocol::{Response, ScanParams};
use crate::state::ServerState;
use core::{FileCache, Scanner, TscannerConfig};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

fn get_changed_files(root: &std::path::Path, branch: &str) -> Result<HashSet<PathBuf>, String> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(branch)
        .current_dir(root)
        .output()
        .map_err(|e| format!("Failed to execute git diff: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git diff failed: {}", stderr));
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| root.join(line.trim()))
        .collect();

    Ok(files)
}

pub fn handle_scan(request_id: u64, params: ScanParams, state: &mut ServerState) -> Response {
    core::log_info(&format!("Scanning workspace: {:?}", params.root));

    let config = if let Some(cfg) = params.config {
        core::log_info("Using config from request params (global storage)");
        cfg
    } else {
        match TscannerConfig::load_from_workspace(&params.root) {
            Ok(c) => {
                core::log_info("Loaded configuration from workspace (.tscanner/rules.json)");
                c
            }
            Err(e) => {
                core::log_info(&format!("Using default configuration: {}", e));
                TscannerConfig::default()
            }
        }
    };

    let config_hash = config.compute_hash();
    core::log_debug(&format!("Config hash: {}", config_hash));

    let cache = Arc::new(FileCache::with_config_hash(config_hash));
    state.cache = cache.clone();

    let scanner = match Scanner::with_cache(config, cache) {
        Ok(s) => s,
        Err(e) => {
            return Response {
                id: request_id,
                result: None,
                error: Some(format!("Failed to create scanner: {}", e)),
            }
        }
    };

    let changed_files = if let Some(ref branch_name) = params.branch {
        match get_changed_files(&params.root, branch_name) {
            Ok(files) => {
                core::log_info(&format!(
                    "Found {} changed files vs {}",
                    files.len(),
                    branch_name
                ));
                Some(files)
            }
            Err(e) => {
                return Response {
                    id: request_id,
                    result: None,
                    error: Some(format!("Failed to get changed files: {}", e)),
                }
            }
        }
    } else {
        None
    };

    let result = scanner.scan(&params.root, changed_files.as_ref());

    state.scanner = Some(scanner);

    Response {
        id: request_id,
        result: Some(serde_json::to_value(&result).unwrap()),
        error: None,
    }
}
