use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_line_col;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoElseReturnRule;

inventory::submit!(RuleRegistration {
    name: "no-else-return",
    factory: || Arc::new(NoElseReturnRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-else-return",
        display_name: "No Else Return",
        description: "Disallows else blocks after return statements. The else is unnecessary since the function already returned.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
    }
});

impl Rule for NoElseReturnRule {
    fn name(&self) -> &str {
        "no-else-return"
    }

    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let mut visitor = ElseReturnVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct ElseReturnVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> ElseReturnVisitor<'a> {
    fn stmt_contains_return(&self, stmt: &Stmt) -> bool {
        let mut collector = ReturnCollector { has_return: false };
        stmt.visit_with(&mut collector);
        collector.has_return
    }
}

impl<'a> Visit for ElseReturnVisitor<'a> {
    fn visit_if_stmt(&mut self, n: &IfStmt) {
        let cons_has_return = self.stmt_contains_return(&n.cons);

        if cons_has_return && n.alt.is_some() {
            if let Some(alt) = &n.alt {
                let span = alt.span();
                let (line, column) = get_line_col(self.source, span.lo.0 as usize);

                self.issues.push(Issue {
                    rule: "no-else-return".to_string(),
                    file: self.path.clone(),
                    line,
                    column,
                    message: "Unnecessary 'else' after 'return'. Remove the 'else' block since the function already returned.".to_string(),
                    severity: Severity::Warning,
                    line_text: None,
                });
            }
        }

        n.visit_children_with(self);
    }
}

struct ReturnCollector {
    has_return: bool,
}

impl Visit for ReturnCollector {
    fn visit_return_stmt(&mut self, _n: &ReturnStmt) {
        self.has_return = true;
    }

    fn visit_function(&mut self, _n: &Function) {}

    fn visit_arrow_expr(&mut self, _n: &ArrowExpr) {}
}
