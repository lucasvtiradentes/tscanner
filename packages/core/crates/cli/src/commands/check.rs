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
) -> Result<()> {
    log_info(&format!(
        "cmd_check: Starting at: {} (no_cache: {}, group_mode: {:?}, pretty: {})",
        path.display(),
        no_cache,
        group_mode,
        pretty_output
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

        if pretty_output {
            let mut rules_map: HashMap<String, String> = HashMap::new();
            for (rule_name, issues) in &issues_by_rule {
                if let Some((_, first_issue)) = issues.first() {
                    rules_map.insert(rule_name.clone(), first_issue.message.clone());
                }
            }

            if !rules_map.is_empty() {
                println!("\n{}", "Rules:".bold());
                println!();
                let mut sorted_rule_names: Vec<_> = rules_map.iter().collect();
                sorted_rule_names.sort_by_key(|(rule, _)| *rule);
                for (rule, message) in sorted_rule_names {
                    println!("  {}: {}", rule, message);
                }
                println!();
                println!("{}", "Rules:".bold());
            }

            for rule_name in sorted_rules {
                let issues = &issues_by_rule[&rule_name];
                let unique_files: HashSet<_> = issues.iter().map(|(path, _)| path).collect();
                println!(
                    "\n{} ({} issues, {} files)",
                    rule_name.bold(),
                    issues.len(),
                    unique_files.len()
                );

                let mut issues_by_file: HashMap<_, Vec<_>> = HashMap::new();
                for (file_path, issue) in issues {
                    issues_by_file
                        .entry(file_path.clone())
                        .or_default()
                        .push(issue);
                }

                let mut sorted_files: Vec<_> = issues_by_file.keys().collect();
                sorted_files.sort();

                for file_path in sorted_files {
                    let file_issues = &issues_by_file[file_path];
                    println!();
                    println!(
                        "  {} ({} issues)",
                        file_path.display().to_string().cyan(),
                        file_issues.len()
                    );

                    for issue in file_issues {
                        let severity_icon = match issue.severity {
                            Severity::Error => "✖".red(),
                            Severity::Warning => "⚠".yellow(),
                        };

                        let location = format!("{}:{}", issue.line, issue.column);

                        if let Some(line_text) = &issue.line_text {
                            let trimmed = line_text.trim();
                            if !trimmed.is_empty() {
                                println!(
                                    "    {} {} -> {}",
                                    severity_icon,
                                    location.dimmed(),
                                    trimmed.dimmed()
                                );
                            } else {
                                println!("    {} {}", severity_icon, location.dimmed());
                            }
                        } else {
                            println!("    {} {}", severity_icon, location.dimmed());
                        }
                    }
                }
            }
        } else {
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
        }
    } else {
        if pretty_output {
            use std::collections::HashMap;

            let mut rules_map: HashMap<String, String> = HashMap::new();
            for file_result in &result.files {
                for issue in &file_result.issues {
                    if !rules_map.contains_key(&issue.rule) {
                        rules_map.insert(issue.rule.clone(), issue.message.clone());
                    }
                }
            }

            if !rules_map.is_empty() {
                println!("\n{}", "Rules:".bold());
                println!();
                let mut sorted_rules: Vec<_> = rules_map.iter().collect();
                sorted_rules.sort_by_key(|(rule, _)| *rule);
                for (rule, message) in sorted_rules {
                    println!("  {}: {}", rule, message);
                }
                println!();
                println!("{}", "Files:".bold());
            }
        }

        for file_result in &result.files {
            if file_result.issues.is_empty() {
                continue;
            }

            let relative_path = pathdiff::diff_paths(&file_result.file, &root)
                .unwrap_or_else(|| file_result.file.clone());

            if pretty_output {
                use std::collections::HashMap;

                let unique_rules: std::collections::HashSet<_> =
                    file_result.issues.iter().map(|i| &i.rule).collect();
                println!(
                    "\n{} - {} issues - {} rules",
                    relative_path.display().to_string().bold(),
                    file_result.issues.len(),
                    unique_rules.len()
                );

                let mut issues_by_rule: HashMap<&str, Vec<&core::types::Issue>> = HashMap::new();
                for issue in &file_result.issues {
                    issues_by_rule
                        .entry(issue.rule.as_str())
                        .or_default()
                        .push(issue);
                }

                let mut sorted_rules: Vec<_> = issues_by_rule.keys().collect();
                sorted_rules.sort();

                for rule_name in sorted_rules {
                    let issues = &issues_by_rule[rule_name];
                    println!();
                    println!("  {} ({} issues)", rule_name.cyan(), issues.len());

                    for issue in issues {
                        let severity_icon = match issue.severity {
                            Severity::Error => "✖".red(),
                            Severity::Warning => "⚠".yellow(),
                        };

                        let location = format!("{}:{}", issue.line, issue.column);

                        if let Some(line_text) = &issue.line_text {
                            let trimmed = line_text.trim();
                            if !trimmed.is_empty() {
                                println!(
                                    "    {} {} -> {}",
                                    severity_icon,
                                    location.dimmed(),
                                    trimmed.dimmed()
                                );
                            } else {
                                println!("    {} {}", severity_icon, location.dimmed());
                            }
                        } else {
                            println!("    {} {}", severity_icon, location.dimmed());
                        }
                    }
                }
            } else {
                println!(
                    "\n{} ({} issues)",
                    relative_path.display().to_string().bold(),
                    file_result.issues.len()
                );

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
    }

    println!();
    let total_issues = error_count + warning_count;

    let unique_rules: std::collections::HashSet<_> = result
        .files
        .iter()
        .flat_map(|f| f.issues.iter().map(|i| &i.rule))
        .collect();

    println!(
        "Issues: {} ({} errors, {} warnings)",
        total_issues.to_string().cyan(),
        error_count.to_string().red(),
        warning_count.to_string().yellow()
    );
    println!("Files: {}", result.files.len());
    println!("Rules: {}", unique_rules.len());

    log_info(&format!(
        "cmd_check: Found {} errors, {} warnings",
        error_count, warning_count
    ));

    if error_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}
