use super::renderer::OutputRenderer;
use super::CheckContext;
use crate::shared::{
    format_duration, print_section_header, print_section_title, rule_type_icon, severity_icon,
    FormattedOutput, OutputFileGroup, OutputRuleGroup, OutputSummary,
};
use colored::*;
use std::collections::HashMap;
use tscanner_constants::{
    icon_ai, icon_builtin, icon_error, icon_hint, icon_info, icon_regex, icon_script, icon_warning,
};
use tscanner_types::{IssueRuleType, ScanResult};

fn get_severity_icon(severity: &str) -> ColoredString {
    let icon = severity_icon(severity);
    match severity {
        "error" => icon.red(),
        "warning" => icon.yellow(),
        "info" => icon.blue(),
        "hint" => icon.dimmed(),
        _ => icon.yellow(),
    }
}

trait IssueDisplay {
    fn severity(&self) -> &str;
    fn line(&self) -> usize;
    fn column(&self) -> usize;
    fn line_text(&self) -> Option<&str>;
}

impl IssueDisplay for tscanner_cli_output::OutputIssue {
    fn severity(&self) -> &str {
        &self.severity
    }
    fn line(&self) -> usize {
        self.line
    }
    fn column(&self) -> usize {
        self.column
    }
    fn line_text(&self) -> Option<&str> {
        self.line_text.as_deref()
    }
}

impl IssueDisplay for tscanner_cli_output::OutputRuleIssue {
    fn severity(&self) -> &str {
        &self.severity
    }
    fn line(&self) -> usize {
        self.line
    }
    fn column(&self) -> usize {
        self.column
    }
    fn line_text(&self) -> Option<&str> {
        self.line_text.as_deref()
    }
}

fn render_issue_location<T: IssueDisplay>(issue: &T) {
    let severity_icon = get_severity_icon(issue.severity());
    let location = format!("{}:{}", issue.line(), issue.column());

    if let Some(line_text) = issue.line_text() {
        let trimmed = line_text.trim();
        if !trimmed.is_empty() {
            println!(
                "    {} {} → {}",
                severity_icon,
                location.dimmed(),
                trimmed.dimmed()
            );
            return;
        }
    }
    println!("    {} {}", severity_icon, location.dimmed());
}

pub struct TextRenderer;

impl OutputRenderer for TextRenderer {
    fn render(&self, ctx: &CheckContext, output: &FormattedOutput, result: &ScanResult) {
        println!();
        print_section_title("Results:");

        match output {
            FormattedOutput::ByFile { files, summary } => {
                self.render_rules_triggered_by_file(files);
                self.render_by_file(files);
                println!();
                self.render_warnings(&result.warnings);
                if ctx.cli_options.show_summary {
                    self.render_summary(summary);
                }
            }
            FormattedOutput::ByRule { rules, summary } => {
                self.render_rules_triggered_by_rule(rules);
                self.render_by_rule(rules);
                println!();
                self.render_warnings(&result.warnings);
                if ctx.cli_options.show_summary {
                    self.render_summary(summary);
                }
            }
        }
    }
}

impl TextRenderer {
    fn render_rules_triggered_by_file(&self, files: &[OutputFileGroup]) {
        let mut rules_map: HashMap<String, (String, IssueRuleType, usize)> = HashMap::new();
        for file in files {
            for issue in &file.issues {
                rules_map
                    .entry(issue.rule.clone())
                    .and_modify(|(_, _, count)| *count += 1)
                    .or_insert((issue.message.clone(), issue.rule_type, 1));
            }
        }

        if rules_map.is_empty() {
            return;
        }

        println!();
        println!("Rules triggered:");
        println!();

        let mut sorted_rules: Vec<_> = rules_map.iter().collect();
        sorted_rules.sort_by_key(|(rule, _)| *rule);

        let max_rule_len = sorted_rules
            .iter()
            .map(|(rule, _)| rule.len())
            .max()
            .unwrap_or(0);

        let max_count_len = sorted_rules
            .iter()
            .map(|(_, (_, _, count))| count.to_string().len())
            .max()
            .unwrap_or(0);

        for (rule, (message, rule_type, count)) in sorted_rules {
            let icon = rule_type_icon(*rule_type);
            println!(
                "  {} {:<rule_width$} | {:>count_width$} | {}",
                icon,
                rule,
                count,
                message,
                rule_width = max_rule_len,
                count_width = max_count_len
            );
        }

        println!();
        println!("Issues grouped by file:");
    }

    fn render_rules_triggered_by_rule(&self, rules: &[OutputRuleGroup]) {
        if rules.is_empty() {
            return;
        }

        println!();
        println!("Rules triggered:");
        println!();

        let max_rule_len = rules.iter().map(|r| r.rule.len()).max().unwrap_or(0);
        let max_count_len = rules
            .iter()
            .map(|r| r.count.to_string().len())
            .max()
            .unwrap_or(0);

        for rule in rules {
            let icon = rule_type_icon(rule.rule_type);
            println!(
                "  {} {:<rule_width$} | {:>count_width$} | {}",
                icon,
                rule.rule,
                rule.count,
                rule.message,
                rule_width = max_rule_len,
                count_width = max_count_len
            );
        }

        println!();
        println!("Issues grouped by rule:");
    }

    fn render_by_file(&self, files: &[OutputFileGroup]) {
        for file in files {
            let mut issues_by_rule: HashMap<&str, Vec<_>> = HashMap::new();
            for issue in &file.issues {
                issues_by_rule.entry(&issue.rule).or_default().push(issue);
            }

            let unique_rules = issues_by_rule.len();
            println!();
            println!(
                "{} - {} issues - {} rules",
                file.file.bold(),
                file.issues.len(),
                unique_rules
            );

            let mut sorted_rules: Vec<_> = issues_by_rule.keys().collect();
            sorted_rules.sort();

            for rule_name in sorted_rules {
                let issues = &issues_by_rule[rule_name];
                let rule_type = issues.first().map(|i| i.rule_type).unwrap_or_default();
                let icon = rule_type_icon(rule_type);

                println!();
                println!("  {} {} ({} issues)", icon, rule_name, issues.len());

                for issue in issues {
                    render_issue_location(*issue);
                }
            }
        }
    }

    fn render_by_rule(&self, rules: &[OutputRuleGroup]) {
        for rule in rules {
            let icon = rule_type_icon(rule.rule_type);

            let mut files_map: HashMap<&str, Vec<_>> = HashMap::new();
            for issue in &rule.issues {
                files_map.entry(&issue.file).or_default().push(issue);
            }

            let unique_files = files_map.len();

            println!();
            println!(
                "{} {} ({} issues, {} files)",
                icon,
                rule.rule.bold(),
                rule.count,
                unique_files
            );

            let mut sorted_files: Vec<_> = files_map.keys().collect();
            sorted_files.sort();

            for file in sorted_files {
                let issues = &files_map[file];

                println!();
                println!("  {} ({} issues)", file, issues.len());

                for issue in issues {
                    render_issue_location(*issue);
                }
            }
        }
    }

    fn render_summary(&self, summary: &OutputSummary) {
        render_summary(summary);
    }

    fn render_warnings(&self, warnings: &[String]) {
        if warnings.is_empty() {
            return;
        }

        print_section_header("Warnings:");
        for warning in warnings {
            println!("  {} {}", icon_warning().yellow(), warning.yellow());
        }
    }
}

fn format_rule_breakdown(parts: &[(usize, &'static str)]) -> String {
    if parts.is_empty() {
        return String::new();
    }
    let formatted: Vec<String> = parts
        .iter()
        .map(|(count, label)| {
            let icon = match *label {
                "builtin" => icon_builtin(),
                "regex" => icon_regex(),
                "script" => icon_script(),
                "ai" => icon_ai(),
                _ => icon_builtin(),
            };
            format!("{} {}", icon, count)
        })
        .collect();
    format!(" ({})", formatted.join(", "))
}

pub fn render_summary(summary: &OutputSummary) {
    print_section_header("Scope:");

    let enabled_breakdown_str = format_rule_breakdown(&summary.enabled_rules_breakdown_parts());

    println!(
        "  {} {}{}",
        "Rules:".dimmed(),
        summary.total_enabled_rules.to_string().cyan(),
        enabled_breakdown_str
    );

    println!(
        "  {} {} ({} cached, {} scanned)",
        "Files:".dimmed(),
        summary.total_files.to_string().cyan(),
        summary.cached_files,
        summary.scanned_files
    );

    println!();
    print_section_header("Results:");

    let issue_parts = summary.issue_parts();
    if issue_parts.is_empty() {
        println!(
            "  {} {}",
            "Issues:".dimmed(),
            summary.total_issues.to_string().cyan(),
        );
    } else {
        let colored_parts: Vec<String> = issue_parts
            .iter()
            .map(|p| {
                let (icon, colored_count) = match p.label {
                    "errors" => (icon_error(), p.count.to_string().red().to_string()),
                    "warnings" => (icon_warning(), p.count.to_string().yellow().to_string()),
                    "infos" => (icon_info(), p.count.to_string().blue().to_string()),
                    "hints" => (icon_hint(), p.count.to_string().dimmed().to_string()),
                    _ => (icon_warning(), p.count.to_string()),
                };
                format!("{} {}", icon, colored_count)
            })
            .collect();
        println!(
            "  {} {} ({})",
            "Issues:".dimmed(),
            summary.total_issues.to_string().cyan(),
            colored_parts.join(", ")
        );
    }

    let breakdown_str = format_rule_breakdown(&summary.rules_breakdown_parts());

    println!(
        "  {} {}{}",
        "Triggered rules:".dimmed(),
        summary.triggered_rules.to_string().cyan(),
        breakdown_str
    );

    println!(
        "  {} {}",
        "Files with issues:".dimmed(),
        summary.files_with_issues.to_string().cyan()
    );

    println!(
        "  {} {}",
        "Duration:".dimmed(),
        format_duration(summary.duration_ms)
    );

    println!();
}
