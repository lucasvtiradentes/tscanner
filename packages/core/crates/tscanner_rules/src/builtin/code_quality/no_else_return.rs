use crate::metadata::RuleType;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};
use tscanner_diagnostics::{Issue, Severity};

pub struct NoElseReturnRule;

inventory::submit!(RuleRegistration {
    name: "no-else-return",
    factory: |_| Arc::new(NoElseReturnRule),
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
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-else-return"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-useless-else"),
        allowed_options: &[],
    }
});

impl Rule for NoElseReturnRule {
    fn name(&self) -> &str {
        "no-else-return"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::FileSource,
    ) -> Vec<Issue> {
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
                let (line, column, end_column) =
                    get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                self.issues.push(Issue {
                    rule: "no-else-return".to_string(),
                    file: self.path.clone(),
                    line,
                    column,
                    end_column,
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
