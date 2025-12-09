use super::renderer::OutputRenderer;
use super::CheckContext;
use crate::shared::{
    JsonFileGroup, JsonIssue, JsonRuleGroup, JsonRuleIssue, JsonSummary, SummaryStats,
};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use tscanner_diagnostics::{GroupMode, ScanResult, Severity};

#[derive(Serialize)]
#[serde(untagged)]
enum JsonOutput {
    ByFile {
        files: Vec<JsonFileGroup>,
        summary: JsonSummary,
    },
    ByRule {
        rules: Vec<JsonRuleGroup>,
        summary: JsonSummary,
    },
}

pub struct JsonRenderer;

impl JsonRenderer {
    pub fn to_json_string(
        root: &Path,
        group_mode: &GroupMode,
        result: &ScanResult,
        stats: &SummaryStats,
    ) -> Option<String> {
        let output = match group_mode {
            GroupMode::File => Self::build_by_file(root, result, stats),
            GroupMode::Rule => Self::build_by_rule(root, result, stats),
        };
        serde_json::to_string_pretty(&output).ok()
    }

    fn build_by_file(root: &Path, result: &ScanResult, stats: &SummaryStats) -> JsonOutput {
        let files: Vec<JsonFileGroup> = result
            .files
            .iter()
            .filter(|f| !f.issues.is_empty())
            .map(|file_result| {
                let relative_path = pathdiff::diff_paths(&file_result.file, root)
                    .unwrap_or_else(|| file_result.file.clone());

                JsonFileGroup {
                    file: relative_path.display().to_string(),
                    issues: file_result
                        .issues
                        .iter()
                        .map(|issue| JsonIssue {
                            rule: issue.rule.clone(),
                            severity: match issue.severity {
                                Severity::Error => "error".to_string(),
                                Severity::Warning => "warning".to_string(),
                            },
                            line: issue.line,
                            column: issue.column,
                            message: issue.message.clone(),
                            line_text: issue.line_text.clone(),
                            rule_type: issue.rule_type,
                        })
                        .collect(),
                }
            })
            .collect();

        JsonOutput::ByFile {
            files,
            summary: JsonSummary::new(result, stats),
        }
    }

    fn build_by_rule(root: &Path, result: &ScanResult, stats: &SummaryStats) -> JsonOutput {
        let mut issues_by_rule: HashMap<String, Vec<JsonRuleIssue>> = HashMap::new();

        for file_result in &result.files {
            let relative_path = pathdiff::diff_paths(&file_result.file, root)
                .unwrap_or_else(|| file_result.file.clone());

            for issue in &file_result.issues {
                issues_by_rule
                    .entry(issue.rule.clone())
                    .or_default()
                    .push(JsonRuleIssue {
                        file: relative_path.display().to_string(),
                        line: issue.line,
                        column: issue.column,
                        message: issue.message.clone(),
                        severity: match issue.severity {
                            Severity::Error => "error".to_string(),
                            Severity::Warning => "warning".to_string(),
                        },
                        line_text: issue.line_text.clone(),
                        rule_type: issue.rule_type,
                    });
            }
        }

        let mut rules: Vec<JsonRuleGroup> = issues_by_rule
            .into_iter()
            .map(|(rule, issues)| JsonRuleGroup {
                count: issues.len(),
                rule,
                issues,
            })
            .collect();

        rules.sort_by(|a, b| a.rule.cmp(&b.rule));

        JsonOutput::ByRule {
            rules,
            summary: JsonSummary::new(result, stats),
        }
    }
}

impl OutputRenderer for JsonRenderer {
    fn render(&self, ctx: &CheckContext, result: &ScanResult, stats: &SummaryStats) {
        if let Some(json) = Self::to_json_string(&ctx.root, &ctx.group_mode, result, stats) {
            println!("{}", json);
        }
    }
}
