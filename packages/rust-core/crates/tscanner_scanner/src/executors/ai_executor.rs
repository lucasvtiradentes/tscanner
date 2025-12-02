use std::path::{Path, PathBuf};
use tscanner_config::AiRuleConfig;
use tscanner_diagnostics::Issue;

pub struct AiExecutor {
    prompts_dir: PathBuf,
    log_warn: fn(&str),
    log_debug: fn(&str),
}

impl AiExecutor {
    pub fn new(workspace_root: &Path) -> Self {
        Self {
            prompts_dir: workspace_root.join(".tscanner").join("prompts"),
            log_warn: |_| {},
            log_debug: |_| {},
        }
    }

    pub fn with_logger(workspace_root: &Path, log_warn: fn(&str), log_debug: fn(&str)) -> Self {
        Self {
            prompts_dir: workspace_root.join(".tscanner").join("prompts"),
            log_warn,
            log_debug,
        }
    }

    pub fn execute_rules(
        &self,
        rules: &[(String, AiRuleConfig)],
        _files: &[(PathBuf, String)],
        _workspace_root: &Path,
    ) -> Vec<Issue> {
        if rules.is_empty() {
            return vec![];
        }

        for (rule_name, rule_config) in rules {
            let prompt_path = self.prompts_dir.join(&rule_config.prompt);
            if !prompt_path.exists() {
                (self.log_warn)(&format!(
                    "AI rule '{}' prompt file not found: {:?}",
                    rule_name, prompt_path
                ));
                continue;
            }

            (self.log_debug)(&format!(
                "AI rule '{}' would use prompt: {:?}",
                rule_name, prompt_path
            ));
        }

        vec![]
    }
}

impl Default for AiExecutor {
    fn default() -> Self {
        Self::new(Path::new("."))
    }
}
