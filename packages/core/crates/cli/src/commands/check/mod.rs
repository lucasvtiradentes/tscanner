pub mod context;
mod filters;
mod git;
mod output;

use anyhow::{Context, Result};
use colored::*;
use core::cache::FileCache;
use core::scanner::Scanner;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::config_loader::load_config_with_custom;
use crate::shared::SummaryStats;
use crate::CliOverrides;
use cli::{GroupMode, OutputFormat};
use context::CheckContext;
use core::{
    log_error, log_info, CliConfig, CliGroupBy, APP_NAME, CONFIG_DIR_NAME, CONFIG_FILE_NAME,
};

#[allow(clippy::too_many_arguments)]
pub fn cmd_check(
    path: &Path,
    no_cache: bool,
    format: Option<OutputFormat>,
    branch: Option<String>,
    file_filter: Option<String>,
    rule_filter: Option<String>,
    continue_on_error: bool,
    config_path: Option<PathBuf>,
    cli_overrides: CliOverrides,
) -> Result<()> {
    let output_format = format.unwrap_or_default();

    log_info(&format!(
        "cmd_check: Starting at: {} (no_cache: {}, group_by: {:?}, format: {:?})",
        path.display(),
        no_cache,
        cli_overrides.group_by,
        output_format
    ));

    let root = fs::canonicalize(path).context("Failed to resolve path")?;

    let (config, cli_config) = match load_config_with_custom(&root, config_path)? {
        Some((cfg, config_file_path)) => {
            log_info(&format!(
                "cmd_check: Config loaded successfully from: {}",
                config_file_path
            ));
            let file_cli_config = cfg.cli.clone().unwrap_or_default();
            (cfg, file_cli_config)
        }
        None => {
            log_error("cmd_check: No config found");
            eprintln!(
                "{}",
                format!("Error: No {} configuration found!", APP_NAME)
                    .red()
                    .bold()
            );
            eprintln!();
            eprintln!("Expected config at:");
            eprintln!(
                "  • {}",
                format!(
                    "{}/{}/{}",
                    root.display(),
                    CONFIG_DIR_NAME,
                    CONFIG_FILE_NAME
                )
                .yellow()
            );

            eprintln!();
            eprintln!(
                "Run {} to create a default configuration,",
                format!("{} init", APP_NAME).cyan()
            );
            eprintln!(
                "or use {} to specify a custom config directory.",
                "--config <path>".cyan()
            );
            std::process::exit(1);
        }
    };

    let resolved_cli = resolve_cli_config(&cli_config, &cli_overrides);
    let effective_group_mode = resolve_group_mode(&resolved_cli);
    let effective_no_cache = no_cache || resolved_cli.no_cache;

    let config_hash = config.compute_hash();
    let cache = if effective_no_cache {
        FileCache::new()
    } else {
        FileCache::with_config_hash(config_hash)
    };

    let scanner = Scanner::with_cache(config, Arc::new(cache), root.clone())
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    if !matches!(output_format, OutputFormat::Json) {
        print_scan_header(effective_no_cache, &effective_group_mode);
    }

    let is_json = matches!(output_format, OutputFormat::Json);
    let (changed_files, modified_lines) = get_branch_changes(&root, &branch, is_json)?;

    let files_to_scan = filters::get_files_to_scan(&root, file_filter.as_deref(), changed_files);

    let mut result = scanner.scan(&root, files_to_scan.as_ref());

    if let Some(ref line_filter) = modified_lines {
        filters::apply_line_filter(&mut result, line_filter);
    }

    if let Some(ref rule_name) = rule_filter {
        filters::apply_rule_filter(&mut result, rule_name);
    }

    log_info(&format!(
        "cmd_check: Scan completed: {} files, {}ms",
        result.files.len(),
        result.duration_ms
    ));

    let stats = SummaryStats::from_result(&result);

    if result.files.is_empty() && !is_json {
        println!("{}", "✓ No issues found!".green().bold());
        return Ok(());
    }

    let ctx = CheckContext::new(root, effective_group_mode, resolved_cli);

    let renderer = output::get_renderer(&output_format);
    renderer.render(&ctx, &result, &stats);

    log_info(&format!(
        "cmd_check: Found {} errors, {} warnings",
        stats.error_count, stats.warning_count
    ));

    if stats.error_count > 0 && !continue_on_error {
        std::process::exit(1);
    }

    Ok(())
}

fn resolve_cli_config(cli_config: &CliConfig, overrides: &CliOverrides) -> CliConfig {
    CliConfig {
        group_by: overrides
            .group_by
            .as_ref()
            .map(|g| match g {
                GroupMode::Rule => CliGroupBy::Rule,
                GroupMode::File => CliGroupBy::File,
            })
            .unwrap_or(cli_config.group_by),
        no_cache: overrides.no_cache.unwrap_or(cli_config.no_cache),
        show_severity: overrides.show_severity.unwrap_or(cli_config.show_severity),
        show_source_line: overrides
            .show_source_line
            .unwrap_or(cli_config.show_source_line),
        show_rule_name: overrides
            .show_rule_name
            .unwrap_or(cli_config.show_rule_name),
        show_description: overrides
            .show_description
            .unwrap_or(cli_config.show_description),
        show_summary_at_footer: overrides
            .show_summary_at_footer
            .unwrap_or(cli_config.show_summary_at_footer),
    }
}

fn resolve_group_mode(cli_config: &CliConfig) -> GroupMode {
    match cli_config.group_by {
        CliGroupBy::Rule => GroupMode::Rule,
        CliGroupBy::File => GroupMode::File,
    }
}

fn print_scan_header(no_cache: bool, group_mode: &GroupMode) {
    println!("{}", "Scanning...".cyan().bold());
    println!();
    let group_by_str = match group_mode {
        GroupMode::Rule => "rule",
        GroupMode::File => "file",
    };
    println!(
        "  {} {}",
        "Cache:".dimmed(),
        if no_cache { "disabled" } else { "enabled" }
    );
    println!("  {} {}", "Group by:".dimmed(), group_by_str);
}

type ModifiedLinesMap = std::collections::HashMap<PathBuf, std::collections::HashSet<usize>>;

fn get_branch_changes(
    root: &Path,
    branch: &Option<String>,
    json_output: bool,
) -> Result<(Option<HashSet<PathBuf>>, Option<ModifiedLinesMap>)> {
    let Some(ref branch_name) = branch else {
        return Ok((None, None));
    };

    match (
        git::get_changed_files(root, branch_name),
        git::get_modified_lines(root, branch_name),
    ) {
        (Ok(files), Ok(lines)) => {
            if !json_output {
                println!(
                    "{}",
                    format!("Comparing with branch: {}", branch_name)
                        .cyan()
                        .bold()
                );
            }
            log_info(&format!(
                "cmd_check: Found {} changed files vs {}",
                files.len(),
                branch_name
            ));
            Ok((Some(files), Some(lines)))
        }
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("{}", format!("Error getting changed files: {}", e).red());
            std::process::exit(1);
        }
    }
}
