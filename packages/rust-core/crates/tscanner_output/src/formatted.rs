use std::collections::HashMap;
use std::path::Path;

use serde::Serialize;
use tscanner_diagnostics::{IssueRuleType, ScanResult, Severity};

use crate::types::{
    OutputFileGroup, OutputIssue, OutputRuleGroup, OutputRuleIssue, OutputSummary, RulesBreakdown,
};

pub struct SummaryStats {
    pub total_issues: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub hint_count: usize,
    pub unique_rules_count: usize,
    pub total_enabled_rules: usize,
    pub rules_breakdown: RulesBreakdown,
}

impl SummaryStats {
    pub fn from_result(
        result: &ScanResult,
        total_enabled_rules: usize,
        rules_breakdown: RulesBreakdown,
    ) -> Self {
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;
        let mut hint_count = 0;
        let mut unique_rules = std::collections::HashSet::new();

        for file_result in &result.files {
            for issue in &file_result.issues {
                match issue.severity {
                    Severity::Error => error_count += 1,
                    Severity::Warning => warning_count += 1,
                    Severity::Info => info_count += 1,
                    Severity::Hint => hint_count += 1,
                }
                unique_rules.insert(&issue.rule);
            }
        }

        Self {
            total_issues: error_count + warning_count + info_count + hint_count,
            error_count,
            warning_count,
            info_count,
            hint_count,
            unique_rules_count: unique_rules.len(),
            total_enabled_rules,
            rules_breakdown,
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum FormattedOutput {
    ByFile {
        files: Vec<OutputFileGroup>,
        summary: OutputSummary,
    },
    ByRule {
        rules: Vec<OutputRuleGroup>,
        summary: OutputSummary,
    },
}

impl FormattedOutput {
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
                            severity: issue.severity.as_str().to_string(),
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

        FormattedOutput::ByFile { files, summary }
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
                    severity: issue.severity.as_str().to_string(),
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

        FormattedOutput::ByRule { rules, summary }
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
            infos: stats.info_count,
            hints: stats.hint_count,
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
            FormattedOutput::ByFile { summary, .. } => summary,
            FormattedOutput::ByRule { summary, .. } => summary,
        }
    }

    pub fn to_json(&self) -> Option<String> {
        serde_json::to_string_pretty(self).ok()
    }

    pub fn files(&self) -> Option<&Vec<OutputFileGroup>> {
        match self {
            FormattedOutput::ByFile { files, .. } => Some(files),
            FormattedOutput::ByRule { .. } => None,
        }
    }

    pub fn rules(&self) -> Option<&Vec<OutputRuleGroup>> {
        match self {
            FormattedOutput::ByFile { .. } => None,
            FormattedOutput::ByRule { rules, .. } => Some(rules),
        }
    }
}
