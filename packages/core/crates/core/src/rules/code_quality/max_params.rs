use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::rules::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use serde::Deserialize;
use std::path::Path;
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
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
        equivalent_eslint_rule: Some("https://eslint.org/docs/latest/rules/max-params"),
        equivalent_biome_rule: None,
        allowed_options: &["maxParams"],
    }
});

impl Rule for MaxParamsRule {
    fn name(&self) -> &str {
        "max-params"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::utils::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = MaxParamsVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
            max_params: self.max_params,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct MaxParamsVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
    max_params: usize,
}

impl<'a> MaxParamsVisitor<'a> {
    fn check_param_count(&mut self, param_count: usize, span: swc_common::Span, name: &str) {
        if param_count > self.max_params {
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "max-params".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: format!(
                    "Function '{}' has {} parameters (max: {}). Consider using an options object.",
                    name, param_count, self.max_params
                ),
                severity: Severity::Warning,
                line_text: None,
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
