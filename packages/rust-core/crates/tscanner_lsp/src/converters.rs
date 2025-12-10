use lsp_types::{CodeDescription, Diagnostic, DiagnosticSeverity, Position, Range, Url};
use tscanner_constants::{app_name, rules_base_url};
use tscanner_types::{Issue, Severity};

fn get_rule_url(rule: &str, category: Option<&str>) -> Option<Url> {
    let category_folder = category?;
    let rule_file = rule.replace('-', "_");
    let url_str = format!("{}/{}/{}.rs", rules_base_url(), category_folder, rule_file);
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
        source: Some(app_name().to_string()),
        message: issue.message.clone(),
        ..Default::default()
    }
}
