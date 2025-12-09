use serde::Serialize;
use tscanner_diagnostics::IssueRuleType;

#[derive(Debug, Clone)]
pub enum GroupMode {
    File,
    Rule,
}

#[derive(Clone, Default, Serialize)]
pub struct RulesBreakdown {
    pub builtin: usize,
    pub regex: usize,
    pub script: usize,
    pub ai: usize,
}

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
