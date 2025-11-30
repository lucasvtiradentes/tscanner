use super::types::{Issue, ScanResult, Severity};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum GroupMode {
    File,
    Rule,
}

pub struct PrettyFormatter;

impl PrettyFormatter {
    pub fn format_by_file(scan_result: &ScanResult, root: &Path) -> String {
        let mut lines: Vec<String> = Vec::new();

        let mut rules_map: HashMap<String, String> = HashMap::new();
        for file_result in &scan_result.files {
            for issue in &file_result.issues {
                if !rules_map.contains_key(&issue.rule) {
                    rules_map.insert(issue.rule.clone(), issue.message.clone());
                }
            }
        }

        if !rules_map.is_empty() {
            lines.push("\nRules triggered:".to_string());
            lines.push(String::new());
            let mut sorted_rules: Vec<_> = rules_map.iter().collect();
            sorted_rules.sort_by_key(|(rule, _)| *rule);

            let max_rule_len = sorted_rules
                .iter()
                .map(|(rule, _)| rule.len())
                .max()
                .unwrap_or(0);

            for (rule, message) in sorted_rules {
                lines.push(format!(
                    "  {:<width$}: {}",
                    rule,
                    message,
                    width = max_rule_len
                ));
            }
            lines.push(String::new());
            lines.push("Issues grouped by file:".to_string());
        }

        for file_result in &scan_result.files {
            if file_result.issues.is_empty() {
                continue;
            }

            let relative_path = pathdiff::diff_paths(&file_result.file, root)
                .unwrap_or_else(|| file_result.file.clone());

            let unique_rules: HashSet<_> = file_result.issues.iter().map(|i| &i.rule).collect();
            lines.push(String::new());
            lines.push(format!(
                "{} - {} issues - {} rules",
                relative_path.display(),
                file_result.issues.len(),
                unique_rules.len()
            ));

            let mut issues_by_rule: HashMap<&str, Vec<&Issue>> = HashMap::new();
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
                lines.push(String::new());
                lines.push(format!("  {} ({} issues)", rule_name, issues.len()));

                for issue in issues {
                    let severity_icon = match issue.severity {
                        Severity::Error => "✖",
                        Severity::Warning => "⚠",
                    };

                    let location = format!("{}:{}", issue.line, issue.column);

                    if let Some(line_text) = &issue.line_text {
                        let trimmed = line_text.trim();
                        if !trimmed.is_empty() {
                            lines
                                .push(format!("    {} {} -> {}", severity_icon, location, trimmed));
                        } else {
                            lines.push(format!("    {} {}", severity_icon, location));
                        }
                    } else {
                        lines.push(format!("    {} {}", severity_icon, location));
                    }
                }
            }
        }

        lines.join("\n")
    }

    pub fn format_by_rule(scan_result: &ScanResult, root: &Path) -> String {
        let mut lines: Vec<String> = Vec::new();

        let mut issues_by_rule: BTreeMap<String, Vec<(PathBuf, &Issue)>> = BTreeMap::new();

        for file_result in &scan_result.files {
            let relative_path = pathdiff::diff_paths(&file_result.file, root)
                .unwrap_or_else(|| file_result.file.clone());

            for issue in &file_result.issues {
                issues_by_rule
                    .entry(issue.rule.clone())
                    .or_default()
                    .push((relative_path.clone(), issue));
            }
        }

        if !issues_by_rule.is_empty() {
            lines.push("\nRules triggered:".to_string());
            lines.push(String::new());

            let max_rule_len = issues_by_rule
                .keys()
                .map(|rule| rule.len())
                .max()
                .unwrap_or(0);

            for (rule_name, issues) in &issues_by_rule {
                if let Some((_, first_issue)) = issues.first() {
                    lines.push(format!(
                        "  {:<width$}: {}",
                        rule_name,
                        first_issue.message,
                        width = max_rule_len
                    ));
                }
            }
            lines.push(String::new());
            lines.push("Issues grouped by rule:".to_string());
        }

        for (rule_name, issues) in issues_by_rule {
            let unique_files: HashSet<_> = issues.iter().map(|(path, _)| path).collect();
            lines.push(String::new());
            lines.push(format!(
                "{} ({} issues, {} files)",
                rule_name,
                issues.len(),
                unique_files.len()
            ));

            let mut issues_by_file: BTreeMap<_, Vec<_>> = BTreeMap::new();
            for (file_path, issue) in issues {
                issues_by_file.entry(file_path).or_default().push(issue);
            }

            for (file_path, file_issues) in issues_by_file {
                lines.push(String::new());
                lines.push(format!(
                    "  {} ({} issues)",
                    file_path.display(),
                    file_issues.len()
                ));

                for issue in file_issues {
                    let severity_icon = match issue.severity {
                        Severity::Error => "✖",
                        Severity::Warning => "⚠",
                    };

                    let location = format!("{}:{}", issue.line, issue.column);

                    if let Some(line_text) = &issue.line_text {
                        let trimmed = line_text.trim();
                        if !trimmed.is_empty() {
                            lines
                                .push(format!("    {} {} -> {}", severity_icon, location, trimmed));
                        } else {
                            lines.push(format!("    {} {}", severity_icon, location));
                        }
                    } else {
                        lines.push(format!("    {} {}", severity_icon, location));
                    }
                }
            }
        }

        lines.join("\n")
    }
}
