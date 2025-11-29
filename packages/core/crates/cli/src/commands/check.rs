use anyhow::{Context, Result};
use colored::*;
use core::cache::FileCache;
use core::scanner::Scanner;
use core::types::Severity;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use crate::config_loader::load_config_with_custom;
use crate::{CliOverrides, GroupMode};
use core::{
    log_error, log_info, CliConfig, CliGroupBy, APP_NAME, CONFIG_DIR_NAME, CONFIG_FILE_NAME,
};

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
    cached_files: usize,
    scanned_files: usize,
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

fn parse_modified_lines(diff_output: &str) -> HashMap<String, HashSet<usize>> {
    let mut file_lines: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut current_file: Option<String> = None;
    let mut current_line: usize = 0;

    for line in diff_output.lines() {
        if line.starts_with("diff --git") {
            current_file = None;
            current_line = 0;
        } else if line.starts_with("+++") {
            if let Some(file_path) = line.strip_prefix("+++ b/") {
                current_file = Some(file_path.to_string());
            }
        } else if line.starts_with("@@") {
            if let Some(hunk_info) = line.split("@@").nth(1) {
                if let Some(new_info) = hunk_info.split_whitespace().nth(1) {
                    if let Some(line_num) = new_info.trim_start_matches('+').split(',').next() {
                        current_line = line_num.parse::<usize>().unwrap_or(0);
                    }
                }
            }
        } else if let Some(ref file) = current_file {
            if line.starts_with('+') && !line.starts_with("+++") {
                file_lines
                    .entry(file.clone())
                    .or_default()
                    .insert(current_line);
                current_line += 1;
            } else if !line.starts_with('-') {
                current_line += 1;
            }
        }
    }

    file_lines
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

fn get_modified_lines(root: &Path, branch: &str) -> Result<HashMap<PathBuf, HashSet<usize>>> {
    let output = Command::new("git")
        .arg("diff")
        .arg(branch)
        .current_dir(root)
        .output()
        .context("Failed to execute git diff")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff failed: {}", stderr);
    }

    let diff_text = String::from_utf8_lossy(&output.stdout);
    let file_lines = parse_modified_lines(&diff_text);

    let result = file_lines
        .into_iter()
        .map(|(file, lines)| (root.join(file), lines))
        .collect();

    Ok(result)
}

#[allow(clippy::too_many_arguments)]
pub fn cmd_check(
    path: &Path,
    no_cache: bool,
    group_mode: GroupMode,
    json_output: bool,
    pretty_output: bool,
    branch: Option<String>,
    file_filter: Option<String>,
    rule_filter: Option<String>,
    continue_on_error: bool,
    config_path: Option<PathBuf>,
    cli_overrides: CliOverrides,
) -> Result<()> {
    log_info(&format!(
        "cmd_check: Starting at: {} (no_cache: {}, group_mode: {:?}, pretty: {})",
        path.display(),
        no_cache,
        group_mode,
        pretty_output
    ));

    let root = fs::canonicalize(path).context("Failed to resolve path")?;

    let (config, cli_config) = match load_config_with_custom(&root, config_path)? {
        Some((cfg, config_file_path)) => {
            log_info(&format!(
                "cmd_check: Config loaded successfully from: {}",
                config_file_path
            ));
            let file_cli_config = cfg.cli.clone().unwrap_or_default();
            (cfg, file_cli_config)
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
            eprintln!("Expected config at:");
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

            eprintln!();
            eprintln!(
                "Run {} to create a default configuration,",
                format!("{} init", APP_NAME).cyan()
            );
            eprintln!(
                "or use {} to specify a custom config directory.",
                "--config <path>".cyan()
            );
            std::process::exit(1);
        }
    };

    let resolved_cli = CliConfig {
        group_by: cli_overrides
            .by_rule
            .map(|v| {
                if v {
                    CliGroupBy::Rule
                } else {
                    CliGroupBy::File
                }
            })
            .unwrap_or(cli_config.group_by),
        no_cache: cli_overrides.no_cache.unwrap_or(cli_config.no_cache),
        show_severity: cli_overrides
            .show_severity
            .unwrap_or(cli_config.show_severity),
        show_source_line: cli_overrides
            .show_source_line
            .unwrap_or(cli_config.show_source_line),
        show_rule_name: cli_overrides
            .show_rule_name
            .unwrap_or(cli_config.show_rule_name),
        show_description: cli_overrides
            .show_description
            .unwrap_or(cli_config.show_description),
        show_summary_at_footer: cli_overrides
            .show_summary_at_footer
            .unwrap_or(cli_config.show_summary_at_footer),
    };

    let effective_group_mode = match resolved_cli.group_by {
        CliGroupBy::Rule => GroupMode::Rule,
        CliGroupBy::File => GroupMode::File,
    };

    let effective_group_mode = if matches!(group_mode, GroupMode::Rule) {
        GroupMode::Rule
    } else {
        effective_group_mode
    };

    let effective_no_cache = no_cache || resolved_cli.no_cache;

    let config_hash = config.compute_hash();
    let cache = if effective_no_cache {
        FileCache::new()
    } else {
        FileCache::with_config_hash(config_hash)
    };

    let scanner = Scanner::with_cache(config, Arc::new(cache), root.clone())
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    if !json_output {
        println!("{}", "Scanning...".cyan().bold());
        println!();
        let group_by_str = match effective_group_mode {
            GroupMode::Rule => "rule",
            GroupMode::File => "file",
        };
        println!(
            "  {} {}",
            "Cache:".dimmed(),
            if effective_no_cache {
                "disabled"
            } else {
                "enabled"
            }
        );
        println!("  {} {}", "Group by:".dimmed(), group_by_str);
    }

    let (changed_files, modified_lines) = if let Some(ref branch_name) = branch {
        match (
            get_changed_files(&root, branch_name),
            get_modified_lines(&root, branch_name),
        ) {
            (Ok(files), Ok(lines)) => {
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
                (Some(files), Some(lines))
            }
            (Err(e), _) | (_, Err(e)) => {
                eprintln!("{}", format!("Error getting changed files: {}", e).red());
                std::process::exit(1);
            }
        }
    } else {
        (None, None)
    };

    let files_to_scan = if let Some(ref file_pattern) = file_filter {
        use glob::Pattern;
        let pattern = Pattern::new(file_pattern)
            .context(format!("Invalid file pattern: {}", file_pattern))?;

        if let Some(mut files) = changed_files {
            let original_count = files.len();
            files.retain(|file_path| {
                let relative_path =
                    pathdiff::diff_paths(file_path, &root).unwrap_or_else(|| file_path.clone());
                pattern.matches_path(&relative_path)
            });
            log_info(&format!(
                "cmd_check: File filter {} → {} files (pattern: {})",
                original_count,
                files.len(),
                file_pattern
            ));
            Some(files)
        } else {
            use walkdir::WalkDir;
            let mut matching_files = HashSet::new();

            for entry in WalkDir::new(&root)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    let file_path = entry.path();
                    let relative_path = pathdiff::diff_paths(file_path, &root)
                        .unwrap_or_else(|| file_path.to_path_buf());

                    if pattern.matches_path(&relative_path) {
                        matching_files.insert(file_path.to_path_buf());
                    }
                }
            }

            log_info(&format!(
                "cmd_check: File filter found {} matching files (pattern: {})",
                matching_files.len(),
                file_pattern
            ));
            Some(matching_files)
        }
    } else {
        changed_files
    };

    let mut result = scanner.scan(&root, files_to_scan.as_ref());

    if let Some(ref line_filter) = modified_lines {
        let original_count = result.files.iter().map(|f| f.issues.len()).sum::<usize>();

        result.files = result
            .files
            .into_iter()
            .filter_map(|mut file_result| {
                if let Some(modified_lines_in_file) = line_filter.get(&file_result.file) {
                    file_result
                        .issues
                        .retain(|issue| modified_lines_in_file.contains(&issue.line));
                    if !file_result.issues.is_empty() {
                        Some(file_result)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let filtered_count = result.files.iter().map(|f| f.issues.len()).sum::<usize>();
        result.total_issues = filtered_count;

        log_info(&format!(
            "cmd_check: Filtered {} → {} issues (only modified lines)",
            original_count, filtered_count
        ));
    }

    if let Some(ref rule_name) = rule_filter {
        let original_count = result.files.iter().map(|f| f.issues.len()).sum::<usize>();

        result.files = result
            .files
            .into_iter()
            .filter_map(|mut file_result| {
                file_result.issues.retain(|issue| issue.rule == *rule_name);
                if !file_result.issues.is_empty() {
                    Some(file_result)
                } else {
                    None
                }
            })
            .collect();

        let filtered_count = result.files.iter().map(|f| f.issues.len()).sum::<usize>();
        result.total_issues = filtered_count;

        log_info(&format!(
            "cmd_check: Rule filter {} → {} issues (rule: {})",
            original_count, filtered_count, rule_name
        ));
    }

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
        let output = match effective_group_mode {
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
                        total_files: result.total_files,
                        cached_files: result.cached_files,
                        scanned_files: result.scanned_files,
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
                        total_files: result.total_files,
                        cached_files: result.cached_files,
                        scanned_files: result.scanned_files,
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

        if error_count > 0 && !continue_on_error {
            std::process::exit(1);
        }

        return Ok(());
    }

    if result.files.is_empty() {
        println!("{}", "✓ No issues found!".green().bold());
        return Ok(());
    }

    if matches!(effective_group_mode, GroupMode::Rule) {
        if pretty_output {
            let formatted = core::PrettyFormatter::format_by_rule(&result, &root);
            print!("{}", formatted);
        } else {
            use std::collections::HashMap;
            use std::collections::HashSet;

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
                let unique_files: HashSet<_> = issues.iter().map(|(path, _)| path).collect();
                println!(
                    "\n{} ({} issues, {} files)",
                    rule_name.bold(),
                    issues.len(),
                    unique_files.len()
                );

                for (file_path, issue) in issues {
                    let location =
                        format!("{}:{}:{}", file_path.display(), issue.line, issue.column).dimmed();

                    let mut parts: Vec<String> = Vec::new();

                    if resolved_cli.show_severity {
                        let icon = match issue.severity {
                            Severity::Error => "✖".red().to_string(),
                            Severity::Warning => "⚠".yellow().to_string(),
                        };
                        parts.push(icon);
                    }

                    parts.push(location.to_string());

                    if resolved_cli.show_description {
                        parts.push(issue.message.clone());
                    }

                    println!("  {}", parts.join(" "));

                    if resolved_cli.show_source_line {
                        if let Some(line_text) = &issue.line_text {
                            let trimmed = line_text.trim();
                            if !trimmed.is_empty() {
                                println!("    {}", trimmed.dimmed());
                            }
                        }
                    }
                }
            }
        }
    } else if pretty_output {
        let formatted = core::PrettyFormatter::format_by_file(&result, &root);
        print!("{}", formatted);
        println!();
    } else {
        for file_result in &result.files {
            if file_result.issues.is_empty() {
                continue;
            }

            let relative_path = pathdiff::diff_paths(&file_result.file, &root)
                .unwrap_or_else(|| file_result.file.clone());

            println!(
                "\n{} ({} issues)",
                relative_path.display().to_string().bold(),
                file_result.issues.len()
            );

            for issue in &file_result.issues {
                let location = format!("{}:{}", issue.line, issue.column).dimmed();

                let mut parts: Vec<String> = Vec::new();

                if resolved_cli.show_severity {
                    let icon = match issue.severity {
                        Severity::Error => "✖".red().to_string(),
                        Severity::Warning => "⚠".yellow().to_string(),
                    };
                    parts.push(icon);
                }

                parts.push(location.to_string());

                if resolved_cli.show_rule_name && resolved_cli.show_description {
                    let rule_name = issue.rule.cyan().to_string();
                    parts.push(format!("{} {}", rule_name, issue.message.dimmed()));
                } else if resolved_cli.show_rule_name {
                    let rule_name = issue.rule.cyan().to_string();
                    parts.push(rule_name);
                } else if resolved_cli.show_description {
                    parts.push(issue.message.clone());
                }

                println!("  {}", parts.join(" "));

                if resolved_cli.show_source_line {
                    if let Some(line_text) = &issue.line_text {
                        let trimmed = line_text.trim();
                        if !trimmed.is_empty() {
                            println!("    {}", trimmed.dimmed());
                        }
                    }
                }
            }
        }
    }

    if resolved_cli.show_summary_at_footer {
        let total_issues = error_count + warning_count;

        let unique_rules: std::collections::HashSet<_> = result
            .files
            .iter()
            .flat_map(|f| f.issues.iter().map(|i| &i.rule))
            .collect();

        if pretty_output {
            println!();
        }
        println!();
        println!(
            "{} {} ({} errors, {} warnings)",
            "Issues:".dimmed(),
            total_issues.to_string().cyan(),
            error_count.to_string().red(),
            warning_count.to_string().yellow()
        );
        println!(
            "{} {} ({} cached, {} scanned)",
            "Files:".dimmed(),
            result.total_files,
            result.cached_files.to_string().green(),
            result.scanned_files.to_string().yellow()
        );
        println!("{} {}", "Rules:".dimmed(), unique_rules.len());
        println!("{} {}ms", "Duration:".dimmed(), result.duration_ms);
        println!();
    }

    log_info(&format!(
        "cmd_check: Found {} errors, {} warnings",
        error_count, warning_count
    ));

    if error_count > 0 && !continue_on_error {
        std::process::exit(1);
    }

    Ok(())
}
