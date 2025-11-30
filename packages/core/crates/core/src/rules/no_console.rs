use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use regex::Regex;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::Program;

pub struct NoConsoleRule;

inventory::submit!(RuleRegistration {
    name: "no-console",
    factory: || Arc::new(NoConsoleRule),
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
        let regex = Regex::new(r"console\.(log|warn|error|info|debug|trace|table|dir|dirxml|group|groupCollapsed|groupEnd|time|timeEnd|timeLog|assert|count|countReset|clear|profile|profileEnd)\s*\(").unwrap();
        let mut issues = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            if let Some(mat) = regex.find(line) {
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
