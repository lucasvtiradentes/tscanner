use crate::converters::issue_to_diagnostic;
use crate::session::Session;
use lsp_server::{Connection, Message, Notification};
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, PublishDiagnosticsParams, Range, Url};
use std::path::Path;
use tscanner_constants::config_file_name;
use tscanner_service::{log_debug, ScanContentParams, Workspace};

type LspError = Box<dyn std::error::Error + Send + Sync>;

fn create_schema_version_diagnostic(content: &str, path: &Path) -> Option<Diagnostic> {
    if path.file_name()?.to_str()? != config_file_name() {
        return None;
    }

    let (line_index, schema_line) = content
        .lines()
        .enumerate()
        .find(|(_, line)| line.trim().starts_with("\"$schema\""))?;

    let schema_path_str = schema_line
        .split(':')
        .nth(1)?
        .trim()
        .trim_matches(|c| c == '"' || c == ',' || c == ' ');

    if schema_path_str.starts_with("http") {
        return None;
    }

    let config_dir = path.parent()?;
    let schema_path = config_dir.join(schema_path_str);

    if !schema_path.exists() {
        return None;
    }

    let schema_content = std::fs::read_to_string(&schema_path).ok()?;
    let schema_json: serde_json::Value = serde_json::from_str(&schema_content).ok()?;
    let schema_version = schema_json.get("version")?.as_str()?;

    let binary_version = env!("CARGO_PKG_VERSION");

    if schema_version == binary_version {
        return None;
    }

    let start_col = schema_line.find("\"$schema\"")?;
    let end_col = schema_line.len();

    Some(Diagnostic {
        range: Range {
            start: Position {
                line: line_index as u32,
                character: start_col as u32,
            },
            end: Position {
                line: line_index as u32,
                character: end_col as u32,
            },
        },
        severity: Some(DiagnosticSeverity::WARNING),
        code: None,
        code_description: None,
        source: Some("tscanner".to_string()),
        message: format!(
            "Schema version ({}) differs from binary version ({}). Update schema recommended.",
            schema_version, binary_version
        ),
        related_information: None,
        tags: None,
        data: None,
    })
}

pub fn publish_diagnostics(
    connection: &Connection,
    uri: &Url,
    path: &Path,
    content: &str,
    session: &mut Session,
) -> Result<(), LspError> {
    if !session.is_initialized() {
        return Ok(());
    }

    let scan_result = {
        let ws = session.workspace();
        let ws_guard = ws.lock().unwrap();
        ws_guard.scan_content(ScanContentParams {
            path: path.to_path_buf(),
            content: content.to_string(),
        })
    };

    let issues = match &scan_result {
        Ok(result) => {
            let filtered: Vec<_> = result
                .issues
                .iter()
                .filter(|issue| issue.file == path)
                .cloned()
                .collect();
            log_debug(&format!(
                "publish_diagnostics: {} issues for {} (filtered from {})",
                filtered.len(),
                path.display(),
                result.issues.len()
            ));
            filtered
        }
        Err(e) => {
            log_debug(&format!("publish_diagnostics error: {:?}", e));
            Vec::new()
        }
    };

    let diags_with_rules: Vec<(Diagnostic, String)> = issues
        .iter()
        .map(|issue| (issue_to_diagnostic(issue), issue.rule.clone()))
        .collect();

    let mut diagnostics: Vec<Diagnostic> =
        diags_with_rules.iter().map(|(d, _)| d.clone()).collect();

    if let Some(schema_diagnostic) = create_schema_version_diagnostic(content, path) {
        diagnostics.push(schema_diagnostic);
    }

    session.diagnostics.insert(uri.clone(), diags_with_rules);

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
    let params = PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics: vec![],
        version: None,
    };
    let notif = Notification {
        method: "textDocument/publishDiagnostics".to_string(),
        params: serde_json::to_value(params)?,
    };
    connection.sender.send(Message::Notification(notif))?;
    Ok(())
}

pub fn clear_all_diagnostics(
    connection: &Connection,
    session: &mut Session,
) -> Result<(), LspError> {
    for uri in session.open_files.keys() {
        clear_diagnostics(connection, uri)?;
    }
    session.diagnostics.clear();
    Ok(())
}
