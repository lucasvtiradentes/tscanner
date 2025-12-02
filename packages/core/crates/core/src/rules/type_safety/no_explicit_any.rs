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
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-explicit-any"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-explicit-any"),
        allowed_options: &[],
    }
});

impl Rule for NoExplicitAnyRule {
    fn name(&self) -> &str {
        "no-explicit-any"
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
        let mut visitor = AnyTypeVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct AnyTypeVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for AnyTypeVisitor<'a> {
    fn visit_ts_keyword_type(&mut self, n: &TsKeywordType) {
        if matches!(n.kind, TsKeywordTypeKind::TsAnyKeyword) {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "no-explicit-any".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: "Found `: any` type annotation".to_string(),
                severity: Severity::Error,
                line_text: None,
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

                self.issues.push(Issue {
                    rule: "no-explicit-any".to_string(),
                    file: self.path.clone(),
                    line,
                    column,
                    end_column,
                    message: "Found `as any` type assertion".to_string(),
                    severity: Severity::Error,
                    line_text: None,
                });
            }
        }
        n.visit_children_with(self);
    }
}
