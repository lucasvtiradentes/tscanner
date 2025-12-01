use super::metadata::RuleType;
use super::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::output::{Issue, Severity};
use crate::rule::{Rule, RuleRegistration};
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoImplicitAnyRule;

inventory::submit!(RuleRegistration {
    name: "no-implicit-any",
    factory: |_| Arc::new(NoImplicitAnyRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-implicit-any",
        display_name: "No Implicit Any",
        description:
            "Detects function parameters without type annotations that implicitly have 'any' type.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::TypeSafety,
        typescript_only: true,
        equivalent_eslint_rule: None,
        equivalent_biome_rule: None,
        allowed_options: &[],
    }
});

impl Rule for NoImplicitAnyRule {
    fn name(&self) -> &str {
        "no-implicit-any"
    }

    fn is_typescript_only(&self) -> bool {
        true
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = ImplicitAnyVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
            in_inferred_context: false,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct ImplicitAnyVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
    in_inferred_context: bool,
}

fn is_array_method(method_name: &str) -> bool {
    const ARRAY_METHODS: &[&str] = &[
        "map",
        "filter",
        "forEach",
        "reduce",
        "reduceRight",
        "find",
        "findIndex",
        "some",
        "every",
        "flatMap",
        "sort",
        "findLast",
        "findLastIndex",
    ];
    ARRAY_METHODS.contains(&method_name)
}

fn is_promise_method(method_name: &str) -> bool {
    const PROMISE_METHODS: &[&str] = &["then", "catch", "finally"];
    PROMISE_METHODS.contains(&method_name)
}

fn is_inferred_callback_context(method_name: &str) -> bool {
    is_array_method(method_name) || is_promise_method(method_name)
}

fn is_promise_constructor(callee: &Expr) -> bool {
    matches!(callee, Expr::Ident(ident) if ident.sym.as_ref() == "Promise")
}

impl<'a> ImplicitAnyVisitor<'a> {
    fn report_issue(&mut self, span: swc_common::Span, message: String) {
        self.issues.push(Issue::from_span(
            "no-implicit-any",
            self.path.clone(),
            self.source,
            span.lo.0 as usize,
            span.hi.0 as usize,
            message,
        ));
    }

    fn check_pat(&mut self, pat: &Pat) {
        if self.in_inferred_context {
            return;
        }

        match pat {
            Pat::Ident(ident) if ident.type_ann.is_none() => {
                self.report_issue(
                    ident.span(),
                    format!(
                        "Parameter '{}' implicitly has 'any' type. Add type annotation.",
                        ident.id.sym
                    ),
                );
            }
            Pat::Array(arr) if arr.type_ann.is_none() => {
                self.report_issue(
                    arr.span(),
                    "Destructured parameter implicitly has 'any' type. Add type annotation."
                        .to_string(),
                );
            }
            Pat::Object(obj) if obj.type_ann.is_none() => {
                self.report_issue(
                    obj.span(),
                    "Destructured parameter implicitly has 'any' type. Add type annotation."
                        .to_string(),
                );
            }
            Pat::Rest(rest) if rest.type_ann.is_none() => {
                self.report_issue(
                    rest.span(),
                    "Rest parameter implicitly has 'any' type. Add type annotation.".to_string(),
                );
            }
            _ => {}
        }
    }

    fn check_param(&mut self, param: &Param) {
        self.check_pat(&param.pat);
    }
}

impl<'a> Visit for ImplicitAnyVisitor<'a> {
    fn visit_function(&mut self, n: &Function) {
        for param in &n.params {
            self.check_param(param);
        }
        n.visit_children_with(self);
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        for pat in &n.params {
            self.check_pat(pat);
        }
        n.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, n: &CallExpr) {
        let has_arrow_callback = n
            .args
            .iter()
            .any(|arg| matches!(arg.expr.as_ref(), Expr::Arrow(_)));

        if has_arrow_callback {
            let prev_context = self.in_inferred_context;
            self.in_inferred_context = true;
            n.visit_children_with(self);
            self.in_inferred_context = prev_context;
            return;
        }

        if let Callee::Expr(callee_expr) = &n.callee {
            if let Expr::Member(member_expr) = callee_expr.as_ref() {
                if let MemberProp::Ident(ident) = &member_expr.prop {
                    let method_name = ident.sym.as_ref();
                    if is_inferred_callback_context(method_name) {
                        let prev_context = self.in_inferred_context;
                        self.in_inferred_context = true;
                        n.visit_children_with(self);
                        self.in_inferred_context = prev_context;
                        return;
                    }
                }
            }
        }
        n.visit_children_with(self);
    }

    fn visit_new_expr(&mut self, n: &NewExpr) {
        let has_arrow_callback = n
            .args
            .as_ref()
            .map(|args| {
                args.iter()
                    .any(|arg| matches!(arg.expr.as_ref(), Expr::Arrow(_)))
            })
            .unwrap_or(false);

        if has_arrow_callback && is_promise_constructor(&n.callee) {
            let prev_context = self.in_inferred_context;
            self.in_inferred_context = true;
            n.visit_children_with(self);
            self.in_inferred_context = prev_context;
            return;
        }

        n.visit_children_with(self);
    }
}
