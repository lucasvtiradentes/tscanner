use anyhow::{Context, Result};
use colored::*;
use core::config::TscannerConfig;
use std::fs;
use std::path::Path;

use core::{log_error, log_info, CONFIG_DIR_NAME, CONFIG_FILE_NAME};

pub fn cmd_init(path: &Path) -> Result<()> {
    log_info(&format!(
        "cmd_init: Initializing config at: {}",
        path.display()
    ));

    let root = fs::canonicalize(path).context("Failed to resolve path")?;
    let config_dir = root.join(CONFIG_DIR_NAME);
    let config_path = config_dir.join(CONFIG_FILE_NAME);

    if config_path.exists() {
        log_error(&format!(
            "cmd_init: Config already exists: {}",
            config_path.display()
        ));
        eprintln!("{}", "Error: Configuration already exists!".red().bold());
        eprintln!("  {}", config_path.display());
        std::process::exit(1);
    }

    let default_config = TscannerConfig::default();
    fs::create_dir_all(&config_dir)
        .context(format!("Failed to create {} directory", CONFIG_DIR_NAME))?;

    let config_json = serde_json::to_string_pretty(&default_config)?;
    fs::write(&config_path, config_json).context("Failed to write config file")?;

    log_info(&format!(
        "cmd_init: Created config: {}",
        config_path.display()
    ));

    println!("{}", "✓ Created default configuration".green().bold());
    println!("  {}", config_path.display());
    println!();
    println!("Edit this file to enable rules and customize settings.");

    Ok(())
}
