use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct AliasImportState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub struct NoAliasImportsRule;

inventory::submit!(RuleRegistration {
    name: "no-alias-imports",
    factory: |_| Arc::new(NoAliasImportsRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-alias-imports",
        display_name: "No Alias Imports",
        description: "Disallows aliased imports (starting with @). Prefer relative imports.",
        rule_type: RuleType::Ast,
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoAliasImportsRule {
    type State = AliasImportState;

    fn name(&self) -> &'static str {
        "no-alias-imports"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = AliasImportVisitor {
            states: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            "Use relative imports instead of aliased imports".to_string(),
        )
    }
}

struct AliasImportVisitor<'a> {
    states: Vec<AliasImportState>,
    source: &'a str,
}

impl<'a> Visit for AliasImportVisitor<'a> {
    fn visit_import_decl(&mut self, n: &ImportDecl) {
        let span = n.src.span;
        let import_start = span.lo.0 as usize;
        let import_end = span.hi.0 as usize;

        if import_start < self.source.len() && import_end <= self.source.len() {
            let src_slice = &self.source[import_start..import_end];
            if src_slice
                .trim_matches('"')
                .trim_matches('\'')
                .starts_with('@')
            {
                let (line, column, end_column) =
                    get_span_positions(self.source, import_start, import_end);

                self.states.push(AliasImportState {
                    line,
                    start_col: column,
                    end_col: end_column,
                });
            }
        }
        n.visit_children_with(self);
    }
}
