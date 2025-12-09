use super::renderer::OutputRenderer;
use super::CheckContext;
use crate::shared::{render_summary, SummaryStats};
use colored::*;
use std::collections::{HashMap, HashSet};
use tscanner_diagnostics::{GroupMode, Issue, IssueRuleType, ScanResult, Severity};

fn rule_type_icon(rule_type: IssueRuleType) -> &'static str {
    match rule_type {
        IssueRuleType::Builtin => "◆",
        IssueRuleType::CustomRegex => "◇",
        IssueRuleType::CustomScript => "▷",
        IssueRuleType::Ai => "✦",
    }
}

fn severity_icon(severity: Severity) -> ColoredString {
    match severity {
        Severity::Error => "✖".red(),
        Severity::Warning => "⚠".yellow(),
    }
}

fn render_source_line(issue: &Issue) {
    if let Some(line_text) = &issue.line_text {
        let trimmed = line_text.trim();
        if !trimmed.is_empty() {
            println!("    {}", trimmed.dimmed());
        }
    }
}

pub struct TextRenderer;

impl OutputRenderer for TextRenderer {
    fn render(&self, ctx: &CheckContext, result: &ScanResult, stats: &SummaryStats) {
        self.render_header(stats);

        match ctx.group_mode {
            GroupMode::Rule => self.render_by_rule(ctx, result),
            GroupMode::File => self.render_by_file(ctx, result),
        }

        println!();

        if ctx.cli_options.show_summary {
            render_summary(result, stats);
        }
    }
}

impl TextRenderer {
    fn render_header(&self, _stats: &SummaryStats) {
        println!();
        println!("{}", "Results:".cyan().bold());
    }

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
            let rule_type = issues.first().map(|(_, i)| i.rule_type).unwrap_or_default();
            let icon = rule_type_icon(rule_type);
            println!(
                "\n{} {} ({} issues, {} files)",
                icon.dimmed(),
                rule_name.bold(),
                issues.len(),
                unique_files.len()
            );

            for (file_path, issue) in issues {
                let location =
                    format!("{}:{}:{}", file_path.display(), issue.line, issue.column).dimmed();

                let mut parts: Vec<String> = Vec::new();

                if ctx.cli_options.show_issue_severity {
                    parts.push(severity_icon(issue.severity).to_string());
                }

                parts.push(location.to_string());

                if ctx.cli_options.show_issue_description {
                    parts.push(issue.message.clone());
                }

                println!("  {}", parts.join(" "));

                if ctx.cli_options.show_issue_source_line {
                    render_source_line(issue);
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

                if ctx.cli_options.show_issue_severity {
                    parts.push(severity_icon(issue.severity).to_string());
                }

                parts.push(location.to_string());

                if ctx.cli_options.show_issue_rule_name && ctx.cli_options.show_issue_description {
                    let icon = rule_type_icon(issue.rule_type).dimmed();
                    let rule_name = issue.rule.cyan().to_string();
                    parts.push(format!("{} {} {}", icon, rule_name, issue.message.dimmed()));
                } else if ctx.cli_options.show_issue_rule_name {
                    let icon = rule_type_icon(issue.rule_type).dimmed();
                    let rule_name = issue.rule.cyan().to_string();
                    parts.push(format!("{} {}", icon, rule_name));
                } else if ctx.cli_options.show_issue_description {
                    parts.push(issue.message.clone());
                }

                println!("  {}", parts.join(" "));

                if ctx.cli_options.show_issue_source_line {
                    render_source_line(issue);
                }
            }
        }
    }
}
