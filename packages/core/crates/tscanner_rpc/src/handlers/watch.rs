use super::common::{error_response, success_response};
use crate::protocol::{Response, WatchParams};
use crate::state::ServerState;
use tscanner_fs::FileWatcher;

pub fn handle_watch(request_id: u64, params: WatchParams, state: &mut ServerState) -> Response {
    match FileWatcher::new(&params.root) {
        Ok(watcher) => {
            state.watcher = Some(watcher);
            success_response(request_id, serde_json::json!({"status": "watching"}))
        }
        Err(e) => error_response(request_id, format!("Failed to start watcher: {}", e)),
    }
}
