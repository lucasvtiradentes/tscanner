use crate::Severity;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuleSource {
    #[default]
    Builtin,
    CustomRegex,
    CustomScript,
    Ai,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Issue {
    pub rule: String,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub message: String,
    pub severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_text: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_ai: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    pub rule_type: RuleSource,
}

impl Issue {
    pub fn new(
        rule: &str,
        file: PathBuf,
        line: usize,
        column: usize,
        end_column: usize,
        message: String,
    ) -> Self {
        Self {
            rule: rule.to_string(),
            file,
            line,
            column,
            end_column,
            message,
            severity: Severity::Error,
            line_text: None,
            is_ai: false,
            category: None,
            rule_type: RuleSource::Builtin,
        }
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_line_text(mut self, line_text: String) -> Self {
        self.line_text = Some(line_text);
        self
    }
}
