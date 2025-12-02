use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::rules::traits::{Rule, RuleRegistration};
use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::Program;

const DEFAULT_METHODS: [&str; 21] = [
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
    DEFAULT_METHODS.iter().map(|s| s.to_string()).collect()
}

impl Default for NoConsoleOptions {
    fn default() -> Self {
        Self {
            methods: default_methods(),
        }
    }
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
        rule_type: RuleType::Regex,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-console"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-console"),
        allowed_options: &["methods"],
    }
});

impl Rule for NoConsoleRule {
    fn name(&self) -> &str {
        "no-console"
    }

    fn check(
        &self,
        _program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut issues = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            if let Some(mat) = self.pattern.find(line) {
                let method = line[mat.start()..mat.end()].trim_end_matches(['(', ' ']);
                issues.push(Issue {
                    rule: self.name().to_string(),
                    file: path.to_path_buf(),
                    line: line_num + 1,
                    column: mat.start() + 1,
                    end_column: mat.end(),
                    message: format!("Unexpected call to {}", method),
                    severity: Severity::Warning,
                    line_text: None,
                });
            }
        }

        issues
    }
}
