mod config_generator;

use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::Path;

use config_generator::{get_all_rules_config, get_default_config};
use core::rules::get_all_rule_metadata;
use core::{config_dir_name, config_file_name, log_error, log_info};

pub fn cmd_init(path: &Path, all_rules: bool) -> Result<()> {
    log_info(&format!(
        "cmd_init: Initializing config at: {} (all_rules: {})",
        path.display(),
        all_rules
    ));

    let root = fs::canonicalize(path).context("Failed to resolve path")?;
    let config_dir = root.join(config_dir_name());
    let config_path = config_dir.join(config_file_name());

    if config_path.exists() {
        log_error(&format!(
            "cmd_init: Config already exists: {}",
            config_path.display()
        ));
        eprintln!("{}", "Error: Configuration already exists!".red().bold());
        eprintln!("  {}", config_path.display());
        std::process::exit(1);
    }

    fs::create_dir_all(&config_dir)
        .context(format!("Failed to create {} directory", config_dir_name()))?;

    let config_content = if all_rules {
        get_all_rules_config()
    } else {
        get_default_config()
    };

    fs::write(&config_path, config_content).context("Failed to write config file")?;

    log_info(&format!(
        "cmd_init: Created config: {}",
        config_path.display()
    ));

    if all_rules {
        let rule_count = get_all_rule_metadata().len();
        println!(
            "{}",
            format!(
                "✓ Created configuration with all {} built-in rules enabled",
                rule_count
            )
            .green()
            .bold()
        );
    } else {
        println!("{}", "✓ Created default configuration".green().bold());
    }
    println!("  {}", config_path.display());
    println!();
    println!("Edit this file to customize rules and settings.");

    Ok(())
}
