use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::collections::HashMap;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct DuplicateImportState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub module_name: String,
    pub first_line: usize,
}

pub struct NoDuplicateImportsRule;

inventory::submit!(RuleRegistration {
    name: "no-duplicate-imports",
    factory: |_| Arc::new(NoDuplicateImportsRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-duplicate-imports",
        display_name: "No Duplicate Imports",
        description: "Disallows multiple import statements from the same module. Merge them into a single import.",
        rule_type: RuleType::Ast,
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-duplicate-imports"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-duplicate-json-keys"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoDuplicateImportsRule {
    type State = DuplicateImportState;

    fn name(&self) -> &'static str {
        "no-duplicate-imports"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = DuplicateImportsVisitor {
            states: Vec::new(),
            source: ctx.source(),
            seen_imports: HashMap::new(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            format!(
                "Module '{}' is already imported at line {}. Merge imports.",
                state.module_name, state.first_line
            ),
        )
    }
}

struct DuplicateImportsVisitor<'a> {
    states: Vec<DuplicateImportState>,
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

                self.states.push(DuplicateImportState {
                    line,
                    start_col: column,
                    end_col: end_column,
                    module_name,
                    first_line,
                });
            } else {
                let (line, _, _) = get_span_positions(self.source, import_start, import_end);
                self.seen_imports.insert(module_name, line);
            }
        }

        n.visit_children_with(self);
    }
}
