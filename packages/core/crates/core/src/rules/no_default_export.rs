use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoDefaultExportRule;

inventory::submit!(RuleRegistration {
    name: "no-default-export",
    factory: || Arc::new(NoDefaultExportRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-default-export",
        display_name: "No Default Export",
        description: "Disallows default exports. Named exports are preferred for better refactoring support and explicit imports.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::Imports,
        typescript_only: false,
    }
});

impl Rule for NoDefaultExportRule {
    fn name(&self) -> &str {
        "no-default-export"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::file_source::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = DefaultExportVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct DefaultExportVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for DefaultExportVisitor<'a> {
    fn visit_export_default_decl(&mut self, n: &ExportDefaultDecl) {
        let (line, column, end_column) =
            get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

        self.issues.push(Issue {
            rule: "no-default-export".to_string(),
            file: self.path.clone(),
            line,
            column,
            end_column,
            message: "Avoid default exports. Use named exports for better refactoring support."
                .to_string(),
            severity: Severity::Warning,
            line_text: None,
        });

        n.visit_children_with(self);
    }

    fn visit_export_default_expr(&mut self, n: &ExportDefaultExpr) {
        let (line, column, end_column) =
            get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);

        self.issues.push(Issue {
            rule: "no-default-export".to_string(),
            file: self.path.clone(),
            line,
            column,
            end_column,
            message: "Avoid default exports. Use named exports for better refactoring support."
                .to_string(),
            severity: Severity::Warning,
            line_text: None,
        });

        n.visit_children_with(self);
    }
}
