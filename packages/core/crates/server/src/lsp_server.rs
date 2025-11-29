use core::{FileCache, Issue, Scanner, Severity, TscannerConfig};
use lsp_server::{Connection, Message, Notification};
use lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, InitializeParams, Position,
    PublishDiagnosticsParams, Range, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, Url,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn run_lsp_server() -> Result<(), LspError> {
    core::log_info("Starting TScanner LSP server");

    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        ..Default::default()
    })?;

    let initialization_params = connection.initialize(server_capabilities)?;
    let params: InitializeParams = serde_json::from_value(initialization_params)?;

    #[allow(deprecated)]
    let workspace_root = params
        .root_uri
        .and_then(|uri| uri.to_file_path().ok())
        .or_else(|| {
            params
                .workspace_folders
                .and_then(|folders| folders.first().and_then(|f| f.uri.to_file_path().ok()))
        })
        .unwrap_or_else(|| PathBuf::from("."));

    core::log_info(&format!("Workspace root: {:?}", workspace_root));

    let config = TscannerConfig::load_from_workspace(&workspace_root).unwrap_or_default();
    let config_hash = config.compute_hash();
    let cache = Arc::new(FileCache::with_config_hash(config_hash));
    let scanner = Scanner::with_cache(config, cache.clone(), workspace_root.clone())
        .map_err(|e| -> LspError { e.to_string().into() })?;

    let mut state = LspState {
        workspace_root,
        scanner,
        open_files: HashMap::new(),
    };

    main_loop(&connection, &mut state)?;

    io_threads.join()?;
    core::log_info("LSP server stopped");
    Ok(())
}

struct LspState {
    #[allow(dead_code)]
    workspace_root: PathBuf,
    scanner: Scanner,
    open_files: HashMap<Url, String>,
}

fn main_loop(connection: &Connection, state: &mut LspState) -> Result<(), LspError> {
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
            }
            Message::Notification(notif) => {
                handle_notification(connection, notif, state)?;
            }
            Message::Response(_resp) => {}
        }
    }
    Ok(())
}

fn handle_notification(
    connection: &Connection,
    notif: Notification,
    state: &mut LspState,
) -> Result<(), LspError> {
    match notif.method.as_str() {
        "textDocument/didOpen" => {
            let params: DidOpenTextDocumentParams = serde_json::from_value(notif.params)?;
            let uri = params.text_document.uri;
            let content = params.text_document.text;

            if let Ok(path) = uri.to_file_path() {
                state.open_files.insert(uri.clone(), content.clone());
                publish_diagnostics(connection, &uri, &path, &content, state)?;
            }
        }
        "textDocument/didChange" => {
            let params: DidChangeTextDocumentParams = serde_json::from_value(notif.params)?;
            let uri = params.text_document.uri;

            if let Some(change) = params.content_changes.first() {
                if let Ok(path) = uri.to_file_path() {
                    let content = change.text.clone();
                    state.open_files.insert(uri.clone(), content.clone());
                    publish_diagnostics(connection, &uri, &path, &content, state)?;
                }
            }
        }
        "textDocument/didSave" => {
            let params: DidSaveTextDocumentParams = serde_json::from_value(notif.params)?;
            let uri = params.text_document.uri;

            if let Ok(path) = uri.to_file_path() {
                if let Some(content) = state.open_files.get(&uri) {
                    publish_diagnostics(connection, &uri, &path, content, state)?;
                }
            }
        }
        "textDocument/didClose" => {
            let params: DidCloseTextDocumentParams = serde_json::from_value(notif.params)?;
            let uri = params.text_document.uri.clone();
            state.open_files.remove(&params.text_document.uri);

            let clear_params = PublishDiagnosticsParams {
                uri,
                diagnostics: vec![],
                version: None,
            };
            let notif = Notification {
                method: "textDocument/publishDiagnostics".to_string(),
                params: serde_json::to_value(clear_params)?,
            };
            connection.sender.send(Message::Notification(notif))?;
        }
        _ => {}
    }
    Ok(())
}

fn publish_diagnostics(
    connection: &Connection,
    uri: &Url,
    path: &Path,
    content: &str,
    state: &LspState,
) -> Result<(), LspError> {
    let diagnostics: Vec<Diagnostic> = state
        .scanner
        .scan_content(path, content)
        .map(|result| result.issues.iter().map(issue_to_diagnostic).collect())
        .unwrap_or_default();

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
