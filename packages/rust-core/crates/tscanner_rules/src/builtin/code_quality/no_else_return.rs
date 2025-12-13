use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct ElseReturn {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct NoElseReturnRule;

inventory::submit!(RuleRegistration {
    name: "no-else-return",
    factory: |_| Arc::new(NoElseReturnRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-else-return",
        display_name: "No Else Return",
        description: "Disallows else blocks after return statements. The else is unnecessary since the function already returned.",
        rule_type: RuleType::Ast,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-else-return"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-useless-else"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoElseReturnRule {
    type State = ElseReturn;

    fn name(&self) -> &'static str {
        "no-else-return"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = ElseReturnVisitor {
            issues: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.issues
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Unnecessary 'else' after 'return'. Remove the 'else' block since the function already returned.".to_string(),
        )
    }
}

struct ElseReturnVisitor<'a> {
    issues: Vec<ElseReturn>,
    source: &'a str,
}

impl<'a> ElseReturnVisitor<'a> {
    fn stmt_contains_return(&self, stmt: &Stmt) -> bool {
        let mut collector = ReturnCollector { has_return: false };
        stmt.visit_with(&mut collector);
        collector.has_return
    }
}

impl<'a> Visit for ElseReturnVisitor<'a> {
    fn visit_if_stmt(&mut self, n: &IfStmt) {
        let cons_has_return = self.stmt_contains_return(&n.cons);

        if cons_has_return && n.alt.is_some() {
            if let Some(alt) = &n.alt {
                let span = alt.span();
                let (line, column, end_column) =
                    get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                self.issues.push(ElseReturn {
                    line,
                    column,
                    end_column,
                });
            }
        }

        n.visit_children_with(self);
    }
}

struct ReturnCollector {
    has_return: bool,
}

impl Visit for ReturnCollector {
    fn visit_return_stmt(&mut self, _n: &ReturnStmt) {
        self.has_return = true;
    }

    fn visit_function(&mut self, _n: &Function) {}

    fn visit_arrow_expr(&mut self, _n: &ArrowExpr) {}
}
