use crate::parser::parse_file;
use crate::rules::Rule;
use crate::types::{FileResult, ScanResult};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tracing::{debug, info};

pub struct Scanner {
    rules: Vec<Box<dyn Rule>>,
}

impl Scanner {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self { rules }
    }

    pub fn scan(&self, root: &Path) -> ScanResult {
        let start = Instant::now();
        info!("Starting scan of {:?}", root);

        let files: Vec<PathBuf> = WalkBuilder::new(root)
            .hidden(false)
            .git_ignore(true)
            .filter_entry(|e| {
                let path = e.path();
                if path.is_dir() {
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    return name != "node_modules" && name != ".git" && name != "dist";
                }
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    ext == "ts" || ext == "tsx"
                } else {
                    false
                }
            })
            .build()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_path_buf())
            .collect();

        let file_count = files.len();
        info!("Found {} TypeScript files", file_count);

        let processed = AtomicUsize::new(0);

        let results: Vec<FileResult> = files
            .par_iter()
            .filter_map(|path| {
                let count = processed.fetch_add(1, Ordering::Relaxed) + 1;
                if count % 100 == 0 {
                    debug!("Processed {}/{} files", count, file_count);
                }

                self.analyze_file(path)
            })
            .filter(|r| !r.issues.is_empty())
            .collect();

        let total_issues: usize = results.iter().map(|r| r.issues.len()).sum();
        let duration = start.elapsed();

        info!(
            "Scan complete: {} issues in {} files ({:.2}ms)",
            total_issues,
            results.len(),
            duration.as_millis()
        );

        ScanResult {
            files: results,
            total_issues,
            duration_ms: duration.as_millis(),
        }
    }

    fn analyze_file(&self, path: &Path) -> Option<FileResult> {
        let source = std::fs::read_to_string(path).ok()?;

        let program = match parse_file(path, &source) {
            Ok(p) => p,
            Err(e) => {
                debug!("Failed to parse {:?}: {}", path, e);
                return None;
            }
        };

        let issues: Vec<_> = self
            .rules
            .par_iter()
            .flat_map(|rule| rule.check(&program, path, &source))
            .collect();

        if issues.is_empty() {
            None
        } else {
            Some(FileResult {
                file: path.to_path_buf(),
                issues,
            })
        }
    }
}
