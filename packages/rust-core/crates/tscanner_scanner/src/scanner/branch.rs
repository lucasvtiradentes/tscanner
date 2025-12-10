use super::Scanner;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tscanner_fs::{get_changed_files, get_modified_lines};
use tscanner_types::{FileResult, ScanResult};

#[derive(Debug)]
pub struct BranchScanResult {
    pub scan_result: ScanResult,
    pub base_branch: String,
    pub changed_file_count: usize,
}

impl Scanner {
    pub fn scan_branch(&self, base_branch: &str) -> Result<BranchScanResult, String> {
        let start = Instant::now();
        (self.log_info)(&format!(
            "Starting branch scan comparing to {}",
            base_branch
        ));

        let changed_files = get_changed_files(&self.root, base_branch)
            .map_err(|e| format!("Failed to get changed files: {}", e))?;

        let modified_lines = get_modified_lines(&self.root, base_branch)
            .map_err(|e| format!("Failed to get modified lines: {}", e))?;

        let changed_file_count = changed_files.len();
        (self.log_debug)(&format!("Found {} changed files", changed_file_count));

        let scan_result =
            self.scan_branch_files(&changed_files, &modified_lines, start, base_branch);

        Ok(BranchScanResult {
            scan_result,
            base_branch: base_branch.to_string(),
            changed_file_count,
        })
    }

    pub fn scan_branch_with_paths(
        &self,
        base_branch: &str,
        paths: &[PathBuf],
    ) -> Result<BranchScanResult, String> {
        let start = Instant::now();
        (self.log_info)(&format!(
            "Starting branch scan comparing to {} with custom paths",
            base_branch
        ));

        let changed_files = get_changed_files(&self.root, base_branch)
            .map_err(|e| format!("Failed to get changed files: {}", e))?;

        let modified_lines = get_modified_lines(&self.root, base_branch)
            .map_err(|e| format!("Failed to get modified lines: {}", e))?;

        let files_in_paths: HashSet<PathBuf> = self
            .collect_files_with_filter(paths, Some(&changed_files))
            .into_iter()
            .collect();

        let filtered_files: HashSet<PathBuf> = changed_files
            .intersection(&files_in_paths)
            .cloned()
            .collect();

        let changed_file_count = filtered_files.len();
        (self.log_debug)(&format!(
            "Found {} changed files in specified paths",
            changed_file_count
        ));

        let scan_result =
            self.scan_branch_files(&filtered_files, &modified_lines, start, base_branch);

        Ok(BranchScanResult {
            scan_result,
            base_branch: base_branch.to_string(),
            changed_file_count,
        })
    }

    fn scan_branch_files(
        &self,
        changed_files: &HashSet<PathBuf>,
        modified_lines: &HashMap<PathBuf, HashSet<usize>>,
        start: Instant,
        _base_branch: &str,
    ) -> ScanResult {
        let files: Vec<PathBuf> = changed_files.iter().cloned().collect();
        let file_count = files.len();

        let processed = AtomicUsize::new(0);
        let cache_hits = AtomicUsize::new(0);

        let regular_start = Instant::now();
        let results: Vec<FileResult> = files
            .par_iter()
            .filter_map(|path| {
                processed.fetch_add(1, Ordering::Relaxed);

                if let Some(cached_issues) = self.cache.get(path) {
                    cache_hits.fetch_add(1, Ordering::Relaxed);
                    if cached_issues.is_empty() {
                        return None;
                    }
                    return Some(FileResult {
                        file: path.clone(),
                        issues: cached_issues,
                    });
                }

                self.run_builtin_executor(path)
            })
            .filter(|r| !r.issues.is_empty())
            .collect();

        let script_issues = self.run_script_rules(&files);

        let mut all_results = results;
        self.merge_issues(&mut all_results, script_issues);

        self.filter_to_modified_lines(&mut all_results, modified_lines);
        let regular_duration = regular_start.elapsed();

        let ai_start = Instant::now();
        let (ai_issues, ai_warning) = if files.is_empty() {
            (vec![], None)
        } else {
            self.run_ai_rules_with_context(&files, Some(modified_lines))
        };
        self.merge_issues(&mut all_results, ai_issues);
        let ai_duration = ai_start.elapsed();

        let total_issues: usize = all_results.iter().map(|r| r.issues.len()).sum();
        let duration = start.elapsed();

        self.cache.flush();

        let cached = cache_hits.load(Ordering::Relaxed);
        let scanned = file_count - cached;

        let warnings = ai_warning.into_iter().collect();

        ScanResult {
            files: all_results,
            total_issues,
            duration_ms: duration.as_millis(),
            regular_rules_duration_ms: regular_duration.as_millis(),
            ai_rules_duration_ms: ai_duration.as_millis(),
            total_files: file_count,
            cached_files: cached,
            scanned_files: scanned,
            warnings,
        }
    }

    fn filter_to_modified_lines(
        &self,
        results: &mut Vec<FileResult>,
        modified_lines: &HashMap<PathBuf, HashSet<usize>>,
    ) {
        for file_result in results.iter_mut() {
            if let Some(lines) = modified_lines.get(&file_result.file) {
                file_result
                    .issues
                    .retain(|issue| lines.contains(&issue.line));
            }
        }
        results.retain(|r| !r.issues.is_empty());
    }
}
