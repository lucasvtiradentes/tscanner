mod files;
mod script_content;

use super::Scanner;
use crate::executors::{
    AiExecutionResult, AiProgressCallback, BuiltinExecutor, ExecuteResult, PreviousAiIssue,
};
use ignore::WalkBuilder;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tscanner_config::{AiRuleClassification, ResolvedAiRuleConfig, ScriptRuleConfig};
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

    pub(crate) fn collect_ai_rules(&self) -> Vec<(String, ResolvedAiRuleConfig)> {
        let mut rules: Vec<_> = self
            .config
            .resolved_ai_rules
            .iter()
            .filter(|rule| rule.classification == AiRuleClassification::CodeCheckable)
            .map(|ai_config| (ai_config.id.clone(), ai_config.clone()))
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
        self.run_ai_rules_with_context_and_progress(file_filter, changed_lines, None, &[])
    }

    pub(crate) fn run_ai_rules_with_context_and_progress(
        &self,
        file_filter: &[PathBuf],
        changed_lines: Option<&HashMap<PathBuf, HashSet<usize>>>,
        progress_callback: Option<AiProgressCallback>,
        previous_issues: &[PreviousAiIssue],
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
            previous_issues,
        )
    }

    pub(crate) fn collect_ai_files(
        &self,
        ai_rules: &[(String, ResolvedAiRuleConfig)],
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
        ai_rules: &[(String, ResolvedAiRuleConfig)],
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
}
