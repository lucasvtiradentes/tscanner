use core::{config_dir_name, config_file_name};
use lsp_server::{Connection, Notification};
use lsp_types::{
    DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, FileChangeType,
};

use super::diagnostics::{clear_all_diagnostics, clear_diagnostics, publish_diagnostics};
use super::state::LspState;

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_notification(
    connection: &Connection,
    notif: Notification,
    state: &mut LspState,
) -> Result<(), LspError> {
    match notif.method.as_str() {
        "textDocument/didOpen" => handle_did_open(connection, notif, state),
        "textDocument/didChange" => handle_did_change(connection, notif, state),
        "textDocument/didSave" => handle_did_save(connection, notif, state),
        "textDocument/didClose" => handle_did_close(connection, notif, state),
        "workspace/didChangeWatchedFiles" => handle_watched_files_change(connection, notif, state),
        _ => Ok(()),
    }
}

fn handle_did_open(
    connection: &Connection,
    notif: Notification,
    state: &mut LspState,
) -> Result<(), LspError> {
    let params: DidOpenTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;
    let content = params.text_document.text;

    if let Ok(path) = uri.to_file_path() {
        state.open_files.insert(uri.clone(), content.clone());
        publish_diagnostics(connection, &uri, &path, &content, state)?;
    }
    Ok(())
}

fn handle_did_change(
    connection: &Connection,
    notif: Notification,
    state: &mut LspState,
) -> Result<(), LspError> {
    let params: DidChangeTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;

    if let Some(change) = params.content_changes.first() {
        if let Ok(path) = uri.to_file_path() {
            let content = change.text.clone();
            state.open_files.insert(uri.clone(), content.clone());
            publish_diagnostics(connection, &uri, &path, &content, state)?;
        }
    }
    Ok(())
}

fn handle_did_save(
    connection: &Connection,
    notif: Notification,
    state: &mut LspState,
) -> Result<(), LspError> {
    let params: DidSaveTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;

    if let Ok(path) = uri.to_file_path() {
        if let Some(content) = state.open_files.get(&uri).cloned() {
            publish_diagnostics(connection, &uri, &path, &content, state)?;
        }
    }
    Ok(())
}

fn handle_did_close(
    connection: &Connection,
    notif: Notification,
    state: &mut LspState,
) -> Result<(), LspError> {
    let params: DidCloseTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri.clone();
    state.open_files.remove(&params.text_document.uri);
    state.diagnostics.remove(&params.text_document.uri);
    clear_diagnostics(connection, &uri)?;
    Ok(())
}

fn handle_watched_files_change(
    connection: &Connection,
    notif: Notification,
    state: &mut LspState,
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
                handle_config_reload(connection, state)?;
                break;
            }
        }
    }
    Ok(())
}

fn handle_config_reload(connection: &Connection, state: &mut LspState) -> Result<(), LspError> {
    core::log_info("Config file changed, reloading...");

    match state.reload_config() {
        Ok(()) => {
            for (uri, content) in state.open_files.clone() {
                if let Ok(path) = uri.to_file_path() {
                    publish_diagnostics(connection, &uri, &path, &content, state)?;
                }
            }
        }
        Err(e) => {
            core::log_error(&e);
            clear_all_diagnostics(connection, state)?;
        }
    }

    Ok(())
}
