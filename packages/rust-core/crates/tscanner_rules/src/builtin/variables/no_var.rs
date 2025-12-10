use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleAction, RuleDiagnostic, TextEdit, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoVarRule;

inventory::submit!(RuleRegistration {
    name: "no-var",
    factory: |_| Arc::new(NoVarRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-var",
        display_name: "No Var",
        description: "Disallows the use of 'var' keyword. Use 'let' or 'const' instead for block-scoped variables.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::Variables,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-var"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-var"),
        ..RuleMetadata::defaults()
    }
});

pub struct VarState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

impl Rule for NoVarRule {
    type State = VarState;

    fn name(&self) -> &'static str {
        "no-var"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = NoVarVisitor {
            states: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            "Use 'let' or 'const' instead of 'var'",
        )
    }

    fn is_fixable(&self) -> bool {
        true
    }

    fn action(&self, ctx: &RuleContext, state: &Self::State) -> Option<RuleAction> {
        let line = ctx.get_line(state.line)?;
        let var_start = line.find("var ")?;
        let var_end = var_start + 3;

        Some(RuleAction::quick_fix(
            "Change 'var' to 'let'",
            vec![TextEdit::replace_line_segment(
                state.line,
                var_start + 1,
                var_end + 1,
                "let",
            )],
        ))
    }
}

struct NoVarVisitor<'a> {
    states: Vec<VarState>,
    source: &'a str,
}

impl<'a> Visit for NoVarVisitor<'a> {
    fn visit_var_decl(&mut self, n: &VarDecl) {
        if matches!(n.kind, VarDeclKind::Var) {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.states.push(VarState {
                line,
                start_col: column,
                end_col: end_column,
            });
        }
        n.visit_children_with(self);
    }
}
