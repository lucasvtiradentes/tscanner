use super::common::{error_response, success_response};
use crate::protocol::Response;
use tscanner_rules::get_all_rule_metadata;

pub fn handle_get_rules_metadata(request_id: u64) -> Response {
    let metadata = get_all_rule_metadata();
    match serde_json::to_value(&metadata) {
        Ok(value) => success_response(request_id, value),
        Err(e) => error_response(request_id, format!("Failed to serialize metadata: {}", e)),
    }
}
