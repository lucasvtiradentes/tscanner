use serde::Serialize;
use tscanner_diagnostics::{ScanResult, Severity};

#[derive(Clone, Default, Serialize)]
pub struct RulesBreakdown {
    pub builtin: usize,
    pub regex: usize,
    pub script: usize,
    pub ai: usize,
}

pub struct SummaryStats {
    pub total_issues: usize,
    pub error_count: usize,
    pub warning_count: usize,
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
            rules_breakdown,
        }
    }
}
