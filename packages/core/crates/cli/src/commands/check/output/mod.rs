mod json;
mod pretty;
mod text;

pub use json::JsonRenderer;
pub use pretty::PrettyRenderer;
pub use text::TextRenderer;

use crate::shared::SummaryStats;
use cli::OutputFormat;
use colored::*;
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

pub fn render_summary(result: &ScanResult, stats: &SummaryStats) {
    let files_with_issues = result.files.iter().filter(|f| !f.issues.is_empty()).count();

    println!();
    println!(
        "{} {} ({} errors, {} warnings)",
        "Issues:".dimmed(),
        stats.total_issues.to_string().cyan(),
        stats.error_count.to_string().red(),
        stats.warning_count.to_string().yellow()
    );
    println!(
        "{} {}/{} ({} cached, {} scanned)",
        "Files with issues:".dimmed(),
        files_with_issues.to_string().cyan(),
        result.total_files,
        result.cached_files.to_string().green(),
        result.scanned_files.to_string().yellow()
    );
    println!(
        "{} {}/{}",
        "Triggered rules:".dimmed(),
        stats.unique_rules_count.to_string().cyan(),
        stats.total_enabled_rules
    );
    println!("{} {}ms", "Duration:".dimmed(), result.duration_ms);
    println!();
}
