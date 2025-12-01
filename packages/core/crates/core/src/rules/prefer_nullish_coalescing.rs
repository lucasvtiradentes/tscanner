use super::metadata::RuleType;
use super::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::output::{Issue, Severity};
use crate::rule::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct PreferNullishCoalescingRule;

inventory::submit!(RuleRegistration {
    name: "prefer-nullish-coalescing",
    factory: |_| Arc::new(PreferNullishCoalescingRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "prefer-nullish-coalescing",
        display_name: "Prefer Nullish Coalescing",
        description: "Suggests using nullish coalescing (??) instead of logical OR (||) for default values. The || operator treats 0, \"\", and false as falsy, which may not be intended.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::Style,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/prefer-nullish-coalescing"),
        equivalent_biome_rule: None,
        allowed_options: &[],
    }
});

impl Rule for PreferNullishCoalescingRule {
    fn name(&self) -> &str {
        "prefer-nullish-coalescing"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = NullishCoalescingVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct NullishCoalescingVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for NullishCoalescingVisitor<'a> {
    fn visit_bin_expr(&mut self, n: &BinExpr) {
        if matches!(n.op, BinaryOp::LogicalOr) {
            let (line, column, end_column) =
                get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "prefer-nullish-coalescing".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: "Use nullish coalescing (??) instead of logical OR (||). The || operator treats 0, \"\", and false as falsy, while ?? only checks for null/undefined.".to_string(),
                severity: Severity::Warning,
                line_text: None,
            });
        }

        n.visit_children_with(self);
    }
}
