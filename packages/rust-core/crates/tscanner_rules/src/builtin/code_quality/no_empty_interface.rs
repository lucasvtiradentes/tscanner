use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct EmptyInterface {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct NoEmptyInterfaceRule;

inventory::submit!(RuleRegistration {
    name: "no-empty-interface",
    factory: |_| Arc::new(NoEmptyInterfaceRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-empty-interface",
        display_name: "No Empty Interface",
        description: "Disallows empty interface declarations. Empty interfaces are equivalent to {} and usually indicate incomplete code.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::CodeQuality,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-empty-interface"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-empty-interface"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoEmptyInterfaceRule {
    type State = EmptyInterface;

    fn name(&self) -> &'static str {
        "no-empty-interface"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = EmptyInterfaceVisitor {
            issues: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.issues
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Empty interface declaration".to_string(),
        )
    }
}

struct EmptyInterfaceVisitor<'a> {
    issues: Vec<EmptyInterface>,
    source: &'a str,
}

impl<'a> Visit for EmptyInterfaceVisitor<'a> {
    fn visit_ts_interface_decl(&mut self, n: &TsInterfaceDecl) {
        if n.body.body.is_empty() {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(EmptyInterface {
                line,
                column,
                end_column,
            });
        }
        n.visit_children_with(self);
    }
}
