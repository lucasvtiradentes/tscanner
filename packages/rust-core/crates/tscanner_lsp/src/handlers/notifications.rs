use crate::scheduler::{AnalysisRequest, AnalysisScheduler};
use crate::session::Session;
use lsp_server::{Connection, Notification};
use lsp_types::{
    DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, FileChangeType,
};
use tscanner_config::{config_dir_name, config_file_name};

use super::diagnostics::{clear_all_diagnostics, clear_diagnostics, publish_diagnostics};

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_notification(
    connection: &Connection,
    notif: Notification,
    session: &mut Session,
    scheduler: &AnalysisScheduler,
) -> Result<(), LspError> {
    match notif.method.as_str() {
        "textDocument/didOpen" => handle_did_open(connection, notif, session),
        "textDocument/didChange" => handle_did_change(notif, session, scheduler),
        "textDocument/didSave" => handle_did_save(connection, notif, session, scheduler),
        "textDocument/didClose" => handle_did_close(connection, notif, session, scheduler),
        "workspace/didChangeWatchedFiles" => {
            handle_watched_files_change(connection, notif, session)
        }
        _ => Ok(()),
    }
}

fn handle_did_open(
    connection: &Connection,
    notif: Notification,
    session: &mut Session,
) -> Result<(), LspError> {
    let params: DidOpenTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;
    let content = params.text_document.text;
    let version = params.text_document.version;

    if let Ok(path) = uri.to_file_path() {
        session.open_document(&uri, content.clone(), version);
        publish_diagnostics(connection, &uri, &path, &content, session)?;
    }
    Ok(())
}

fn handle_did_change(
    notif: Notification,
    session: &mut Session,
    scheduler: &AnalysisScheduler,
) -> Result<(), LspError> {
    let params: DidChangeTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;
    let version = params.text_document.version;

    if let Some(change) = params.content_changes.into_iter().last() {
        if let Ok(path) = uri.to_file_path() {
            let content = change.text;
            session.update_document(&uri, content.clone(), version);

            scheduler.schedule(AnalysisRequest {
                uri,
                path,
                content,
                version,
            });
        }
    }
    Ok(())
}

fn handle_did_save(
    connection: &Connection,
    notif: Notification,
    session: &mut Session,
    scheduler: &AnalysisScheduler,
) -> Result<(), LspError> {
    let params: DidSaveTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;

    scheduler.cancel(&uri);

    if let Ok(path) = uri.to_file_path() {
        if let Some(doc) = session.get_document(&uri) {
            publish_diagnostics(connection, &uri, &path, &doc.content.clone(), session)?;
        }
    }
    Ok(())
}

fn handle_did_close(
    connection: &Connection,
    notif: Notification,
    session: &mut Session,
    scheduler: &AnalysisScheduler,
) -> Result<(), LspError> {
    let params: DidCloseTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri.clone();

    scheduler.cancel(&uri);
    session.close_document(&uri);
    clear_diagnostics(connection, &uri)?;
    Ok(())
}

fn handle_watched_files_change(
    connection: &Connection,
    notif: Notification,
    session: &mut Session,
) -> Result<(), LspError> {
    let params: DidChangeWatchedFilesParams = serde_json::from_value(notif.params)?;

    for change in params.changes {
        if let Ok(path) = change.uri.to_file_path() {
            let is_config_file = path
                .file_name()
                .map(|n| n == config_file_name())
                .unwrap_or(false)
                && path
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|n| n == config_dir_name())
                    .unwrap_or(false);

            if is_config_file
                && (change.typ == FileChangeType::CREATED || change.typ == FileChangeType::CHANGED)
            {
                handle_config_reload(connection, session)?;
                break;
            }
        }
    }
    Ok(())
}

fn handle_config_reload(connection: &Connection, session: &mut Session) -> Result<(), LspError> {
    match session.reload_config() {
        Ok(()) => {
            for (uri, doc) in session.open_files.clone() {
                if let Ok(path) = uri.to_file_path() {
                    publish_diagnostics(connection, &uri, &path, &doc.content, session)?;
                }
            }
        }
        Err(_) => {
            clear_all_diagnostics(connection, session)?;
        }
    }

    Ok(())
}
