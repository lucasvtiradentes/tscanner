mod issue;
mod summary;

use super::renderer::OutputRenderer;
use super::CheckContext;
use crate::shared::{
    print_section_header, print_section_title, rule_type_icon, FormattedOutput, OutputFileGroup,
    OutputRuleGroup, OutputSummary,
};
use colored::*;
use std::cmp::Reverse;
use std::collections::HashMap;
use tscanner_constants::{icon_error, icon_warning};
use tscanner_types::{IssueRuleType, ScanResult};

use issue::render_issue_location;
pub use summary::render_summary;

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
                self.render_messages(result);
                if ctx.cli_options.show_summary {
                    self.render_summary(summary);
                }
            }
            FormattedOutput::ByRule { rules, summary } => {
                self.render_rules_triggered_by_rule(rules);
                self.render_by_rule(rules);
                println!();
                self.render_messages(result);
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
        sorted_rules.sort_by_key(|rule| Reverse(rule.1 .2));

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

        let mut sorted_rules: Vec<_> = rules.iter().collect();
        sorted_rules.sort_by_key(|rule| Reverse(rule.count));

        let max_rule_len = sorted_rules.iter().map(|r| r.rule.len()).max().unwrap_or(0);
        let max_count_len = sorted_rules
            .iter()
            .map(|r| r.count.to_string().len())
            .max()
            .unwrap_or(0);

        for rule in sorted_rules {
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
        let mut sorted_files: Vec<_> = files.iter().collect();
        sorted_files.sort_by_key(|file| Reverse(file.issues.len()));

        for file in sorted_files {
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
        let mut sorted_rules: Vec<_> = rules.iter().collect();
        sorted_rules.sort_by_key(|rule| Reverse(rule.count));

        for rule in sorted_rules {
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

            let mut sorted_files: Vec<_> = files_map.iter().collect();
            sorted_files.sort_by_key(|file| Reverse(file.1.len()));

            for (file, issues) in sorted_files {
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

    fn render_messages(&self, result: &ScanResult) {
        if !result.notes.is_empty() {
            print_section_header("Notes:");
            for note in &result.notes {
                println!("  {} {}", "ℹ".blue(), note.dimmed());
            }
        }

        if !result.warnings.is_empty() {
            print_section_header("Warnings:");
            for warning in &result.warnings {
                println!("  {} {}", icon_warning().yellow(), warning.yellow());
            }
        }

        if !result.errors.is_empty() {
            print_section_header("Errors:");
            for error in &result.errors {
                println!("  {} {}", icon_error().red(), error.red());
            }
        }

        if !result.notes.is_empty() || !result.warnings.is_empty() || !result.errors.is_empty() {
            println!();
        }
    }
}
