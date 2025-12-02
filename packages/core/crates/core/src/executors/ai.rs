use crate::config::AiRuleConfig;
use crate::output::Issue;
use std::path::{Path, PathBuf};

pub struct AiExecutor {
    prompts_dir: PathBuf,
}

impl AiExecutor {
    pub fn new(workspace_root: &Path) -> Self {
        Self {
            prompts_dir: workspace_root.join(".tscanner").join("prompts"),
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
                crate::utils::log_warn(&format!(
                    "AI rule '{}' prompt file not found: {:?}",
                    rule_name, prompt_path
                ));
                continue;
            }

            crate::utils::log_debug(&format!(
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
