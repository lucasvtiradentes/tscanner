use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct DynamicImportState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub struct NoDynamicImportRule;

inventory::submit!(RuleRegistration {
    name: "no-dynamic-import",
    factory: |_| Arc::new(NoDynamicImportRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-dynamic-import",
        display_name: "No Dynamic Import",
        description: "Disallows dynamic import() expressions. Dynamic imports make static analysis harder and can impact bundle optimization.",
        rule_type: RuleType::Ast,
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoDynamicImportRule {
    type State = DynamicImportState;

    fn name(&self) -> &'static str {
        "no-dynamic-import"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = DynamicImportVisitor {
            states: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            "Dynamic import() is not allowed. Use static imports at the top of the file."
                .to_string(),
        )
    }
}

struct DynamicImportVisitor<'a> {
    states: Vec<DynamicImportState>,
    source: &'a str,
}

impl<'a> Visit for DynamicImportVisitor<'a> {
    fn visit_call_expr(&mut self, n: &CallExpr) {
        if let Callee::Import(_) = &n.callee {
            let (line, column, end_column) =
                get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

            self.states.push(DynamicImportState {
                line,
                start_col: column,
                end_col: end_column,
            });
        }
        n.visit_children_with(self);
    }
}
