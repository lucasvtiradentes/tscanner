use anyhow::{Context, Result};
use colored::*;
use core::cache::FileCache;
use core::scanner::Scanner;
use core::types::Severity;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::config_loader::{get_vscode_global_config_path, load_config};
use crate::GroupMode;
use core::{log_error, log_info, APP_NAME, CONFIG_DIR_NAME, CONFIG_FILE_NAME};

pub fn cmd_check(path: &Path, no_cache: bool, group_mode: GroupMode) -> Result<()> {
    log_info(&format!(
        "cmd_check: Starting at: {} (no_cache: {}, group_mode: {:?})",
        path.display(),
        no_cache,
        group_mode
    ));

    let root = fs::canonicalize(path).context("Failed to resolve path")?;

    let config = match load_config(&root)? {
        Some(cfg) => {
            log_info("cmd_check: Config loaded successfully");
            cfg
        }
        None => {
            log_error("cmd_check: No config found");
            eprintln!(
                "{}",
                format!("Error: No {} configuration found!", APP_NAME)
                    .red()
                    .bold()
            );
            eprintln!();
            eprintln!("Searched for config in:");
            eprintln!(
                "  • {}",
                format!(
                    "{}/{}/{}",
                    root.display(),
                    CONFIG_DIR_NAME,
                    CONFIG_FILE_NAME
                )
                .yellow()
            );

            if let Some(global_path) = get_vscode_global_config_path(&root) {
                eprintln!("  • {}", global_path.display().to_string().yellow());
            }

            eprintln!();
            eprintln!(
                "Run {} to create a default configuration.",
                format!("{} init", APP_NAME).cyan()
            );
            std::process::exit(1);
        }
    };

    let config_hash = config.compute_hash();
    let cache = if no_cache {
        FileCache::new()
    } else {
        FileCache::with_config_hash(config_hash)
    };

    let scanner =
        Scanner::with_cache(config, Arc::new(cache)).map_err(|e| anyhow::anyhow!("{}", e))?;

    println!("{}", "Scanning...".cyan().bold());
    let result = scanner.scan(&root);

    log_info(&format!(
        "cmd_check: Scan completed: {} files, {}ms",
        result.files.len(),
        result.duration_ms
    ));

    if result.files.is_empty() {
        println!("{}", "✓ No issues found!".green().bold());
        return Ok(());
    }

    let mut error_count = 0;
    let mut warning_count = 0;

    if matches!(group_mode, GroupMode::Rule) {
        use std::collections::HashMap;

        let mut issues_by_rule: HashMap<String, Vec<_>> = HashMap::new();

        for file_result in &result.files {
            let relative_path = pathdiff::diff_paths(&file_result.file, &root)
                .unwrap_or_else(|| file_result.file.clone());

            for issue in &file_result.issues {
                match issue.severity {
                    Severity::Error => error_count += 1,
                    Severity::Warning => warning_count += 1,
                }

                issues_by_rule
                    .entry(issue.rule.clone())
                    .or_insert_with(Vec::new)
                    .push((relative_path.clone(), issue));
            }
        }

        let mut sorted_rules: Vec<_> = issues_by_rule.keys().cloned().collect();
        sorted_rules.sort();

        for rule_name in sorted_rules {
            let issues = &issues_by_rule[&rule_name];
            println!("\n{} ({} issues)", rule_name.bold(), issues.len());

            for (file_path, issue) in issues {
                let severity_icon = match issue.severity {
                    Severity::Error => "✖".red(),
                    Severity::Warning => "⚠".yellow(),
                };

                let location =
                    format!("{}:{}:{}", file_path.display(), issue.line, issue.column).dimmed();

                println!("  {} {} {}", severity_icon, location, issue.message);

                if let Some(line_text) = &issue.line_text {
                    let trimmed = line_text.trim();
                    if !trimmed.is_empty() {
                        println!("    {}", trimmed.dimmed());
                    }
                }
            }
        }
    } else {
        for file_result in &result.files {
            if file_result.issues.is_empty() {
                continue;
            }

            let relative_path = pathdiff::diff_paths(&file_result.file, &root)
                .unwrap_or_else(|| file_result.file.clone());

            println!("\n{}", relative_path.display().to_string().bold());

            for issue in &file_result.issues {
                match issue.severity {
                    Severity::Error => error_count += 1,
                    Severity::Warning => warning_count += 1,
                }

                let severity_icon = match issue.severity {
                    Severity::Error => "✖".red(),
                    Severity::Warning => "⚠".yellow(),
                };

                let location = format!("{}:{}", issue.line, issue.column).dimmed();
                let rule_name = format!("[{}]", issue.rule).cyan();

                println!(
                    "  {} {} {} {}",
                    severity_icon, location, issue.message, rule_name
                );

                if let Some(line_text) = &issue.line_text {
                    let trimmed = line_text.trim();
                    if !trimmed.is_empty() {
                        println!("    {}", trimmed.dimmed());
                    }
                }
            }
        }
    }

    println!();
    let total_issues = error_count + warning_count;
    println!(
        "{} {} errors, {} warnings -> total {} issues",
        if error_count > 0 {
            "✖".red()
        } else {
            "✓".green()
        },
        error_count.to_string().red(),
        warning_count.to_string().yellow(),
        total_issues.to_string().cyan()
    );
    println!(
        "Scanned {} files in {}ms",
        result.files.len(),
        result.duration_ms
    );

    log_info(&format!(
        "cmd_check: Found {} errors, {} warnings",
        error_count, warning_count
    ));

    if error_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}
