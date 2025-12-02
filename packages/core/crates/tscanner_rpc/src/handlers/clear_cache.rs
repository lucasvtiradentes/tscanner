use super::common::success_response;
use crate::protocol::Response;
use crate::state::ServerState;

pub fn handle_clear_cache(request_id: u64, state: &mut ServerState) -> Response {
    state.cache.clear();
    if let Some(scanner) = &state.scanner {
        scanner.clear_script_cache();
    }
    success_response(request_id, serde_json::json!({"cleared": true}))
}
