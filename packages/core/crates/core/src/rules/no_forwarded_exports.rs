use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::utils::get_span_positions;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoForwardedExportsRule;

inventory::submit!(RuleRegistration {
    name: "no-forwarded-exports",
    factory: |_| Arc::new(NoForwardedExportsRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-forwarded-exports",
        display_name: "No Forwarded Exports",
        description: "Disallows re-exporting from other modules. This includes direct re-exports (export { X } from 'module'), star re-exports (export * from 'module'), and re-exporting imported values.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-re-export-all"),
        allowed_options: &[],
    }
});

impl Rule for NoForwardedExportsRule {
    fn name(&self) -> &str {
        "no-forwarded-exports"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = ForwardedExportsVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
            imported_names: HashSet::new(),
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct ForwardedExportsVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
    imported_names: HashSet<String>,
}

impl<'a> ForwardedExportsVisitor<'a> {
    fn get_source_value(&self, span: swc_common::Span) -> String {
        let start = span.lo.0 as usize;
        let end = span.hi.0 as usize;
        if start < self.source.len() && end <= self.source.len() {
            self.source[start..end]
                .trim_matches('"')
                .trim_matches('\'')
                .to_string()
        } else {
            String::new()
        }
    }
}

impl<'a> Visit for ForwardedExportsVisitor<'a> {
    fn visit_import_decl(&mut self, n: &ImportDecl) {
        for specifier in &n.specifiers {
            match specifier {
                ImportSpecifier::Named(named) => {
                    let local_name = named.local.sym.to_string();
                    self.imported_names.insert(local_name);
                }
                ImportSpecifier::Default(default) => {
                    let local_name = default.local.sym.to_string();
                    self.imported_names.insert(local_name);
                }
                ImportSpecifier::Namespace(ns) => {
                    let local_name = ns.local.sym.to_string();
                    self.imported_names.insert(local_name);
                }
            }
        }
        n.visit_children_with(self);
    }

    fn visit_named_export(&mut self, n: &NamedExport) {
        if let Some(src) = &n.src {
            let (line, column, end_column) =
                get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);
            let src_value = self.get_source_value(src.span);

            self.issues.push(Issue {
                rule: "no-forwarded-exports".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: format!(
                    "Avoid re-exporting from '{}'. Import and use directly instead.",
                    src_value
                ),
                severity: Severity::Warning,
                line_text: None,
            });
        } else {
            for specifier in &n.specifiers {
                if let ExportSpecifier::Named(named) = specifier {
                    let orig_name = match &named.orig {
                        ModuleExportName::Ident(ident) => ident.sym.to_string(),
                        ModuleExportName::Str(s) => self.get_source_value(s.span),
                    };

                    if self.imported_names.contains(&orig_name) {
                        let (line, column, end_column) = get_span_positions(
                            self.source,
                            named.span.lo.0 as usize,
                            named.span.hi.0 as usize,
                        );

                        self.issues.push(Issue {
                            rule: "no-forwarded-exports".to_string(),
                            file: self.path.clone(),
                            line,
                            column,
                            end_column,
                            message: format!(
                                "Avoid re-exporting '{}'. Import and use directly instead.",
                                orig_name
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

    fn visit_export_all(&mut self, n: &ExportAll) {
        let (line, column, end_column) =
            get_span_positions(self.source, n.span.lo.0 as usize, n.span.hi.0 as usize);
        let src_value = self.get_source_value(n.src.span);

        self.issues.push(Issue {
            rule: "no-forwarded-exports".to_string(),
            file: self.path.clone(),
            line,
            column,
            end_column,
            message: format!(
                "Avoid star re-export from '{}'. Import and use directly instead.",
                src_value
            ),
            severity: Severity::Warning,
            line_text: None,
        });

        n.visit_children_with(self);
    }
}
