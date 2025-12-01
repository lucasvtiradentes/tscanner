use super::cache::FileCache;
use super::parser::parse_file;
use super::registry::RuleRegistry;
use super::script_executor::ScriptExecutor;
use crate::config::{compile_globset, CustomRuleConfig, ScriptRuleConfig, TscannerConfig};
use crate::output::{ContentScanResult, FileResult, Issue, ScanResult};
use crate::utils::{DisableDirectives, FileSource};
use globset::GlobSet;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

const JS_TS_EXTENSIONS: &[&str] = &["ts", "tsx", "js", "jsx", "mjs", "cjs", "mts", "cts"];

fn is_js_ts_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| JS_TS_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
}

pub struct Scanner {
    registry: RuleRegistry,
    config: TscannerConfig,
    cache: Arc<FileCache>,
    root: PathBuf,
    global_include: GlobSet,
    global_exclude: GlobSet,
    script_executor: ScriptExecutor,
}

impl Scanner {
    pub fn new(config: TscannerConfig, root: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = RuleRegistry::with_config(&config)?;
        let config_hash = config.compute_hash();
        let global_include = compile_globset(&config.files.include)?;
        let global_exclude = compile_globset(&config.files.exclude)?;
        let script_executor = ScriptExecutor::new(&root);
        Ok(Self {
            registry,
            config,
            cache: Arc::new(FileCache::with_config_hash(config_hash)),
            root,
            global_include,
            global_exclude,
            script_executor,
        })
    }

    pub fn with_cache(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        root: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = RuleRegistry::with_config(&config)?;
        let global_include = compile_globset(&config.files.include)?;
        let global_exclude = compile_globset(&config.files.exclude)?;
        let script_executor = ScriptExecutor::new(&root);
        Ok(Self {
            registry,
            config,
            cache,
            root,
            global_include,
            global_exclude,
            script_executor,
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

        let mut files: Vec<PathBuf> = Vec::new();

        for root in roots {
            let root_buf = root.to_path_buf();
            let exclude_clone = self.global_exclude.clone();
            let root_clone = root_buf.clone();
            let include_clone = self.global_include.clone();
            let exclude_clone2 = self.global_exclude.clone();

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
        crate::utils::log_debug(&format!("Found {} files to scan", file_count));

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

        let script_issues = self.run_script_rules(&files);

        let mut all_results = results;
        self.merge_script_issues(&mut all_results, script_issues);

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

    fn run_script_rules(&self, _files: &[PathBuf]) -> Vec<Issue> {
        let script_rules: Vec<(String, ScriptRuleConfig)> = self
            .config
            .custom_rules
            .iter()
            .filter_map(|(name, config)| {
                if let CustomRuleConfig::Script(script_config) = config {
                    if script_config.base.enabled {
                        return Some((name.clone(), script_config.clone()));
                    }
                }
                None
            })
            .collect();

        if script_rules.is_empty() {
            return vec![];
        }

        let needed_patterns: HashSet<&str> = script_rules
            .iter()
            .flat_map(|(_, cfg)| cfg.base.include.iter().map(|s| s.as_str()))
            .collect();

        if needed_patterns.is_empty() {
            return vec![];
        }

        let exclude_patterns: HashSet<&str> = script_rules
            .iter()
            .flat_map(|(_, cfg)| cfg.base.exclude.iter().map(|s| s.as_str()))
            .collect();

        let all_files: Vec<(PathBuf, String)> = WalkBuilder::new(&self.root)
            .hidden(false)
            .git_ignore(true)
            .build()
            .flatten()
            .filter(|entry| {
                let path = entry.path();
                if !path.is_file() {
                    return false;
                }
                let relative = path.strip_prefix(&self.root).unwrap_or(path);
                let relative_str = relative.to_string_lossy();
                let matches_include = needed_patterns
                    .iter()
                    .any(|pattern| glob_match::glob_match(pattern, &relative_str));
                let matches_exclude = exclude_patterns
                    .iter()
                    .any(|pattern| glob_match::glob_match(pattern, &relative_str));
                matches_include && !matches_exclude
            })
            .filter_map(|entry| {
                let path = entry.path().to_path_buf();
                std::fs::read_to_string(&path)
                    .ok()
                    .map(|content| (path, content))
            })
            .collect();

        if all_files.is_empty() {
            return vec![];
        }

        self.script_executor
            .execute_rules(&script_rules, &all_files, &self.root)
    }

    fn merge_script_issues(&self, results: &mut Vec<FileResult>, script_issues: Vec<Issue>) {
        if script_issues.is_empty() {
            return;
        }

        let mut issues_by_file: HashMap<PathBuf, Vec<Issue>> = HashMap::new();
        for issue in script_issues {
            issues_by_file
                .entry(issue.file.clone())
                .or_default()
                .push(issue);
        }

        for (file, issues) in issues_by_file {
            if let Some(file_result) = results.iter_mut().find(|r| r.file == file) {
                file_result.issues.extend(issues);
            } else {
                results.push(FileResult { file, issues });
            }
        }
    }

    pub fn scan_single(&self, path: &Path) -> Option<FileResult> {
        self.cache.invalidate(path);
        self.analyze_file(path)
    }

    pub fn scan_content(&self, path: &Path, content: &str) -> Option<ContentScanResult> {
        let builtin_result = self.process_source(path, content, false);
        let (script_issues, related_files) = self.run_script_rules_for_content(path, content);

        let mut all_issues = builtin_result.map(|r| r.issues).unwrap_or_default();
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

    fn run_script_rules_for_content(
        &self,
        path: &Path,
        content: &str,
    ) -> (Vec<Issue>, Vec<PathBuf>) {
        use crate::config::ScriptMode;

        let script_rules: Vec<(String, ScriptRuleConfig)> = self
            .config
            .custom_rules
            .iter()
            .filter_map(|(name, config)| {
                if let CustomRuleConfig::Script(script_config) = config {
                    if script_config.base.enabled {
                        return Some((name.clone(), script_config.clone()));
                    }
                }
                None
            })
            .collect();

        if script_rules.is_empty() {
            return (vec![], vec![]);
        }

        let has_batch_rules = script_rules
            .iter()
            .any(|(_, cfg)| cfg.mode == ScriptMode::Batch && cfg.base.include.len() > 1);

        let all_related_files_cache: HashMap<PathBuf, String> = if has_batch_rules {
            self.collect_all_related_files(path, content, &script_rules)
        } else {
            HashMap::new()
        };

        let mut all_issues = Vec::new();
        let mut all_related_files = HashSet::new();

        for (rule_name, script_config) in &script_rules {
            let rule_include = compile_globset(&script_config.base.include).ok();
            let rule_exclude = compile_globset(&script_config.base.exclude).ok();

            let relative_path = path.strip_prefix(&self.root).unwrap_or(path);
            let matches_include = rule_include
                .as_ref()
                .map(|g| g.is_match(relative_path))
                .unwrap_or(true);
            let matches_exclude = rule_exclude
                .as_ref()
                .map(|g| g.is_match(relative_path))
                .unwrap_or(false);

            if !matches_include || matches_exclude {
                continue;
            }

            let is_batch_with_multiple =
                script_config.mode == ScriptMode::Batch && script_config.base.include.len() > 1;

            let files: Vec<(PathBuf, String)> = if is_batch_with_multiple {
                all_related_files_cache
                    .iter()
                    .filter(|(p, _)| {
                        let rel = p.strip_prefix(&self.root).unwrap_or(p);
                        let matches_inc = rule_include
                            .as_ref()
                            .map(|g| g.is_match(rel))
                            .unwrap_or(true);
                        let matches_exc = rule_exclude
                            .as_ref()
                            .map(|g| g.is_match(rel))
                            .unwrap_or(false);
                        matches_inc && !matches_exc
                    })
                    .map(|(p, c)| (p.clone(), c.clone()))
                    .collect()
            } else {
                vec![(path.to_path_buf(), content.to_string())]
            };

            if is_batch_with_multiple {
                for (file_path, _) in &files {
                    if file_path != path {
                        all_related_files.insert(file_path.clone());
                    }
                }
            }

            let issues = self.script_executor.execute_rules(
                &[(rule_name.clone(), script_config.clone())],
                &files,
                &self.root,
            );
            all_issues.extend(issues);
        }

        (all_issues, all_related_files.into_iter().collect())
    }

    fn collect_all_related_files(
        &self,
        changed_path: &Path,
        changed_content: &str,
        rules: &[(String, ScriptRuleConfig)],
    ) -> HashMap<PathBuf, String> {
        use crate::config::ScriptMode;

        let all_patterns: HashSet<&str> = rules
            .iter()
            .filter(|(_, cfg)| cfg.mode == ScriptMode::Batch && cfg.base.include.len() > 1)
            .flat_map(|(_, cfg)| cfg.base.include.iter().map(|s| s.as_str()))
            .collect();

        if all_patterns.is_empty() {
            return HashMap::new();
        }

        let mut files = HashMap::new();
        files.insert(changed_path.to_path_buf(), changed_content.to_string());

        for entry in WalkBuilder::new(&self.root)
            .hidden(false)
            .git_ignore(true)
            .build()
            .flatten()
        {
            let entry_path = entry.path();
            if !entry_path.is_file() || entry_path == changed_path {
                continue;
            }

            let relative = entry_path.strip_prefix(&self.root).unwrap_or(entry_path);
            let relative_str = relative.to_string_lossy();

            let matches = all_patterns
                .iter()
                .any(|pattern| glob_match::glob_match(pattern, &relative_str));

            if matches {
                if let Ok(content) = std::fs::read_to_string(entry_path) {
                    files.insert(entry_path.to_path_buf(), content);
                }
            }
        }

        files
    }

    fn analyze_file(&self, path: &Path) -> Option<FileResult> {
        if !is_js_ts_file(path) {
            return None;
        }
        let source = std::fs::read_to_string(path).ok()?;
        self.process_source(path, &source, true)
    }

    fn process_source(&self, path: &Path, source: &str, use_cache: bool) -> Option<FileResult> {
        if !is_js_ts_file(path) {
            return None;
        }

        let directives = DisableDirectives::from_source(source);

        if directives.file_disabled {
            if use_cache {
                self.cache.insert(path.to_path_buf(), Vec::new());
            }
            return None;
        }

        let file_source = FileSource::from_path(path);

        let program = match parse_file(path, source) {
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
                let mut rule_issues = rule.check(&program, path, source, file_source);
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

        if use_cache {
            self.cache.insert(path.to_path_buf(), issues.clone());
        }

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

    pub fn clear_script_cache(&self) {
        self.script_executor.clear_cache();
    }
}
