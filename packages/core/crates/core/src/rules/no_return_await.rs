use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoReturnAwaitRule;

inventory::submit!(RuleRegistration {
    name: "no-return-await",
    factory: || Arc::new(NoReturnAwaitRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-return-await",
        display_name: "No Return Await",
        description: "Disallows redundant 'return await' in async functions. The await is unnecessary since the function already returns a Promise.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/return-await"),
        equivalent_biome_rule: None,
    }
});

impl Rule for NoReturnAwaitRule {
    fn name(&self) -> &str {
        "no-return-await"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = ReturnAwaitVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
            in_async_context: false,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct ReturnAwaitVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
    in_async_context: bool,
}

impl<'a> Visit for ReturnAwaitVisitor<'a> {
    fn visit_function(&mut self, n: &Function) {
        let was_async = self.in_async_context;
        self.in_async_context = n.is_async;
        n.visit_children_with(self);
        self.in_async_context = was_async;
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        let was_async = self.in_async_context;
        self.in_async_context = n.is_async;
        n.visit_children_with(self);
        self.in_async_context = was_async;
    }

    fn visit_return_stmt(&mut self, n: &ReturnStmt) {
        if self.in_async_context {
            if let Some(arg) = &n.arg {
                if let Expr::Await(await_expr) = arg.as_ref() {
                    let span = await_expr.span();
                    let (line, column, end_column) =
                        get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                    self.issues.push(Issue {
                        rule: "no-return-await".to_string(),
                        file: self.path.clone(),
                        line,
                        column,
                        end_column,
                        message: "Redundant 'return await' found. Remove 'await' since async function already returns a Promise.".to_string(),
                        severity: Severity::Warning,
                        line_text: None,
                    });
                }
            }
        }
        n.visit_children_with(self);
    }
}
