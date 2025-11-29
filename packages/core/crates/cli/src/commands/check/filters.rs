use core::log_info;
use core::types::ScanResult;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub fn apply_line_filter(result: &mut ScanResult, line_filter: &HashMap<PathBuf, HashSet<usize>>) {
    let original_count = result.files.iter().map(|f| f.issues.len()).sum::<usize>();

    result.files = result
        .files
        .drain(..)
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

    log_info(&format!(
        "filters: Filtered {} → {} issues (only modified lines)",
        original_count, filtered_count
    ));
}

pub fn apply_rule_filter(result: &mut ScanResult, rule_name: &str) {
    let original_count = result.files.iter().map(|f| f.issues.len()).sum::<usize>();

    result.files = result
        .files
        .drain(..)
        .filter_map(|mut file_result| {
            file_result.issues.retain(|issue| issue.rule == rule_name);
            if !file_result.issues.is_empty() {
                Some(file_result)
            } else {
                None
            }
        })
        .collect();

    let filtered_count = result.files.iter().map(|f| f.issues.len()).sum::<usize>();
    result.total_issues = filtered_count;

    log_info(&format!(
        "filters: Rule filter {} → {} issues (rule: {})",
        original_count, filtered_count, rule_name
    ));
}

pub fn get_files_to_scan(
    root: &Path,
    file_pattern: Option<&str>,
    changed_files: Option<HashSet<PathBuf>>,
) -> Option<HashSet<PathBuf>> {
    let Some(file_pattern) = file_pattern else {
        return changed_files;
    };

    use glob::Pattern;
    let pattern = match Pattern::new(file_pattern) {
        Ok(p) => p,
        Err(_) => return changed_files,
    };

    if let Some(mut files) = changed_files {
        let original_count = files.len();
        files.retain(|file_path| {
            let relative_path =
                pathdiff::diff_paths(file_path, root).unwrap_or_else(|| file_path.clone());
            pattern.matches_path(&relative_path)
        });
        log_info(&format!(
            "filters: File filter {} → {} files (pattern: {})",
            original_count,
            files.len(),
            file_pattern
        ));
        Some(files)
    } else {
        use walkdir::WalkDir;
        let mut matching_files = HashSet::new();

        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let relative_path = pathdiff::diff_paths(file_path, root)
                    .unwrap_or_else(|| file_path.to_path_buf());

                if pattern.matches_path(&relative_path) {
                    matching_files.insert(file_path.to_path_buf());
                }
            }
        }

        log_info(&format!(
            "filters: File filter found {} matching files (pattern: {})",
            matching_files.len(),
            file_pattern
        ));
        Some(matching_files)
    }
}
