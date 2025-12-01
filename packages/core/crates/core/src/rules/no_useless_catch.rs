use super::metadata::RuleType;
use super::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::output::{Issue, Severity};
use crate::rule::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoUselessCatchRule;

inventory::submit!(RuleRegistration {
    name: "no-useless-catch",
    factory: |_| Arc::new(NoUselessCatchRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-useless-catch",
        display_name: "No Useless Catch",
        description: "Disallows catch blocks that only rethrow the caught error. Remove the try-catch or add meaningful error handling.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-useless-catch"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-useless-catch"),
        allowed_options: &[],
    }
});

impl Rule for NoUselessCatchRule {
    fn name(&self) -> &str {
        "no-useless-catch"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = UselessCatchVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct UselessCatchVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for UselessCatchVisitor<'a> {
    fn visit_try_stmt(&mut self, n: &TryStmt) {
        if let Some(handler) = &n.handler {
            if handler.body.stmts.len() == 1 {
                if let Some(Stmt::Throw(throw_stmt)) = handler.body.stmts.first() {
                    if let Expr::Ident(throw_ident) = throw_stmt.arg.as_ref() {
                        if let Some(Pat::Ident(catch_param)) = &handler.param {
                            if throw_ident.sym == catch_param.sym {
                                let span = handler.span();
                                let (line, column, end_column) = get_span_positions(
                                    self.source,
                                    span.lo.0 as usize,
                                    span.hi.0 as usize,
                                );

                                self.issues.push(Issue {
                                    rule: "no-useless-catch".to_string(),
                                    file: self.path.clone(),
                                    line,
                                    column,
                                    end_column,
                                    message: "Useless catch block that only rethrows the error. Remove the try-catch or add meaningful error handling.".to_string(),
                                    severity: Severity::Warning,
                                    line_text: None,
                                });
                            }
                        }
                    }
                }
            }
        }
        n.visit_children_with(self);
    }
}
