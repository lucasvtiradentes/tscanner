use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoUnreachableCodeRule;

inventory::submit!(RuleRegistration {
    name: "no-unreachable-code",
    factory: |_| Arc::new(NoUnreachableCodeRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-unreachable-code",
        display_name: "No Unreachable Code",
        description: "Detects code after return, throw, break, or continue statements. This code will never execute.",
        rule_type: RuleType::Ast,
        category: RuleCategory::BugPrevention,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-unreachable"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-unreachable"),
        ..RuleMetadata::defaults()
    }
});

pub struct UnreachableCodeState {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

impl Rule for NoUnreachableCodeRule {
    type State = UnreachableCodeState;

    fn name(&self) -> &'static str {
        "no-unreachable-code"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = UnreachableCodeVisitor {
            states: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Unreachable code detected after return/throw/break/continue".to_string(),
        )
    }
}

struct UnreachableCodeVisitor<'a> {
    states: Vec<UnreachableCodeState>,
    source: &'a str,
}

impl<'a> UnreachableCodeVisitor<'a> {
    fn is_terminating_stmt(&self, stmt: &Stmt) -> bool {
        matches!(
            stmt,
            Stmt::Return(_) | Stmt::Throw(_) | Stmt::Break(_) | Stmt::Continue(_)
        )
    }

    fn check_block_statements(&mut self, stmts: &[Stmt]) {
        let mut found_terminator = false;

        for stmt in stmts {
            if found_terminator {
                let span = stmt.span();
                let (line, column, end_column) =
                    get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                self.states.push(UnreachableCodeState {
                    line,
                    column,
                    end_column,
                });
                break;
            }

            if self.is_terminating_stmt(stmt) {
                found_terminator = true;
            }
        }
    }
}

impl<'a> Visit for UnreachableCodeVisitor<'a> {
    fn visit_block_stmt(&mut self, n: &BlockStmt) {
        self.check_block_statements(&n.stmts);
        n.visit_children_with(self);
    }

    fn visit_switch_case(&mut self, n: &SwitchCase) {
        self.check_block_statements(&n.cons);
        n.visit_children_with(self);
    }
}
