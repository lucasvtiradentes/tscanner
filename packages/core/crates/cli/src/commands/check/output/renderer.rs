use crate::shared::SummaryStats;
use cli::OutputFormat;
use core::ScanResult;

use super::json::JsonRenderer;
use super::pretty::PrettyRenderer;
use super::text::TextRenderer;
use super::CheckContext;

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
