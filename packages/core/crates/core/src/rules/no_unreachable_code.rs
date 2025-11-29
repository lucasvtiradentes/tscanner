use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoUnreachableCodeRule;

inventory::submit!(RuleRegistration {
    name: "no-unreachable-code",
    factory: || Arc::new(NoUnreachableCodeRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-unreachable-code",
        display_name: "No Unreachable Code",
        description: "Detects code after return, throw, break, or continue statements. This code will never execute.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::BugPrevention,
        typescript_only: false,
    }
});

impl Rule for NoUnreachableCodeRule {
    fn name(&self) -> &str {
        "no-unreachable-code"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::file_source::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = UnreachableCodeVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct UnreachableCodeVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> UnreachableCodeVisitor<'a> {
    fn is_terminating_stmt(&self, stmt: &Stmt) -> bool {
        matches!(
            stmt,
            Stmt::Return(_) | Stmt::Throw(_) | Stmt::Break(_) | Stmt::Continue(_)
        )
    }

    fn check_block_statements(&mut self, stmts: &[Stmt]) {
        let mut found_terminator = false;

        for stmt in stmts {
            if found_terminator {
                let span = stmt.span();
                let (line, column, end_column) =
                    get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                self.issues.push(Issue {
                    rule: "no-unreachable-code".to_string(),
                    file: self.path.clone(),
                    line,
                    column,
                    end_column,
                    message: "Unreachable code detected after return/throw/break/continue"
                        .to_string(),
                    severity: Severity::Error,
                    line_text: None,
                });
                break;
            }

            if self.is_terminating_stmt(stmt) {
                found_terminator = true;
            }
        }
    }
}

impl<'a> Visit for UnreachableCodeVisitor<'a> {
    fn visit_block_stmt(&mut self, n: &BlockStmt) {
        self.check_block_statements(&n.stmts);
        n.visit_children_with(self);
    }

    fn visit_switch_case(&mut self, n: &SwitchCase) {
        self.check_block_statements(&n.cons);
        n.visit_children_with(self);
    }
}
