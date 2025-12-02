use super::common::{create_scanner, error_response, load_config_with_fallback, success_response};
use crate::protocol::{Response, ScanContentParams};
use crate::state::ServerState;

pub fn handle_scan_content(
    request_id: u64,
    params: ScanContentParams,
    state: &mut ServerState,
) -> Response {
    let config = match load_config_with_fallback(params.config, &params.root) {
        Ok(c) => c,
        Err(e) => return error_response(request_id, e),
    };

    let scanner = match create_scanner(config, state.cache.clone(), &params.root) {
        Ok(s) => s,
        Err(e) => return error_response(request_id, e),
    };

    match scanner.scan_content(&params.file, &params.content) {
        Some(result) => match serde_json::to_value(&result) {
            Ok(value) => success_response(request_id, value),
            Err(e) => error_response(
                request_id,
                format!("Failed to serialize scan results: {}", e),
            ),
        },
        None => success_response(
            request_id,
            serde_json::json!({"file": params.file, "issues": [], "related_files": []}),
        ),
    }
}
