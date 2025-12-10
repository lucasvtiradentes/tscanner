use super::common::{get_home_dir, resolve_command_path, AiProviderImpl};
use std::path::PathBuf;
use tscanner_config::{gemini_args, gemini_command};

pub struct GeminiProvider;

impl AiProviderImpl for GeminiProvider {
    fn get_command(&self, _custom_command: Option<&str>) -> Result<(String, Vec<String>), String> {
        let cmd_name = gemini_command();
        let resolved_cmd = resolve_command_path(cmd_name, self.get_hardcoded_paths())?;
        Ok((resolved_cmd, gemini_args().to_vec()))
    }

    fn get_hardcoded_paths(&self) -> Vec<PathBuf> {
        let Some(home) = get_home_dir() else {
            return vec![];
        };

        let cmd_name = gemini_command();
        let mut paths = vec![
            home.join(".npm").join("bin").join(cmd_name),
            home.join(".npm-global").join("bin").join(cmd_name),
        ];

        if cfg!(target_os = "windows") {
            if let Ok(appdata) = std::env::var("APPDATA") {
                paths.push(
                    PathBuf::from(appdata)
                        .join("npm")
                        .join(format!("{}.cmd", cmd_name)),
                );
            }
        }

        paths
    }
}
