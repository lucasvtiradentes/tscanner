use crate::metadata::RuleType;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::traits::{Rule, RuleRegistration};
use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::Program;
use tscanner_diagnostics::{Issue, Severity};

const DEFAULT_KEYWORDS: [&str; 6] = ["TODO", "FIXME", "HACK", "XXX", "NOTE", "BUG"];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoTodoCommentsOptions {
    #[serde(default = "default_keywords")]
    keywords: Vec<String>,
}

fn default_keywords() -> Vec<String> {
    DEFAULT_KEYWORDS.iter().map(|s| s.to_string()).collect()
}

impl Default for NoTodoCommentsOptions {
    fn default() -> Self {
        Self {
            keywords: default_keywords(),
        }
    }
}

pub struct NoTodoCommentsRule {
    pattern: Regex,
}

impl NoTodoCommentsRule {
    pub fn new(options: Option<&serde_json::Value>) -> Self {
        let keywords = options
            .and_then(|v| serde_json::from_value::<NoTodoCommentsOptions>(v.clone()).ok())
            .map(|o| o.keywords)
            .unwrap_or_else(default_keywords);

        let pattern_str = format!(r"//\s*({})", keywords.join("|"));
        let pattern = Regex::new(&pattern_str)
            .unwrap_or_else(|_| Regex::new(r"//\s*(TODO|FIXME|HACK|XXX|NOTE|BUG)").unwrap());

        Self { pattern }
    }
}

inventory::submit!(RuleRegistration {
    name: "no-todo-comments",
    factory: |options| Arc::new(NoTodoCommentsRule::new(options)),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-todo-comments",
        display_name: "No TODO Comments",
        description: "Detects TODO, FIXME, and similar comment markers.",
        rule_type: RuleType::Regex,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-warning-comments"),
        equivalent_biome_rule: None,
        allowed_options: &["keywords"],
    }
});

impl Rule for NoTodoCommentsRule {
    fn name(&self) -> &str {
        "no-todo-comments"
    }

    fn check(
        &self,
        _program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::FileSource,
    ) -> Vec<Issue> {
        let mut issues = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            if let Some(mat) = self.pattern.find(line) {
                issues.push(Issue {
                    rule: self.name().to_string(),
                    file: path.to_path_buf(),
                    line: line_num + 1,
                    column: mat.start() + 1,
                    end_column: mat.end() + 1,
                    message: "Comment marker found. Consider creating an issue instead".to_string(),
                    severity: Severity::Warning,
                    line_text: None,
                });
            }
        }

        issues
    }
}
