use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleAction, RuleDiagnostic, TextEdit, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct PreferConstRule;

inventory::submit!(RuleRegistration {
    name: "prefer-const",
    factory: |_| Arc::new(PreferConstRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "prefer-const",
        display_name: "Prefer Const",
        description: "Suggests using 'const' instead of 'let' when variables are never reassigned.",
        rule_type: RuleType::Ast,
        category: RuleCategory::Variables,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/prefer-const"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/use-const"),
        ..RuleMetadata::defaults()
    }
});

pub struct ConstState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub variable_name: String,
}

impl Rule for PreferConstRule {
    type State = ConstState;

    fn name(&self) -> &'static str {
        "prefer-const"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut collector = VariableCollector {
            let_declarations: HashMap::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut collector);

        let mut checker = ReassignmentChecker {
            reassigned: HashSet::new(),
        };
        ctx.program().visit_with(&mut checker);

        let mut states = Vec::new();

        for (name, (line, column, end_column)) in collector.let_declarations {
            if !checker.reassigned.contains(&name) {
                states.push(ConstState {
                    line,
                    start_col: column,
                    end_col: end_column,
                    variable_name: name,
                });
            }
        }

        states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            format!(
                "'{}' is never reassigned, use 'const' instead",
                state.variable_name
            ),
        )
    }

    fn is_fixable(&self) -> bool {
        true
    }

    fn action(&self, ctx: &RuleContext, state: &Self::State) -> Option<RuleAction> {
        let line = ctx.get_line(state.line)?;
        let let_start = line.find("let ")?;
        let let_end = let_start + 3;

        Some(RuleAction::quick_fix(
            "Change 'let' to 'const'",
            vec![TextEdit::replace_line_segment(
                state.line,
                let_start + 1,
                let_end + 1,
                "const",
            )],
        ))
    }
}

struct VariableCollector<'a> {
    let_declarations: HashMap<String, (usize, usize, usize)>,
    source: &'a str,
}

impl<'a> Visit for VariableCollector<'a> {
    fn visit_var_decl(&mut self, n: &VarDecl) {
        if matches!(n.kind, VarDeclKind::Let) {
            for decl in &n.decls {
                if let Pat::Ident(ident) = &decl.name {
                    let name = ident.id.sym.to_string();
                    let span = ident.span();
                    let (line, column, end_column) =
                        get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);
                    self.let_declarations
                        .insert(name, (line, column, end_column));
                }
            }
        }
        n.visit_children_with(self);
    }
}

struct ReassignmentChecker {
    reassigned: HashSet<String>,
}

impl Visit for ReassignmentChecker {
    fn visit_assign_expr(&mut self, n: &AssignExpr) {
        if let AssignTarget::Simple(SimpleAssignTarget::Ident(ident)) = &n.left {
            self.reassigned.insert(ident.id.sym.to_string());
        }
        n.visit_children_with(self);
    }

    fn visit_update_expr(&mut self, n: &UpdateExpr) {
        if let Expr::Ident(ident) = &*n.arg {
            self.reassigned.insert(ident.sym.to_string());
        }
        n.visit_children_with(self);
    }
}
