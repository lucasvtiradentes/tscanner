use crate::scanner::Scanner;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tscanner_config::{compile_globset, ScriptRuleConfig};
use tscanner_types::Issue;

impl Scanner {
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

        for entry in ignore::WalkBuilder::new(&self.root)
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
