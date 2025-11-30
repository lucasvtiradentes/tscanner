mod json;
mod pretty;
mod text;

pub use json::JsonRenderer;
pub use pretty::PrettyRenderer;
pub use text::TextRenderer;

use crate::shared::SummaryStats;
use cli::OutputFormat;
use core::types::ScanResult;

use super::context::CheckContext;

pub trait OutputRenderer {
    fn render(&self, ctx: &CheckContext, result: &ScanResult, stats: &SummaryStats);
}

pub fn get_renderer(format: &OutputFormat) -> Box<dyn OutputRenderer> {
    match format {
        OutputFormat::Json => Box::new(JsonRenderer),
        OutputFormat::Pretty => Box::new(PrettyRenderer),
        OutputFormat::Text => Box::new(TextRenderer),
    }
}
