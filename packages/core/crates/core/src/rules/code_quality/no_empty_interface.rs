use crate::output::{Issue, Severity};
use crate::rules::metadata::RuleType;
use crate::rules::metadata::{RuleCategory, RuleMetadata, RuleMetadataRegistration};
use crate::rules::traits::{Rule, RuleRegistration};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoEmptyInterfaceRule;

inventory::submit!(RuleRegistration {
    name: "no-empty-interface",
    factory: |_| Arc::new(NoEmptyInterfaceRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-empty-interface",
        display_name: "No Empty Interface",
        description: "Disallows empty interface declarations. Empty interfaces are equivalent to {} and usually indicate incomplete code.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: true,
        equivalent_eslint_rule: Some("https://typescript-eslint.io/rules/no-empty-interface"),
        equivalent_biome_rule: Some("https://biomejs.dev/linter/rules/no-empty-interface"),
        allowed_options: &[],
    }
});

impl Rule for NoEmptyInterfaceRule {
    fn name(&self) -> &str {
        "no-empty-interface"
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
        let mut visitor = EmptyInterfaceVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct EmptyInterfaceVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for EmptyInterfaceVisitor<'a> {
    fn visit_ts_interface_decl(&mut self, n: &TsInterfaceDecl) {
        if n.body.body.is_empty() {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "no-empty-interface".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: "Empty interface declaration".to_string(),
                severity: Severity::Warning,
                line_text: None,
            });
        }
        n.visit_children_with(self);
    }
}

impl<'a> EmptyInterfaceVisitor<'a> {}
