use super::renderer::OutputRenderer;
use super::CheckContext;
use crate::shared::{FormattedOutput, OutputFileGroup, OutputRuleGroup, OutputSummary};
use colored::*;
use std::collections::HashMap;
use tscanner_config::{icon_ai, icon_builtin, icon_error, icon_regex, icon_script, icon_warning};
use tscanner_diagnostics::{IssueRuleType, ScanResult};

fn rule_type_icon(rule_type: IssueRuleType) -> &'static str {
    match rule_type {
        IssueRuleType::Builtin => icon_builtin(),
        IssueRuleType::CustomRegex => icon_regex(),
        IssueRuleType::CustomScript => icon_script(),
        IssueRuleType::Ai => icon_ai(),
    }
}

fn format_duration(ms: u128) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        let total_seconds = ms / 1000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}m {}s", minutes, seconds)
    }
}

pub struct TextRenderer;

impl OutputRenderer for TextRenderer {
    fn render(&self, ctx: &CheckContext, output: &FormattedOutput, _result: &ScanResult) {
        println!();
        println!("{}", "Results:".cyan().bold());

        match output {
            FormattedOutput::ByFile { files, summary } => {
                self.render_rules_triggered_by_file(files);
                self.render_by_file(files);
                println!();
                if ctx.cli_options.show_summary {
                    self.render_summary(summary);
                }
            }
            FormattedOutput::ByRule { rules, summary } => {
                self.render_rules_triggered_by_rule(rules);
                self.render_by_rule(rules);
                println!();
                if ctx.cli_options.show_summary {
                    self.render_summary(summary);
                }
            }
        }
    }
}

impl TextRenderer {
    fn render_rules_triggered_by_file(&self, files: &[OutputFileGroup]) {
        let mut rules_map: HashMap<String, (String, IssueRuleType)> = HashMap::new();
        for file in files {
            for issue in &file.issues {
                if !rules_map.contains_key(&issue.rule) {
                    rules_map.insert(issue.rule.clone(), (issue.message.clone(), issue.rule_type));
                }
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

        for (rule, (message, rule_type)) in sorted_rules {
            let icon = rule_type_icon(*rule_type);
            println!(
                "  {} {:<width$}: {}",
                icon,
                rule,
                message,
                width = max_rule_len
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

        for rule in rules {
            let icon = rule_type_icon(rule.rule_type);
            println!(
                "  {} {:<width$}: {}",
                icon,
                rule.rule,
                rule.message,
                width = max_rule_len
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
                    let severity_icon = if issue.severity == "error" {
                        icon_error().red()
                    } else {
                        icon_warning().yellow()
                    };

                    let location = format!("{}:{}", issue.line, issue.column);

                    if let Some(ref line_text) = issue.line_text {
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
                    let severity_icon = if issue.severity == "error" {
                        icon_error().red()
                    } else {
                        icon_warning().yellow()
                    };

                    let location = format!("{}:{}", issue.line, issue.column);

                    if let Some(ref line_text) = issue.line_text {
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
    }

    fn render_summary(&self, summary: &OutputSummary) {
        println!("{}", "Summary:".cyan().bold());
        println!();
        println!(
            "  {} {} ({} errors, {} warnings)",
            "Issues:".dimmed(),
            summary.total_issues.to_string().cyan(),
            summary.errors.to_string().red(),
            summary.warnings.to_string().yellow()
        );

        let breakdown = &summary.triggered_rules_breakdown;
        let breakdown_parts: Vec<String> = [
            (breakdown.builtin, "builtin"),
            (breakdown.regex, "regex"),
            (breakdown.script, "script"),
            (breakdown.ai, "ai"),
        ]
        .iter()
        .filter(|(count, _)| *count > 0)
        .map(|(count, label)| format!("{} {}", count, label))
        .collect();

        let breakdown_str = if breakdown_parts.is_empty() {
            String::new()
        } else {
            format!(" ({})", breakdown_parts.join(", "))
        };

        println!(
            "  {} {}/{}{}",
            "Triggered rules:".dimmed(),
            summary.triggered_rules.to_string().cyan(),
            summary.total_enabled_rules,
            breakdown_str
        );

        println!(
            "  {} {}/{}",
            "Files with issues:".dimmed(),
            summary.files_with_issues.to_string().cyan(),
            summary.total_files
        );

        println!(
            "  {} {}",
            "Duration:".dimmed(),
            format_duration(summary.duration_ms)
        );

        println!();
    }
}
