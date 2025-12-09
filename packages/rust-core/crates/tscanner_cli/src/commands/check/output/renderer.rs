use crate::shared::FormattedOutput;
use tscanner_cli::OutputFormat;
use tscanner_diagnostics::ScanResult;

use super::json::JsonRenderer;
use super::text::TextRenderer;
use super::CheckContext;

pub trait OutputRenderer {
    fn render(&self, ctx: &CheckContext, output: &FormattedOutput, result: &ScanResult);
}

pub fn get_renderer(format: &OutputFormat) -> Box<dyn OutputRenderer> {
    match format {
        OutputFormat::Json => Box::new(JsonRenderer),
        OutputFormat::Text => Box::new(TextRenderer),
    }
}
