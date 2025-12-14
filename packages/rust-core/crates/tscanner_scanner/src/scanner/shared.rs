use super::Scanner;
use crate::executors::{AiExecutionResult, AiProgressCallback, BuiltinExecutor, ExecuteResult};
use ignore::WalkBuilder;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tscanner_config::{compile_globset, AiRuleConfig, ScriptRuleConfig};
use tscanner_types::{FileResult, Issue};

impl Scanner {
    pub(crate) fn run_builtin_executor(&self, path: &Path) -> Option<FileResult> {
        let source = std::fs::read_to_string(path).ok()?;
        let executor =
            BuiltinExecutor::with_logger(&self.registry, &self.config, &self.root, self.log_debug);

        match executor.execute(path, &source) {
            ExecuteResult::Skip | ExecuteResult::Disabled | ExecuteResult::Empty => {
                self.cache.insert(path.to_path_buf(), Vec::new());
                None
            }
            ExecuteResult::ParseError => None,
            ExecuteResult::Ok(file_result) => {
                self.cache
                    .insert(path.to_path_buf(), file_result.issues.clone());
                Some(file_result)
            }
        }
    }

    pub(crate) fn run_builtin_executor_no_cache(
        &self,
        path: &Path,
        content: &str,
    ) -> Option<FileResult> {
        let executor =
            BuiltinExecutor::with_logger(&self.registry, &self.config, &self.root, self.log_debug);

        match executor.execute(path, content) {
            ExecuteResult::Skip => None,
            ExecuteResult::ParseError => None,
            ExecuteResult::Disabled | ExecuteResult::Empty => None,
            ExecuteResult::Ok(file_result) => Some(file_result),
        }
    }

    pub(crate) fn collect_script_rules(&self) -> Vec<(String, ScriptRuleConfig)> {
        self.config
            .rules
            .script
            .iter()
            .map(|(name, script_config)| (name.clone(), script_config.clone()))
            .collect()
    }

    pub(crate) fn collect_ai_rules(&self) -> Vec<(String, AiRuleConfig)> {
        let mut rules: Vec<_> = self
            .config
            .ai_rules
            .iter()
            .map(|(name, ai_config)| (name.clone(), ai_config.clone()))
            .collect();
        rules.sort_by(|(a, _), (b, _)| a.cmp(b));
        rules
    }

    pub(crate) fn run_script_rules(&self, _files: &[PathBuf]) -> (Vec<Issue>, Vec<String>) {
        let script_rules = self.collect_script_rules();
        if script_rules.is_empty() {
            return (vec![], vec![]);
        }

        let all_files = self.collect_script_files(&script_rules);
        if all_files.is_empty() {
            return (vec![], vec![]);
        }

        (self.log_debug)(&format!(
            "Running {} script rules on {} files (requested {} files but collecting all matching files)",
            script_rules.len(),
            all_files.len(),
            _files.len()
        ));

        let (issues, warnings) =
            self.script_executor
                .execute_rules(&script_rules, &all_files, &self.root);

        (self.log_debug)(&format!(
            "Script rules found {} total issues across all files",
            issues.len()
        ));

        (issues, warnings)
    }

    pub(crate) fn collect_script_files(
        &self,
        script_rules: &[(String, ScriptRuleConfig)],
    ) -> Vec<(PathBuf, String)> {
        let include_patterns: HashSet<&str> = script_rules
            .iter()
            .flat_map(|(_, cfg)| cfg.include.iter().map(|s| s.as_str()))
            .collect();
        let exclude_patterns: HashSet<&str> = script_rules
            .iter()
            .flat_map(|(_, cfg)| cfg.exclude.iter().map(|s| s.as_str()))
            .collect();
        self.collect_files_by_patterns(&include_patterns, &exclude_patterns, None)
    }

    pub(crate) fn run_ai_rules_with_context(
        &self,
        file_filter: &[PathBuf],
        changed_lines: Option<&HashMap<PathBuf, HashSet<usize>>>,
    ) -> AiExecutionResult {
        self.run_ai_rules_with_context_and_progress(file_filter, changed_lines, None)
    }

    pub(crate) fn run_ai_rules_with_context_and_progress(
        &self,
        file_filter: &[PathBuf],
        changed_lines: Option<&HashMap<PathBuf, HashSet<usize>>>,
        progress_callback: Option<AiProgressCallback>,
    ) -> AiExecutionResult {
        let ai_rules = self.collect_ai_rules();
        if ai_rules.is_empty() {
            return AiExecutionResult::default();
        }

        let all_files = if file_filter.is_empty() {
            self.collect_ai_files(&ai_rules)
        } else {
            self.collect_ai_files_from_filter(&ai_rules, file_filter)
        };

        if all_files.is_empty() {
            return AiExecutionResult::default();
        }

        self.ai_executor.execute_rules_with_progress(
            &ai_rules,
            &all_files,
            &self.root,
            changed_lines,
            progress_callback,
        )
    }

    pub(crate) fn collect_ai_files(
        &self,
        ai_rules: &[(String, AiRuleConfig)],
    ) -> Vec<(PathBuf, String)> {
        let include_patterns: HashSet<&str> = ai_rules
            .iter()
            .flat_map(|(_, cfg)| cfg.include.iter().map(|s| s.as_str()))
            .collect();
        let exclude_patterns: HashSet<&str> = ai_rules
            .iter()
            .flat_map(|(_, cfg)| cfg.exclude.iter().map(|s| s.as_str()))
            .collect();
        self.collect_files_by_patterns(&include_patterns, &exclude_patterns, None)
    }

    pub(crate) fn collect_ai_files_from_filter(
        &self,
        ai_rules: &[(String, AiRuleConfig)],
        file_filter: &[PathBuf],
    ) -> Vec<(PathBuf, String)> {
        let include_patterns: HashSet<&str> = ai_rules
            .iter()
            .flat_map(|(_, cfg)| cfg.include.iter().map(|s| s.as_str()))
            .collect();
        let exclude_patterns: HashSet<&str> = ai_rules
            .iter()
            .flat_map(|(_, cfg)| cfg.exclude.iter().map(|s| s.as_str()))
            .collect();
        self.collect_files_by_patterns(&include_patterns, &exclude_patterns, Some(file_filter))
    }

    fn collect_files_by_patterns(
        &self,
        include_patterns: &HashSet<&str>,
        exclude_patterns: &HashSet<&str>,
        file_filter: Option<&[PathBuf]>,
    ) -> Vec<(PathBuf, String)> {
        if include_patterns.is_empty() {
            return vec![];
        }

        let matches_patterns = |path: &Path| -> bool {
            if !path.is_file() {
                return false;
            }
            let relative = path.strip_prefix(&self.root).unwrap_or(path);
            let relative_str = relative.to_string_lossy();
            let matches_include = include_patterns
                .iter()
                .any(|pattern| glob_match::glob_match(pattern, &relative_str));
            let matches_exclude = exclude_patterns
                .iter()
                .any(|pattern| glob_match::glob_match(pattern, &relative_str));
            matches_include && !matches_exclude
        };

        match file_filter {
            Some(files) => files
                .iter()
                .filter(|p| matches_patterns(p))
                .filter_map(|path| {
                    std::fs::read_to_string(path)
                        .ok()
                        .map(|content| (path.clone(), content))
                })
                .collect(),
            None => WalkBuilder::new(&self.root)
                .hidden(false)
                .git_ignore(true)
                .build()
                .flatten()
                .filter(|entry| matches_patterns(entry.path()))
                .filter_map(|entry| {
                    let path = entry.path().to_path_buf();
                    std::fs::read_to_string(&path)
                        .ok()
                        .map(|content| (path, content))
                })
                .collect(),
        }
    }

    pub(crate) fn merge_issues(&self, results: &mut Vec<FileResult>, issues: Vec<Issue>) {
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

    pub(crate) fn collect_files_with_filter(
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
            let custom_include_clone = self.custom_include.clone();
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
                    if exclude_clone2.is_match(relative) {
                        return false;
                    }
                    let matches_global = include_clone.is_match(relative);
                    let matches_custom = custom_include_clone
                        .as_ref()
                        .map(|g| g.is_match(relative))
                        .unwrap_or(false);
                    matches_global || matches_custom
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

    pub(crate) fn run_script_rules_for_content(
        &self,
        path: &Path,
        content: &str,
    ) -> (Vec<Issue>, Vec<PathBuf>) {
        let script_rules = self.collect_script_rules();
        if script_rules.is_empty() {
            return (vec![], vec![]);
        }

        let has_multi_file_rules = script_rules.iter().any(|(_, cfg)| cfg.include.len() > 1);

        let all_related_files_cache: HashMap<PathBuf, String> = if has_multi_file_rules {
            self.collect_all_related_files(path, content, &script_rules)
        } else {
            HashMap::new()
        };

        let mut all_issues = Vec::new();
        let mut all_related_files = HashSet::new();

        for (rule_name, script_config) in &script_rules {
            let rule_include = compile_globset(&script_config.include).ok();
            let rule_exclude = compile_globset(&script_config.exclude).ok();

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

            let is_multi_file = script_config.include.len() > 1;

            let files: Vec<(PathBuf, String)> = if is_multi_file {
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

            if is_multi_file {
                for (file_path, _) in &files {
                    if file_path != path {
                        all_related_files.insert(file_path.clone());
                    }
                }
            }

            let (issues, _warnings) = self.script_executor.execute_rules(
                &[(rule_name.clone(), script_config.clone())],
                &files,
                &self.root,
            );

            for issue in issues {
                if issue.file == path {
                    all_issues.push(issue);
                }
            }
        }

        (all_issues, all_related_files.into_iter().collect())
    }

    pub(crate) fn collect_all_related_files(
        &self,
        changed_path: &Path,
        changed_content: &str,
        rules: &[(String, ScriptRuleConfig)],
    ) -> HashMap<PathBuf, String> {
        let all_patterns: HashSet<&str> = rules
            .iter()
            .filter(|(_, cfg)| cfg.include.len() > 1)
            .flat_map(|(_, cfg)| cfg.include.iter().map(|s| s.as_str()))
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
}
