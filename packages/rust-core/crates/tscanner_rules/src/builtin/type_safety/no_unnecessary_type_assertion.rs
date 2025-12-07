use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct UnnecessaryTypeAssertionMatch {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub literal_type: String,
    pub asserted_type: String,
}

pub struct NoUnnecessaryTypeAssertionRule;

inventory::submit!(RuleRegistration {
    name: "no-unnecessary-type-assertion",
    factory: |_| Arc::new(NoUnnecessaryTypeAssertionRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-unnecessary-type-assertion",
        display_name: "No Unnecessary Type Assertion",
        description: "Disallows type assertions on values that are already of the asserted type (e.g., \"hello\" as string, 123 as number).",
        rule_type: RuleType::Ast,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-unnecessary-type-assertion"),
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoUnnecessaryTypeAssertionRule {
    type State = UnnecessaryTypeAssertionMatch;

    fn name(&self) -> &'static str {
        "no-unnecessary-type-assertion"
    }

    fn is_typescript_only(&self) -> bool {
        true
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = UnnecessaryTypeAssertionVisitor {
            matches: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.matches
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            format!(
                "Unnecessary type assertion: {} is already of type {}",
                state.literal_type, state.asserted_type
            ),
        )
    }
}

struct UnnecessaryTypeAssertionVisitor<'a> {
    matches: Vec<UnnecessaryTypeAssertionMatch>,
    source: &'a str,
}

impl<'a> Visit for UnnecessaryTypeAssertionVisitor<'a> {
    fn visit_ts_as_expr(&mut self, n: &TsAsExpr) {
        if let Some((literal_type, asserted_type)) = check_unnecessary_assertion(n) {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.matches.push(UnnecessaryTypeAssertionMatch {
                line,
                column,
                end_column,
                literal_type: literal_type.to_string(),
                asserted_type: asserted_type.to_string(),
            });
        }

        n.visit_children_with(self);
    }
}

fn check_unnecessary_assertion(n: &TsAsExpr) -> Option<(&'static str, &'static str)> {
    if let TsType::TsKeywordType(kw) = &*n.type_ann {
        match kw.kind {
            TsKeywordTypeKind::TsStringKeyword => {
                if matches!(&*n.expr, Expr::Lit(Lit::Str(_))) {
                    return Some(("string literal", "string"));
                }
            }
            TsKeywordTypeKind::TsNumberKeyword => {
                if matches!(&*n.expr, Expr::Lit(Lit::Num(_))) {
                    return Some(("number literal", "number"));
                }
            }
            TsKeywordTypeKind::TsBooleanKeyword => {
                if matches!(&*n.expr, Expr::Lit(Lit::Bool(_))) {
                    return Some(("boolean literal", "boolean"));
                }
            }
            _ => {}
        }
    }
    None
}
