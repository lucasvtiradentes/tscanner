use super::renderer::OutputRenderer;
use super::CheckContext;
use crate::shared::{render_summary, SummaryStats};
use colored::*;
use tscanner_diagnostics::{GroupMode, PrettyFormatter, ScanResult};

pub struct TextRenderer;

impl OutputRenderer for TextRenderer {
    fn render(&self, ctx: &CheckContext, result: &ScanResult, stats: &SummaryStats) {
        self.render_header(stats);

        match ctx.group_mode {
            GroupMode::File => {
                let formatted = PrettyFormatter::format_by_file(result, &ctx.root);
                print!("{}", formatted);
                println!();
            }
            GroupMode::Rule => {
                let formatted = PrettyFormatter::format_by_rule(result, &ctx.root);
                print!("{}", formatted);
            }
        }

        println!();

        if ctx.cli_options.show_summary {
            render_summary(result, stats);
        }
    }
}

impl TextRenderer {
    fn render_header(&self, _stats: &SummaryStats) {
        println!();
        println!("{}", "Results:".cyan().bold());
    }
}
