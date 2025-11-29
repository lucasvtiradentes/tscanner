use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_span_positions;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoMagicNumbersRule;

inventory::submit!(RuleRegistration {
    name: "no-magic-numbers",
    factory: || Arc::new(NoMagicNumbersRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-magic-numbers",
        display_name: "No Magic Numbers",
        description: "Detects magic numbers in code (literals other than 0, 1, -1). Use named constants instead for better readability and maintainability.",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::CodeQuality,
        typescript_only: false,
    }
});

impl Rule for NoMagicNumbersRule {
    fn name(&self) -> &str {
        "no-magic-numbers"
    }

    fn check(
        &self,
        program: &Program,
        path: &Path,
        source: &str,
        _file_source: crate::file_source::FileSource,
    ) -> Vec<Issue> {
        let mut visitor = MagicNumberVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct MagicNumberVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for MagicNumberVisitor<'a> {
    fn visit_number(&mut self, n: &Number) {
        let value = n.value;

        if value != 0.0 && value != 1.0 && value != -1.0 {
            let span = n.span();
            let (line, column, end_column) =
                get_span_positions(self.source, span.lo.0 as usize, span.hi.0 as usize);

            self.issues.push(Issue {
                rule: "no-magic-numbers".to_string(),
                file: self.path.clone(),
                line,
                column,
                end_column,
                message: format!(
                    "Magic number '{}' found. Consider using a named constant instead",
                    value
                ),
                severity: Severity::Warning,
                line_text: None,
            });
        }
    }
}

impl<'a> MagicNumberVisitor<'a> {}
