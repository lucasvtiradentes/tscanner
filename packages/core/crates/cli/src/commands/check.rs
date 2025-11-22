use anyhow::{Context, Result};
use colored::*;
use core::cache::FileCache;
use core::scanner::Scanner;
use core::types::Severity;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use crate::config_loader::{get_vscode_global_config_path, load_config};
use crate::GroupMode;
use core::{log_error, log_info, APP_NAME, CONFIG_DIR_NAME, CONFIG_FILE_NAME};

#[derive(Serialize)]
struct JsonIssue {
    rule: String,
    severity: String,
    line: usize,
    column: usize,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_text: Option<String>,
}

#[derive(Serialize)]
struct JsonFileGroup {
    file: String,
    issues: Vec<JsonIssue>,
}

#[derive(Serialize)]
struct JsonRuleGroup {
    rule: String,
    count: usize,
    issues: Vec<JsonRuleIssue>,
}

#[derive(Serialize)]
struct JsonRuleIssue {
    file: String,
    line: usize,
    column: usize,
    message: String,
    severity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_text: Option<String>,
}

#[derive(Serialize)]
struct JsonSummary {
    total_files: usize,
    total_issues: usize,
    errors: usize,
    warnings: usize,
    duration_ms: u128,
}

#[derive(Serialize)]
#[serde(untagged)]
enum JsonOutput {
    ByFile {
        files: Vec<JsonFileGroup>,
        summary: JsonSummary,
    },
    ByRule {
        rules: Vec<JsonRuleGroup>,
        summary: JsonSummary,
    },
}

fn get_changed_files(root: &Path, branch: &str) -> Result<HashSet<PathBuf>> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(branch)
        .current_dir(root)
        .output()
        .context("Failed to execute git diff")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff failed: {}", stderr);
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| root.join(line.trim()))
        .collect();

    Ok(files)
}

pub fn cmd_check(
    path: &Path,
    no_cache: bool,
    group_mode: GroupMode,
    json_output: bool,
    branch: Option<String>,
) -> Result<()> {
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

    if !json_output {
        println!("{}", "Scanning...".cyan().bold());
    }

    let changed_files = if let Some(ref branch_name) = branch {
        match get_changed_files(&root, branch_name) {
            Ok(files) => {
                if !json_output {
                    println!(
                        "{}",
                        format!("Comparing with branch: {}", branch_name)
                            .cyan()
                            .bold()
                    );
                }
                log_info(&format!(
                    "cmd_check: Found {} changed files vs {}",
                    files.len(),
                    branch_name
                ));
                Some(files)
            }
            Err(e) => {
                eprintln!("{}", format!("Error getting changed files: {}", e).red());
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let result = scanner.scan(&root, changed_files.as_ref());

    log_info(&format!(
        "cmd_check: Scan completed: {} files, {}ms",
        result.files.len(),
        result.duration_ms
    ));

    let mut error_count = 0;
    let mut warning_count = 0;

    for file_result in &result.files {
        for issue in &file_result.issues {
            match issue.severity {
                Severity::Error => error_count += 1,
                Severity::Warning => warning_count += 1,
            }
        }
    }

    if json_output {
        let output = match group_mode {
            GroupMode::File => {
                let files: Vec<JsonFileGroup> = result
                    .files
                    .iter()
                    .filter(|f| !f.issues.is_empty())
                    .map(|file_result| {
                        let relative_path = pathdiff::diff_paths(&file_result.file, &root)
                            .unwrap_or_else(|| file_result.file.clone());

                        JsonFileGroup {
                            file: relative_path.display().to_string(),
                            issues: file_result
                                .issues
                                .iter()
                                .map(|issue| JsonIssue {
                                    rule: issue.rule.clone(),
                                    severity: match issue.severity {
                                        Severity::Error => "error".to_string(),
                                        Severity::Warning => "warning".to_string(),
                                    },
                                    line: issue.line,
                                    column: issue.column,
                                    message: issue.message.clone(),
                                    line_text: issue.line_text.clone(),
                                })
                                .collect(),
                        }
                    })
                    .collect();

                JsonOutput::ByFile {
                    files,
                    summary: JsonSummary {
                        total_files: result.files.len(),
                        total_issues: error_count + warning_count,
                        errors: error_count,
                        warnings: warning_count,
                        duration_ms: result.duration_ms,
                    },
                }
            }
            GroupMode::Rule => {
                use std::collections::HashMap;

                let mut issues_by_rule: HashMap<String, Vec<JsonRuleIssue>> = HashMap::new();

                for file_result in &result.files {
                    let relative_path = pathdiff::diff_paths(&file_result.file, &root)
                        .unwrap_or_else(|| file_result.file.clone());

                    for issue in &file_result.issues {
                        issues_by_rule
                            .entry(issue.rule.clone())
                            .or_default()
                            .push(JsonRuleIssue {
                                file: relative_path.display().to_string(),
                                line: issue.line,
                                column: issue.column,
                                message: issue.message.clone(),
                                severity: match issue.severity {
                                    Severity::Error => "error".to_string(),
                                    Severity::Warning => "warning".to_string(),
                                },
                                line_text: issue.line_text.clone(),
                            });
                    }
                }

                let mut rules: Vec<JsonRuleGroup> = issues_by_rule
                    .into_iter()
                    .map(|(rule, issues)| JsonRuleGroup {
                        count: issues.len(),
                        rule,
                        issues,
                    })
                    .collect();

                rules.sort_by(|a, b| a.rule.cmp(&b.rule));

                JsonOutput::ByRule {
                    rules,
                    summary: JsonSummary {
                        total_files: result.files.len(),
                        total_issues: error_count + warning_count,
                        errors: error_count,
                        warnings: warning_count,
                        duration_ms: result.duration_ms,
                    },
                }
            }
        };

        let json = serde_json::to_string_pretty(&output)?;
        println!("{}", json);

        if error_count > 0 {
            std::process::exit(1);
        }

        return Ok(());
    }

    if result.files.is_empty() {
        println!("{}", "✓ No issues found!".green().bold());
        return Ok(());
    }

    if matches!(group_mode, GroupMode::Rule) {
        use std::collections::HashMap;

        let mut issues_by_rule: HashMap<String, Vec<_>> = HashMap::new();

        for file_result in &result.files {
            let relative_path = pathdiff::diff_paths(&file_result.file, &root)
                .unwrap_or_else(|| file_result.file.clone());

            for issue in &file_result.issues {
                issues_by_rule
                    .entry(issue.rule.clone())
                    .or_default()
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
