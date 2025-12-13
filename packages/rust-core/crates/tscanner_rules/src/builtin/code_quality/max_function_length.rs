use crate::context::RuleContext;
use crate::metadata::{
    RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleOption, RuleOptionSchema, RuleType,
};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::count_statements;
use crate::utils::get_span_positions;
use serde::Deserialize;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

const DEFAULT_MAX_LENGTH: usize = 50;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MaxFunctionLengthOptions {
    #[serde(default = "default_max_length")]
    max_length: usize,
}

fn default_max_length() -> usize {
    DEFAULT_MAX_LENGTH
}

impl Default for MaxFunctionLengthOptions {
    fn default() -> Self {
        Self {
            max_length: DEFAULT_MAX_LENGTH,
        }
    }
}

pub struct LongFunction {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct MaxFunctionLengthRule {
    max_length: usize,
}

impl MaxFunctionLengthRule {
    pub fn new(options: Option<&serde_json::Value>) -> Self {
        let max_length = options
            .and_then(|v| serde_json::from_value::<MaxFunctionLengthOptions>(v.clone()).ok())
            .map(|o| o.max_length)
            .unwrap_or(DEFAULT_MAX_LENGTH);

        Self { max_length }
    }
}

inventory::submit!(RuleRegistration {
    name: "max-function-length",
    factory: |options| Arc::new(MaxFunctionLengthRule::new(options)),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "max-function-length",
        display_name: "Max Function Length",
        description:
            "Enforces a maximum number of statements in functions. Long functions are harder to understand and maintain.",
        rule_type: RuleType::Ast,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/max-lines-per-function"),
        equivalent_biome_rule: None,
        options: &[RuleOption {
            name: "maxLength",
            description: "Maximum number of statements allowed in a function",
            schema: RuleOptionSchema::Integer {
                default: DEFAULT_MAX_LENGTH as i64,
                minimum: Some(1),
            },
        }],
        ..RuleMetadata::defaults()
    }
});

impl Rule for MaxFunctionLengthRule {
    type State = LongFunction;

    fn name(&self) -> &'static str {
        "max-function-length"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = MaxFunctionLengthVisitor {
            issues: Vec::new(),
            source: ctx.source(),
            max_length: self.max_length,
        };
        ctx.program().visit_with(&mut visitor);
        visitor.issues
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Function is too long. Consider breaking it into smaller functions.".to_string(),
        )
    }
}

struct MaxFunctionLengthVisitor<'a> {
    issues: Vec<LongFunction>,
    source: &'a str,
    max_length: usize,
}

impl<'a> MaxFunctionLengthVisitor<'a> {
    fn check_function_length(&mut self, body: &BlockStmt, span: swc_common::Span, _name: &str) {
        let stmt_count = count_statements(&body.stmts);

        if stmt_count > self.max_length {
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(LongFunction {
                line,
                column,
                end_column,
            });
        }
    }
}

impl<'a> Visit for MaxFunctionLengthVisitor<'a> {
    fn visit_function(&mut self, n: &Function) {
        if let Some(body) = &n.body {
            self.check_function_length(body, n.span, "anonymous");
        }
        n.visit_children_with(self);
    }

    fn visit_fn_decl(&mut self, n: &FnDecl) {
        if let Some(body) = &n.function.body {
            self.check_function_length(body, n.function.span, n.ident.sym.as_ref());
        }
        n.visit_children_with(self);
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        if let BlockStmtOrExpr::BlockStmt(body) = &*n.body {
            self.check_function_length(body, n.span, "arrow");
        }
        n.visit_children_with(self);
    }
}
