use core::{FileCache, Scanner, TscannerConfig};
use lsp_server::{Connection, Message, Request, Response};
use lsp_types::{
    CodeActionKind, CodeActionOptions, CodeActionParams, CodeActionProviderCapability,
    InitializeParams, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use super::code_actions::handle_code_action;
use super::notifications::handle_notification;
use super::state::LspState;

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn run_lsp_server() -> Result<(), LspError> {
    core::log_info("Starting TScanner LSP server");

    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
            code_action_kinds: Some(vec![CodeActionKind::QUICKFIX]),
            ..Default::default()
        })),
        ..Default::default()
    })?;

    let initialization_params = connection.initialize(server_capabilities)?;
    let params: InitializeParams = serde_json::from_value(initialization_params)?;

    let workspace_root = extract_workspace_root(&params);
    core::log_info(&format!("Workspace root: {:?}", workspace_root));

    let (config, scanner) = initialize_scanner(&workspace_root);
    let mut state = LspState::new(workspace_root, config, scanner);

    main_loop(&connection, &mut state)?;

    io_threads.join()?;
    core::log_info("LSP server stopped");
    Ok(())
}

#[allow(deprecated)]
fn extract_workspace_root(params: &InitializeParams) -> PathBuf {
    params
        .root_uri
        .as_ref()
        .and_then(|uri| uri.to_file_path().ok())
        .or_else(|| {
            params
                .workspace_folders
                .as_ref()
                .and_then(|folders| folders.first().and_then(|f| f.uri.to_file_path().ok()))
        })
        .unwrap_or_else(|| PathBuf::from("."))
}

fn initialize_scanner(workspace_root: &Path) -> (Option<TscannerConfig>, Option<Scanner>) {
    match TscannerConfig::load_from_workspace(workspace_root) {
        Ok(config) => {
            let config_hash = config.compute_hash();
            let cache = Arc::new(FileCache::with_config_hash(config_hash));
            match Scanner::with_cache(config.clone(), cache, workspace_root.to_path_buf()) {
                Ok(scanner) => (Some(config), Some(scanner)),
                Err(e) => {
                    core::log_error(&format!("Failed to create scanner: {}", e));
                    (None, None)
                }
            }
        }
        Err(e) => {
            core::log_error(&format!("Config error, LSP disabled: {}", e));
            (None, None)
        }
    }
}

fn main_loop(connection: &Connection, state: &mut LspState) -> Result<(), LspError> {
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                handle_request(connection, req, state)?;
            }
            Message::Notification(notif) => {
                handle_notification(connection, notif, state)?;
            }
            Message::Response(_) => {}
        }
    }
    Ok(())
}

fn handle_request(
    connection: &Connection,
    req: Request,
    state: &mut LspState,
) -> Result<(), LspError> {
    if req.method == "textDocument/codeAction" {
        let params: CodeActionParams = serde_json::from_value(req.params)?;
        let actions = handle_code_action(params, state);
        let response = Response::new_ok(req.id, actions);
        connection.sender.send(Message::Response(response))?;
    }
    Ok(())
}
