use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResult {
    pub file: PathBuf,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub files: Vec<FileResult>,
    pub total_issues: usize,
    pub duration_ms: u128,
    pub total_files: usize,
    pub cached_files: usize,
    pub scanned_files: usize,
}

impl ScanResult {
    pub fn filter_by_modified_lines(&mut self, line_filter: &HashMap<PathBuf, HashSet<usize>>) {
        let original_count = self.files.iter().map(|f| f.issues.len()).sum::<usize>();

        self.files = self
            .files
            .drain(..)
            .filter_map(|mut file_result| {
                if let Some(modified_lines_in_file) = line_filter.get(&file_result.file) {
                    file_result
                        .issues
                        .retain(|issue| modified_lines_in_file.contains(&issue.line));
                    if !file_result.issues.is_empty() {
                        Some(file_result)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let filtered_count = self.files.iter().map(|f| f.issues.len()).sum::<usize>();
        self.total_issues = filtered_count;

        crate::log_info(&format!(
            "Filtered {} → {} issues (only modified lines)",
            original_count, filtered_count
        ));
    }

    pub fn filter_by_rule(&mut self, rule_name: &str) {
        let original_count = self.files.iter().map(|f| f.issues.len()).sum::<usize>();

        self.files = self
            .files
            .drain(..)
            .filter_map(|mut file_result| {
                file_result.issues.retain(|issue| issue.rule == rule_name);
                if !file_result.issues.is_empty() {
                    Some(file_result)
                } else {
                    None
                }
            })
            .collect();

        let filtered_count = self.files.iter().map(|f| f.issues.len()).sum::<usize>();
        self.total_issues = filtered_count;

        crate::log_info(&format!(
            "Rule filter {} → {} issues (rule: {})",
            original_count, filtered_count, rule_name
        ));
    }
}
