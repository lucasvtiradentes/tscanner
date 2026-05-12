mod header;
mod helpers;
mod progress;
mod targets;
mod types;

use anyhow::{Context, Result};
use colored::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;

use crate::shared::{fatal_error_and_exit, print_section_title, FormattedOutput, SummaryStats};
use tscanner_cli::{CliGroupMode, CliRuleKind, CliSeverity, OutputFormat};
use tscanner_cli_output::GroupMode;
use tscanner_config::{AiExecutionMode, AiProvider};
use tscanner_scanner::{
    AiProgressCallback, ConfigExt, RegularRulesCompleteCallback, ScanCallbacks,
};
use tscanner_service::log_info;

use super::context::CheckContext;
use super::output;

type ModifiedLinesMap = HashMap<PathBuf, HashSet<usize>>;

pub use types::CliOptions;

#[allow(clippy::too_many_arguments)]
pub fn cmd_check(
    paths: &[PathBuf],
    no_cache: bool,
    group_by: Option<CliGroupMode>,
    format: Option<OutputFormat>,
    json_output: Option<PathBuf>,
    branch: Option<String>,
    staged: bool,
    uncommitted: bool,
    glob_filter: Option<String>,
    rule_filter: Option<String>,
    severity_filter: Option<CliSeverity>,
    kind_filter: Option<CliRuleKind>,
    continue_on_error: bool,
    include_ai: bool,
    only_ai: bool,
    config_path: Option<PathBuf>,
) -> Result<()> {
    let mode_flags = [staged, uncommitted, branch.is_some()]
        .iter()
        .filter(|&&x| x)
        .count();
    if mode_flags > 1 {
        fatal_error_and_exit(
            "--staged, --uncommitted, and --branch are mutually exclusive",
            &[],
        );
    }

    let output_format = format.unwrap_or_default();

    let root = fs::canonicalize(".").context("Failed to resolve current directory")?;
    let scan_paths = targets::resolve_scan_paths(&root, paths, staged, uncommitted)?;

    log_info(&format!(
        "cmd_check: Root: {}, Scan paths: {:?} (no_cache: {}, group_by: {:?}, format: {:?}, staged: {}, uncommitted: {})",
        root.display(),
        scan_paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
        no_cache,
        group_by,
        output_format,
        staged,
        uncommitted
    ));

    let (config, resolved_config_path, mut config_warnings) =
        helpers::load_check_config(&root, config_path);

    if let Some(warning) = helpers::check_schema_version_mismatch(&resolved_config_path) {
        config_warnings.push(warning);
    }

    let cli_options = helpers::build_cli_options(group_by);
    let effective_group_mode = helpers::resolve_group_mode(&cli_options);
    let effective_ai_mode = helpers::resolve_ai_mode(include_ai, only_ai);

    let config_hash = config.compute_hash();
    let ai_provider = config.ai.as_ref().and_then(|ai| ai.provider);

    if ai_provider.is_none() && effective_ai_mode != AiExecutionMode::Ignore {
        fatal_error_and_exit(
            "AI rules enabled but no provider configured",
            &[
                "Add a provider to your config file:",
                &format!("  {}", "\"ai\": { \"provider\": \"claude\" }".yellow()),
                "",
                &format!("Available providers: {}", AiProvider::all_names().cyan()),
            ],
        );
    }

    let (builtin_count, regex_count, script_count, ai_count) =
        config.count_enabled_rules_breakdown();
    let (rules_breakdown, total_enabled_rules) = helpers::build_rules_breakdown(
        effective_ai_mode,
        builtin_count,
        regex_count,
        script_count,
        ai_count,
    );
    let scanner =
        helpers::build_scanner(&root, config, &resolved_config_path, no_cache, config_hash)?;

    let is_json = matches!(output_format, OutputFormat::Json);

    let (files_to_scan, modified_lines, scan_mode) = targets::resolve_scan_targets(
        &root,
        &scan_paths,
        glob_filter.as_deref(),
        branch.as_ref(),
        staged,
        uncommitted,
    )?;

    if !is_json {
        header::render_scan_header(header::ScanHeaderParams {
            root: &root,
            resolved_config_path: &resolved_config_path,
            show_settings: cli_options.show_settings,
            scan_mode,
            output_format: output_format.clone(),
            group_mode: effective_group_mode.clone(),
            ai_mode: effective_ai_mode,
            ai_provider,
            cache_enabled: !no_cache,
            continue_on_error,
            glob_filter: glob_filter.clone(),
            rule_filter: rule_filter.clone(),
            severity_filter: severity_filter.as_ref().map(|s| s.as_str().to_string()),
            kind_filter: kind_filter.as_ref().map(|k| k.as_str().to_string()),
        });
    }

    let regular_rules_count =
        rules_breakdown.builtin + rules_breakdown.regex + rules_breakdown.script;

    let scan_skipped = files_to_scan
        .as_ref()
        .map(|f| f.is_empty())
        .unwrap_or(false);

    let regular_rules_callback: Option<RegularRulesCompleteCallback> =
        if !is_json && regular_rules_count > 0 && !scan_skipped {
            let count = regular_rules_count;
            Some(Arc::new(move |duration_ms: u128| {
                progress::render_rules_status(
                    "Regular rules",
                    count,
                    progress::RuleStatus::Completed(duration_ms),
                );
                let _ = io::stdout().flush();
            }))
        } else {
            None
        };

    let ai_progress_callback: Option<AiProgressCallback> = progress::build_ai_progress_callback(
        effective_ai_mode != AiExecutionMode::Ignore,
        is_json,
        scan_skipped,
    );

    let mut result = scanner.scan_codebase_with_callbacks(
        &scan_paths,
        files_to_scan.as_ref(),
        effective_ai_mode,
        modified_lines.as_ref(),
        ScanCallbacks {
            on_regular_rules_complete: regular_rules_callback,
            on_ai_progress: ai_progress_callback,
        },
    );

    for warning in config_warnings {
        result.warnings.push(warning);
    }

    if scan_skipped {
        result.notes.push(
            "Scan skipped: no files to analyze (staged/branch has no matching files)".to_string(),
        );
        if !is_json {
            if regular_rules_count > 0 {
                progress::render_rules_status(
                    "Regular rules",
                    regular_rules_count,
                    progress::RuleStatus::Skipped,
                );
            }
            if rules_breakdown.ai > 0 {
                progress::render_rules_status(
                    "AI rules",
                    rules_breakdown.ai,
                    progress::RuleStatus::Skipped,
                );
            }
        }
    } else if !is_json && rules_breakdown.ai > 0 {
        let ai_status = if !result.errors.is_empty() {
            progress::RuleStatus::Error(result.ai_rules_duration_ms)
        } else {
            progress::RuleStatus::Completed(result.ai_rules_duration_ms)
        };
        progress::render_rules_status("AI rules", rules_breakdown.ai, ai_status);
    }

    helpers::apply_result_filters(
        &mut result,
        modified_lines.as_ref(),
        rule_filter.as_deref(),
        severity_filter.as_ref(),
        kind_filter.as_ref(),
    );

    log_info(&format!(
        "cmd_check: Scan completed: {} files, {}ms",
        result.files.len(),
        result.duration_ms
    ));

    let stats = SummaryStats::from_result(&result, total_enabled_rules, rules_breakdown);

    if result.files.is_empty() && !is_json {
        let formatted_output = match effective_group_mode {
            GroupMode::File => FormattedOutput::build_by_file(&root, &result, &stats),
            GroupMode::Rule => FormattedOutput::build_by_rule(&root, &result, &stats),
        };

        println!();
        print_section_title("Results:");
        println!();
        println!("{}", "✓ No issues found!".green().bold());

        helpers::render_scan_messages(&result);

        if result.notes.is_empty() && result.warnings.is_empty() && result.errors.is_empty() {
            println!();
        }
        if cli_options.show_summary {
            output::render_summary(formatted_output.summary());
        }

        if let Some(ref json_path) = json_output {
            helpers::write_json_output(json_path, &formatted_output)?;
        }

        if !result.errors.is_empty() && !continue_on_error {
            std::process::exit(1);
        }

        return Ok(());
    }

    let formatted_output = match effective_group_mode {
        GroupMode::File => FormattedOutput::build_by_file(&root, &result, &stats),
        GroupMode::Rule => FormattedOutput::build_by_rule(&root, &result, &stats),
    };

    let ctx = CheckContext::new(cli_options);

    let renderer = output::get_renderer(&output_format);
    renderer.render(&ctx, &formatted_output, &result);

    if let Some(ref json_path) = json_output {
        helpers::write_json_output(json_path, &formatted_output)?;
    }

    log_info(&format!(
        "cmd_check: Found {} errors, {} warnings",
        stats.error_count, stats.warning_count
    ));

    let has_scan_errors = !result.errors.is_empty();
    if (stats.error_count > 0 || has_scan_errors) && !continue_on_error {
        std::process::exit(1);
    }

    Ok(())
}
