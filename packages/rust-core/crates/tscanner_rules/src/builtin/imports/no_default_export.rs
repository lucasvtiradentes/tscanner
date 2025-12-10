use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct DefaultExportState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub struct NoDefaultExportRule;

inventory::submit!(RuleRegistration {
    name: "no-default-export",
    factory: |_| Arc::new(NoDefaultExportRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-default-export",
        display_name: "No Default Export",
        description: "Disallows default exports. Named exports are preferred for better refactoring support and explicit imports.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-default-export"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoDefaultExportRule {
    type State = DefaultExportState;

    fn name(&self) -> &'static str {
        "no-default-export"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = DefaultExportVisitor {
            states: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            "Avoid default exports. Use named exports for better refactoring support.".to_string(),
        )
    }
}

struct DefaultExportVisitor<'a> {
    states: Vec<DefaultExportState>,
    source: &'a str,
}

impl<'a> Visit for DefaultExportVisitor<'a> {
    fn visit_export_default_decl(&mut self, n: &ExportDefaultDecl) {
        let (line, column, end_column) =
            get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

        self.states.push(DefaultExportState {
            line,
            start_col: column,
            end_col: end_column,
        });

        n.visit_children_with(self);
    }

    fn visit_export_default_expr(&mut self, n: &ExportDefaultExpr) {
        let (line, column, end_column) =
            get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

        self.states.push(DefaultExportState {
            line,
            start_col: column,
            end_col: end_column,
        });

        n.visit_children_with(self);
    }
}
