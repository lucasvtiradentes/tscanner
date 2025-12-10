use super::helpers::{create_scanner_or_respond, load_config_or_respond};
use crate::custom_requests::ScanFileParams;
use crate::session::Session;
use lsp_server::{Connection, Message, Request, Response};

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_scan_file(
    connection: &Connection,
    req: Request,
    session: &mut Session,
) -> Result<(), LspError> {
    let params: ScanFileParams = serde_json::from_value(req.params)?;

    let Some(config) = load_config_or_respond(connection, &req.id, &params.root, None)? else {
        return Ok(());
    };

    let Some(scanner) = create_scanner_or_respond(
        connection,
        &req.id,
        config,
        session.cache.clone(),
        params.root.clone(),
    )?
    else {
        return Ok(());
    };

    session.scanner = Some(scanner);

    let result = session
        .scanner
        .as_ref()
        .and_then(|s| s.scan_single(&params.file));

    let response = match result {
        Some(file_result) => Response::new_ok(req.id, serde_json::to_value(&file_result)?),
        None => Response::new_ok(
            req.id,
            serde_json::json!({"file": params.file, "issues": []}),
        ),
    };

    connection.sender.send(Message::Response(response))?;
    Ok(())
}
