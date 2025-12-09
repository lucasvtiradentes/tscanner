use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use tscanner_diagnostics::{IssueRuleType, ScanResult, Severity};

use super::summary::{RulesBreakdown, SummaryStats};

#[derive(Clone, Serialize)]
pub struct OutputIssue {
    pub rule: String,
    pub severity: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_text: Option<String>,
    pub rule_type: IssueRuleType,
}

#[derive(Clone, Serialize)]
pub struct OutputFileGroup {
    pub file: String,
    pub issues: Vec<OutputIssue>,
}

#[derive(Clone, Serialize)]
pub struct OutputRuleGroup {
    pub rule: String,
    pub rule_type: IssueRuleType,
    pub message: String,
    pub count: usize,
    pub issues: Vec<OutputRuleIssue>,
}

#[derive(Clone, Serialize)]
pub struct OutputRuleIssue {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_text: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct OutputSummary {
    pub total_files: usize,
    pub files_with_issues: usize,
    pub total_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub triggered_rules: usize,
    pub triggered_rules_breakdown: RulesBreakdown,
    pub total_enabled_rules: usize,
    pub enabled_rules_breakdown: RulesBreakdown,
    pub duration_ms: u128,
}

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum CliOutput {
    ByFile {
        files: Vec<OutputFileGroup>,
        summary: OutputSummary,
    },
    ByRule {
        rules: Vec<OutputRuleGroup>,
        summary: OutputSummary,
    },
}

impl CliOutput {
    pub fn build_by_file(root: &Path, result: &ScanResult, stats: &SummaryStats) -> Self {
        let summary = Self::build_summary(result, stats);

        let files: Vec<OutputFileGroup> = result
            .files
            .iter()
            .filter(|f| !f.issues.is_empty())
            .map(|file_result| {
                let relative_path = pathdiff::diff_paths(&file_result.file, root)
                    .unwrap_or_else(|| file_result.file.clone());

                OutputFileGroup {
                    file: relative_path.display().to_string(),
                    issues: file_result
                        .issues
                        .iter()
                        .map(|issue| OutputIssue {
                            rule: issue.rule.clone(),
                            severity: match issue.severity {
                                Severity::Error => "error".to_string(),
                                Severity::Warning => "warning".to_string(),
                            },
                            line: issue.line,
                            column: issue.column,
                            message: issue.message.clone(),
                            line_text: issue.line_text.clone(),
                            rule_type: issue.rule_type,
                        })
                        .collect(),
                }
            })
            .collect();

        CliOutput::ByFile { files, summary }
    }

    pub fn build_by_rule(root: &Path, result: &ScanResult, stats: &SummaryStats) -> Self {
        let summary = Self::build_summary(result, stats);

        let mut rules_map: HashMap<String, (IssueRuleType, String, Vec<OutputRuleIssue>)> =
            HashMap::new();

        for file_result in &result.files {
            let relative_path = pathdiff::diff_paths(&file_result.file, root)
                .unwrap_or_else(|| file_result.file.clone());

            for issue in &file_result.issues {
                let entry = rules_map.entry(issue.rule.clone()).or_insert((
                    issue.rule_type,
                    issue.message.clone(),
                    Vec::new(),
                ));

                entry.2.push(OutputRuleIssue {
                    file: relative_path.display().to_string(),
                    line: issue.line,
                    column: issue.column,
                    severity: match issue.severity {
                        Severity::Error => "error".to_string(),
                        Severity::Warning => "warning".to_string(),
                    },
                    line_text: issue.line_text.clone(),
                });
            }
        }

        let mut rules: Vec<OutputRuleGroup> = rules_map
            .into_iter()
            .map(|(rule, (rule_type, message, issues))| OutputRuleGroup {
                rule,
                rule_type,
                message,
                count: issues.len(),
                issues,
            })
            .collect();

        rules.sort_by(|a, b| a.rule.cmp(&b.rule));

        CliOutput::ByRule { rules, summary }
    }

    fn build_summary(result: &ScanResult, stats: &SummaryStats) -> OutputSummary {
        let files_with_issues = result.files.iter().filter(|f| !f.issues.is_empty()).count();
        let triggered_breakdown = Self::compute_triggered_breakdown(result);

        OutputSummary {
            total_files: result.total_files,
            files_with_issues,
            total_issues: stats.total_issues,
            errors: stats.error_count,
            warnings: stats.warning_count,
            triggered_rules: stats.unique_rules_count,
            triggered_rules_breakdown: triggered_breakdown,
            total_enabled_rules: stats.total_enabled_rules,
            enabled_rules_breakdown: stats.rules_breakdown.clone(),
            duration_ms: result.duration_ms,
        }
    }

    fn compute_triggered_breakdown(result: &ScanResult) -> RulesBreakdown {
        let mut unique_rules: HashMap<String, IssueRuleType> = HashMap::new();

        for file_result in &result.files {
            for issue in &file_result.issues {
                unique_rules
                    .entry(issue.rule.clone())
                    .or_insert(issue.rule_type);
            }
        }

        let mut breakdown = RulesBreakdown::default();
        for rule_type in unique_rules.values() {
            match rule_type {
                IssueRuleType::Builtin => breakdown.builtin += 1,
                IssueRuleType::CustomRegex => breakdown.regex += 1,
                IssueRuleType::CustomScript => breakdown.script += 1,
                IssueRuleType::Ai => breakdown.ai += 1,
            }
        }
        breakdown
    }

    pub fn summary(&self) -> &OutputSummary {
        match self {
            CliOutput::ByFile { summary, .. } => summary,
            CliOutput::ByRule { summary, .. } => summary,
        }
    }

    pub fn to_json(&self) -> Option<String> {
        serde_json::to_string_pretty(self).ok()
    }
}
