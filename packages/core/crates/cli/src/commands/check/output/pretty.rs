use super::OutputRenderer;
use crate::commands::check::context::CheckContext;
use crate::shared::{render_summary, SummaryStats};
use cli::GroupMode;
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

        println!();

        if ctx.cli_config.show_summary {
            render_summary(result, stats);
        }
    }
}
