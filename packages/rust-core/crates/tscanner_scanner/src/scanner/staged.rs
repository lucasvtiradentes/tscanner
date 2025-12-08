use super::Scanner;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tscanner_diagnostics::{FileResult, ScanResult};
use tscanner_fs::{get_staged_files, get_staged_modified_lines};

#[derive(Debug)]
pub struct StagedScanResult {
    pub scan_result: ScanResult,
    pub staged_file_count: usize,
}

impl Scanner {
    pub fn scan_staged(&self) -> Result<StagedScanResult, String> {
        let start = Instant::now();
        (self.log_info)("Starting staged files scan");

        let staged_files = get_staged_files(&self.root)
            .map_err(|e| format!("Failed to get staged files: {}", e))?;

        let staged_lines = get_staged_modified_lines(&self.root)
            .map_err(|e| format!("Failed to get staged modified lines: {}", e))?;

        let staged_file_count = staged_files.len();
        (self.log_debug)(&format!("Found {} staged files", staged_file_count));

        let scan_result = self.scan_staged_files(&staged_files, &staged_lines, start);

        Ok(StagedScanResult {
            scan_result,
            staged_file_count,
        })
    }

    pub fn scan_staged_with_paths(&self, paths: &[PathBuf]) -> Result<StagedScanResult, String> {
        let start = Instant::now();
        (self.log_info)("Starting staged files scan with custom paths");

        let staged_files = get_staged_files(&self.root)
            .map_err(|e| format!("Failed to get staged files: {}", e))?;

        let staged_lines = get_staged_modified_lines(&self.root)
            .map_err(|e| format!("Failed to get staged modified lines: {}", e))?;

        let files_in_paths: HashSet<PathBuf> = self
            .collect_files_with_filter(paths, Some(&staged_files))
            .into_iter()
            .collect();

        let filtered_files: HashSet<PathBuf> = staged_files
            .intersection(&files_in_paths)
            .cloned()
            .collect();

        let staged_file_count = filtered_files.len();
        (self.log_debug)(&format!(
            "Found {} staged files in specified paths",
            staged_file_count
        ));

        let scan_result = self.scan_staged_files(&filtered_files, &staged_lines, start);

        Ok(StagedScanResult {
            scan_result,
            staged_file_count,
        })
    }

    fn scan_staged_files(
        &self,
        staged_files: &HashSet<PathBuf>,
        staged_lines: &HashMap<PathBuf, HashSet<usize>>,
        start: Instant,
    ) -> ScanResult {
        let files: Vec<PathBuf> = staged_files.iter().cloned().collect();
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

        self.filter_to_staged_lines(&mut all_results, staged_lines);
        let regular_duration = regular_start.elapsed();

        let ai_start = Instant::now();
        let (ai_issues, ai_warning) = self.run_ai_rules_with_context(&files, Some(staged_lines));
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

    fn filter_to_staged_lines(
        &self,
        results: &mut Vec<FileResult>,
        staged_lines: &HashMap<PathBuf, HashSet<usize>>,
    ) {
        for file_result in results.iter_mut() {
            if let Some(lines) = staged_lines.get(&file_result.file) {
                file_result
                    .issues
                    .retain(|issue| lines.contains(&issue.line));
            }
        }
        results.retain(|r| !r.issues.is_empty());
    }
}
