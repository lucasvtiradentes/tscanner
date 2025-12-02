use crate::metadata::RuleType;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};
use tscanner_diagnostics::{Issue, Severity};

pub struct NoAsyncWithoutAwaitRule;

inventory::submit!(RuleRegistration {
    name: "no-async-without-await",
    factory: |_| Arc::new(NoAsyncWithoutAwaitRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-async-without-await",
        display_name: "No Async Without Await",
        description: "Disallows async functions that don't use await. The async keyword is unnecessary if await is never used.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/require-await"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/use-await"),
        allowed_options: &[],
    }
});

impl Rule for NoAsyncWithoutAwaitRule {
    fn name(&self) -> &str {
        "no-async-without-await"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = AsyncWithoutAwaitVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct AsyncWithoutAwaitVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> AsyncWithoutAwaitVisitor<'a> {
    fn check_async_body(&mut self, body: &BlockStmt, span: swc_common::Span, is_async: bool) {
        if !is_async {
            return;
        }

        let mut await_checker = AwaitChecker { has_await: false };
        body.visit_with(&mut await_checker);

        if !await_checker.has_await {
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "no-async-without-await".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: "Async function does not use await. Remove 'async' keyword if await is not needed.".to_string(),
                severity: Severity::Warning,
                line_text: None,
            });
        }
    }

    fn check_async_arrow_body(
        &mut self,
        body: &BlockStmtOrExpr,
        span: swc_common::Span,
        is_async: bool,
    ) {
        if !is_async {
            return;
        }

        let mut await_checker = AwaitChecker { has_await: false };
        body.visit_with(&mut await_checker);

        if !await_checker.has_await {
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "no-async-without-await".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: "Async function does not use await. Remove 'async' keyword if await is not needed.".to_string(),
                severity: Severity::Warning,
                line_text: None,
            });
        }
    }
}

impl<'a> Visit for AsyncWithoutAwaitVisitor<'a> {
    fn visit_function(&mut self, n: &Function) {
        if let Some(body) = &n.body {
            self.check_async_body(body, n.span, n.is_async);
        }
        n.visit_children_with(self);
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        self.check_async_arrow_body(&n.body, n.span, n.is_async);
        n.visit_children_with(self);
    }
}

struct AwaitChecker {
    has_await: bool,
}

impl Visit for AwaitChecker {
    fn visit_await_expr(&mut self, _n: &AwaitExpr) {
        self.has_await = true;
    }

    fn visit_function(&mut self, _n: &Function) {}

    fn visit_arrow_expr(&mut self, _n: &ArrowExpr) {}
}
