use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_line_col;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct PreferNullishCoalescingRule;

inventory::submit!(RuleRegistration {
    name: "prefer-nullish-coalescing",
    factory: || Arc::new(PreferNullishCoalescingRule),
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
    }
});

impl Rule for PreferNullishCoalescingRule {
    fn name(&self) -> &str {
        "prefer-nullish-coalescing"
    }

    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue> {
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
            let span_start = n.span.lo.0 as usize;
            let (line, column) = get_line_col(self.source, span_start);

            self.issues.push(Issue {
                rule: "prefer-nullish-coalescing".to_string(),
                file: self.path.clone(),
                line,
                column,
                message: "Use nullish coalescing (??) instead of logical OR (||). The || operator treats 0, \"\", and false as falsy, while ?? only checks for null/undefined.".to_string(),
                severity: Severity::Warning,
                line_text: None,
            });
        }

        n.visit_children_with(self);
    }
}
