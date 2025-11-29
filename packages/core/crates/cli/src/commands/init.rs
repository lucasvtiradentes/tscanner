use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::Path;

use core::rules::get_all_rule_metadata;
use core::{log_error, log_info, CONFIG_DIR_NAME, CONFIG_FILE_NAME};

const DEFAULT_CONFIG_JSON: &str = include_str!("../../../../../../assets/default-config.json");
const TSCANNER_VERSION: &str = env!("CARGO_PKG_VERSION");

fn generate_all_rules_config() -> String {
    let metadata = get_all_rule_metadata();
    let mut rule_names: Vec<&str> = metadata.iter().map(|m| m.name).collect();
    rule_names.sort();

    let rules_json: Vec<String> = rule_names
        .iter()
        .map(|name| format!("    \"{}\": {{}}", name))
        .collect();

    format!(
        r#"{{
  "$schema": "https://unpkg.com/tscanner@{}/schema.json",
  "builtinRules": {{
{}
  }},
  "customRules": {{}},
  "files": {{
    "include": ["**/*.ts", "**/*.tsx", "**/*.js", "**/*.jsx", "**/*.mjs", "**/*.cjs"],
    "exclude": ["**/node_modules/**", "**/dist/**", "**/build/**", "**/.git/**"]
  }},
  "lsp": {{
    "errors": true,
    "warnings": false
  }}
}}"#,
        TSCANNER_VERSION,
        rules_json.join(",\n")
    )
}

pub fn cmd_init(path: &Path, all_rules: bool) -> Result<()> {
    log_info(&format!(
        "cmd_init: Initializing config at: {} (all_rules: {})",
        path.display(),
        all_rules
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

    fs::create_dir_all(&config_dir)
        .context(format!("Failed to create {} directory", CONFIG_DIR_NAME))?;

    let config_content = if all_rules {
        generate_all_rules_config()
    } else {
        DEFAULT_CONFIG_JSON.to_string()
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
