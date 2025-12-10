use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct EmptyClass {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct NoEmptyClassRule;

inventory::submit!(RuleRegistration {
    name: "no-empty-class",
    factory: |_| Arc::new(NoEmptyClassRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-empty-class",
        display_name: "No Empty Class",
        description: "Disallows empty classes without methods or properties.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoEmptyClassRule {
    type State = EmptyClass;

    fn name(&self) -> &'static str {
        "no-empty-class"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = EmptyClassVisitor {
            issues: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.issues
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Empty class without methods or properties".to_string(),
        )
    }
}

struct EmptyClassVisitor<'a> {
    issues: Vec<EmptyClass>,
    source: &'a str,
}

impl<'a> Visit for EmptyClassVisitor<'a> {
    fn visit_class(&mut self, n: &Class) {
        if n.body.is_empty() {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(EmptyClass {
                line,
                column,
                end_column,
            });
        }
        n.visit_children_with(self);
    }
}
