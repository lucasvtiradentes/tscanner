use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoConstantConditionRule;

inventory::submit!(RuleRegistration {
    name: "no-constant-condition",
    factory: |_| Arc::new(NoConstantConditionRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-constant-condition",
        display_name: "No Constant Condition",
        description: "Disallows constant expressions in conditions (if/while/for/ternary). Likely a programming error.",
        rule_type: RuleType::Ast,
        category: RuleCategory::BugPrevention,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-constant-condition"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-constant-condition"),
        ..RuleMetadata::defaults()
    }
});

pub struct ConstantConditionState {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub context: String,
}

impl Rule for NoConstantConditionRule {
    type State = ConstantConditionState;

    fn name(&self) -> &'static str {
        "no-constant-condition"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = ConstantConditionVisitor {
            states: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            format!("Constant condition in {}", state.context),
        )
    }
}

struct ConstantConditionVisitor<'a> {
    states: Vec<ConstantConditionState>,
    source: &'a str,
}

impl<'a> ConstantConditionVisitor<'a> {
    fn is_constant(expr: &Expr) -> bool {
        match expr {
            Expr::Lit(Lit::Bool(_)) => true,
            Expr::Lit(Lit::Num(_)) => true,
            Expr::Lit(Lit::Str(_)) => true,
            Expr::Lit(Lit::Null(_)) => true,
            Expr::Unary(unary)
                if matches!(unary.op, UnaryOp::Bang | UnaryOp::Minus | UnaryOp::Plus) =>
            {
                Self::is_constant(&unary.arg)
            }
            _ => false,
        }
    }

    fn check_condition(&mut self, test: &Expr, context: &str) {
        if Self::is_constant(test) {
            let span = test.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.states.push(ConstantConditionState {
                line,
                column,
                end_column,
                context: context.to_string(),
            });
        }
    }
}

impl<'a> Visit for ConstantConditionVisitor<'a> {
    fn visit_if_stmt(&mut self, n: &IfStmt) {
        self.check_condition(&n.test, "if statement");
        n.visit_children_with(self);
    }

    fn visit_while_stmt(&mut self, n: &WhileStmt) {
        self.check_condition(&n.test, "while loop");
        n.visit_children_with(self);
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileStmt) {
        self.check_condition(&n.test, "do-while loop");
        n.visit_children_with(self);
    }

    fn visit_for_stmt(&mut self, n: &ForStmt) {
        if let Some(test) = &n.test {
            self.check_condition(test, "for loop");
        }
        n.visit_children_with(self);
    }

    fn visit_cond_expr(&mut self, n: &CondExpr) {
        self.check_condition(&n.test, "ternary expression");
        n.visit_children_with(self);
    }
}
