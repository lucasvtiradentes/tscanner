use super::common::{get_home_dir, resolve_command_path, AiProviderImpl};
use std::path::PathBuf;

pub struct GeminiProvider;

const GEMINI_ARGS: &[&str] = &["-o", "text"];

impl AiProviderImpl for GeminiProvider {
    fn get_command(&self, _custom_command: Option<&str>) -> Result<(String, Vec<String>), String> {
        let cmd_name = "gemini";
        let resolved_cmd = resolve_command_path(cmd_name, self.get_hardcoded_paths())?;
        Ok((
            resolved_cmd,
            GEMINI_ARGS.iter().map(|s| s.to_string()).collect(),
        ))
    }

    fn get_hardcoded_paths(&self) -> Vec<PathBuf> {
        let Some(home) = get_home_dir() else {
            return vec![];
        };

        let mut paths = vec![
            home.join(".npm").join("bin").join("gemini"),
            home.join(".npm-global").join("bin").join("gemini"),
        ];

        if cfg!(target_os = "windows") {
            if let Ok(appdata) = std::env::var("APPDATA") {
                paths.push(PathBuf::from(appdata).join("npm").join("gemini.cmd"));
            }
        }

        paths
    }
}
