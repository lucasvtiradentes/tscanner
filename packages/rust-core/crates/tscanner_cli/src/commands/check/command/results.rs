use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::commands::check::output;
use crate::shared::{print_section_title, FormattedOutput, SummaryStats};
use colored::*;
use tscanner_cli_output::GroupMode;
use tscanner_types::ScanResult;

use super::{helpers, CliOptions};

pub(super) struct NoIssuesParams<'a> {
    pub is_json: bool,
    pub effective_group_mode: &'a GroupMode,
    pub root: &'a Path,
    pub result: &'a ScanResult,
    pub stats: &'a SummaryStats,
    pub cli_options: &'a CliOptions,
    pub json_output: Option<&'a PathBuf>,
    pub continue_on_error: bool,
}

pub(super) fn render_no_issues_if_needed(params: NoIssuesParams<'_>) -> Result<bool> {
    if params.is_json || !params.result.files.is_empty() {
        return Ok(false);
    }

    let formatted_output = match params.effective_group_mode {
        GroupMode::File => FormattedOutput::build_by_file(params.root, params.result, params.stats),
        GroupMode::Rule => FormattedOutput::build_by_rule(params.root, params.result, params.stats),
    };

    println!();
    print_section_title("Results:");
    println!();
    println!("{}", "✓ No issues found!".green().bold());

    helpers::render_scan_messages(params.result);

    if params.result.notes.is_empty()
        && params.result.warnings.is_empty()
        && params.result.errors.is_empty()
    {
        println!();
    }
    if params.cli_options.show_summary {
        output::render_summary(formatted_output.summary());
    }

    if let Some(json_path) = params.json_output {
        helpers::write_json_output(json_path, &formatted_output)?;
    }

    if !params.result.errors.is_empty() && !params.continue_on_error {
        std::process::exit(1);
    }

    Ok(true)
}
