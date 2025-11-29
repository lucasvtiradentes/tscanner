use super::OutputRenderer;
use crate::commands::check::context::CheckContext;
use crate::shared::SummaryStats;
use crate::GroupMode;
use colored::*;
use core::types::ScanResult;

pub struct PrettyRenderer;

impl OutputRenderer for PrettyRenderer {
    fn render(&self, ctx: &CheckContext, result: &ScanResult, stats: &SummaryStats) {
        match ctx.group_mode {
            GroupMode::File => {
                let formatted = core::PrettyFormatter::format_by_file(result, &ctx.root);
                print!("{}", formatted);
                println!();
            }
            GroupMode::Rule => {
                let formatted = core::PrettyFormatter::format_by_rule(result, &ctx.root);
                print!("{}", formatted);
            }
        }

        if ctx.cli_config.show_summary_at_footer {
            self.render_summary(ctx, result, stats);
        }
    }
}

impl PrettyRenderer {
    fn render_summary(&self, _ctx: &CheckContext, result: &ScanResult, stats: &SummaryStats) {
        println!();
        println!();
        println!(
            "{} {} ({} errors, {} warnings)",
            "Issues:".dimmed(),
            stats.total_issues.to_string().cyan(),
            stats.error_count.to_string().red(),
            stats.warning_count.to_string().yellow()
        );
        println!(
            "{} {} ({} cached, {} scanned)",
            "Files:".dimmed(),
            result.total_files,
            result.cached_files.to_string().green(),
            result.scanned_files.to_string().yellow()
        );
        println!("{} {}", "Rules:".dimmed(), stats.unique_rules_count);
        println!("{} {}ms", "Duration:".dimmed(), result.duration_ms);
        println!();
    }
}
