use super::common::{get_home_dir, resolve_command_path, AiProviderImpl};
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
}
