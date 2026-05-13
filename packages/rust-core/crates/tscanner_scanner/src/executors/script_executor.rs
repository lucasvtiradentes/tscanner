mod batch;
mod process;
mod types;

use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tscanner_cache::ScriptCache;
use tscanner_config::ScriptRuleConfig;
use tscanner_constants::config_dir_name;
use tscanner_types::Issue;

pub use types::{ScriptError, ScriptFile, ScriptInput, ScriptOutput};

pub struct ScriptExecutor {
    cache: Arc<ScriptCache>,
    config_dir: PathBuf,
    log_error: fn(&str),
    log_debug: fn(&str),
}

impl ScriptExecutor {
    pub fn new(
        workspace_root: &Path,
        config_dir: Option<PathBuf>,
        cache: Arc<ScriptCache>,
        log_error: Option<fn(&str)>,
        log_debug: Option<fn(&str)>,
    ) -> Self {
        Self {
            cache,
            config_dir: config_dir.unwrap_or_else(|| workspace_root.join(config_dir_name())),
            log_error: log_error.unwrap_or(|_| {}),
            log_debug: log_debug.unwrap_or(|_| {}),
        }
    }

    pub fn with_config_dir(config_dir: PathBuf) -> Self {
        Self::new(
            Path::new("."),
            Some(config_dir),
            Arc::new(ScriptCache::new()),
            None,
            None,
        )
    }

    pub fn with_logger(
        workspace_root: &Path,
        cache: Arc<ScriptCache>,
        log_error: fn(&str),
        log_debug: fn(&str),
    ) -> Self {
        Self::new(
            workspace_root,
            None,
            cache,
            Some(log_error),
            Some(log_debug),
        )
    }

    pub fn with_config_dir_and_logger(
        config_dir: PathBuf,
        cache: Arc<ScriptCache>,
        log_error: fn(&str),
        log_debug: fn(&str),
    ) -> Self {
        Self::new(
            Path::new("."),
            Some(config_dir),
            cache,
            Some(log_error),
            Some(log_debug),
        )
    }

    pub fn execute_rules(
        &self,
        rules: &[(String, ScriptRuleConfig)],
        all_files: &[(PathBuf, String)],
        workspace_root: &Path,
    ) -> (Vec<Issue>, Vec<String>) {
        let results: Vec<(Vec<Issue>, Option<String>)> = rules
            .par_iter()
            .map(|(rule_name, rule_config)| {
                let matching_files: Vec<_> = all_files
                    .iter()
                    .filter(|(path, _)| self.file_matches_rule(path, workspace_root, rule_config))
                    .collect();

                if matching_files.is_empty() {
                    return (vec![], None);
                }

                (self.log_debug)(&format!(
                    "Script rule '{}' starting: {} matching files",
                    rule_name,
                    matching_files.len()
                ));
                let start = Instant::now();

                match self.execute_rule(rule_name, rule_config, &matching_files, workspace_root) {
                    Ok(issues) => {
                        (self.log_debug)(&format!(
                            "Script rule '{}' finished in {}ms with {} issues",
                            rule_name,
                            start.elapsed().as_millis(),
                            issues.len()
                        ));
                        (issues, None)
                    }
                    Err(e) => {
                        let warning = format!("Script rule '{}' failed: {}", rule_name, e);
                        (self.log_error)(&warning);
                        (vec![], Some(warning))
                    }
                }
            })
            .collect();

        let issues: Vec<Issue> = results.iter().flat_map(|(i, _)| i.clone()).collect();
        let warnings: Vec<String> = results.iter().filter_map(|(_, w)| w.clone()).collect();

        (issues, warnings)
    }

    fn file_matches_rule(
        &self,
        path: &Path,
        workspace_root: &Path,
        rule_config: &ScriptRuleConfig,
    ) -> bool {
        super::utils::file_matches_patterns(
            path,
            workspace_root,
            &rule_config.include,
            &rule_config.exclude,
        )
    }

    fn execute_rule(
        &self,
        rule_name: &str,
        rule_config: &ScriptRuleConfig,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
    ) -> Result<Vec<Issue>, ScriptError> {
        let script_path = self.extract_script_path(&rule_config.command);

        let files_owned: Vec<(PathBuf, String)> =
            files.iter().map(|(p, c)| (p.clone(), c.clone())).collect();

        if let Some(cached) = self.cache.get(rule_name, &script_path, &files_owned) {
            return Ok(cached);
        }

        let issues = self.execute_batch(rule_name, rule_config, files, workspace_root)?;

        self.cache
            .insert(rule_name, &script_path, &files_owned, issues.clone());

        Ok(issues)
    }

    fn extract_script_path(&self, command: &str) -> PathBuf {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if let Some(last) = parts.last() {
            self.config_dir.join(last)
        } else {
            self.config_dir.join(command)
        }
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    pub fn flush_cache(&self) {
        self.cache.flush();
    }
}

impl Default for ScriptExecutor {
    fn default() -> Self {
        Self::new(
            Path::new("."),
            None,
            Arc::new(ScriptCache::new()),
            None,
            None,
        )
    }
}
