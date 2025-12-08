use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct InferrableTypeMatch {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub var_name: String,
}

pub struct NoInferrableTypesRule;

inventory::submit!(RuleRegistration {
    name: "no-inferrable-types",
    factory: |_| Arc::new(NoInferrableTypesRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-inferrable-types",
        display_name: "No Inferrable Types",
        description: "Disallows explicit type annotations on variables initialized with literal values. TypeScript can infer these types automatically.",
        rule_type: RuleType::Ast,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-inferrable-types"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-inferrable-types"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoInferrableTypesRule {
    type State = InferrableTypeMatch;

    fn name(&self) -> &'static str {
        "no-inferrable-types"
    }

    fn is_typescript_only(&self) -> bool {
        true
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = InferrableTypesVisitor {
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
                "Type annotation on variable '{}' is redundant. TypeScript can infer this type from the literal value.",
                state.var_name
            ),
        )
    }
}

struct InferrableTypesVisitor<'a> {
    matches: Vec<InferrableTypeMatch>,
    source: &'a str,
}

impl<'a> Visit for InferrableTypesVisitor<'a> {
    fn visit_var_declarator(&mut self, n: &VarDeclarator) {
        if let Pat::Ident(ident) = &n.name {
            if let Some(type_ann) = &ident.type_ann {
                if let Some(init) = &n.init {
                    let should_report = match init.as_ref() {
                        Expr::Lit(lit) => match lit {
                            Lit::Num(_) => matches!(
                                &*type_ann.type_ann,
                                TsType::TsKeywordType(kw) if matches!(kw.kind, TsKeywordTypeKind::TsNumberKeyword)
                            ),
                            Lit::Str(_) => matches!(
                                &*type_ann.type_ann,
                                TsType::TsKeywordType(kw) if matches!(kw.kind, TsKeywordTypeKind::TsStringKeyword)
                            ),
                            Lit::Bool(_) => matches!(
                                &*type_ann.type_ann,
                                TsType::TsKeywordType(kw) if matches!(kw.kind, TsKeywordTypeKind::TsBooleanKeyword)
                            ),
                            _ => false,
                        },
                        _ => false,
                    };

                    if should_report {
                        let span = type_ann.span();
                        let (line, column, end_column) =
                            get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                        self.matches.push(InferrableTypeMatch {
                            line,
                            column,
                            end_column,
                            var_name: ident.id.sym.to_string(),
                        });
                    }
                }
            }
        }
        n.visit_children_with(self);
    }
}
