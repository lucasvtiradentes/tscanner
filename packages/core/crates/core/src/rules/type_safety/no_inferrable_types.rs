use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::rules::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

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
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-inferrable-types"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-inferrable-types"),
        allowed_options: &[],
    }
});

impl Rule for NoInferrableTypesRule {
    fn name(&self) -> &str {
        "no-inferrable-types"
    }

    fn is_typescript_only(&self) -> bool {
        true
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = InferrableTypesVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct InferrableTypesVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
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

                        self.issues.push(Issue {
                            rule: "no-inferrable-types".to_string(),
                            file: self.path.clone(),
                            line,
                            column,
                            end_column,
                            message: format!(
                                "Type annotation on variable '{}' is redundant. TypeScript can infer this type from the literal value.",
                                ident.id.sym
                            ),
                            severity: Severity::Warning,
                            line_text: None,
                        });
                    }
                }
            }
        }
        n.visit_children_with(self);
    }
}
