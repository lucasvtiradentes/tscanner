use super::common::AiProviderImpl;
use std::path::PathBuf;

pub struct CustomProvider;

impl AiProviderImpl for CustomProvider {
    fn get_command(&self, custom_command: Option<&str>) -> Result<(String, Vec<String>), String> {
        match custom_command {
            Some(cmd) => Ok((cmd.to_string(), vec![])),
            None => Err("Custom provider requires 'command' field in config".to_string()),
        }
    }

    fn get_hardcoded_paths(&self) -> Vec<PathBuf> {
        vec![]
    }
}
