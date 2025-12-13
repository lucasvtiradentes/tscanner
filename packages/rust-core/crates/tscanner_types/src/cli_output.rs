use crate::enums::IssueRuleType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CliOutputIssue {
    pub rule: String,
    pub severity: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_text: Option<String>,
    pub rule_type: IssueRuleType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CliOutputFileGroup {
    pub file: String,
    pub issues: Vec<CliOutputIssue>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RulesBreakdown {
    pub builtin: usize,
    pub regex: usize,
    pub script: usize,
    pub ai: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CliOutputSummary {
    pub total_files: usize,
    pub cached_files: usize,
    pub scanned_files: usize,
    pub files_with_issues: usize,
    pub total_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
    pub hints: usize,
    pub triggered_rules: usize,
    pub triggered_rules_breakdown: RulesBreakdown,
    pub total_enabled_rules: usize,
    pub enabled_rules_breakdown: RulesBreakdown,
    pub duration_ms: u128,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CliOutputByFile {
    pub files: Vec<CliOutputFileGroup>,
    pub summary: CliOutputSummary,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CliOutputRuleIssue {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_text: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CliOutputRuleGroup {
    pub rule: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<IssueRuleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub count: usize,
    pub issues: Vec<CliOutputRuleIssue>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CliOutputByRule {
    pub rules: Vec<CliOutputRuleGroup>,
    pub summary: CliOutputSummary,
}
