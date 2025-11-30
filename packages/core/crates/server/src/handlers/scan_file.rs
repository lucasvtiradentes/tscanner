use super::common::{create_scanner, error_response, load_config_from_workspace, success_response};
use crate::protocol::{Response, ScanFileParams};
use crate::state::ServerState;

pub fn handle_scan_file(
    request_id: u64,
    params: ScanFileParams,
    state: &mut ServerState,
) -> Response {
    core::log_debug(&format!("Scanning single file: {:?}", params.file));

    let config = match load_config_from_workspace(&params.root) {
        Ok(c) => c,
        Err(e) => return error_response(request_id, e),
    };

    let scanner = match create_scanner(config, state.cache.clone(), &params.root) {
        Ok(s) => s,
        Err(e) => return error_response(request_id, e),
    };

    state.scanner = Some(scanner);

    match state
        .scanner
        .as_ref()
        .and_then(|s| s.scan_single(&params.file))
    {
        Some(result) => match serde_json::to_value(&result) {
            Ok(value) => success_response(request_id, value),
            Err(e) => error_response(
                request_id,
                format!("Failed to serialize scan results: {}", e),
            ),
        },
        None => success_response(
            request_id,
            serde_json::json!({"file": params.file, "issues": []}),
        ),
    }
}
