use lsp_server::{Connection, Message, Request, Response};
use tscanner_rules::get_all_rule_metadata;

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_get_rules_metadata(connection: &Connection, req: Request) -> Result<(), LspError> {
    let metadata = get_all_rule_metadata();
    let response = Response::new_ok(req.id, serde_json::to_value(&metadata)?);
    connection.sender.send(Message::Response(response))?;

    Ok(())
}
