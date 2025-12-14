use super::helpers::load_config_or_respond;
use crate::custom_requests::{AiProgressNotification, AiProgressParams, ScanParams};
use crate::session::Session;
use lsp_server::{Connection, Message, Notification as LspNotification, Request, Response};
use lsp_types::notification::Notification;
use std::path::PathBuf;
use std::sync::Arc;
use tscanner_cache::{AiCache, FileCache, ScriptCache};
use tscanner_config::AiExecutionMode;
use tscanner_constants::resolve_config_dir;
use tscanner_git::{
    get_changed_files, get_modified_lines, get_uncommitted_files, get_uncommitted_modified_lines,
};
use tscanner_scanner::{AiProgressCallback, ConfigExt, Scanner};

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

    let no_cache = params.no_cache.unwrap_or(false);
    let config_hash = config.compute_hash();
    let (cache, ai_cache, script_cache) = if no_cache {
        (
            Arc::new(FileCache::new()),
            Arc::new(AiCache::new()),
            Arc::new(ScriptCache::new()),
        )
    } else {
        (
            Arc::new(FileCache::with_config_hash(config_hash)),
            Arc::new(AiCache::with_config_hash(config_hash)),
            Arc::new(ScriptCache::with_config_hash(config_hash)),
        )
    };

    session.cache = cache.clone();

    let resolved_config_dir = resolve_config_dir(&PathBuf::from(&params.root), params.config_dir);
    let scanner = match Scanner::with_caches_and_config_dir(
        config,
        cache,
        ai_cache,
        script_cache,
        params.root.clone(),
        resolved_config_dir,
    ) {
        Ok(s) => s,
        Err(e) => {
            let response = Response::new_err(
                req.id.clone(),
                lsp_server::ErrorCode::InternalError as i32,
                format!("Failed to create scanner: {}", e),
            );
            connection.sender.send(Message::Response(response))?;
            return Ok(());
        }
    };

    let (changed_files, modified_lines) = if params.staged.unwrap_or(false) {
        match (
            get_uncommitted_files(&params.root),
            get_uncommitted_modified_lines(&params.root),
        ) {
            (Ok(files), Ok(lines)) => (Some(files), Some(lines)),
            (Err(e), _) | (_, Err(e)) => {
                let response = Response::new_err(
                    req.id,
                    lsp_server::ErrorCode::InternalError as i32,
                    format!("Failed to get uncommitted files: {}", e),
                );
                connection.sender.send(Message::Response(response))?;
                return Ok(());
            }
        }
    } else if let Some(ref branch_name) = params.branch {
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
        use tscanner_logger::log_debug;
        use tscanner_types::IssueRuleType;

        let before_files = result.files.len();
        let before_issues = result.total_issues;
        let script_issues_before: usize = result
            .files
            .iter()
            .flat_map(|f| &f.issues)
            .filter(|i| i.rule_type == IssueRuleType::CustomScript)
            .count();

        log_debug(&format!(
            "Before filtering: {} files with {} total issues ({} script issues)",
            before_files, before_issues, script_issues_before
        ));

        result.filter_by_modified_lines(line_filter);

        let after_files = result.files.len();
        let after_issues = result.total_issues;
        let script_issues_after: usize = result
            .files
            .iter()
            .flat_map(|f| &f.issues)
            .filter(|i| i.rule_type == IssueRuleType::CustomScript)
            .count();

        log_debug(&format!(
            "After filtering: {} files with {} issues ({} script issues) - removed {} files, {} issues ({} script)",
            after_files, after_issues, script_issues_after,
            before_files - after_files,
            before_issues - after_issues,
            script_issues_before - script_issues_after
        ));
    }

    session.scanner = Some(scanner);

    let response = Response::new_ok(req.id, serde_json::to_value(&result)?);
    connection.sender.send(Message::Response(response))?;

    Ok(())
}
