use core::types::{ScanResult, Severity};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonSummary {
    pub total_files: usize,
    pub cached_files: usize,
    pub scanned_files: usize,
    pub total_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub duration_ms: u128,
    pub total_enabled_rules: usize,
}

pub struct SummaryStats {
    pub total_issues: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub unique_rules_count: usize,
    pub total_enabled_rules: usize,
}

impl SummaryStats {
    pub fn from_result(result: &ScanResult, total_enabled_rules: usize) -> Self {
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut unique_rules = std::collections::HashSet::new();

        for file_result in &result.files {
            for issue in &file_result.issues {
                match issue.severity {
                    Severity::Error => error_count += 1,
                    Severity::Warning => warning_count += 1,
                }
                unique_rules.insert(&issue.rule);
            }
        }

        Self {
            total_issues: error_count + warning_count,
            error_count,
            warning_count,
            unique_rules_count: unique_rules.len(),
            total_enabled_rules,
        }
    }
}

impl JsonSummary {
    pub fn new(result: &ScanResult, stats: &SummaryStats) -> Self {
        Self {
            total_files: result.total_files,
            cached_files: result.cached_files,
            scanned_files: result.scanned_files,
            total_issues: stats.total_issues,
            errors: stats.error_count,
            warnings: stats.warning_count,
            duration_ms: result.duration_ms,
            total_enabled_rules: stats.total_enabled_rules,
        }
    }
}
