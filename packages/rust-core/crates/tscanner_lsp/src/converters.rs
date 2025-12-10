use lsp_types::{CodeDescription, Diagnostic, DiagnosticSeverity, Position, Range, Url};
use tscanner_types::{Issue, Severity};

const RULES_BASE_URL: &str = "https://github.com/lucasvtiradentes/tscanner/blob/main/packages/rust-core/crates/tscanner_rules/src/builtin";

fn get_rule_url(rule: &str, category: Option<&str>) -> Option<Url> {
    let category_folder = category?;
    let rule_file = rule.replace('-', "_");
    let url_str = format!("{}/{}/{}.rs", RULES_BASE_URL, category_folder, rule_file);
    Url::parse(&url_str).ok()
}

pub fn issue_to_diagnostic(issue: &Issue) -> Diagnostic {
    let line = (issue.line.saturating_sub(1)) as u32;
    let column = (issue.column.saturating_sub(1)) as u32;
    let end_column = (issue.end_column.saturating_sub(1)) as u32;

    let code_description =
        get_rule_url(&issue.rule, issue.category.as_deref()).map(|href| CodeDescription { href });

    Diagnostic {
        range: Range {
            start: Position {
                line,
                character: column,
            },
            end: Position {
                line,
                character: end_column,
            },
        },
        severity: Some(match issue.severity {
            Severity::Error => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
            Severity::Info => DiagnosticSeverity::INFORMATION,
            Severity::Hint => DiagnosticSeverity::HINT,
        }),
        code: Some(lsp_types::NumberOrString::String(issue.rule.clone())),
        code_description,
        source: Some("tscanner".to_string()),
        message: issue.message.clone(),
        ..Default::default()
    }
}
