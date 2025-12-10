use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use crate::utils::is_ternary_expr;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NestedTernary {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct NoNestedTernaryRule;

inventory::submit!(RuleRegistration {
    name: "no-nested-ternary",
    factory: |_| Arc::new(NoNestedTernaryRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-nested-ternary",
        display_name: "No Nested Ternary",
        description:
            "Disallows nested ternary expressions. Nested ternaries are hard to read and should be replaced with if-else statements.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-nested-ternary"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-nested-ternary"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoNestedTernaryRule {
    type State = NestedTernary;

    fn name(&self) -> &'static str {
        "no-nested-ternary"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = NestedTernaryVisitor {
            issues: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.issues
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Nested ternary expressions are not allowed. Use if-else statements for better readability.".to_string(),
        )
    }
}

struct NestedTernaryVisitor<'a> {
    issues: Vec<NestedTernary>,
    source: &'a str,
}

impl<'a> Visit for NestedTernaryVisitor<'a> {
    fn visit_cond_expr(&mut self, n: &CondExpr) {
        if is_ternary_expr(&n.cons) || is_ternary_expr(&n.alt) {
            let (line, column, end_column) =
                get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

            self.issues.push(NestedTernary {
                line,
                column,
                end_column,
            });
        }
        n.visit_children_with(self);
    }
}
