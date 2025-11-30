use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::utils::get_span_positions;
use crate::utils::is_ternary_expr;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoNestedTernaryRule;

inventory::submit!(RuleRegistration {
    name: "no-nested-ternary",
    factory: || Arc::new(NoNestedTernaryRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-nested-ternary",
        display_name: "No Nested Ternary",
        description:
            "Disallows nested ternary expressions. Nested ternaries are hard to read and should be replaced with if-else statements.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-nested-ternary"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-nested-ternary"),
    }
});

impl Rule for NoNestedTernaryRule {
    fn name(&self) -> &str {
        "no-nested-ternary"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = NestedTernaryVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct NestedTernaryVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for NestedTernaryVisitor<'a> {
    fn visit_cond_expr(&mut self, n: &CondExpr) {
        if is_ternary_expr(&n.cons) || is_ternary_expr(&n.alt) {
            let (line, column, end_column) =
                get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "no-nested-ternary".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: "Nested ternary expressions are not allowed. Use if-else statements for better readability.".to_string(),
                severity: Severity::Warning,
                line_text: None,
            });
        }
        n.visit_children_with(self);
    }
}
