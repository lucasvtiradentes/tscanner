use anyhow::{anyhow, Result};
use std::path::PathBuf;
use tscanner_constants::{config_dir_name, config_file_name};
use tscanner_scanner::load_config;

pub fn validate(config_path: Option<PathBuf>) -> Result<()> {
    let path = config_path.unwrap_or_else(|| PathBuf::from("."));

    load_config(&path, config_dir_name(), config_file_name())
        .map(|_config| {
            println!("✓ Config is valid");
        })
        .map_err(|e| anyhow!("{}", e))
}
