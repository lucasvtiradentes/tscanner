use super::metadata::RuleType;
use super::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::output::{Issue, Severity};
use crate::rule::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoRelativeImportsRule;

inventory::submit!(RuleRegistration {
    name: "no-relative-imports",
    factory: |_| Arc::new(NoRelativeImportsRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-relative-imports",
        display_name: "No Relative Imports",
        description: "Detects relative imports (starting with './' or '../'). Prefer absolute imports with @ prefix for better maintainability.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
        allowed_options: &[],
    }
});

impl Rule for NoRelativeImportsRule {
    fn name(&self) -> &str {
        "no-relative-imports"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = RelativeImportVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct RelativeImportVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for RelativeImportVisitor<'a> {
    fn visit_import_decl(&mut self, n: &ImportDecl) {
        let span = n.src.span;
        let import_start = span.lo.0 as usize;
        let import_end = span.hi.0 as usize;

        if import_start < self.source.len() && import_end <= self.source.len() {
            let src_slice = &self.source[import_start..import_end];
            if src_slice
                .trim_matches('"')
                .trim_matches('\'')
                .starts_with('.')
            {
                let (line, column, end_column) =
                    get_span_positions(self.source, import_start, import_end);

                self.issues.push(Issue {
                    rule: "no-relative-imports".to_string(),
                    file: self.path.clone(),
                    line,
                    column,
                    end_column,
                    message: "Use absolute imports with @ prefix instead of relative imports"
                        .to_string(),
                    severity: Severity::Warning,
                    line_text: None,
                });
            }
        }
        n.visit_children_with(self);
    }
}

impl<'a> RelativeImportVisitor<'a> {}
