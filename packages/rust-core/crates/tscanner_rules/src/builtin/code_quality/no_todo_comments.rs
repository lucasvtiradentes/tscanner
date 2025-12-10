use crate::context::RuleContext;
use crate::metadata::{
    RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration, RuleOption,
    RuleOptionSchema,
};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use regex::Regex;
use serde::Deserialize;
use std::sync::Arc;

const DEFAULT_KEYWORDS: &[&str] = &["TODO"];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoTodoCommentsOptions {
    #[serde(default = "default_keywords")]
    keywords: Vec<String>,
}

fn default_keywords() -> Vec<String> {
    DEFAULT_KEYWORDS.iter().map(|s| (*s).to_string()).collect()
}

impl Default for NoTodoCommentsOptions {
    fn default() -> Self {
        Self {
            keywords: default_keywords(),
        }
    }
}

pub struct TodoComment {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
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

        let pattern_str = format!(r"(?i)//\s*({})", keywords.join("|"));
        let pattern =
            Regex::new(&pattern_str).unwrap_or_else(|_| Regex::new(r"(?i)//\s*(TODO)").unwrap());

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
        description: "Detects TODO comments (case insensitive). Configure 'keywords' option to detect additional markers like FIXME, HACK, etc.",
        rule_type: RuleExecutionKind::Regex,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-warning-comments"),
        equivalent_biome_rule: None,
        options: &[RuleOption {
            name: "keywords",
            description: "Comment keywords to detect (case insensitive)",
            schema: RuleOptionSchema::Array {
                items: "string",
                default: DEFAULT_KEYWORDS,
            },
        }],
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoTodoCommentsRule {
    type State = TodoComment;

    fn name(&self) -> &'static str {
        "no-todo-comments"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut matches = Vec::new();

        for (line_num, line) in ctx.source().lines().enumerate() {
            if let Some(mat) = self.pattern.find(line) {
                matches.push(TodoComment {
                    line: line_num + 1,
                    start_col: mat.start() + 1,
                    end_col: mat.end() + 1,
                });
            }
        }

        matches
    }

    fn diagnostic(&self, _ctx: &RuleContext, _state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(_state.line, _state.start_col, _state.end_col),
            "Comment marker found. Consider creating an issue instead".to_string(),
        )
    }
}
