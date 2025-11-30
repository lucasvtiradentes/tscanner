use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

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
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-unnecessary-type-assertion"),
        equivalent_biome_rule: None,
        allowed_options: &[],
    }
});

impl Rule for NoUnnecessaryTypeAssertionRule {
    fn name(&self) -> &str {
        "no-unnecessary-type-assertion"
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
        let mut visitor = UnnecessaryTypeAssertionVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct UnnecessaryTypeAssertionVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for UnnecessaryTypeAssertionVisitor<'a> {
    fn visit_ts_as_expr(&mut self, n: &TsAsExpr) {
        if let Some((literal_type, asserted_type)) = check_unnecessary_assertion(n) {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "no-unnecessary-type-assertion".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: format!(
                    "Unnecessary type assertion: {} is already of type {}",
                    literal_type, asserted_type
                ),
                severity: Severity::Warning,
                line_text: None,
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
