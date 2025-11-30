use super::cache::FileCache;
use super::parser::parse_file;
use super::registry::RuleRegistry;
use crate::config::{compile_globset, TscannerConfig};
use crate::output::{FileResult, ScanResult};
use crate::utils::{DisableDirectives, FileSource};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub struct Scanner {
    registry: RuleRegistry,
    config: TscannerConfig,
    cache: Arc<FileCache>,
    root: PathBuf,
}

impl Scanner {
    pub fn new(config: TscannerConfig, root: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = RuleRegistry::with_config(&config)?;
        let config_hash = config.compute_hash();
        Ok(Self {
            registry,
            config,
            cache: Arc::new(FileCache::with_config_hash(config_hash)),
            root,
        })
    }

    pub fn with_cache(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        root: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = RuleRegistry::with_config(&config)?;
        Ok(Self {
            registry,
            config,
            cache,
            root,
        })
    }

    pub fn scan(&self, root: &Path, file_filter: Option<&HashSet<PathBuf>>) -> ScanResult {
        self.scan_multi(&[root.to_path_buf()], file_filter)
    }

    pub fn scan_multi(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
    ) -> ScanResult {
        let start = Instant::now();
        crate::utils::log_info(&format!("Starting scan of {:?}", roots));

        let global_include = compile_globset(&self.config.files.include)
            .expect("Failed to compile global include patterns");
        let global_exclude = compile_globset(&self.config.files.exclude)
            .expect("Failed to compile global exclude patterns");

        let mut files: Vec<PathBuf> = Vec::new();

        for root in roots {
            let root_buf = root.to_path_buf();
            let exclude_clone = global_exclude.clone();
            let root_clone = root_buf.clone();
            let include_clone = global_include.clone();
            let exclude_clone2 = global_exclude.clone();

            let root_files: Vec<PathBuf> = WalkBuilder::new(root)
                .hidden(false)
                .git_ignore(true)
                .filter_entry(move |e| {
                    let path = e.path();
                    if path.is_dir() {
                        let relative = path.strip_prefix(&root_clone).unwrap_or(path);
                        return !exclude_clone.is_match(relative);
                    }
                    true
                })
                .build()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let path = e.path();
                    if !path.is_file() {
                        return false;
                    }
                    let relative = path.strip_prefix(&root_buf).unwrap_or(path);
                    include_clone.is_match(relative) && !exclude_clone2.is_match(relative)
                })
                .map(|e| e.path().to_path_buf())
                .collect();

            files.extend(root_files);
        }

        if let Some(filter) = file_filter {
            files.retain(|f| filter.contains(f));
            crate::utils::log_info(&format!(
                "Filtered to {} files (from {})",
                files.len(),
                filter.len()
            ));
        }

        let file_count = files.len();
        crate::utils::log_debug(&format!("Found {} TypeScript files", file_count));

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

                self.analyze_file(path)
            })
            .filter(|r| !r.issues.is_empty())
            .collect();

        let total_issues: usize = results.iter().map(|r| r.issues.len()).sum();
        let duration = start.elapsed();

        self.cache.flush();

        let cached = cache_hits.load(Ordering::Relaxed);
        let scanned = file_count - cached;

        ScanResult {
            files: results,
            total_issues,
            duration_ms: duration.as_millis(),
            total_files: file_count,
            cached_files: cached,
            scanned_files: scanned,
        }
    }

    pub fn scan_single(&self, path: &Path) -> Option<FileResult> {
        self.cache.invalidate(path);
        self.analyze_file(path)
    }

    pub fn scan_content(&self, path: &Path, content: &str) -> Option<FileResult> {
        let directives = DisableDirectives::from_source(content);

        if directives.file_disabled {
            return None;
        }

        let file_source = FileSource::from_path(path);

        let program = match parse_file(path, content) {
            Ok(p) => p,
            Err(e) => {
                crate::utils::log_debug(&format!("Failed to parse {:?}: {}", path, e));
                return None;
            }
        };

        let enabled_rules = self
            .registry
            .get_enabled_rules(path, &self.root, &self.config);
        let source_lines: Vec<&str> = content.lines().collect();

        let issues: Vec<_> = enabled_rules
            .iter()
            .filter(|(rule, _)| !(rule.is_typescript_only() && file_source.is_javascript()))
            .flat_map(|(rule, severity)| {
                let mut rule_issues = rule.check(&program, path, content, file_source);
                for issue in &mut rule_issues {
                    issue.severity = *severity;
                    if issue.line > 0 && issue.line <= source_lines.len() {
                        issue.line_text = Some(source_lines[issue.line - 1].to_string());
                    }
                }
                rule_issues
            })
            .filter(|issue| !directives.is_rule_disabled(issue.line, &issue.rule))
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

    fn analyze_file(&self, path: &Path) -> Option<FileResult> {
        let source = std::fs::read_to_string(path).ok()?;

        let directives = DisableDirectives::from_source(&source);

        if directives.file_disabled {
            self.cache.insert(path.to_path_buf(), Vec::new());
            return None;
        }

        let file_source = FileSource::from_path(path);

        let program = match parse_file(path, &source) {
            Ok(p) => p,
            Err(e) => {
                crate::utils::log_debug(&format!("Failed to parse {:?}: {}", path, e));
                return None;
            }
        };

        let enabled_rules = self
            .registry
            .get_enabled_rules(path, &self.root, &self.config);
        let source_lines: Vec<&str> = source.lines().collect();

        let issues: Vec<_> = enabled_rules
            .iter()
            .filter(|(rule, _)| !(rule.is_typescript_only() && file_source.is_javascript()))
            .flat_map(|(rule, severity)| {
                let mut rule_issues = rule.check(&program, path, &source, file_source);
                for issue in &mut rule_issues {
                    issue.severity = *severity;
                    if issue.line > 0 && issue.line <= source_lines.len() {
                        issue.line_text = Some(source_lines[issue.line - 1].to_string());
                    }
                }
                rule_issues
            })
            .filter(|issue| !directives.is_rule_disabled(issue.line, &issue.rule))
            .collect();

        self.cache.insert(path.to_path_buf(), issues.clone());

        if issues.is_empty() {
            None
        } else {
            Some(FileResult {
                file: path.to_path_buf(),
                issues,
            })
        }
    }

    pub fn cache(&self) -> Arc<FileCache> {
        self.cache.clone()
    }
}
