use crate::capabilities::server_capabilities;
use crate::handlers::{handle_code_action, handle_notification};
use crate::session::Session;
use lsp_server::{Connection, Message, Request, Response};
use lsp_types::{CodeActionParams, InitializeParams};
use std::path::PathBuf;

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn run_lsp_server() -> Result<(), LspError> {
    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(server_capabilities())?;
    let initialization_params = connection.initialize(server_capabilities)?;
    let params: InitializeParams = serde_json::from_value(initialization_params)?;

    let workspace_root = extract_workspace_root(&params);
    let mut session = Session::new();

    if let Some(root) = workspace_root {
        session.set_root(root);
    }

    main_loop(&connection, &mut session)?;

    io_threads.join()?;
    Ok(())
}

#[allow(deprecated)]
fn extract_workspace_root(params: &InitializeParams) -> Option<PathBuf> {
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
}

fn main_loop(connection: &Connection, session: &mut Session) -> Result<(), LspError> {
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                handle_request(connection, req, session)?;
            }
            Message::Notification(notif) => {
                handle_notification(connection, notif, session)?;
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
    if req.method == "textDocument/codeAction" {
        let params: CodeActionParams = serde_json::from_value(req.params)?;
        let actions = handle_code_action(params, session);
        let response = Response::new_ok(req.id, actions);
        connection.sender.send(Message::Response(response))?;
    }
    Ok(())
}
