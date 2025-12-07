use crate::custom_requests::ScanContentParams;
use crate::session::Session;
use lsp_server::{Connection, Message, Request, Response};
use tscanner_config::{config_dir_name, config_file_name};
use tscanner_scanner::{load_config, Scanner};
use tscanner_service::log_debug;

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_scan_content(
    connection: &Connection,
    req: Request,
    session: &mut Session,
) -> Result<(), LspError> {
    let params: ScanContentParams = serde_json::from_value(req.params)?;

    let config = if let Some(cfg) = params.config {
        cfg
    } else {
        match load_config(&params.root, config_dir_name(), config_file_name()) {
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
