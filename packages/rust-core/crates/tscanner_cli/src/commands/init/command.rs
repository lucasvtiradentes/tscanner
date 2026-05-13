use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::Path;

use crate::shared::{fatal_error_and_exit, print_section_title};
use tscanner_constants::{ai_rules_dir, config_dir_name, config_file_name, script_rules_dir};
use tscanner_rules::get_all_rule_metadata;
use tscanner_service::{log_error, log_info};

use super::config_generator::{
    get_default_config, get_full_config, write_example_files, AI_RULE_EXAMPLE, SCRIPT_RULE_EXAMPLE,
};

const LOCAL_CONFIG_GITIGNORE: &str = "local.jsonc\n";

pub fn cmd_init(path: &Path, minimal: bool) -> Result<()> {
    log_info(&format!(
        "cmd_init: Initializing config at: {} (minimal: {})",
        path.display(),
        minimal
    ));

    let root = fs::canonicalize(path).context("Failed to resolve path")?;
    let config_dir = root.join(config_dir_name());
    let config_path = config_dir.join(config_file_name());

    if config_path.exists() {
        log_error(&format!(
            "cmd_init: Config already exists: {}",
            config_path.display()
        ));
        fatal_error_and_exit(
            "Configuration already exists!",
            &[&format!("{}", config_path.display())],
        );
    }

    fs::create_dir_all(&config_dir)
        .context(format!("Failed to create {} directory", config_dir_name()))?;

    let config_content = if minimal {
        get_default_config()
    } else {
        get_full_config()
    };

    fs::write(&config_path, &config_content).context("Failed to write config file")?;
    fs::write(config_dir.join(".gitignore"), LOCAL_CONFIG_GITIGNORE)
        .context("Failed to write local config gitignore")?;

    log_info(&format!(
        "cmd_init: Created config: {}",
        config_path.display()
    ));

    if minimal {
        println!("{}", "✓ Created minimal configuration".green().bold());
        println!("  {}", config_path.display());
    } else {
        write_example_files(&config_dir)?;

        let rule_count = get_all_rule_metadata().len();
        println!(
            "{}",
            format!(
                "✓ Created full configuration with {} built-in rules + examples",
                rule_count
            )
            .green()
            .bold()
        );
        println!("  {}", config_path.display());
        println!();
        print_section_title("Created example files:");
        println!("  {}/{}", script_rules_dir(), SCRIPT_RULE_EXAMPLE.0);
        println!("  {}/{}", ai_rules_dir(), AI_RULE_EXAMPLE.0);
    }
    println!();
    println!("Edit this file to customize rules and settings.");
    println!(
        "Personal project settings go in {}/local.jsonc (gitignored).",
        config_dir_name()
    );

    Ok(())
}
