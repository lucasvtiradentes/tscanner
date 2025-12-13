use crate::enums::Severity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayIssue {
    pub line: usize,
    pub column: usize,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileIssues {
    pub file_path: String,
    pub issues: Vec<DisplayIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleGroup {
    pub rule_name: String,
    pub severity: Severity,
    pub issue_count: usize,
    pub file_count: usize,
    pub files: Vec<FileIssues>,
}
