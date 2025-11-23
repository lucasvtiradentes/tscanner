use anyhow::{Context, Result};
use colored::*;
use core::types::Severity;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config_loader::load_config_with_custom;
use core::{log_error, log_info, APP_DISPLAY_NAME, APP_NAME};

pub fn cmd_rules(path: &Path, config_path: Option<PathBuf>) -> Result<()> {
    log_info(&format!("cmd_rules: Listing rules at: {}", path.display()));

    let root = fs::canonicalize(path).context("Failed to resolve path")?;

    let (config, config_file_path) = match load_config_with_custom(&root, config_path)? {
        Some((cfg, path)) => {
            log_info(&format!("cmd_rules: Config loaded from: {}", path));
            (cfg, path)
        }
        None => {
            log_error("cmd_rules: No config found");
            eprintln!(
                "{}",
                format!("Error: No {} configuration found!", APP_NAME)
                    .red()
                    .bold()
            );
            eprintln!();
            eprintln!(
                "Run {} to create a default configuration.",
                format!("{} init", APP_NAME).cyan()
            );
            std::process::exit(1);
        }
    };

    println!(
        "{}",
        format!("{} Rules Configuration", APP_DISPLAY_NAME)
            .cyan()
            .bold()
    );
    println!("Config: {}\n", config_file_path.dimmed());

    let mut all_rules: Vec<(&String, bool, Severity, Option<&String>)> = Vec::new();

    for (name, rule_config) in &config.builtin_rules {
        all_rules.push((
            name,
            rule_config.enabled.unwrap_or(true),
            rule_config.severity.unwrap_or(Severity::Warning),
            None,
        ));
    }

    for (name, rule_config) in &config.custom_rules {
        all_rules.push((
            name,
            rule_config.enabled,
            rule_config.severity,
            rule_config.pattern.as_ref(),
        ));
    }

    let mut enabled_rules: Vec<_> = all_rules
        .iter()
        .filter(|(_, enabled, _, _)| *enabled)
        .collect();
    enabled_rules.sort_by_key(|(name, _, _, _)| *name);

    if enabled_rules.is_empty() {
        log_info("cmd_rules: No rules enabled");
        println!("{}", "No rules enabled.".yellow());
        return Ok(());
    }

    log_info(&format!("cmd_rules: {} enabled rules", enabled_rules.len()));
    println!("{} enabled rules:\n", enabled_rules.len());

    for (name, _, severity, pattern) in enabled_rules {
        let severity_badge = match severity {
            Severity::Error => "ERROR".red(),
            Severity::Warning => "WARN".yellow(),
        };

        let rule_type = if pattern.is_some() {
            "REGEX".magenta()
        } else {
            "AST".cyan()
        };

        print!("  {} ", "•".cyan());
        print!("{} ", name.bold());
        print!("[{}] ", rule_type);
        println!("{}", severity_badge);

        if let Some(pattern_str) = pattern {
            println!("    Pattern: {}", pattern_str.yellow());
        }
    }

    let disabled_count = all_rules
        .iter()
        .filter(|(_, enabled, _, _)| !*enabled)
        .count();
    if disabled_count > 0 {
        println!("\n{} disabled rules", disabled_count.to_string().dimmed());
    }

    Ok(())
}
