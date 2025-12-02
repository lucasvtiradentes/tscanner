use super::Scanner;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tscanner_diagnostics::{ContentScanResult, FileResult, ScanResult};

impl Scanner {
    pub fn scan_codebase(&self, roots: &[PathBuf]) -> ScanResult {
        self.scan_codebase_with_filter(roots, None)
    }

    pub fn scan_codebase_with_filter(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
    ) -> ScanResult {
        let start = Instant::now();
        (self.log_info)(&format!("Starting codebase scan of {:?}", roots));

        let files = self.collect_files_with_filter(roots, file_filter);
        let file_count = files.len();
        (self.log_debug)(&format!("Found {} files to scan", file_count));

        let processed = AtomicUsize::new(0);
        let cache_hits = AtomicUsize::new(0);

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
        let ai_issues = self.run_ai_rules(&files);

        let mut all_results = results;
        self.merge_issues(&mut all_results, script_issues);
        self.merge_issues(&mut all_results, ai_issues);

        let total_issues: usize = all_results.iter().map(|r| r.issues.len()).sum();
        let duration = start.elapsed();

        self.cache.flush();

        let cached = cache_hits.load(Ordering::Relaxed);
        let scanned = file_count - cached;

        ScanResult {
            files: all_results,
            total_issues,
            duration_ms: duration.as_millis(),
            total_files: file_count,
            cached_files: cached,
            scanned_files: scanned,
        }
    }

    pub fn scan_single(&self, path: &Path) -> Option<FileResult> {
        self.cache.invalidate(path);
        self.run_builtin_executor(path)
    }

    pub fn scan_content(&self, path: &Path, content: &str) -> Option<ContentScanResult> {
        let builtin_result = self.run_builtin_executor_no_cache(path, content);
        let (script_issues, related_files) = self.run_script_rules_for_content(path, content);

        let mut all_issues = match builtin_result {
            Some(r) => r.issues,
            None => Vec::new(),
        };
        all_issues.extend(script_issues);

        if all_issues.is_empty() && related_files.is_empty() {
            return None;
        }

        Some(ContentScanResult {
            file: path.to_path_buf(),
            issues: all_issues,
            related_files,
        })
    }

    pub fn scan(&self, root: &Path, file_filter: Option<&HashSet<PathBuf>>) -> ScanResult {
        self.scan_codebase_with_filter(&[root.to_path_buf()], file_filter)
    }

    pub fn scan_multi(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
    ) -> ScanResult {
        self.scan_codebase_with_filter(roots, file_filter)
    }
}
