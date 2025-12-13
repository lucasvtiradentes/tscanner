use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::collections::HashSet;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoFloatingPromisesRule;

inventory::submit!(RuleRegistration {
    name: "no-floating-promises",
    factory: |_| Arc::new(NoFloatingPromisesRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-floating-promises",
        display_name: "No Floating Promises",
        description: "Disallows floating promises (promises used as statements without await, .then(), or .catch()). Unhandled promises can lead to silent failures.",
        rule_type: RuleType::Ast,
        category: RuleCategory::BugPrevention,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-floating-promises"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-floating-promises"),
        ..RuleMetadata::defaults()
    }
});

pub struct FloatingPromiseState {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

struct AsyncFunctionCollector {
    async_functions: HashSet<String>,
}

impl Visit for AsyncFunctionCollector {
    fn visit_fn_decl(&mut self, n: &FnDecl) {
        if n.function.is_async {
            self.async_functions.insert(n.ident.sym.to_string());
        }
        n.visit_children_with(self);
    }

    fn visit_var_declarator(&mut self, n: &VarDeclarator) {
        if let Pat::Ident(ident) = &n.name {
            if let Some(init) = &n.init {
                let is_async = match init.as_ref() {
                    Expr::Arrow(arrow) => arrow.is_async,
                    Expr::Fn(fn_expr) => fn_expr.function.is_async,
                    _ => false,
                };
                if is_async {
                    self.async_functions.insert(ident.id.sym.to_string());
                }
            }
        }
        n.visit_children_with(self);
    }
}

fn collect_async_functions(program: &Program) -> HashSet<String> {
    let mut collector = AsyncFunctionCollector {
        async_functions: HashSet::new(),
    };
    program.visit_with(&mut collector);
    collector.async_functions
}

impl Rule for NoFloatingPromisesRule {
    type State = FloatingPromiseState;

    fn name(&self) -> &'static str {
        "no-floating-promises"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let async_functions = collect_async_functions(ctx.program());

        let mut visitor = FloatingPromiseVisitor {
            states: Vec::new(),
            source: ctx.source(),
            async_functions,
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Promise-returning expression used without handling. Use await, .then(), .catch(), or assign to a variable.".to_string(),
        )
    }
}

struct FloatingPromiseVisitor<'a> {
    states: Vec<FloatingPromiseState>,
    source: &'a str,
    async_functions: std::collections::HashSet<String>,
}

impl<'a> Visit for FloatingPromiseVisitor<'a> {
    fn visit_expr_stmt(&mut self, n: &ExprStmt) {
        if let Expr::Call(call_expr) = n.expr.as_ref() {
            if is_promise_expression(call_expr, &self.async_functions)
                && !is_handled_promise(call_expr)
            {
                let span = call_expr.span();
                let (line, column, end_column) =
                    get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

                self.states.push(FloatingPromiseState {
                    line,
                    column,
                    end_column,
                });
            }
        }
        n.visit_children_with(self);
    }
}

fn is_promise_expression(call_expr: &CallExpr, async_functions: &HashSet<String>) -> bool {
    if is_promise_static_method(call_expr) {
        return true;
    }

    if is_fetch_call(call_expr) {
        return true;
    }

    if is_new_promise(call_expr) {
        return true;
    }

    if is_chained_promise_call(call_expr) {
        return true;
    }

    if is_async_function_call(call_expr, async_functions) {
        return true;
    }

    false
}

fn is_chained_promise_call(call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Member(member_expr) = expr.as_ref() {
            if let MemberProp::Ident(ident) = &member_expr.prop {
                let method_name = ident.sym.as_ref();
                if method_name == "then" || method_name == "catch" || method_name == "finally" {
                    return is_promise_at_root(member_expr.obj.as_ref());
                }
            }
        }
    }
    false
}

fn is_promise_at_root(expr: &Expr) -> bool {
    match expr {
        Expr::Call(call_expr) => {
            if is_promise_static_method(call_expr) || is_fetch_call(call_expr) {
                return true;
            }
            if let Callee::Expr(callee) = &call_expr.callee {
                if let Expr::Member(member_expr) = callee.as_ref() {
                    return is_promise_at_root(member_expr.obj.as_ref());
                }
            }
            false
        }
        Expr::New(new_expr) => {
            if let Expr::Ident(ident) = new_expr.callee.as_ref() {
                return ident.sym.as_ref() == "Promise";
            }
            false
        }
        _ => false,
    }
}

fn is_promise_static_method(call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Member(member_expr) = expr.as_ref() {
            if let Expr::Ident(obj) = member_expr.obj.as_ref() {
                if obj.sym.as_ref() == "Promise" {
                    return true;
                }
            }
        }
    }
    false
}

fn is_fetch_call(call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Ident(ident) = expr.as_ref() {
            return ident.sym.as_ref() == "fetch";
        }
    }
    false
}

fn is_new_promise(call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::New(new_expr) = expr.as_ref() {
            if let Expr::Ident(ident) = new_expr.callee.as_ref() {
                return ident.sym.as_ref() == "Promise";
            }
        }
    }
    false
}

fn is_async_function_call(call_expr: &CallExpr, async_functions: &HashSet<String>) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Ident(ident) = expr.as_ref() {
            return async_functions.contains(ident.sym.as_ref());
        }
    }
    false
}

fn is_handled_promise(call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Member(member_expr) = expr.as_ref() {
            if let MemberProp::Ident(ident) = &member_expr.prop {
                let method_name = ident.sym.as_ref();
                if method_name == "catch" {
                    return true;
                }
                if method_name == "then" && call_expr.args.len() >= 2 {
                    return true;
                }
                if method_name == "finally" {
                    if let Expr::Call(inner_call) = member_expr.obj.as_ref() {
                        return is_handled_promise(inner_call);
                    }
                }
            }
        }
    }
    false
}
