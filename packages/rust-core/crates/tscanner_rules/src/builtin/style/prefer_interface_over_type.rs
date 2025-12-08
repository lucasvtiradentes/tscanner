use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct TypeAliasMatch {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct PreferInterfaceOverTypeRule;

inventory::submit!(RuleRegistration {
    name: "prefer-interface-over-type",
    factory: |_| Arc::new(PreferInterfaceOverTypeRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "prefer-interface-over-type",
        display_name: "Prefer Interface Over Type",
        description: "Suggests using 'interface' keyword instead of 'type' for consistency.",
        rule_type: RuleType::Ast,
        category: RuleCategory::Style,
        typescript_only: true,
        equivalent_eslint_rule: Some(
            "https://typescript-eslint.io/rules/consistent-type-definitions"
        ),
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for PreferInterfaceOverTypeRule {
    type State = TypeAliasMatch;

    fn name(&self) -> &'static str {
        "prefer-interface-over-type"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = TypeAliasVisitor {
            matches: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.matches
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Use 'interface' instead of 'type' for object types".to_string(),
        )
    }
}

struct TypeAliasVisitor<'a> {
    matches: Vec<TypeAliasMatch>,
    source: &'a str,
}

impl<'a> Visit for TypeAliasVisitor<'a> {
    fn visit_ts_type_alias_decl(&mut self, n: &TsTypeAliasDecl) {
        if let TsType::TsTypeLit(_) = n.type_ann.as_ref() {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.matches.push(TypeAliasMatch {
                line,
                column,
                end_column,
            });
        }
        n.visit_children_with(self);
    }
}
