use super::format_duration;
use colored::*;
use serde::Serialize;
use tscanner_diagnostics::{IssueRuleType, ScanResult, Severity};

#[derive(Clone, Default, Serialize)]
pub struct RulesBreakdown {
    pub builtin: usize,
    pub regex: usize,
    pub script: usize,
    pub ai: usize,
}

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
    pub rules_breakdown: RulesBreakdown,
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

pub fn compute_triggered_breakdown(result: &ScanResult) -> RulesBreakdown {
    let mut unique_rules: std::collections::HashMap<String, IssueRuleType> =
        std::collections::HashMap::new();

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
            rules_breakdown: stats.rules_breakdown.clone(),
        }
    }
}

pub fn render_summary(
    result: &ScanResult,
    stats: &SummaryStats,
    triggered_breakdown: &RulesBreakdown,
) {
    let files_with_issues = result.files.iter().filter(|f| !f.issues.is_empty()).count();

    println!("{}", "Summary:".cyan().bold());
    println!();
    println!(
        "  {} {} ({} errors, {} warnings)",
        "Issues:".dimmed(),
        stats.total_issues.to_string().cyan(),
        stats.error_count.to_string().red(),
        stats.warning_count.to_string().yellow()
    );

    let breakdown_parts: Vec<String> = [
        (triggered_breakdown.builtin, "builtin"),
        (triggered_breakdown.regex, "regex"),
        (triggered_breakdown.script, "script"),
        (triggered_breakdown.ai, "ai"),
    ]
    .iter()
    .filter(|(count, _)| *count > 0)
    .map(|(count, label)| format!("{} {}", count, label))
    .collect();
    let breakdown_str = if breakdown_parts.is_empty() {
        String::new()
    } else {
        format!(" ({})", breakdown_parts.join(", "))
    };
    println!(
        "  {} {}/{}{}",
        "Triggered rules:".dimmed(),
        stats.unique_rules_count.to_string().cyan(),
        stats.total_enabled_rules,
        breakdown_str
    );

    println!(
        "  {} {}/{}",
        "Files with issues:".dimmed(),
        files_with_issues.to_string().cyan(),
        result.total_files
    );

    println!(
        "  {} {}",
        "Duration:".dimmed(),
        format_duration(result.duration_ms)
    );
    println!();
}
