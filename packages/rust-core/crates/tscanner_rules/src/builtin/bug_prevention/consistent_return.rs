use crate::metadata::RuleType;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};
use tscanner_diagnostics::{Issue, Severity};

pub struct ConsistentReturnRule;

inventory::submit!(RuleRegistration {
    name: "consistent-return",
    factory: |_| Arc::new(ConsistentReturnRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "consistent-return",
        display_name: "Consistent Return",
        description: "Requires consistent return behavior in functions. Either all code paths return a value or none do.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::BugPrevention,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/consistent-return"),
        equivalent_biome_rule: None,
        allowed_options: &[],
    }
});

impl Rule for ConsistentReturnRule {
    fn name(&self) -> &str {
        "consistent-return"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = ConsistentReturnVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct ConsistentReturnVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> ConsistentReturnVisitor<'a> {
    fn check_function_body(&mut self, body: &BlockStmt, func_span: swc_common::Span) {
        let mut return_collector = ReturnCollector {
            returns: Vec::new(),
        };
        body.visit_with(&mut return_collector);

        if return_collector.returns.is_empty() {
            return;
        }

        let value_returns: Vec<_> = return_collector
            .returns
            .iter()
            .filter(|r| r.has_value)
            .collect();
        let no_value_returns: Vec<_> = return_collector
            .returns
            .iter()
            .filter(|r| !r.has_value)
            .collect();

        if !value_returns.is_empty()
            && !no_value_returns.is_empty()
            && value_returns.len() > no_value_returns.len()
        {
            let (line, column, end_column) = get_span_positions(
                self.source,
                func_span.lo.0 as usize,
                func_span.hi.0 as usize,
            );

            self.issues.push(Issue {
                rule: "consistent-return".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message:
                    "Function has inconsistent return statements. Some return values, others don't."
                        .to_string(),
                severity: Severity::Warning,
                line_text: None,
            });
        }
    }
}

impl<'a> Visit for ConsistentReturnVisitor<'a> {
    fn visit_function(&mut self, n: &Function) {
        if let Some(body) = &n.body {
            self.check_function_body(body, n.span);
        }
        n.visit_children_with(self);
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        if let BlockStmtOrExpr::BlockStmt(body) = &*n.body {
            self.check_function_body(body, n.span);
        }
        n.visit_children_with(self);
    }
}

struct ReturnInfo {
    has_value: bool,
}

struct ReturnCollector {
    returns: Vec<ReturnInfo>,
}

impl Visit for ReturnCollector {
    fn visit_return_stmt(&mut self, n: &ReturnStmt) {
        self.returns.push(ReturnInfo {
            has_value: n.arg.is_some(),
        });
        n.visit_children_with(self);
    }

    fn visit_function(&mut self, _n: &Function) {}

    fn visit_arrow_expr(&mut self, _n: &ArrowExpr) {}
}
