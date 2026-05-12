use super::common::{get_home_dir, resolve_command_path, truncate_error, AiProviderImpl};
use std::path::PathBuf;
use tscanner_constants::{codex_args, codex_command};

pub struct CodexProvider;

impl AiProviderImpl for CodexProvider {
    fn get_command(&self, model: Option<&str>) -> Result<(String, Vec<String>), String> {
        let cmd_name = codex_command();
        let resolved_cmd = resolve_command_path(cmd_name, self.get_hardcoded_paths())?;
        let mut args = codex_args().to_vec();
        if let Some(model) = model {
            args.push("--model".to_string());
            args.push(model.to_string());
        }
        Ok((resolved_cmd, args))
    }

    fn get_hardcoded_paths(&self) -> Vec<PathBuf> {
        let Some(home) = get_home_dir() else {
            return vec![];
        };

        let cmd_name = codex_command();
        vec![
            home.join(".codex").join("bin").join(cmd_name),
            home.join(".local").join("bin").join(cmd_name),
            home.join(".npm-global").join("bin").join(cmd_name),
        ]
    }

    fn parse_error(&self, error_output: &str) -> String {
        let lower = error_output.to_lowercase();

        if lower.contains("auth") || lower.contains("login") {
            return "Codex authentication failed. Run 'codex login' to authenticate.".to_string();
        }

        if lower.contains("rate") || lower.contains("quota") {
            return "Codex quota or rate limit exceeded. Wait a moment and try again.".to_string();
        }

        truncate_error(error_output)
    }
}
