use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NonNullAssertionMatch {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct NoNonNullAssertionRule;

inventory::submit!(RuleRegistration {
    name: "no-non-null-assertion",
    factory: |_| Arc::new(NoNonNullAssertionRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-non-null-assertion",
        display_name: "No Non-Null Assertion",
        description: "Disallows the non-null assertion operator (!). Use proper null checks or optional chaining instead.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-non-null-assertion"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-non-null-assertion"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoNonNullAssertionRule {
    type State = NonNullAssertionMatch;

    fn name(&self) -> &'static str {
        "no-non-null-assertion"
    }

    fn is_typescript_only(&self) -> bool {
        true
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = NonNullAssertionVisitor {
            matches: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.matches
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Avoid non-null assertion operator (!). Use proper null checks or optional chaining instead.".to_string(),
        )
    }
}

struct NonNullAssertionVisitor<'a> {
    matches: Vec<NonNullAssertionMatch>,
    source: &'a str,
}

impl<'a> Visit for NonNullAssertionVisitor<'a> {
    fn visit_ts_non_null_expr(&mut self, n: &TsNonNullExpr) {
        let (line, column, end_column) =
            get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

        self.matches.push(NonNullAssertionMatch {
            line,
            column,
            end_column,
        });

        n.visit_children_with(self);
    }
}
