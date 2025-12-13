use super::Scanner;
use crate::executors::{AiProgressCallback, ChangedLinesMap, RegularRulesCompleteCallback};
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tscanner_config::AiExecutionMode;
use tscanner_types::{ContentScanResult, FileResult, ScanResult};

pub struct ScanCallbacks {
    pub on_regular_rules_complete: Option<RegularRulesCompleteCallback>,
    pub on_ai_progress: Option<AiProgressCallback>,
}

impl Scanner {
    pub fn scan_codebase(&self, roots: &[PathBuf]) -> ScanResult {
        self.scan_codebase_with_filter(roots, None)
    }

    pub fn scan_codebase_with_filter(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
    ) -> ScanResult {
        self.scan_codebase_with_filter_and_ai_mode(roots, file_filter, AiExecutionMode::Ignore)
    }

    pub fn scan_codebase_with_filter_and_ai_mode(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
        ai_mode: AiExecutionMode,
    ) -> ScanResult {
        self.scan_codebase_with_filter_and_ai_mode_and_lines(roots, file_filter, ai_mode, None)
    }

    pub fn scan_codebase_with_filter_and_ai_mode_and_lines(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
        ai_mode: AiExecutionMode,
        changed_lines: Option<&ChangedLinesMap>,
    ) -> ScanResult {
        self.scan_codebase_with_callbacks(
            roots,
            file_filter,
            ai_mode,
            changed_lines,
            ScanCallbacks {
                on_regular_rules_complete: None,
                on_ai_progress: None,
            },
        )
    }

    pub fn scan_codebase_with_progress(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
        ai_mode: AiExecutionMode,
        changed_lines: Option<&ChangedLinesMap>,
        ai_progress_callback: Option<AiProgressCallback>,
    ) -> ScanResult {
        self.scan_codebase_with_callbacks(
            roots,
            file_filter,
            ai_mode,
            changed_lines,
            ScanCallbacks {
                on_regular_rules_complete: None,
                on_ai_progress: ai_progress_callback,
            },
        )
    }

    pub fn scan_codebase_with_callbacks(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
        ai_mode: AiExecutionMode,
        changed_lines: Option<&ChangedLinesMap>,
        callbacks: ScanCallbacks,
    ) -> ScanResult {
        let start = Instant::now();
        (self.log_info)(&format!(
            "Starting codebase scan of {:?} (ai_mode: {:?})",
            roots, ai_mode
        ));

        if let Some(filter) = file_filter {
            if filter.is_empty() {
                (self.log_info)("No files to scan (empty file filter), skipping");
                return ScanResult {
                    files: Vec::new(),
                    total_issues: 0,
                    duration_ms: start.elapsed().as_millis(),
                    regular_rules_duration_ms: 0,
                    ai_rules_duration_ms: 0,
                    total_files: 0,
                    cached_files: 0,
                    scanned_files: 0,
                    warnings: Vec::new(),
                };
            }
        }

        let (files, ai_files_count) = if ai_mode == AiExecutionMode::Only {
            let ai_rules = self.collect_ai_rules();
            let ai_files = self.collect_ai_files(&ai_rules);
            let count = ai_files.len();
            (Vec::new(), count)
        } else {
            let files = self.collect_files_with_filter(roots, file_filter);
            (files, 0)
        };
        let file_count = if ai_mode == AiExecutionMode::Only {
            ai_files_count
        } else {
            files.len()
        };
        (self.log_debug)(&format!("Found {} files to scan", file_count));

        let processed = AtomicUsize::new(0);
        let cache_hits = AtomicUsize::new(0);

        let regular_start = Instant::now();
        let results: Vec<FileResult> = if ai_mode == AiExecutionMode::Only {
            Vec::new()
        } else {
            files
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
                .collect()
        };

        let (script_issues, script_warnings) = if ai_mode == AiExecutionMode::Only {
            (Vec::new(), Vec::new())
        } else {
            self.run_script_rules(&files)
        };
        let regular_duration = regular_start.elapsed();

        if let Some(ref cb) = callbacks.on_regular_rules_complete {
            cb(regular_duration.as_millis());
        }

        let ai_start = Instant::now();
        let (ai_issues, ai_warning) = if ai_mode == AiExecutionMode::Ignore {
            (Vec::new(), None)
        } else {
            self.run_ai_rules_with_context_and_progress(
                &[],
                changed_lines,
                callbacks.on_ai_progress,
            )
        };
        let ai_duration = ai_start.elapsed();

        let mut all_results = results;
        self.merge_issues(&mut all_results, script_issues);
        self.merge_issues(&mut all_results, ai_issues);

        let total_issues: usize = all_results.iter().map(|r| r.issues.len()).sum();
        let duration = start.elapsed();

        self.cache.flush();
        self.ai_cache.flush();

        let cached = cache_hits.load(Ordering::Relaxed);
        let scanned = file_count - cached;

        let mut warnings: Vec<String> = script_warnings;
        warnings.extend(ai_warning);

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
