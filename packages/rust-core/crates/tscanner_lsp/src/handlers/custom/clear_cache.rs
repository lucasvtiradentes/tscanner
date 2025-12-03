use crate::custom_requests::ClearCacheResult;
use crate::session::Session;
use lsp_server::{Connection, Message, Request, Response};

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_clear_cache(
    connection: &Connection,
    req: Request,
    session: &mut Session,
) -> Result<(), LspError> {
    session.cache.clear();
    if let Some(scanner) = &session.scanner {
        scanner.clear_script_cache();
    }

    let result = ClearCacheResult { cleared: true };
    let response = Response::new_ok(req.id, serde_json::to_value(&result)?);
    connection.sender.send(Message::Response(response))?;

    Ok(())
}
