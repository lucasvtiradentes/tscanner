use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct OptionalChainMatch {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct PreferOptionalChainRule;

inventory::submit!(RuleRegistration {
    name: "prefer-optional-chain",
    factory: |_| Arc::new(PreferOptionalChainRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "prefer-optional-chain",
        display_name: "Prefer Optional Chain",
        description: "Suggests using optional chaining (?.) instead of logical AND (&&) chains for null checks.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::Style,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/prefer-optional-chain"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/use-optional-chain"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for PreferOptionalChainRule {
    type State = OptionalChainMatch;

    fn name(&self) -> &'static str {
        "prefer-optional-chain"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = OptionalChainVisitor {
            matches: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.matches
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Use optional chaining (?.) instead of logical AND (&&) for null checks.".to_string(),
        )
    }
}

struct OptionalChainVisitor<'a> {
    matches: Vec<OptionalChainMatch>,
    source: &'a str,
}

impl<'a> OptionalChainVisitor<'a> {
    fn get_ident_name(expr: &Expr) -> Option<&str> {
        if let Expr::Ident(ident) = expr {
            Some(ident.sym.as_ref())
        } else {
            None
        }
    }

    fn get_member_expr_object_name(expr: &Expr) -> Option<&str> {
        if let Expr::Member(member) = expr {
            if let MemberProp::Ident(_) = &member.prop {
                return Self::get_ident_name(&member.obj);
            }
        }
        None
    }
}

impl<'a> Visit for OptionalChainVisitor<'a> {
    fn visit_bin_expr(&mut self, n: &BinExpr) {
        if matches!(n.op, BinaryOp::LogicalAnd) {
            if let Some(left_name) = Self::get_ident_name(&n.left) {
                if let Some(right_obj_name) = Self::get_member_expr_object_name(&n.right) {
                    if left_name == right_obj_name {
                        let (line, column, end_column) = get_span_positions(
                            self.source,
                            n.span.lo.0 as usize,
                            n.span.hi.0 as usize,
                        );

                        self.matches.push(OptionalChainMatch {
                            line,
                            column,
                            end_column,
                        });
                    }
                }
            }
        }

        n.visit_children_with(self);
    }
}
