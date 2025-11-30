use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config_loader::load_config_with_custom;
use core::Severity;
use core::{app_name, log_error, log_info};

use super::output;
use super::types::RuleInfo;

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
                format!("Error: No {} configuration found!", app_name())
                    .red()
                    .bold()
            );
            eprintln!();
            eprintln!(
                "Run {} to create a default configuration.",
                format!("{} init", app_name()).cyan()
            );
            std::process::exit(1);
        }
    };

    let rules = collect_rules(&config);

    log_info(&format!(
        "cmd_rules: {} total rules, {} enabled",
        rules.len(),
        rules.iter().filter(|r| r.enabled).count()
    ));

    output::print_header(&config_file_path);
    output::print_rules(&rules);

    Ok(())
}

fn collect_rules(config: &core::config::TscannerConfig) -> Vec<RuleInfo> {
    let mut rules = Vec::new();

    for (name, rule_config) in &config.builtin_rules {
        rules.push(RuleInfo {
            name: name.clone(),
            enabled: rule_config.enabled.unwrap_or(true),
            severity: rule_config.severity.unwrap_or(Severity::Warning),
            pattern: None,
        });
    }

    for (name, rule_config) in &config.custom_rules {
        rules.push(RuleInfo {
            name: name.clone(),
            enabled: rule_config.enabled,
            severity: rule_config.severity,
            pattern: rule_config.pattern.clone(),
        });
    }

    rules
}
