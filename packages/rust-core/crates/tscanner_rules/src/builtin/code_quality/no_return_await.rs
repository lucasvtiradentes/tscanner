use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct ReturnAwait {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct NoReturnAwaitRule;

inventory::submit!(RuleRegistration {
    name: "no-return-await",
    factory: |_| Arc::new(NoReturnAwaitRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-return-await",
        display_name: "No Return Await",
        description: "Disallows redundant 'return await' in async functions. The await is unnecessary since the function already returns a Promise.",
        rule_type: RuleType::Ast,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/return-await"),
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoReturnAwaitRule {
    type State = ReturnAwait;

    fn name(&self) -> &'static str {
        "no-return-await"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = ReturnAwaitVisitor {
            issues: Vec::new(),
            source: ctx.source(),
            in_async_context: false,
        };
        ctx.program().visit_with(&mut visitor);
        visitor.issues
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Redundant 'return await' found. Remove 'await' since async function already returns a Promise.".to_string(),
        )
    }
}

struct ReturnAwaitVisitor<'a> {
    issues: Vec<ReturnAwait>,
    source: &'a str,
    in_async_context: bool,
}

impl<'a> Visit for ReturnAwaitVisitor<'a> {
    fn visit_function(&mut self, n: &Function) {
        let was_async = self.in_async_context;
        self.in_async_context = n.is_async;
        n.visit_children_with(self);
        self.in_async_context = was_async;
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        let was_async = self.in_async_context;
        self.in_async_context = n.is_async;
        n.visit_children_with(self);
        self.in_async_context = was_async;
    }

    fn visit_return_stmt(&mut self, n: &ReturnStmt) {
        if self.in_async_context {
            if let Some(arg) = &n.arg {
                if let Expr::Await(await_expr) = arg.as_ref() {
                    let span = await_expr.span();
                    let (line, column, end_column) =
                        get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                    self.issues.push(ReturnAwait {
                        line,
                        column,
                        end_column,
                    });
                }
            }
        }
        n.visit_children_with(self);
    }
}
