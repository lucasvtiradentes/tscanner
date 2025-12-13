use super::common::{get_home_dir, resolve_command_path, truncate_error, AiProviderImpl};
use std::path::PathBuf;
use tscanner_constants::{claude_args, claude_command};

pub struct ClaudeProvider;

impl AiProviderImpl for ClaudeProvider {
    fn get_command(&self, _custom_command: Option<&str>) -> Result<(String, Vec<String>), String> {
        let cmd_name = claude_command();
        let resolved_cmd = resolve_command_path(cmd_name, self.get_hardcoded_paths())?;
        Ok((resolved_cmd, claude_args().to_vec()))
    }

    fn get_hardcoded_paths(&self) -> Vec<PathBuf> {
        let Some(home) = get_home_dir() else {
            return vec![];
        };

        let cmd_name = claude_command();
        vec![
            home.join(".claude").join("local").join(cmd_name),
            home.join(".claude").join("bin").join(cmd_name),
            home.join(".local").join("bin").join(cmd_name),
            home.join(".npm-global").join("bin").join(cmd_name),
        ]
    }

    fn parse_error(&self, error_output: &str) -> String {
        let lower = error_output.to_lowercase();

        if lower.contains("authentication_error") || lower.contains("oauth token has expired") {
            return "Claude authentication expired. Run 'claude /login' to re-authenticate."
                .to_string();
        }

        if lower.contains("invalid_api_key") || lower.contains("invalid api key") {
            return "Claude API key is invalid. Check your credentials or run 'claude /login'."
                .to_string();
        }

        if lower.contains("rate_limit") || lower.contains("rate limit") {
            return "Claude rate limit exceeded. Wait a moment and try again.".to_string();
        }

        if lower.contains("quota") || lower.contains("insufficient_quota") {
            return "Claude quota exceeded. Check your usage limits.".to_string();
        }

        if lower.contains("permission") || lower.contains("forbidden") {
            return "Permission denied. Check your Claude API credentials.".to_string();
        }

        truncate_error(error_output)
    }
}
