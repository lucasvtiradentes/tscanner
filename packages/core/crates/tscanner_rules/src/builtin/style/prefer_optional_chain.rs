use crate::metadata::RuleType;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};
use tscanner_diagnostics::{Issue, Severity};

pub struct PreferOptionalChainRule;

inventory::submit!(RuleRegistration {
    name: "prefer-optional-chain",
    factory: |_| Arc::new(PreferOptionalChainRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "prefer-optional-chain",
        display_name: "Prefer Optional Chain",
        description: "Suggests using optional chaining (?.) instead of logical AND (&&) chains for null checks.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::Style,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/prefer-optional-chain"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/use-optional-chain"),
        allowed_options: &[],
    }
});

impl Rule for PreferOptionalChainRule {
    fn name(&self) -> &str {
        "prefer-optional-chain"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = OptionalChainVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct OptionalChainVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> OptionalChainVisitor<'a> {
    fn get_ident_name(expr: &Expr) -> Option<&str> {
        if let Expr::Ident(ident) = expr {
            Some(ident.sym.as_ref())
        } else {
            None
        }
    }

    fn get_member_expr_object_name(expr: &Expr) -> Option<&str> {
        if let Expr::Member(member) = expr {
            if let MemberProp::Ident(_) = &member.prop {
                return Self::get_ident_name(&member.obj);
            }
        }
        None
    }
}

impl<'a> Visit for OptionalChainVisitor<'a> {
    fn visit_bin_expr(&mut self, n: &BinExpr) {
        if matches!(n.op, BinaryOp::LogicalAnd) {
            if let Some(left_name) = Self::get_ident_name(&n.left) {
                if let Some(right_obj_name) = Self::get_member_expr_object_name(&n.right) {
                    if left_name == right_obj_name {
                        let (line, column, end_column) = get_span_positions(
                            self.source,
                            n.span.lo.0 as usize,
                            n.span.hi.0 as usize,
                        );

                        self.issues.push(Issue {
                            rule: "prefer-optional-chain".to_string(),
                            file: self.path.clone(),
                            line,
                            column,
                            end_column,
                            message: "Use optional chaining (?.) instead of logical AND (&&) for null checks.".to_string(),
                            severity: Severity::Warning,
                            line_text: None,
                        });
                    }
                }
            }
        }

        n.visit_children_with(self);
    }
}
