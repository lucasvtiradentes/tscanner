use crate::custom_requests::ScanFileParams;
use crate::session::Session;
use lsp_server::{Connection, Message, Request, Response};
use tscanner_config::{config_dir_name, config_file_name};
use tscanner_scanner::{load_config, Scanner};

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_scan_file(
    connection: &Connection,
    req: Request,
    session: &mut Session,
) -> Result<(), LspError> {
    let params: ScanFileParams = serde_json::from_value(req.params)?;

    let config = match load_config(&params.root, config_dir_name(), config_file_name()) {
        Ok(c) => c,
        Err(e) => {
            let response = Response::new_err(
                req.id,
                lsp_server::ErrorCode::InternalError as i32,
                e.to_string(),
            );
            connection.sender.send(Message::Response(response))?;
            return Ok(());
        }
    };

    let scanner = match Scanner::with_cache(config, session.cache.clone(), params.root.clone()) {
        Ok(s) => s,
        Err(e) => {
            let response = Response::new_err(
                req.id,
                lsp_server::ErrorCode::InternalError as i32,
                format!("Failed to create scanner: {}", e),
            );
            connection.sender.send(Message::Response(response))?;
            return Ok(());
        }
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
