use super::common::{get_home_dir, resolve_command_path, AiProviderImpl};
use std::path::PathBuf;

pub struct ClaudeProvider;

const CLAUDE_ARGS: &[&str] = &["-p", "--output-format", "text", "--model", "haiku"];

impl AiProviderImpl for ClaudeProvider {
    fn get_command(&self, _custom_command: Option<&str>) -> Result<(String, Vec<String>), String> {
        let cmd_name = "claude";
        let resolved_cmd = resolve_command_path(cmd_name, self.get_hardcoded_paths())?;
        Ok((
            resolved_cmd,
            CLAUDE_ARGS.iter().map(|s| s.to_string()).collect(),
        ))
    }

    fn get_hardcoded_paths(&self) -> Vec<PathBuf> {
        let Some(home) = get_home_dir() else {
            return vec![];
        };

        vec![
            home.join(".claude").join("local").join("claude"),
            home.join(".claude").join("bin").join("claude"),
            home.join(".local").join("bin").join("claude"),
            home.join(".npm-global").join("bin").join("claude"),
        ]
    }
}
