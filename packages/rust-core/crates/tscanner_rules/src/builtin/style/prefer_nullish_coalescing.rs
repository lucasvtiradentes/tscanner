use crate::metadata::RuleType;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};
use tscanner_diagnostics::{Issue, Severity};

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
        _file_source: crate::FileSource,
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

fn is_likely_fallback_value(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(_) => true,
        Expr::Array(arr) => arr.elems.is_empty(),
        Expr::Object(obj) => obj.props.is_empty(),
        Expr::Ident(_) => true,
        Expr::Member(_) => true,
        Expr::Call(_) => true,
        Expr::OptChain(_) => true,
        _ => false,
    }
}

fn is_likely_boolean_method(call: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call.callee {
        if let Expr::Member(member) = expr.as_ref() {
            if let MemberProp::Ident(ident) = &member.prop {
                let method_name = ident.sym.as_ref();
                return matches!(
                    method_name,
                    "startsWith"
                        | "endsWith"
                        | "includes"
                        | "has"
                        | "hasOwnProperty"
                        | "some"
                        | "every"
                        | "test"
                        | "match"
                        | "isArray"
                        | "isNaN"
                );
            }
        }
    }
    false
}

fn is_boolean_expression(expr: &Expr) -> bool {
    match expr {
        Expr::Unary(unary) if matches!(unary.op, UnaryOp::Bang) => true,
        Expr::Bin(bin)
            if matches!(
                bin.op,
                BinaryOp::EqEq
                    | BinaryOp::NotEq
                    | BinaryOp::EqEqEq
                    | BinaryOp::NotEqEq
                    | BinaryOp::Lt
                    | BinaryOp::LtEq
                    | BinaryOp::Gt
                    | BinaryOp::GtEq
            ) =>
        {
            true
        }
        Expr::Call(call) => is_likely_boolean_method(call),
        _ => false,
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
            let right_is_fallback = is_likely_fallback_value(&n.right);
            let left_is_boolean = is_boolean_expression(&n.left);

            if right_is_fallback && !left_is_boolean {
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
        }

        n.visit_children_with(self);
    }
}
