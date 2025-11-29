use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_span_positions;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoDuplicateImportsRule;

inventory::submit!(RuleRegistration {
    name: "no-duplicate-imports",
    factory: || Arc::new(NoDuplicateImportsRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-duplicate-imports",
        display_name: "No Duplicate Imports",
        description: "Disallows multiple import statements from the same module. Merge them into a single import.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::Imports,
        typescript_only: false,
    }
});

impl Rule for NoDuplicateImportsRule {
    fn name(&self) -> &str {
        "no-duplicate-imports"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::file_source::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = DuplicateImportsVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
            seen_imports: HashMap::new(),
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct DuplicateImportsVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
    seen_imports: HashMap<String, usize>,
}

impl<'a> Visit for DuplicateImportsVisitor<'a> {
    fn visit_import_decl(&mut self, n: &ImportDecl) {
        let span = n.src.span;
        let import_start = span.lo.0 as usize;
        let import_end = span.hi.0 as usize;

        if import_start < self.source.len() && import_end <= self.source.len() {
            let src_slice = &self.source[import_start..import_end];
            let module_name = src_slice.trim_matches('"').trim_matches('\'').to_string();

            if let Some(&first_line) = self.seen_imports.get(&module_name) {
                let (line, column, end_column) =
                    get_span_positions(self.source, import_start, import_end);

                self.issues.push(Issue {
                    rule: "no-duplicate-imports".to_string(),
                    file: self.path.clone(),
                    line,
                    column,
                    end_column,
                    message: format!(
                        "Module '{}' is already imported at line {}. Merge imports.",
                        module_name, first_line
                    ),
                    severity: Severity::Warning,
                    line_text: None,
                });
            } else {
                let (line, _, _) = get_span_positions(self.source, import_start, import_end);
                self.seen_imports.insert(module_name, line);
            }
        }

        n.visit_children_with(self);
    }
}

impl<'a> DuplicateImportsVisitor<'a> {}
