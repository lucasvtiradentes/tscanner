use super::helpers::{create_scanner_or_respond, load_config_or_respond};
use crate::custom_requests::{AiProgressNotification, AiProgressParams, ScanParams};
use crate::session::Session;
use lsp_server::{Connection, Message, Notification as LspNotification, Request, Response};
use lsp_types::notification::Notification;
use std::sync::Arc;
use tscanner_cache::FileCache;
use tscanner_config::AiExecutionMode;
use tscanner_fs::{get_changed_files, get_modified_lines};
use tscanner_scanner::{AiProgressCallback, ConfigExt};

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_scan(
    connection: &Connection,
    req: Request,
    session: &mut Session,
) -> Result<(), LspError> {
    let params: ScanParams = serde_json::from_value(req.params)?;

    let Some(config) = load_config_or_respond(connection, &req.id, &params.root, params.config)?
    else {
        return Ok(());
    };

    let config_hash = config.compute_hash();
    let cache = Arc::new(FileCache::with_config_hash(config_hash));
    session.cache = cache.clone();

    let Some(scanner) =
        create_scanner_or_respond(connection, &req.id, config, cache, params.root.clone())?
    else {
        return Ok(());
    };

    let (changed_files, modified_lines) = if let Some(ref branch_name) = params.branch {
        match (
            get_changed_files(&params.root, branch_name),
            get_modified_lines(&params.root, branch_name),
        ) {
            (Ok(files), Ok(lines)) => (Some(files), Some(lines)),
            (Err(e), _) | (_, Err(e)) => {
                let response = Response::new_err(
                    req.id,
                    lsp_server::ErrorCode::InternalError as i32,
                    format!("Failed to get changed files: {}", e),
                );
                connection.sender.send(Message::Response(response))?;
                return Ok(());
            }
        }
    } else {
        (None, None)
    };

    let ai_mode = params.ai_mode.unwrap_or(AiExecutionMode::Ignore);

    let progress_callback: Option<AiProgressCallback> = if ai_mode != AiExecutionMode::Ignore {
        let sender = connection.sender.clone();
        Some(Arc::new(move |event| {
            let params: AiProgressParams = event.into();
            let notification = LspNotification::new(
                AiProgressNotification::METHOD.to_string(),
                serde_json::to_value(params).unwrap_or_default(),
            );
            let _ = sender.send(Message::Notification(notification));
        }))
    } else {
        None
    };

    let mut result = scanner.scan_codebase_with_progress(
        std::slice::from_ref(&params.root),
        changed_files.as_ref(),
        ai_mode,
        modified_lines.as_ref(),
        progress_callback,
    );

    if let Some(ref line_filter) = modified_lines {
        result.filter_by_modified_lines(line_filter);
    }

    session.scanner = Some(scanner);

    let response = Response::new_ok(req.id, serde_json::to_value(&result)?);
    connection.sender.send(Message::Response(response))?;

    Ok(())
}
