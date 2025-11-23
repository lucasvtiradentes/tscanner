use crate::protocol::{Response, ScanParams};
use crate::state::ServerState;
use core::{FileCache, Scanner, TscannerConfig};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

fn parse_modified_lines(diff_output: &str) -> HashMap<String, HashSet<usize>> {
    let mut file_lines: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut current_file: Option<String> = None;
    let mut current_line: usize = 0;

    for line in diff_output.lines() {
        if line.starts_with("diff --git") {
            current_file = None;
            current_line = 0;
        } else if line.starts_with("+++") {
            if let Some(file_path) = line.strip_prefix("+++ b/") {
                current_file = Some(file_path.to_string());
            }
        } else if line.starts_with("@@") {
            if let Some(hunk_info) = line.split("@@").nth(1) {
                if let Some(new_info) = hunk_info.split_whitespace().nth(1) {
                    if let Some(line_num) = new_info.trim_start_matches('+').split(',').next() {
                        current_line = line_num.parse::<usize>().unwrap_or(0);
                    }
                }
            }
        } else if let Some(ref file) = current_file {
            if line.starts_with('+') && !line.starts_with("+++") {
                file_lines
                    .entry(file.clone())
                    .or_default()
                    .insert(current_line);
                current_line += 1;
            } else if !line.starts_with('-') {
                current_line += 1;
            }
        }
    }

    file_lines
}

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

fn get_modified_lines(
    root: &std::path::Path,
    branch: &str,
) -> Result<HashMap<PathBuf, HashSet<usize>>, String> {
    let output = Command::new("git")
        .arg("diff")
        .arg(branch)
        .current_dir(root)
        .output()
        .map_err(|e| format!("Failed to execute git diff: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git diff failed: {}", stderr));
    }

    let diff_text = String::from_utf8_lossy(&output.stdout);
    let file_lines = parse_modified_lines(&diff_text);

    let result = file_lines
        .into_iter()
        .map(|(file, lines)| (root.join(file), lines))
        .collect();

    Ok(result)
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
