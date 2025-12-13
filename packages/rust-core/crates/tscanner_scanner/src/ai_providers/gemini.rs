use super::common::{get_home_dir, resolve_command_path, truncate_error, AiProviderImpl};
use std::path::PathBuf;
use tscanner_constants::{gemini_args, gemini_command};

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

    fn parse_error(&self, error_output: &str) -> String {
        let lower = error_output.to_lowercase();

        if lower.contains("unauthenticated") || lower.contains("not authenticated") {
            return "Gemini not authenticated. Run 'gemini auth login' to authenticate."
                .to_string();
        }

        if lower.contains("invalid") && lower.contains("credentials") {
            return "Gemini credentials invalid. Run 'gemini auth login' to re-authenticate."
                .to_string();
        }

        if lower.contains("quota") || lower.contains("rate") {
            return "Gemini quota or rate limit exceeded. Wait a moment and try again.".to_string();
        }

        if lower.contains("permission") || lower.contains("forbidden") {
            return "Permission denied. Check your Gemini API credentials.".to_string();
        }

        truncate_error(error_output)
    }
}
