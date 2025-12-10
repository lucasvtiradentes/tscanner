use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct InterfaceMatch {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub name: String,
}

pub struct PreferTypeOverInterfaceRule;

inventory::submit!(RuleRegistration {
    name: "prefer-type-over-interface",
    factory: |_| Arc::new(PreferTypeOverInterfaceRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "prefer-type-over-interface",
        display_name: "Prefer Type Over Interface",
        description: "Suggests using 'type' keyword instead of 'interface' for consistency. Type aliases are more flexible and composable.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::Style,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/consistent-type-definitions"),
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for PreferTypeOverInterfaceRule {
    type State = InterfaceMatch;

    fn name(&self) -> &'static str {
        "prefer-type-over-interface"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = InterfaceVisitor {
            matches: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.matches
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            format!("Prefer 'type' over 'interface' for '{}'", state.name),
        )
    }
}

struct InterfaceVisitor<'a> {
    matches: Vec<InterfaceMatch>,
    source: &'a str,
}

impl<'a> Visit for InterfaceVisitor<'a> {
    fn visit_ts_interface_decl(&mut self, n: &TsInterfaceDecl) {
        let span = n.span();
        let (line, column, end_column) =
            get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

        self.matches.push(InterfaceMatch {
            line,
            column,
            end_column,
            name: n.id.sym.to_string(),
        });

        n.visit_children_with(self);
    }
}
