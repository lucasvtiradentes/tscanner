use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct SingleOrArrayUnionMatch {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub base_type: String,
}

pub struct NoSingleOrArrayUnionRule;

inventory::submit!(RuleRegistration {
    name: "no-single-or-array-union",
    factory: |_| Arc::new(NoSingleOrArrayUnionRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-single-or-array-union",
        display_name: "No Single Or Array Union",
        description: "Disallows union types that combine a type with its array form (e.g., `string | string[]`, `number | number[]`). Prefer using a consistent type to avoid handling multiple cases in function implementations.",
        rule_type: RuleType::Ast,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoSingleOrArrayUnionRule {
    type State = SingleOrArrayUnionMatch;

    fn name(&self) -> &'static str {
        "no-single-or-array-union"
    }

    fn is_typescript_only(&self) -> bool {
        true
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = SingleOrArrayUnionVisitor {
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
                "Avoid union of '{}' with '{}[]'. Use consistent type to avoid multiple code paths.",
                state.base_type, state.base_type
            ),
        )
    }
}

struct SingleOrArrayUnionVisitor<'a> {
    matches: Vec<SingleOrArrayUnionMatch>,
    source: &'a str,
}

fn get_type_key(ts_type: &TsType) -> Option<String> {
    match ts_type {
        TsType::TsKeywordType(kw) => {
            let name = match kw.kind {
                TsKeywordTypeKind::TsStringKeyword => "string",
                TsKeywordTypeKind::TsNumberKeyword => "number",
                TsKeywordTypeKind::TsBooleanKeyword => "boolean",
                TsKeywordTypeKind::TsBigIntKeyword => "bigint",
                TsKeywordTypeKind::TsSymbolKeyword => "symbol",
                TsKeywordTypeKind::TsObjectKeyword => "object",
                _ => return None,
            };
            Some(name.to_string())
        }
        TsType::TsTypeRef(type_ref) => {
            if let TsEntityName::Ident(ident) = &type_ref.type_name {
                Some(ident.sym.to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn is_array_of(ts_type: &TsType, base_type_key: &str) -> bool {
    if let TsType::TsArrayType(arr) = ts_type {
        if let Some(elem_key) = get_type_key(&arr.elem_type) {
            return elem_key == base_type_key;
        }
    }
    false
}

impl<'a> Visit for SingleOrArrayUnionVisitor<'a> {
    fn visit_ts_union_type(&mut self, n: &TsUnionType) {
        let types: Vec<&TsType> = n.types.iter().map(|t| t.as_ref()).collect();

        for base_type in &types {
            if let Some(base_key) = get_type_key(base_type) {
                for other_type in &types {
                    if is_array_of(other_type, &base_key) {
                        let span = n.span();
                        let (line, column, end_column) =
                            get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                        self.matches.push(SingleOrArrayUnionMatch {
                            line,
                            column,
                            end_column,
                            base_type: base_key.clone(),
                        });

                        n.visit_children_with(self);
                        return;
                    }
                }
            }
        }

        n.visit_children_with(self);
    }
}
