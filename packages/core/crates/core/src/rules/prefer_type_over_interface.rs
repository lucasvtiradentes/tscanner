use super::metadata::RuleType;
use super::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::output::{Issue, Severity};
use crate::rule::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct PreferTypeOverInterfaceRule;

inventory::submit!(RuleRegistration {
    name: "prefer-type-over-interface",
    factory: |_| Arc::new(PreferTypeOverInterfaceRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "prefer-type-over-interface",
        display_name: "Prefer Type Over Interface",
        description: "Suggests using 'type' keyword instead of 'interface' for consistency. Type aliases are more flexible and composable.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::Style,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/consistent-type-definitions"),
        equivalent_biome_rule: None,
        allowed_options: &[],
    }
});

impl Rule for PreferTypeOverInterfaceRule {
    fn name(&self) -> &str {
        "prefer-type-over-interface"
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
        let mut visitor = InterfaceVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct InterfaceVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for InterfaceVisitor<'a> {
    fn visit_ts_interface_decl(&mut self, n: &TsInterfaceDecl) {
        let span = n.span();
        let (line, column, end_column) =
            get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

        self.issues.push(Issue {
            rule: "prefer-type-over-interface".to_string(),
            file: self.path.clone(),
            line,
            column,
            end_column,
            message: format!("Prefer 'type' over 'interface' for '{}'", n.id.sym),
            severity: Severity::Warning,
            line_text: None,
        });

        n.visit_children_with(self);
    }
}

impl<'a> InterfaceVisitor<'a> {}
