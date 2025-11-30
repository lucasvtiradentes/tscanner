use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

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
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
        allowed_options: &[],
    }
});

impl Rule for NoSingleOrArrayUnionRule {
    fn name(&self) -> &str {
        "no-single-or-array-union"
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
        let mut visitor = SingleOrArrayUnionVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct SingleOrArrayUnionVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
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

                        self.issues.push(Issue {
                            rule: "no-single-or-array-union".to_string(),
                            file: self.path.clone(),
                            line,
                            column,
                            end_column,
                            message: format!(
                                "Avoid union of '{}' with '{}[]'. Use consistent type to avoid multiple code paths.",
                                base_key, base_key
                            ),
                            severity: Severity::Warning,
                            line_text: None,
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
