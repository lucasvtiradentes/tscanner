use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::collections::HashSet;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct ForwardedExportState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub message: String,
}

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
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-re-export-all"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoForwardedExportsRule {
    type State = ForwardedExportState;

    fn name(&self) -> &'static str {
        "no-forwarded-exports"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = ForwardedExportsVisitor {
            states: Vec::new(),
            source: ctx.source(),
            imported_names: HashSet::new(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            state.message.clone(),
        )
    }
}

struct ForwardedExportsVisitor<'a> {
    states: Vec<ForwardedExportState>,
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

            self.states.push(ForwardedExportState {
                line,
                start_col: column,
                end_col: end_column,
                message: format!(
                    "Avoid re-exporting from '{}'. Import and use directly instead.",
                    src_value
                ),
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

                        self.states.push(ForwardedExportState {
                            line,
                            start_col: column,
                            end_col: end_column,
                            message: format!(
                                "Avoid re-exporting '{}'. Import and use directly instead.",
                                orig_name
                            ),
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

        self.states.push(ForwardedExportState {
            line,
            start_col: column,
            end_col: end_column,
            message: format!(
                "Avoid star re-export from '{}'. Import and use directly instead.",
                src_value
            ),
        });

        n.visit_children_with(self);
    }
}
