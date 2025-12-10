use super::helpers::{create_scanner_or_respond, load_config_or_respond};
use crate::custom_requests::ScanContentParams;
use crate::session::Session;
use lsp_server::{Connection, Message, Request, Response};
use tscanner_service::log_debug;

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_scan_content(
    connection: &Connection,
    req: Request,
    session: &mut Session,
) -> Result<(), LspError> {
    let params: ScanContentParams = serde_json::from_value(req.params)?;

    let Some(config) = load_config_or_respond(connection, &req.id, &params.root, params.config)?
    else {
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

    let result = scanner.scan_content(&params.file, &params.content);
    log_debug(&format!(
        "handle_scan_content: {} -> {:?} issues",
        params.file.display(),
        result.as_ref().map(|r| r.issues.len())
    ));

    let response = match result {
        Some(content_result) => Response::new_ok(req.id, serde_json::to_value(&content_result)?),
        None => Response::new_ok(
            req.id,
            serde_json::json!({"file": params.file, "issues": [], "related_files": []}),
        ),
    };

    connection.sender.send(Message::Response(response))?;
    Ok(())
}
