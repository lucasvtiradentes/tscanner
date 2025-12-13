use std::collections::HashMap;

use tscanner_types::IssueRuleType;

use crate::display::{format_duration, rule_type_icon, severity_icon};
use crate::formatted::FormattedOutput;
use crate::types::{OutputFileGroup, OutputRuleGroup, OutputSummary};

fn format_issue_line(
    severity_icon: &str,
    location: &str,
    line_text: Option<&str>,
    indent: &str,
) -> String {
    if let Some(text) = line_text {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return format!("{}{} {} -> {}", indent, severity_icon, location, trimmed);
        }
    }
    format!("{}{} {}", indent, severity_icon, location)
}

impl FormattedOutput {
    pub fn to_plain_text(&self, include_summary: bool) -> String {
        match self {
            FormattedOutput::ByFile { files, summary } => {
                render_by_file(files, summary, include_summary)
            }
            FormattedOutput::ByRule { rules, summary } => {
                render_by_rule(rules, summary, include_summary)
            }
        }
    }
}

fn render_by_file(
    files: &[OutputFileGroup],
    summary: &OutputSummary,
    include_summary: bool,
) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push(String::new());
    lines.push("Results:".to_string());

    let mut rules_map: HashMap<String, (String, IssueRuleType)> = HashMap::new();
    for file in files {
        for issue in &file.issues {
            if !rules_map.contains_key(&issue.rule) {
                rules_map.insert(issue.rule.clone(), (issue.message.clone(), issue.rule_type));
            }
        }
    }

    if !rules_map.is_empty() {
        lines.push(String::new());
        lines.push("Rules triggered:".to_string());
        lines.push(String::new());

        let mut sorted_rules: Vec<_> = rules_map.iter().collect();
        sorted_rules.sort_by_key(|(rule, _)| *rule);

        let max_rule_len = sorted_rules
            .iter()
            .map(|(rule, _)| rule.len())
            .max()
            .unwrap_or(0);

        for (rule, (message, rule_type)) in sorted_rules {
            let icon = rule_type_icon(*rule_type);
            lines.push(format!(
                "  {} {:<width$}: {}",
                icon,
                rule,
                message,
                width = max_rule_len
            ));
        }

        lines.push(String::new());
        lines.push("Issues grouped by file:".to_string());
    }

    for file in files {
        let mut issues_by_rule: HashMap<&str, Vec<_>> = HashMap::new();
        for issue in &file.issues {
            issues_by_rule.entry(&issue.rule).or_default().push(issue);
        }

        let unique_rules = issues_by_rule.len();
        lines.push(String::new());
        lines.push(format!(
            "{} - {} issues - {} rules",
            file.file,
            file.issues.len(),
            unique_rules
        ));

        let mut sorted_rules: Vec<_> = issues_by_rule.keys().collect();
        sorted_rules.sort();

        for rule_name in sorted_rules {
            let issues = &issues_by_rule[rule_name];
            let rule_type = issues.first().map(|i| i.rule_type).unwrap_or_default();
            let icon = rule_type_icon(rule_type);

            lines.push(String::new());
            lines.push(format!(
                "  {} {} ({} issues)",
                icon,
                rule_name,
                issues.len()
            ));

            for issue in issues {
                let icon = severity_icon(&issue.severity);
                let location = format!("{}:{}", issue.line, issue.column);
                lines.push(format_issue_line(
                    icon,
                    &location,
                    issue.line_text.as_deref(),
                    "    ",
                ));
            }
        }
    }

    lines.push(String::new());

    if include_summary {
        render_summary_lines(&mut lines, summary);
    }

    lines.join("\n")
}

fn render_by_rule(
    rules: &[OutputRuleGroup],
    summary: &OutputSummary,
    include_summary: bool,
) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push(String::new());
    lines.push("Results:".to_string());

    if !rules.is_empty() {
        lines.push(String::new());
        lines.push("Rules triggered:".to_string());
        lines.push(String::new());

        let max_rule_len = rules.iter().map(|r| r.rule.len()).max().unwrap_or(0);

        for rule in rules {
            let icon = rule_type_icon(rule.rule_type);
            lines.push(format!(
                "  {} {:<width$}: {}",
                icon,
                rule.rule,
                rule.message,
                width = max_rule_len
            ));
        }

        lines.push(String::new());
        lines.push("Issues grouped by rule:".to_string());
    }

    for rule in rules {
        let icon = rule_type_icon(rule.rule_type);

        let mut files_map: HashMap<&str, Vec<_>> = HashMap::new();
        for issue in &rule.issues {
            files_map.entry(&issue.file).or_default().push(issue);
        }

        let unique_files = files_map.len();

        lines.push(String::new());
        lines.push(format!(
            "{} {} ({} issues, {} files)",
            icon, rule.rule, rule.count, unique_files
        ));

        let mut sorted_files: Vec<_> = files_map.keys().collect();
        sorted_files.sort();

        for file in sorted_files {
            let issues = &files_map[file];

            lines.push(String::new());
            lines.push(format!("  {} ({} issues)", file, issues.len()));

            for issue in issues {
                let icon = severity_icon(&issue.severity);
                let location = format!("{}:{}", issue.line, issue.column);
                lines.push(format_issue_line(
                    icon,
                    &location,
                    issue.line_text.as_deref(),
                    "    ",
                ));
            }
        }
    }

    lines.push(String::new());

    if include_summary {
        render_summary_lines(&mut lines, summary);
    }

    lines.join("\n")
}

fn render_summary_lines(lines: &mut Vec<String>, summary: &OutputSummary) {
    lines.push("Summary:".to_string());
    lines.push(String::new());

    lines.push(format!("  Issues: {}", summary.format_issues_plain()));

    let breakdown_parts = summary.rules_breakdown_parts();
    let breakdown_str = if breakdown_parts.is_empty() {
        String::new()
    } else {
        let parts: Vec<String> = breakdown_parts
            .iter()
            .map(|(count, label)| format!("{} {}", count, label))
            .collect();
        format!(" ({})", parts.join(", "))
    };

    lines.push(format!(
        "  Triggered rules: {}/{}{}",
        summary.triggered_rules, summary.total_enabled_rules, breakdown_str
    ));

    lines.push(format!(
        "  Files with issues: {}/{}",
        summary.files_with_issues, summary.total_files
    ));

    lines.push(format!(
        "  Files: {} ({} cached, {} scanned)",
        summary.total_files, summary.cached_files, summary.scanned_files
    ));

    lines.push(format!(
        "  Duration: {}",
        format_duration(summary.duration_ms)
    ));

    lines.push(String::new());
}
