use std::path::PathBuf;
use tscanner_config::AiProvider;

use super::{ClaudeProvider, CustomProvider, GeminiProvider};

pub trait AiProviderImpl {
    fn get_command(&self, custom_command: Option<&str>) -> Result<(String, Vec<String>), String>;
    fn get_hardcoded_paths(&self) -> Vec<PathBuf>;
    fn parse_error(&self, error_output: &str) -> String {
        truncate_error(error_output)
    }
}

pub fn truncate_error(error_output: &str) -> String {
    if error_output.len() > 200 {
        format!("{}...", &error_output[..200])
    } else {
        error_output.to_string()
    }
}

pub fn parse_provider_error(provider: Option<&AiProvider>, error_output: &str) -> String {
    match provider {
        Some(AiProvider::Claude) => ClaudeProvider.parse_error(error_output),
        Some(AiProvider::Gemini) => GeminiProvider.parse_error(error_output),
        Some(AiProvider::Custom) => CustomProvider.parse_error(error_output),
        None => truncate_error(error_output),
    }
}

pub fn resolve_provider_command(
    provider: Option<&AiProvider>,
    custom_command: Option<&str>,
) -> Result<(String, Vec<String>), String> {
    match provider {
        Some(AiProvider::Claude) => ClaudeProvider.get_command(custom_command),
        Some(AiProvider::Gemini) => GeminiProvider.get_command(custom_command),
        Some(AiProvider::Custom) => CustomProvider.get_command(custom_command),
        None => Err("AI provider not configured. Add 'ai.provider' to your config.".to_string()),
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
