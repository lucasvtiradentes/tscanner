use core::{Issue, Severity};
use lsp_server::{Connection, Message, Notification};
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, PublishDiagnosticsParams, Range, Url};
use std::path::Path;

use super::state::LspState;

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn publish_diagnostics(
    connection: &Connection,
    uri: &Url,
    path: &Path,
    content: &str,
    state: &mut LspState,
) -> Result<(), LspError> {
    let Some(scanner) = &state.scanner else {
        return Ok(());
    };

    let Some(config) = &state.config else {
        return Ok(());
    };

    let issues: Vec<Issue> = scanner
        .scan_content(path, content)
        .map(|result| result.issues)
        .unwrap_or_default();

    let lsp_config = config.lsp.as_ref();
    let show_errors = lsp_config.map(|c| c.errors).unwrap_or(true);
    let show_warnings = lsp_config.map(|c| c.warnings).unwrap_or(true);

    let filtered_issues: Vec<&Issue> = issues
        .iter()
        .filter(|issue| match issue.severity {
            Severity::Error => show_errors,
            Severity::Warning => show_warnings,
        })
        .collect();

    let diags_with_rules: Vec<(Diagnostic, String)> = filtered_issues
        .iter()
        .map(|issue| (issue_to_diagnostic(issue), issue.rule.clone()))
        .collect();

    let diagnostics: Vec<Diagnostic> = diags_with_rules.iter().map(|(d, _)| d.clone()).collect();

    state.diagnostics.insert(uri.clone(), diags_with_rules);

    let params = PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics,
        version: None,
    };

    let notif = Notification {
        method: "textDocument/publishDiagnostics".to_string(),
        params: serde_json::to_value(params)?,
    };

    connection.sender.send(Message::Notification(notif))?;
    Ok(())
}

pub fn clear_diagnostics(connection: &Connection, uri: &Url) -> Result<(), LspError> {
    let clear_params = PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics: vec![],
        version: None,
    };
    let notif = Notification {
        method: "textDocument/publishDiagnostics".to_string(),
        params: serde_json::to_value(clear_params)?,
    };
    connection.sender.send(Message::Notification(notif))?;
    Ok(())
}

pub fn clear_all_diagnostics(
    connection: &Connection,
    state: &mut LspState,
) -> Result<(), LspError> {
    for uri in state.open_files.keys() {
        clear_diagnostics(connection, uri)?;
    }
    state.diagnostics.clear();
    Ok(())
}

fn issue_to_diagnostic(issue: &Issue) -> Diagnostic {
    let line = (issue.line.saturating_sub(1)) as u32;
    let column = (issue.column.saturating_sub(1)) as u32;
    let end_column = (issue.end_column.saturating_sub(1)) as u32;

    let range = Range {
        start: Position {
            line,
            character: column,
        },
        end: Position {
            line,
            character: end_column,
        },
    };

    let severity = match issue.severity {
        Severity::Error => DiagnosticSeverity::ERROR,
        Severity::Warning => DiagnosticSeverity::WARNING,
    };

    Diagnostic {
        range,
        severity: Some(severity),
        code: Some(lsp_types::NumberOrString::String(issue.rule.clone())),
        source: Some("tscanner".to_string()),
        message: issue.message.clone(),
        ..Default::default()
    }
}
