use crate::context::RuleContext;
use crate::metadata::RuleExecutionKind;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct RelativeImportState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub struct NoRelativeImportsRule;

inventory::submit!(RuleRegistration {
    name: "no-relative-imports",
    factory: |_| Arc::new(NoRelativeImportsRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-relative-imports",
        display_name: "No Relative Imports",
        description: "Detects relative imports (starting with './' or '../'). Prefer absolute imports with @ prefix for better maintainability.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoRelativeImportsRule {
    type State = RelativeImportState;

    fn name(&self) -> &'static str {
        "no-relative-imports"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = RelativeImportVisitor {
            states: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            "Use absolute imports with @ prefix instead of relative imports".to_string(),
        )
    }
}

struct RelativeImportVisitor<'a> {
    states: Vec<RelativeImportState>,
    source: &'a str,
}

impl<'a> Visit for RelativeImportVisitor<'a> {
    fn visit_import_decl(&mut self, n: &ImportDecl) {
        let span = n.src.span;
        let import_start = span.lo.0 as usize;
        let import_end = span.hi.0 as usize;

        if import_start < self.source.len() && import_end <= self.source.len() {
            let src_slice = &self.source[import_start..import_end];
            if src_slice
                .trim_matches('"')
                .trim_matches('\'')
                .starts_with('.')
            {
                let (line, column, end_column) =
                    get_span_positions(self.source, import_start, import_end);

                self.states.push(RelativeImportState {
                    line,
                    start_col: column,
                    end_col: end_column,
                });
            }
        }
        n.visit_children_with(self);
    }
}
