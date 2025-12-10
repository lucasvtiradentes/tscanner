use serde::Serialize;
use tscanner_types::IssueRuleType;

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
    pub infos: usize,
    pub hints: usize,
    pub triggered_rules: usize,
    pub triggered_rules_breakdown: RulesBreakdown,
    pub total_enabled_rules: usize,
    pub enabled_rules_breakdown: RulesBreakdown,
    pub duration_ms: u128,
}

pub struct IssuePart {
    pub count: usize,
    pub label: &'static str,
}

impl OutputSummary {
    pub fn issue_parts(&self) -> Vec<IssuePart> {
        let mut parts = Vec::new();
        if self.errors > 0 {
            parts.push(IssuePart {
                count: self.errors,
                label: "errors",
            });
        }
        if self.warnings > 0 {
            parts.push(IssuePart {
                count: self.warnings,
                label: "warnings",
            });
        }
        if self.infos > 0 {
            parts.push(IssuePart {
                count: self.infos,
                label: "infos",
            });
        }
        if self.hints > 0 {
            parts.push(IssuePart {
                count: self.hints,
                label: "hints",
            });
        }
        parts
    }

    pub fn format_issues_plain(&self) -> String {
        let parts = self.issue_parts();
        if parts.is_empty() {
            format!("{}", self.total_issues)
        } else {
            let breakdown: Vec<String> = parts
                .iter()
                .map(|p| format!("{} {}", p.count, p.label))
                .collect();
            format!("{} ({})", self.total_issues, breakdown.join(", "))
        }
    }

    pub fn rules_breakdown_parts(&self) -> Vec<(usize, &'static str)> {
        let breakdown = &self.triggered_rules_breakdown;
        [
            (breakdown.builtin, "builtin"),
            (breakdown.regex, "regex"),
            (breakdown.script, "script"),
            (breakdown.ai, "ai"),
        ]
        .into_iter()
        .filter(|(count, _)| *count > 0)
        .collect()
    }
}
