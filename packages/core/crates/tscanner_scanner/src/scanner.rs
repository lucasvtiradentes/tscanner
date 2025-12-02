use crate::ai_executor::AiExecutor;
use crate::builtin_executor::{BuiltinExecutor, ExecuteResult};
use crate::config_ext::ConfigExt;
use crate::script_executor::ScriptExecutor;
use globset::GlobSet;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tscanner_cache::FileCache;
use tscanner_config::{
    compile_globset, AiRuleConfig, CustomRuleConfig, ScriptMode, ScriptRuleConfig, TscannerConfig,
};
use tscanner_diagnostics::{ContentScanResult, FileResult, Issue, ScanResult};
use tscanner_rules::RuleRegistry;

pub struct Scanner {
    registry: RuleRegistry,
    config: TscannerConfig,
    cache: Arc<FileCache>,
    root: PathBuf,
    global_include: GlobSet,
    global_exclude: GlobSet,
    script_executor: ScriptExecutor,
    ai_executor: AiExecutor,
    log_info: fn(&str),
    log_debug: fn(&str),
}

impl Scanner {
    pub fn new(config: TscannerConfig, root: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_logger(config, root, |_| {}, |_| {}, |_| {}, |_| {})
    }

    pub fn with_cache(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        root: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_cache_and_logger(config, cache, root, |_| {}, |_| {}, |_| {}, |_| {})
    }

    pub fn with_logger(
        config: TscannerConfig,
        root: PathBuf,
        log_info: fn(&str),
        log_debug: fn(&str),
        log_error: fn(&str),
        log_warn: fn(&str),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config_hash = config.compute_hash();
        let cache = Arc::new(FileCache::with_config_hash(config_hash));
        Self::with_cache_and_logger(
            config, cache, root, log_info, log_debug, log_error, log_warn,
        )
    }

    pub fn with_cache_and_logger(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        root: PathBuf,
        log_info: fn(&str),
        log_debug: fn(&str),
        log_error: fn(&str),
        log_warn: fn(&str),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = RuleRegistry::with_config(
            &config,
            ConfigExt::compile_builtin_rule,
            ConfigExt::compile_custom_rule,
            log_info,
            log_error,
        )?;
        let global_include = compile_globset(&config.files.include)?;
        let global_exclude = compile_globset(&config.files.exclude)?;
        let script_executor = ScriptExecutor::with_logger(&root, log_error, log_debug);
        let ai_executor = AiExecutor::with_logger(&root, log_warn, log_debug);
        Ok(Self {
            registry,
            config,
            cache,
            root,
            global_include,
            global_exclude,
            script_executor,
            ai_executor,
            log_info,
            log_debug,
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
        (self.log_info)(&format!("Starting scan of {:?}", roots));

        let files = self.collect_files(roots, file_filter);
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

    fn collect_files(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
    ) -> Vec<PathBuf> {
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
            (self.log_info)(&format!(
                "Filtered to {} files (from {})",
                files.len(),
                filter.len()
            ));
        }

        files
    }

    fn run_builtin_executor(&self, path: &Path) -> Option<FileResult> {
        let source = std::fs::read_to_string(path).ok()?;
        let executor =
            BuiltinExecutor::with_logger(&self.registry, &self.config, &self.root, self.log_debug);

        match executor.execute(path, &source) {
            ExecuteResult::Skip => None,
            ExecuteResult::ParseError => None,
            ExecuteResult::Disabled | ExecuteResult::Empty => {
                self.cache.insert(path.to_path_buf(), Vec::new());
                None
            }
            ExecuteResult::Ok(file_result) => {
                self.cache
                    .insert(path.to_path_buf(), file_result.issues.clone());
                Some(file_result)
            }
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

        let all_files = self.collect_script_files(&script_rules);
        if all_files.is_empty() {
            return vec![];
        }

        self.script_executor
            .execute_rules(&script_rules, &all_files, &self.root)
    }

    fn collect_script_files(
        &self,
        script_rules: &[(String, ScriptRuleConfig)],
    ) -> Vec<(PathBuf, String)> {
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

        WalkBuilder::new(&self.root)
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
            .collect()
    }

    fn run_ai_rules(&self, _files: &[PathBuf]) -> Vec<Issue> {
        let ai_rules: Vec<(String, AiRuleConfig)> = self
            .config
            .custom_rules
            .iter()
            .filter_map(|(name, config)| {
                if let CustomRuleConfig::Ai(ai_config) = config {
                    if ai_config.base.enabled {
                        return Some((name.clone(), ai_config.clone()));
                    }
                }
                None
            })
            .collect();

        if ai_rules.is_empty() {
            return vec![];
        }

        self.ai_executor.execute_rules(&ai_rules, &[], &self.root)
    }

    fn merge_issues(&self, results: &mut Vec<FileResult>, issues: Vec<Issue>) {
        if issues.is_empty() {
            return;
        }

        let mut issues_by_file: HashMap<PathBuf, Vec<Issue>> = HashMap::new();
        for issue in issues {
            issues_by_file
                .entry(issue.file.clone())
                .or_default()
                .push(issue);
        }

        for (file, file_issues) in issues_by_file {
            if let Some(file_result) = results.iter_mut().find(|r| r.file == file) {
                file_result.issues.extend(file_issues);
            } else {
                results.push(FileResult {
                    file,
                    issues: file_issues,
                });
            }
        }
    }

    pub fn scan_single(&self, path: &Path) -> Option<FileResult> {
        self.cache.invalidate(path);
        self.run_builtin_executor(path)
    }

    pub fn scan_content(&self, path: &Path, content: &str) -> Option<ContentScanResult> {
        let executor =
            BuiltinExecutor::with_logger(&self.registry, &self.config, &self.root, self.log_debug);
        let builtin_result = executor.execute(path, content);
        let (script_issues, related_files) = self.run_script_rules_for_content(path, content);

        let mut all_issues = match builtin_result {
            ExecuteResult::Ok(r) => r.issues,
            _ => Vec::new(),
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

    fn run_script_rules_for_content(
        &self,
        path: &Path,
        content: &str,
    ) -> (Vec<Issue>, Vec<PathBuf>) {
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
                if let Ok(file_content) = std::fs::read_to_string(entry_path) {
                    files.insert(entry_path.to_path_buf(), file_content);
                }
            }
        }

        files
    }

    pub fn cache(&self) -> Arc<FileCache> {
        self.cache.clone()
    }

    pub fn clear_script_cache(&self) {
        self.script_executor.clear_cache();
    }
}
