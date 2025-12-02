use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use tscanner_diagnostics::{Issue, Severity};

pub fn issue_to_diagnostic(issue: &Issue) -> Diagnostic {
    let line = (issue.line.saturating_sub(1)) as u32;
    let column = (issue.column.saturating_sub(1)) as u32;
    let end_column = (issue.end_column.saturating_sub(1)) as u32;

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
        }),
        code: Some(lsp_types::NumberOrString::String(issue.rule.clone())),
        source: Some("tscanner".to_string()),
        message: issue.message.clone(),
        ..Default::default()
    }
}
