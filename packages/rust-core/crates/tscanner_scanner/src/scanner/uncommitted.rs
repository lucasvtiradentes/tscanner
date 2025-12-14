use super::Scanner;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tscanner_git::{get_uncommitted_files, get_uncommitted_modified_lines};
use tscanner_types::{FileResult, ScanResult};

#[derive(Debug)]
pub struct UncommittedScanResult {
    pub scan_result: ScanResult,
    pub uncommitted_file_count: usize,
}

impl Scanner {
    pub fn scan_uncommitted(&self) -> Result<UncommittedScanResult, String> {
        let start = Instant::now();
        (self.log_info)("Starting uncommitted files scan");

        let uncommitted_files = get_uncommitted_files(&self.root)
            .map_err(|e| format!("Failed to get uncommitted files: {}", e))?;

        let uncommitted_lines = get_uncommitted_modified_lines(&self.root)
            .map_err(|e| format!("Failed to get uncommitted modified lines: {}", e))?;

        let uncommitted_file_count = uncommitted_files.len();
        (self.log_debug)(&format!(
            "Found {} uncommitted files",
            uncommitted_file_count
        ));

        let scan_result =
            self.scan_uncommitted_files(&uncommitted_files, &uncommitted_lines, start);

        Ok(UncommittedScanResult {
            scan_result,
            uncommitted_file_count,
        })
    }

    pub fn scan_uncommitted_with_paths(
        &self,
        paths: &[PathBuf],
    ) -> Result<UncommittedScanResult, String> {
        let start = Instant::now();
        (self.log_info)("Starting uncommitted files scan with custom paths");

        let uncommitted_files = get_uncommitted_files(&self.root)
            .map_err(|e| format!("Failed to get uncommitted files: {}", e))?;

        let uncommitted_lines = get_uncommitted_modified_lines(&self.root)
            .map_err(|e| format!("Failed to get uncommitted modified lines: {}", e))?;

        let files_in_paths: HashSet<PathBuf> = self
            .collect_files_with_filter(paths, Some(&uncommitted_files))
            .into_iter()
            .collect();

        let filtered_files: HashSet<PathBuf> = uncommitted_files
            .intersection(&files_in_paths)
            .cloned()
            .collect();

        let uncommitted_file_count = filtered_files.len();
        (self.log_debug)(&format!(
            "Found {} uncommitted files in specified paths",
            uncommitted_file_count
        ));

        let scan_result = self.scan_uncommitted_files(&filtered_files, &uncommitted_lines, start);

        Ok(UncommittedScanResult {
            scan_result,
            uncommitted_file_count,
        })
    }

    fn scan_uncommitted_files(
        &self,
        uncommitted_files: &HashSet<PathBuf>,
        uncommitted_lines: &HashMap<PathBuf, HashSet<usize>>,
        start: Instant,
    ) -> ScanResult {
        let files: Vec<PathBuf> = uncommitted_files.iter().cloned().collect();
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

        let (script_issues, script_warnings) = self.run_script_rules(&files);

        let mut all_results = results;
        self.merge_issues(&mut all_results, script_issues);

        self.filter_to_uncommitted_lines(&mut all_results, uncommitted_lines);
        let regular_duration = regular_start.elapsed();

        let ai_start = Instant::now();
        let ai_result = if files.is_empty() {
            crate::executors::AiExecutionResult::default()
        } else {
            self.run_ai_rules_with_context(&files, Some(uncommitted_lines))
        };
        self.merge_issues(&mut all_results, ai_result.issues);
        let ai_duration = ai_start.elapsed();

        let total_issues: usize = all_results.iter().map(|r| r.issues.len()).sum();
        let duration = start.elapsed();

        self.cache.flush();
        self.ai_cache.flush();
        self.script_cache.flush();

        let regular_cache_hits = cache_hits.load(Ordering::Relaxed);
        let cached = regular_cache_hits + ai_result.cache_hits;
        let scanned = file_count.saturating_sub(cached);

        let mut warnings: Vec<String> = script_warnings;
        warnings.extend(ai_result.warnings);

        ScanResult {
            files: all_results,
            total_issues,
            duration_ms: duration.as_millis(),
            regular_rules_duration_ms: regular_duration.as_millis(),
            ai_rules_duration_ms: ai_duration.as_millis(),
            total_files: file_count,
            cached_files: cached,
            scanned_files: scanned,
            notes: Vec::new(),
            warnings,
            errors: ai_result.errors,
        }
    }

    fn filter_to_uncommitted_lines(
        &self,
        results: &mut Vec<FileResult>,
        uncommitted_lines: &HashMap<PathBuf, HashSet<usize>>,
    ) {
        for file_result in results.iter_mut() {
            if let Some(lines) = uncommitted_lines.get(&file_result.file) {
                file_result
                    .issues
                    .retain(|issue| lines.contains(&issue.line));
            }
        }
        results.retain(|r| !r.issues.is_empty());
    }
}
