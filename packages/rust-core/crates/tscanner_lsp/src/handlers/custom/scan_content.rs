use super::helpers::{create_scanner_or_respond, load_config_or_respond};
use crate::custom_requests::ScanContentParams;
use crate::session::Session;
use lsp_server::{Connection, Message, Request, Response};
use tscanner_git::{get_modified_lines, get_uncommitted_modified_lines};
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
        params.config_dir,
    )?
    else {
        return Ok(());
    };

    let mut result = scanner.scan_content(&params.file, &params.content);

    if let Some(ref branch) = params.branch {
        if let Ok(all_modified_lines) = get_modified_lines(&params.root, branch) {
            if let Some(file_modified_lines) = all_modified_lines.get(&params.file) {
                log_debug(&format!(
                    "handle_scan_content: [BRANCH] filtering by {} modified lines for {} (vs {})",
                    file_modified_lines.len(),
                    params.file.display(),
                    branch
                ));

                if let Some(ref mut content_result) = result {
                    let before_count = content_result.issues.len();
                    content_result
                        .issues
                        .retain(|issue| file_modified_lines.contains(&issue.line));
                    let after_count = content_result.issues.len();

                    log_debug(&format!(
                        "handle_scan_content: [BRANCH] filtered {} -> {} issues for {}",
                        before_count,
                        after_count,
                        params.file.display()
                    ));
                }
            }
        }
    } else if params.uncommitted.unwrap_or(false) {
        if let Ok(all_modified_lines) = get_uncommitted_modified_lines(&params.root) {
            if let Some(file_modified_lines) = all_modified_lines.get(&params.file) {
                log_debug(&format!(
                    "handle_scan_content: [UNCOMMITTED] filtering by {} modified lines for {}",
                    file_modified_lines.len(),
                    params.file.display()
                ));

                if let Some(ref mut content_result) = result {
                    let before_count = content_result.issues.len();
                    content_result
                        .issues
                        .retain(|issue| file_modified_lines.contains(&issue.line));
                    let after_count = content_result.issues.len();

                    log_debug(&format!(
                        "handle_scan_content: [UNCOMMITTED] filtered {} -> {} issues for {}",
                        before_count,
                        after_count,
                        params.file.display()
                    ));
                }
            }
        }
    }

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
