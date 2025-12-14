use crate::capabilities::server_capabilities;
use crate::handlers::custom::{
    handle_clear_cache, handle_format_results, handle_get_rules_metadata, handle_scan,
    handle_scan_content, handle_scan_file, handle_validate_config,
};
use crate::handlers::{handle_code_action, handle_notification};
use crate::scheduler::AnalysisScheduler;
use crate::session::Session;
use lsp_server::{Connection, Message, Request, Response};
use lsp_types::{CodeActionParams, InitializeParams, ServerInfo};
use std::path::PathBuf;
use tscanner_constants::{
    lsp_method_clear_cache, lsp_method_format_results, lsp_method_get_rules_metadata,
    lsp_method_scan, lsp_method_scan_content, lsp_method_scan_file, lsp_method_validate_config,
};

const TSCANNER_VERSION: &str = env!("CARGO_PKG_VERSION");

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn run_lsp_server() -> Result<(), LspError> {
    let (connection, io_threads) = Connection::stdio();

    let server_info = ServerInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: Some(TSCANNER_VERSION.to_string()),
    };

    let init_result = serde_json::json!({
        "capabilities": server_capabilities(),
        "serverInfo": server_info,
    });

    let initialization_params = connection.initialize(init_result)?;
    let params: InitializeParams = serde_json::from_value(initialization_params)?;

    let workspace_root = extract_workspace_root(&params);
    let mut session = Session::new();

    if let Some(root) = workspace_root {
        session.set_root(root);
    }

    let scheduler = AnalysisScheduler::new(connection.sender.clone(), session.workspace());

    main_loop(&connection, &mut session, &scheduler)?;

    io_threads.join()?;
    Ok(())
}

fn extract_workspace_root(params: &InitializeParams) -> Option<PathBuf> {
    params
        .workspace_folders
        .as_ref()
        .and_then(|folders| folders.first().and_then(|f| f.uri.to_file_path().ok()))
}

fn main_loop(
    connection: &Connection,
    session: &mut Session,
    scheduler: &AnalysisScheduler,
) -> Result<(), LspError> {
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                handle_request(connection, req, session)?;
            }
            Message::Notification(notif) => {
                handle_notification(connection, notif, session, scheduler)?;
            }
            Message::Response(_) => {}
        }
    }
    Ok(())
}

fn handle_request(
    connection: &Connection,
    req: Request,
    session: &mut Session,
) -> Result<(), LspError> {
    match req.method.as_str() {
        "textDocument/codeAction" => {
            let params: CodeActionParams = serde_json::from_value(req.params.clone())?;
            let actions = handle_code_action(params, session);
            let response = Response::new_ok(req.id, actions);
            connection.sender.send(Message::Response(response))?;
        }
        method if method == lsp_method_scan() => {
            handle_scan(connection, req, session)?;
        }
        method if method == lsp_method_scan_file() => {
            handle_scan_file(connection, req, session)?;
        }
        method if method == lsp_method_scan_content() => {
            handle_scan_content(connection, req, session)?;
        }
        method if method == lsp_method_clear_cache() => {
            handle_clear_cache(connection, req, session)?;
        }
        method if method == lsp_method_get_rules_metadata() => {
            handle_get_rules_metadata(connection, req)?;
        }
        method if method == lsp_method_format_results() => {
            handle_format_results(connection, req)?;
        }
        method if method == lsp_method_validate_config() => {
            handle_validate_config(connection, req, session)?;
        }
        _ => {}
    }
    Ok(())
}
