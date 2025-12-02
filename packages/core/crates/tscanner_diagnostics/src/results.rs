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
    pub total_files: usize,
    pub cached_files: usize,
    pub scanned_files: usize,
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
                        .retain(|issue| modified_lines.contains(&issue.line));
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
}
