use super::types::AiError;
use super::{AiExecutor, ChangedLinesMap};
use crate::executors::utils;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use tscanner_config::{AiConfig, AiRuleConfig};
use tscanner_types::Issue;

impl AiExecutor {
    pub(super) fn file_matches_rule(
        &self,
        path: &Path,
        workspace_root: &Path,
        rule_config: &AiRuleConfig,
    ) -> bool {
        utils::file_matches_patterns(
            path,
            workspace_root,
            &rule_config.include,
            &rule_config.exclude,
        )
    }

    pub(super) fn execute_rule(
        &self,
        rule_name: &str,
        rule_config: &AiRuleConfig,
        files: &[&(PathBuf, String)],
        workspace_root: &Path,
        ai_config: &AiConfig,
        changed_lines: Option<&ChangedLinesMap>,
    ) -> (Result<Vec<Issue>, AiError>, bool) {
        let prompt_path = self.ai_rules_dir.join(&rule_config.prompt);
        if !prompt_path.exists() {
            return (Err(AiError::PromptNotFound(prompt_path)), false);
        }

        let files_owned: Vec<(PathBuf, String)> =
            files.iter().map(|(p, c)| (p.clone(), c.clone())).collect();

        if let Some(in_flight_flag) = self.in_flight.get(rule_name) {
            in_flight_flag.store(true, Ordering::SeqCst);
            std::thread::sleep(Duration::from_millis(100));
        }

        if let Some(cached_issues) = self.cache.get(rule_name, &prompt_path, &files_owned) {
            (self.log_warn)(&format!(
                "AI rule '{}' cache hit ({} cached issues)",
                rule_name,
                cached_issues.len()
            ));
            return (
                Ok(self.validate_cached_issues(&cached_issues, files, workspace_root)),
                true,
            );
        }

        let prompt_content = match std::fs::read_to_string(&prompt_path) {
            Ok(content) => content,
            Err(e) => return (Err(AiError::IoError(e)), false),
        };

        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        self.in_flight
            .insert(rule_name.to_string(), cancelled.clone());

        let result = self.call_ai_and_parse(
            rule_name,
            rule_config,
            &prompt_content,
            files,
            workspace_root,
            ai_config,
            changed_lines,
            &cancelled,
        );

        self.in_flight.remove(rule_name);

        if cancelled.load(Ordering::SeqCst) {
            (self.log_debug)(&format!("AI rule '{}' was cancelled", rule_name));
            return (Ok(vec![]), false);
        }

        if let Ok(ref issues) = result {
            self.cache
                .insert(rule_name, &prompt_path, &files_owned, issues.clone());
        }

        (result, false)
    }
}
