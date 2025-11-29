mod json;
mod pretty;
mod text;

pub use json::JsonRenderer;
pub use pretty::PrettyRenderer;
pub use text::TextRenderer;

use super::context::CheckContext;
use crate::shared::SummaryStats;
use core::types::ScanResult;

pub trait OutputRenderer {
    fn render(&self, ctx: &CheckContext, result: &ScanResult, stats: &SummaryStats);
}

pub fn get_renderer(json: bool, pretty: bool) -> Box<dyn OutputRenderer> {
    match (json, pretty) {
        (true, _) => Box::new(JsonRenderer),
        (_, true) => Box::new(PrettyRenderer),
        _ => Box::new(TextRenderer),
    }
}
