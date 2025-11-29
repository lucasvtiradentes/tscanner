use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoFloatingPromisesRule;

inventory::submit!(RuleRegistration {
    name: "no-floating-promises",
    factory: || Arc::new(NoFloatingPromisesRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-floating-promises",
        display_name: "No Floating Promises",
        description: "Disallows floating promises (promises used as statements without await, .then(), or .catch()). Unhandled promises can lead to silent failures.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::BugPrevention,
    }
});

impl Rule for NoFloatingPromisesRule {
    fn name(&self) -> &str {
        "no-floating-promises"
    }

    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let mut visitor = FloatingPromiseVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct FloatingPromiseVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for FloatingPromiseVisitor<'a> {
    fn visit_expr_stmt(&mut self, n: &ExprStmt) {
        if let Expr::Call(call_expr) = n.expr.as_ref() {
            if !is_handled_promise_call(call_expr) {
                let span = call_expr.span();
                let (line, column, end_column) =
                    get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                self.issues.push(Issue {
                    rule: "no-floating-promises".to_string(),
                    file: self.path.clone(),
                    line,
                    column,
                    end_column,
                    message: "Promise-returning expression used without handling. Use await, .then(), .catch(), or assign to a variable.".to_string(),
                    severity: Severity::Warning,
                    line_text: None,
                });
            }
        }
        n.visit_children_with(self);
    }
}

fn is_handled_promise_call(call_expr: &CallExpr) -> bool {
    match &call_expr.callee {
        Callee::Expr(expr) => {
            if let Expr::Member(member_expr) = expr.as_ref() {
                if let MemberProp::Ident(ident) = &member_expr.prop {
                    let method_name = ident.sym.as_ref();
                    return method_name == "then"
                        || method_name == "catch"
                        || method_name == "finally";
                }
            }
            false
        }
        _ => false,
    }
}
