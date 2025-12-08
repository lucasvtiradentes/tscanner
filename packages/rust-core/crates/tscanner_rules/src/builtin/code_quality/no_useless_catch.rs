use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct UselessCatch {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct NoUselessCatchRule;

inventory::submit!(RuleRegistration {
    name: "no-useless-catch",
    factory: |_| Arc::new(NoUselessCatchRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-useless-catch",
        display_name: "No Useless Catch",
        description: "Disallows catch blocks that only rethrow the caught error. Remove the try-catch or add meaningful error handling.",
        rule_type: RuleType::Ast,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-useless-catch"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-useless-catch"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoUselessCatchRule {
    type State = UselessCatch;

    fn name(&self) -> &'static str {
        "no-useless-catch"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = UselessCatchVisitor {
            issues: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.issues
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Useless catch block that only rethrows the error. Remove the try-catch or add meaningful error handling.".to_string(),
        )
    }
}

struct UselessCatchVisitor<'a> {
    issues: Vec<UselessCatch>,
    source: &'a str,
}

impl<'a> Visit for UselessCatchVisitor<'a> {
    fn visit_try_stmt(&mut self, n: &TryStmt) {
        if let Some(handler) = &n.handler {
            if handler.body.stmts.len() == 1 {
                if let Some(Stmt::Throw(throw_stmt)) = handler.body.stmts.first() {
                    if let Expr::Ident(throw_ident) = throw_stmt.arg.as_ref() {
                        if let Some(Pat::Ident(catch_param)) = &handler.param {
                            if throw_ident.sym == catch_param.sym {
                                let span = handler.span();
                                let (line, column, end_column) = get_span_positions(
                                    self.source,
                                    span.lo.0 as usize,
                                    span.hi.0 as usize,
                                );

                                self.issues.push(UselessCatch {
                                    line,
                                    column,
                                    end_column,
                                });
                            }
                        }
                    }
                }
            }
        }
        n.visit_children_with(self);
    }
}
