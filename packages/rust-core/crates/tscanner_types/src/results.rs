use crate::enums::{IssueRuleType, Severity};
use crate::Issue;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResult {
    pub file: PathBuf,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentScanResult {
    pub file: PathBuf,
    pub issues: Vec<Issue>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub related_files: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub files: Vec<FileResult>,
    pub total_issues: usize,
    pub duration_ms: u128,
    pub regular_rules_duration_ms: u128,
    pub ai_rules_duration_ms: u128,
    pub total_files: usize,
    pub cached_files: usize,
    pub scanned_files: usize,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub notes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<String>,
}

impl ScanResult {
    pub fn filter_by_modified_lines(&mut self, line_filter: &HashMap<PathBuf, HashSet<usize>>) {
        self.files = self
            .files
            .drain(..)
            .filter_map(|mut file_result| {
                if let Some(modified_lines) = line_filter.get(&file_result.file) {
                    file_result
                        .issues
                        .retain(|issue| issue.is_ai() || modified_lines.contains(&issue.line));
                    if !file_result.issues.is_empty() {
                        Some(file_result)
                    } else {
                        None
                    }
                } else {
                    let has_ai_issues = file_result.issues.iter().any(|i| i.is_ai());
                    if has_ai_issues {
                        file_result.issues.retain(|issue| issue.is_ai());
                        Some(file_result)
                    } else {
                        None
                    }
                }
            })
            .collect();

        self.total_issues = self.files.iter().map(|f| f.issues.len()).sum();
    }

    pub fn filter_by_rule(&mut self, rule_name: &str) {
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

        self.total_issues = self.files.iter().map(|f| f.issues.len()).sum();
    }

    pub fn filter_by_severity(&mut self, min_severity: Severity) {
        let min_level = severity_level(min_severity);
        self.files = self
            .files
            .drain(..)
            .filter_map(|mut file_result| {
                file_result
                    .issues
                    .retain(|issue| severity_level(issue.severity) <= min_level);
                if !file_result.issues.is_empty() {
                    Some(file_result)
                } else {
                    None
                }
            })
            .collect();

        self.total_issues = self.files.iter().map(|f| f.issues.len()).sum();
    }

    pub fn filter_by_rule_type(&mut self, rule_type: IssueRuleType) {
        self.files = self
            .files
            .drain(..)
            .filter_map(|mut file_result| {
                file_result
                    .issues
                    .retain(|issue| issue.rule_type == rule_type);
                if !file_result.issues.is_empty() {
                    Some(file_result)
                } else {
                    None
                }
            })
            .collect();

        self.total_issues = self.files.iter().map(|f| f.issues.len()).sum();
    }
}

fn severity_level(severity: Severity) -> u8 {
    match severity {
        Severity::Error => 0,
        Severity::Warning => 1,
        Severity::Info => 2,
        Severity::Hint => 3,
    }
}
