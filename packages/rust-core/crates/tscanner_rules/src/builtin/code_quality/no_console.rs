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

const DEFAULT_METHODS: &[&str] = &[
    "log",
    "warn",
    "error",
    "info",
    "debug",
    "trace",
    "table",
    "dir",
    "dirxml",
    "group",
    "groupCollapsed",
    "groupEnd",
    "time",
    "timeEnd",
    "timeLog",
    "assert",
    "count",
    "countReset",
    "clear",
    "profile",
    "profileEnd",
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoConsoleOptions {
    #[serde(default = "default_methods")]
    methods: Vec<String>,
}

fn default_methods() -> Vec<String> {
    DEFAULT_METHODS.iter().map(|s| (*s).to_string()).collect()
}

impl Default for NoConsoleOptions {
    fn default() -> Self {
        Self {
            methods: default_methods(),
        }
    }
}

pub struct ConsoleMatch {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub method: String,
}

pub struct NoConsoleRule {
    pattern: Regex,
}

impl NoConsoleRule {
    pub fn new(options: Option<&serde_json::Value>) -> Self {
        let methods = options
            .and_then(|v| serde_json::from_value::<NoConsoleOptions>(v.clone()).ok())
            .map(|o| o.methods)
            .unwrap_or_else(default_methods);

        let pattern_str = format!(r"console\.({})\s*\(", methods.join("|"));
        let pattern = Regex::new(&pattern_str)
            .unwrap_or_else(|_| Regex::new(r"console\.(log|warn|error|info|debug)\s*\(").unwrap());

        Self { pattern }
    }
}

inventory::submit!(RuleRegistration {
    name: "no-console",
    factory: |options| Arc::new(NoConsoleRule::new(options)),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-console",
        display_name: "No Console",
        description: "Disallow the use of console methods. Console statements should be removed before committing to production.",
        rule_type: RuleExecutionKind::Regex,
        category: RuleCategory::CodeQuality,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-console"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-console"),
        options: &[RuleOption {
            name: "methods",
            description: "Console methods to disallow",
            schema: RuleOptionSchema::Array {
                items: "string",
                default: DEFAULT_METHODS,
            },
        }],
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoConsoleRule {
    type State = ConsoleMatch;

    fn name(&self) -> &'static str {
        "no-console"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut matches = Vec::new();

        for (line_num, line) in ctx.source().lines().enumerate() {
            if let Some(mat) = self.pattern.find(line) {
                let method = line[mat.start()..mat.end()]
                    .trim_end_matches(['(', ' '])
                    .to_string();
                matches.push(ConsoleMatch {
                    line: line_num + 1,
                    start_col: mat.start() + 1,
                    end_col: mat.end(),
                    method,
                });
            }
        }

        matches
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            format!("Unexpected call to {}", state.method),
        )
    }
}
