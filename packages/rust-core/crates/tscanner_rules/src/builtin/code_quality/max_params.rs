use crate::context::RuleContext;
use crate::metadata::{
    RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleOption, RuleOptionSchema, RuleType,
};
use crate::signals::{RuleDiagnostic, TextRange};
use crate::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use serde::Deserialize;
use std::sync::Arc;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

const DEFAULT_MAX_PARAMS: usize = 4;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MaxParamsOptions {
    #[serde(default = "default_max_params")]
    max_params: usize,
}

fn default_max_params() -> usize {
    DEFAULT_MAX_PARAMS
}

impl Default for MaxParamsOptions {
    fn default() -> Self {
        Self {
            max_params: DEFAULT_MAX_PARAMS,
        }
    }
}

pub struct TooManyParams {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

pub struct MaxParamsRule {
    max_params: usize,
}

impl MaxParamsRule {
    pub fn new(options: Option<&serde_json::Value>) -> Self {
        let max_params = options
            .and_then(|v| serde_json::from_value::<MaxParamsOptions>(v.clone()).ok())
            .map(|o| o.max_params)
            .unwrap_or(DEFAULT_MAX_PARAMS);

        Self { max_params }
    }
}

inventory::submit!(RuleRegistration {
    name: "max-params",
    factory: |options| Arc::new(MaxParamsRule::new(options)),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "max-params",
        display_name: "Max Parameters",
        description:
            "Limits the number of parameters in a function. Functions with many parameters should use an options object instead.",
        rule_type: RuleType::Ast,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/max-params"),
        equivalent_biome_rule: None,
        options: &[RuleOption {
            name: "maxParams",
            description: "Maximum number of parameters allowed in a function",
            schema: RuleOptionSchema::Integer {
                default: DEFAULT_MAX_PARAMS as i64,
                minimum: Some(1),
            },
        }],
        ..RuleMetadata::defaults()
    }
});

impl Rule for MaxParamsRule {
    type State = TooManyParams;

    fn name(&self) -> &'static str {
        "max-params"
    }

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State> {
        let mut visitor = MaxParamsVisitor {
            issues: Vec::new(),
            source: ctx.source(),
            max_params: self.max_params,
        };
        ctx.program().visit_with(&mut visitor);
        visitor.issues
    }

    fn diagnostic(&self, _ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic {
        RuleDiagnostic::new(
            TextRange::single_line(state.line, state.column, state.end_column),
            "Function has too many parameters. Consider using an options object.".to_string(),
        )
    }
}

struct MaxParamsVisitor<'a> {
    issues: Vec<TooManyParams>,
    source: &'a str,
    max_params: usize,
}

impl<'a> MaxParamsVisitor<'a> {
    fn check_param_count(&mut self, param_count: usize, span: swc_common::Span, _name: &str) {
        if param_count > self.max_params {
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(TooManyParams {
                line,
                column,
                end_column,
            });
        }
    }
}

impl<'a> Visit for MaxParamsVisitor<'a> {
    fn visit_function(&mut self, n: &Function) {
        self.check_param_count(n.params.len(), n.span, "anonymous");
        n.visit_children_with(self);
    }

    fn visit_fn_decl(&mut self, n: &FnDecl) {
        self.check_param_count(
            n.function.params.len(),
            n.function.span,
            n.ident.sym.as_ref(),
        );
        n.visit_children_with(self);
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        self.check_param_count(n.params.len(), n.span, "arrow");
        n.visit_children_with(self);
    }

    fn visit_class_method(&mut self, n: &ClassMethod) {
        let method_name = match &n.key {
            PropName::Ident(ident) => ident.sym.as_ref(),
            _ => "method",
        };
        self.check_param_count(n.function.params.len(), n.function.span, method_name);
        n.visit_children_with(self);
    }
}
