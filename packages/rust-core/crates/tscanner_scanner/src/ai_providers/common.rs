use std::path::PathBuf;
use tscanner_config::AiProvider;

use super::{ClaudeProvider, CustomProvider, GeminiProvider};

pub trait AiProviderImpl {
    fn get_command(&self, custom_command: Option<&str>) -> Result<(String, Vec<String>), String>;
    fn get_hardcoded_paths(&self) -> Vec<PathBuf>;
}

pub fn resolve_provider_command(
    provider: &AiProvider,
    custom_command: Option<&str>,
) -> Result<(String, Vec<String>), String> {
    match provider {
        AiProvider::Claude => ClaudeProvider.get_command(custom_command),
        AiProvider::Gemini => GeminiProvider.get_command(custom_command),
        AiProvider::Custom => CustomProvider.get_command(custom_command),
    }
}

pub(crate) fn resolve_command_path(
    cmd_name: &str,
    hardcoded_paths: Vec<PathBuf>,
) -> Result<String, String> {
    for path in hardcoded_paths {
        if path.exists() {
            return Ok(path.to_string_lossy().to_string());
        }
    }

    which::which(cmd_name)
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|_| {
            format!(
                "'{}' not found in PATH or default install locations. Install it or use 'custom' provider with full path",
                cmd_name
            )
        })
}

pub(crate) fn get_home_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(PathBuf::from)
}
