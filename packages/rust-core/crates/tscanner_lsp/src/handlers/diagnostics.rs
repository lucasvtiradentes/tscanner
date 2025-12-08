use crate::converters::issue_to_diagnostic;
use crate::session::Session;
use lsp_server::{Connection, Message, Notification};
use lsp_types::{Diagnostic, PublishDiagnosticsParams, Url};
use std::path::Path;
use tscanner_service::{log_debug, ScanContentParams, Workspace};

type LspError = Box<dyn std::error::Error + Send + Sync>;

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
            log_debug(&format!(
                "publish_diagnostics: {} issues for {}",
                result.issues.len(),
                path.display()
            ));
            result.issues.clone()
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

    let diagnostics: Vec<Diagnostic> = diags_with_rules.iter().map(|(d, _)| d.clone()).collect();

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
