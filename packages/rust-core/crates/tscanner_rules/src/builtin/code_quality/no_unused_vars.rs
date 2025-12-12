use crate::context::RuleContext;
use crate::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleType};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct UnusedVar {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub name: String,
}

pub struct NoUnusedVarsRule;

inventory::submit!(RuleRegistration {
    name: "no-unused-vars",
    factory: |_| Arc::new(NoUnusedVarsRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-unused-vars",
        display_name: "No Unused Variables",
        description: "Detects variables that are declared but never used in the code.",
        rule_type: RuleType::Ast,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/no-unused-vars"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-unused-variables"),
        ..RuleMetadata::defaults()
    }
});

impl Rule for NoUnusedVarsRule {
    type State = UnusedVar;

    fn name(&self) -> &'static str {
        "no-unused-vars"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = UnusedVarsVisitor {
            declared_vars: HashMap::new(),
            used_vars: HashSet::new(),
        };
        ctx.program().visit_with(&mut visitor);

        let mut issues = Vec::new();
        for (name, span) in &visitor.declared_vars {
            if !visitor.used_vars.contains(name) && !name.starts_with('_') {
                let (line, column, end_column) =
                    get_span_positions(ctx.source(), span.lo.0 as usize, span.hi.0 as usize);

                issues.push(UnusedVar {
                    line,
                    column,
                    end_column,
                    name: name.clone(),
                });
            }
        }

        issues
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            format!("Variable '{}' is declared but never used.", state.name),
        )
    }
}

struct UnusedVarsVisitor {
    declared_vars: HashMap<String, swc_common::Span>,
    used_vars: HashSet<String>,
}

impl Visit for UnusedVarsVisitor {
    fn visit_var_decl(&mut self, n: &VarDecl) {
        for decl in &n.decls {
            if let Pat::Ident(ident) = &decl.name {
                self.declared_vars
                    .insert(ident.id.sym.to_string(), ident.span());
            }
        }
        n.visit_children_with(self);
    }

    fn visit_ident(&mut self, n: &Ident) {
        self.used_vars.insert(n.sym.to_string());
        n.visit_children_with(self);
    }

    fn visit_fn_decl(&mut self, n: &FnDecl) {
        self.declared_vars
            .insert(n.ident.sym.to_string(), n.ident.span());
        n.visit_children_with(self);
    }
}
