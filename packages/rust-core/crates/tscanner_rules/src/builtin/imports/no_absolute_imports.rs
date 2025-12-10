use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleExecutionKind, RuleMetadata, RuleMetadataRegistration};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct AbsoluteImportState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub struct NoAbsoluteImportsRule;

inventory::submit!(RuleRegistration {
    name: "no-absolute-imports",
    factory: |_| Arc::new(NoAbsoluteImportsRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-absolute-imports",
        display_name: "No Absolute Imports",
        description:
            "Disallows absolute imports without alias. Prefer relative or aliased imports.",
        rule_type: RuleExecutionKind::Ast,
        category: RuleCategory::Imports,
        typescript_only: false,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoAbsoluteImportsRule {
    type State = AbsoluteImportState;

    fn name(&self) -> &'static str {
        "no-absolute-imports"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = AbsoluteImportVisitor {
            states: Vec::new(),
            source: ctx.source(),
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            "Use relative or aliased imports instead of absolute imports".to_string(),
        )
    }
}

struct AbsoluteImportVisitor<'a> {
    states: Vec<AbsoluteImportState>,
    source: &'a str,
}

impl<'a> Visit for AbsoluteImportVisitor<'a> {
    fn visit_import_decl(&mut self, n: &ImportDecl) {
        let span = n.src.span;
        let import_start = span.lo.0 as usize;
        let import_end = span.hi.0 as usize;

        if import_start < self.source.len() && import_end <= self.source.len() {
            let src_slice = &self.source[import_start..import_end];
            let import_path = src_slice.trim_matches('"').trim_matches('\'');

            if !import_path.starts_with('.')
                && !import_path.starts_with('@')
                && !import_path.starts_with("node:")
            {
                let is_builtin = matches!(
                    import_path,
                    "fs" | "path"
                        | "http"
                        | "https"
                        | "crypto"
                        | "os"
                        | "util"
                        | "events"
                        | "stream"
                        | "buffer"
                        | "child_process"
                        | "url"
                        | "querystring"
                        | "zlib"
                        | "net"
                        | "tls"
                        | "dns"
                        | "assert"
                        | "cluster"
                        | "dgram"
                        | "readline"
                        | "repl"
                        | "tty"
                        | "vm"
                        | "worker_threads"
                );

                if !is_builtin {
                    let (line, column, end_column) =
                        get_span_positions(self.source, import_start, import_end);

                    self.states.push(AbsoluteImportState {
                        line,
                        start_col: column,
                        end_col: end_column,
                    });
                }
            }
        }
        n.visit_children_with(self);
    }
}
