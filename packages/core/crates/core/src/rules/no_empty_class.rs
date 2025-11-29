use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoEmptyClassRule;

inventory::submit!(RuleRegistration {
    name: "no-empty-class",
    factory: || Arc::new(NoEmptyClassRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-empty-class",
        display_name: "No Empty Class",
        description: "Disallows empty classes without methods or properties.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
    }
});

impl Rule for NoEmptyClassRule {
    fn name(&self) -> &str {
        "no-empty-class"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::file_source::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = EmptyClassVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct EmptyClassVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for EmptyClassVisitor<'a> {
    fn visit_class(&mut self, n: &Class) {
        if n.body.is_empty() {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "no-empty-class".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: "Empty class without methods or properties".to_string(),
                severity: Severity::Warning,
                line_text: None,
            });
        }
        n.visit_children_with(self);
    }
}

impl<'a> EmptyClassVisitor<'a> {}
