use crate::protocol::Response;
use crate::state::ServerState;

pub fn handle_clear_cache(request_id: u64, state: &mut ServerState) -> Response {
    core::log_info("Clearing file cache");
    state.cache.clear();
    Response {
        id: request_id,
        result: Some(serde_json::json!({"cleared": true})),
        error: None,
    }
}
