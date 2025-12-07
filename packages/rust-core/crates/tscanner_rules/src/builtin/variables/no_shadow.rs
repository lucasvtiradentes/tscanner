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

pub struct NoShadowRule;

inventory::submit!(RuleRegistration {
    name: "no-shadow",
    factory: |_| Arc::new(NoShadowRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-shadow",
        display_name: "No Shadow",
        description: "Disallows variable declarations that shadow variables in outer scopes. Shadowing can lead to confusing code and subtle bugs.",
        rule_type: RuleType::Ast,
        category: RuleCategory::Variables,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-shadow"),
        equivalent_biome_rule: None,
        ..RuleMetadata::defaults()
    }
});

pub struct ShadowState {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub variable_name: String,
}

impl Rule for NoShadowRule {
    type State = ShadowState;

    fn name(&self) -> &'static str {
        "no-shadow"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = ShadowVisitor {
            states: Vec::new(),
            source: ctx.source(),
            scope_stack: vec![HashSet::new()],
        };
        ctx.program().visit_with(&mut visitor);
        visitor.states
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.start_col, state.end_col),
            format!(
                "Variable '{}' shadows a variable in an outer scope.",
                state.variable_name
            ),
        )
    }
}

struct ShadowVisitor<'a> {
    states: Vec<ShadowState>,
    source: &'a str,
    scope_stack: Vec<HashSet<String>>,
}

impl<'a> ShadowVisitor<'a> {
    fn push_scope(&mut self) {
        self.scope_stack.push(HashSet::new());
    }

    fn pop_scope(&mut self) {
        self.scope_stack.pop();
    }

    fn current_scope(&mut self) -> Option<&mut HashSet<String>> {
        self.scope_stack.last_mut()
    }

    fn is_shadowing(&self, name: &str) -> bool {
        if self.scope_stack.len() <= 1 {
            return false;
        }

        for i in 0..self.scope_stack.len() - 1 {
            if self.scope_stack[i].contains(name) {
                return true;
            }
        }
        false
    }

    fn add_variable(&mut self, name: String, span: swc_common::Span) {
        if self.is_shadowing(&name) {
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);
            self.states.push(ShadowState {
                line,
                start_col: column,
                end_col: end_column,
                variable_name: name.clone(),
            });
        }
        if let Some(scope) = self.current_scope() {
            scope.insert(name);
        }
    }

    fn add_param(&mut self, pat: &Pat) {
        match pat {
            Pat::Ident(ident) => {
                self.add_variable(ident.id.sym.to_string(), ident.span());
            }
            Pat::Array(arr) => {
                for elem in arr.elems.iter().flatten() {
                    self.add_param(elem);
                }
            }
            Pat::Object(obj) => {
                for prop in &obj.props {
                    match prop {
                        ObjectPatProp::KeyValue(kv) => {
                            self.add_param(&kv.value);
                        }
                        ObjectPatProp::Assign(assign) => {
                            self.add_variable(assign.key.sym.to_string(), assign.key.span());
                        }
                        ObjectPatProp::Rest(rest) => {
                            self.add_param(&rest.arg);
                        }
                    }
                }
            }
            Pat::Rest(rest) => {
                self.add_param(&rest.arg);
            }
            Pat::Assign(assign) => {
                self.add_param(&assign.left);
            }
            _ => {}
        }
    }
}

impl<'a> Visit for ShadowVisitor<'a> {
    fn visit_var_decl(&mut self, n: &VarDecl) {
        for decl in &n.decls {
            match &decl.name {
                Pat::Ident(ident) => {
                    self.add_variable(ident.id.sym.to_string(), ident.span());
                }
                Pat::Array(_arr) => {
                    self.add_param(&decl.name);
                }
                Pat::Object(_obj) => {
                    self.add_param(&decl.name);
                }
                _ => {}
            }
        }
        n.visit_children_with(self);
    }

    fn visit_fn_decl(&mut self, n: &FnDecl) {
        self.push_scope();
        for param in &n.function.params {
            self.add_param(&param.pat);
        }
        n.function.visit_children_with(self);
        self.pop_scope();
    }

    fn visit_function(&mut self, n: &Function) {
        self.push_scope();
        for param in &n.params {
            self.add_param(&param.pat);
        }
        if let Some(body) = &n.body {
            body.visit_with(self);
        }
        self.pop_scope();
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        self.push_scope();
        for param in &n.params {
            self.add_param(param);
        }
        n.body.visit_with(self);
        self.pop_scope();
    }

    fn visit_block_stmt(&mut self, n: &BlockStmt) {
        self.push_scope();
        n.visit_children_with(self);
        self.pop_scope();
    }

    fn visit_catch_clause(&mut self, n: &CatchClause) {
        self.push_scope();
        if let Some(param) = &n.param {
            self.add_param(param);
        }
        n.body.visit_with(self);
        self.pop_scope();
    }
}
