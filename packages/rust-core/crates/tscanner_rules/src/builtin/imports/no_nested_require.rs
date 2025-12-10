use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NestedRequireState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub struct NoNestedRequireRule;

inventory::submit!(RuleRegistration {
    name: "no-nested-require",
    factory: |_| Arc::new(NoNestedRequireRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-nested-require",
        display_name: "No Nested Require",
        description: "Disallows require() calls inside functions, blocks, or conditionals. Require statements should be at the top level for static analysis.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/global-require"),
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoNestedRequireRule {
    type State = NestedRequireState;

    fn name(&self) -> &'static str {
        "no-nested-require"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = NestedRequireVisitor {
            states: Vec::new(),
            source: ctx.source(),
            depth: 0,
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            "require() calls should not be nested inside functions or blocks. Move to top level for static analysis.".to_string(),
        )
    }
}

struct NestedRequireVisitor<'a> {
    states: Vec<NestedRequireState>,
    source: &'a str,
    depth: usize,
}

impl<'a> Visit for NestedRequireVisitor<'a> {
    fn visit_function(&mut self, n: &Function) {
        self.depth += 1;
        n.visit_children_with(self);
        self.depth -= 1;
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        self.depth += 1;
        n.visit_children_with(self);
        self.depth -= 1;
    }

    fn visit_block_stmt(&mut self, n: &BlockStmt) {
        self.depth += 1;
        n.visit_children_with(self);
        self.depth -= 1;
    }

    fn visit_if_stmt(&mut self, n: &IfStmt) {
        self.depth += 1;
        n.visit_children_with(self);
        self.depth -= 1;
    }

    fn visit_call_expr(&mut self, n: &CallExpr) {
        if self.depth > 0 {
            if let Callee::Expr(box_expr) = &n.callee {
                if let Expr::Ident(ident) = &**box_expr {
                    if ident.sym.as_ref() == "require" {
                        let (line, column, end_column) = get_span_positions(
                            self.source,
                            n.span.lo.0 as usize,
                            n.span.hi.0 as usize,
                        );

                        self.states.push(NestedRequireState {
                            line,
                            start_col: column,
                            end_col: end_column,
                        });
                    }
                }
            }
        }
        n.visit_children_with(self);
    }
}
