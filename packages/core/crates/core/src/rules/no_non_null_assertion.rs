use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoNonNullAssertionRule;

inventory::submit!(RuleRegistration {
    name: "no-non-null-assertion",
    factory: || Arc::new(NoNonNullAssertionRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-non-null-assertion",
        display_name: "No Non-Null Assertion",
        description: "Disallows the non-null assertion operator (!). Use proper null checks or optional chaining instead.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-non-null-assertion"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-non-null-assertion"),
    }
});

impl Rule for NoNonNullAssertionRule {
    fn name(&self) -> &str {
        "no-non-null-assertion"
    }

    fn is_typescript_only(&self) -> bool {
        true
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::file_source::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = NonNullAssertionVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct NonNullAssertionVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for NonNullAssertionVisitor<'a> {
    fn visit_ts_non_null_expr(&mut self, n: &TsNonNullExpr) {
        let (line, column, end_column) =
            get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

        self.issues.push(Issue {
            rule: "no-non-null-assertion".to_string(),
            file: self.path.clone(),
            line,
            column,
            end_column,
            message: "Avoid non-null assertion operator (!). Use proper null checks or optional chaining instead.".to_string(),
            severity: Severity::Warning,
            line_text: None,
        });

        n.visit_children_with(self);
    }
}
