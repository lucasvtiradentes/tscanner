use super::OutputRenderer;
use crate::commands::check::context::CheckContext;
use crate::shared::{render_summary, SummaryStats};
use cli::GroupMode;
use colored::*;
use core::types::{ScanResult, Severity};
use std::collections::{HashMap, HashSet};

pub struct TextRenderer;

impl OutputRenderer for TextRenderer {
    fn render(&self, ctx: &CheckContext, result: &ScanResult, stats: &SummaryStats) {
        match ctx.group_mode {
            GroupMode::Rule => self.render_by_rule(ctx, result),
            GroupMode::File => self.render_by_file(ctx, result),
        }

        if ctx.cli_config.show_summary {
            render_summary(result, stats);
        }
    }
}

impl TextRenderer {
    fn render_by_rule(&self, ctx: &CheckContext, result: &ScanResult) {
        let mut issues_by_rule: HashMap<String, Vec<_>> = HashMap::new();

        for file_result in &result.files {
            let relative_path = pathdiff::diff_paths(&file_result.file, &ctx.root)
                .unwrap_or_else(|| file_result.file.clone());

            for issue in &file_result.issues {
                issues_by_rule
                    .entry(issue.rule.clone())
                    .or_default()
                    .push((relative_path.clone(), issue.clone()));
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

                if ctx.cli_config.show_severity {
                    let icon = match issue.severity {
                        Severity::Error => "✖".red().to_string(),
                        Severity::Warning => "⚠".yellow().to_string(),
                    };
                    parts.push(icon);
                }

                parts.push(location.to_string());

                if ctx.cli_config.show_description {
                    parts.push(issue.message.clone());
                }

                println!("  {}", parts.join(" "));

                if ctx.cli_config.show_source_line {
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

    fn render_by_file(&self, ctx: &CheckContext, result: &ScanResult) {
        for file_result in &result.files {
            if file_result.issues.is_empty() {
                continue;
            }

            let relative_path = pathdiff::diff_paths(&file_result.file, &ctx.root)
                .unwrap_or_else(|| file_result.file.clone());

            println!(
                "\n{} ({} issues)",
                relative_path.display().to_string().bold(),
                file_result.issues.len()
            );

            for issue in &file_result.issues {
                let location = format!("{}:{}", issue.line, issue.column).dimmed();

                let mut parts: Vec<String> = Vec::new();

                if ctx.cli_config.show_severity {
                    let icon = match issue.severity {
                        Severity::Error => "✖".red().to_string(),
                        Severity::Warning => "⚠".yellow().to_string(),
                    };
                    parts.push(icon);
                }

                parts.push(location.to_string());

                if ctx.cli_config.show_rule_name && ctx.cli_config.show_description {
                    let rule_name = issue.rule.cyan().to_string();
                    parts.push(format!("{} {}", rule_name, issue.message.dimmed()));
                } else if ctx.cli_config.show_rule_name {
                    let rule_name = issue.rule.cyan().to_string();
                    parts.push(rule_name);
                } else if ctx.cli_config.show_description {
                    parts.push(issue.message.clone());
                }

                println!("  {}", parts.join(" "));

                if ctx.cli_config.show_source_line {
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
}
