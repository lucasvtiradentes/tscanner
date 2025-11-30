use super::renderer::OutputRenderer;
use super::CheckContext;
use crate::shared::{
    JsonFileGroup, JsonIssue, JsonRuleGroup, JsonRuleIssue, JsonSummary, SummaryStats,
};
use cli::GroupMode;
use core::types::{ScanResult, Severity};
use serde::Serialize;
use std::collections::HashMap;

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

impl OutputRenderer for JsonRenderer {
    fn render(&self, ctx: &CheckContext, result: &ScanResult, stats: &SummaryStats) {
        let output = match ctx.group_mode {
            GroupMode::File => self.render_by_file(ctx, result, stats),
            GroupMode::Rule => self.render_by_rule(ctx, result, stats),
        };

        if let Ok(json) = serde_json::to_string_pretty(&output) {
            println!("{}", json);
        }
    }
}

impl JsonRenderer {
    fn render_by_file(
        &self,
        ctx: &CheckContext,
        result: &ScanResult,
        stats: &SummaryStats,
    ) -> JsonOutput {
        let files: Vec<JsonFileGroup> = result
            .files
            .iter()
            .filter(|f| !f.issues.is_empty())
            .map(|file_result| {
                let relative_path = pathdiff::diff_paths(&file_result.file, &ctx.root)
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

    fn render_by_rule(
        &self,
        ctx: &CheckContext,
        result: &ScanResult,
        stats: &SummaryStats,
    ) -> JsonOutput {
        let mut issues_by_rule: HashMap<String, Vec<JsonRuleIssue>> = HashMap::new();

        for file_result in &result.files {
            let relative_path = pathdiff::diff_paths(&file_result.file, &ctx.root)
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
