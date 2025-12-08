use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct ExplicitAnyMatch {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub message: String,
}

pub struct NoExplicitAnyRule;

inventory::submit!(RuleRegistration {
    name: "no-explicit-any",
    factory: |_| Arc::new(NoExplicitAnyRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-explicit-any",
        display_name: "No Explicit Any",
        description: "Detects usage of TypeScript 'any' type (`: any` and `as any`). Using 'any' defeats the purpose of TypeScript's type system.",
        rule_type: RuleType::Ast,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-explicit-any"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-explicit-any"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoExplicitAnyRule {
    type State = ExplicitAnyMatch;

    fn name(&self) -> &'static str {
        "no-explicit-any"
    }

    fn is_typescript_only(&self) -> bool {
        true
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = AnyTypeVisitor {
            matches: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.matches
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            state.message.clone(),
        )
    }
}

struct AnyTypeVisitor<'a> {
    matches: Vec<ExplicitAnyMatch>,
    source: &'a str,
}

impl<'a> Visit for AnyTypeVisitor<'a> {
    fn visit_ts_keyword_type(&mut self, n: &TsKeywordType) {
        if matches!(n.kind, TsKeywordTypeKind::TsAnyKeyword) {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.matches.push(ExplicitAnyMatch {
                line,
                column,
                end_column,
                message: "Found \": any\" type annotation".to_string(),
            });
        }
        n.visit_children_with(self);
    }

    fn visit_ts_as_expr(&mut self, n: &TsAsExpr) {
        if let TsType::TsKeywordType(ref kw) = &*n.type_ann {
            if matches!(kw.kind, TsKeywordTypeKind::TsAnyKeyword) {
                let span = kw.span();
                let (line, column, end_column) =
                    get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                self.matches.push(ExplicitAnyMatch {
                    line,
                    column,
                    end_column,
                    message: "Found \"as any\" type assertion".to_string(),
                });
                n.expr.visit_with(self);
                return;
            }
        }
        n.visit_children_with(self);
    }
}
