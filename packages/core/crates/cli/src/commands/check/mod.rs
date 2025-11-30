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
use cli::{GroupMode, OutputFormat};
use context::CheckContext;
use core::{
    app_name, config_dir_name, config_file_name, log_error, log_info, CliConfig, CliGroupBy,
};

#[allow(clippy::too_many_arguments)]
pub fn cmd_check(
    paths: &[PathBuf],
    no_cache: bool,
    group_by: Option<GroupMode>,
    format: Option<OutputFormat>,
    branch: Option<String>,
    staged: bool,
    glob_filter: Option<String>,
    rule_filter: Option<String>,
    continue_on_error: bool,
    config_path: Option<PathBuf>,
) -> Result<()> {
    if staged && branch.is_some() {
        eprintln!(
            "{}",
            "Error: --staged and --branch are mutually exclusive".red()
        );
        std::process::exit(1);
    }

    let output_format = format.unwrap_or_default();

    let root = fs::canonicalize(".").context("Failed to resolve current directory")?;
    let scan_paths: Vec<PathBuf> = if staged {
        vec![root.clone()]
    } else {
        paths
            .iter()
            .map(|p| {
                fs::canonicalize(p).context(format!("Failed to resolve path: {}", p.display()))
            })
            .collect::<Result<Vec<_>>>()?
    };

    log_info(&format!(
        "cmd_check: Root: {}, Scan paths: {:?} (no_cache: {}, group_by: {:?}, format: {:?}, staged: {})",
        root.display(),
        scan_paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
        no_cache,
        group_by,
        output_format,
        staged
    ));

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
                format!("Error: No {} configuration found!", app_name())
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
                    config_dir_name(),
                    config_file_name()
                )
                .yellow()
            );

            eprintln!();
            eprintln!(
                "Run {} to create a default configuration,",
                format!("{} init", app_name()).cyan()
            );
            eprintln!(
                "or use {} to specify a custom config directory.",
                "--config <path>".cyan()
            );
            std::process::exit(1);
        }
    };

    let resolved_cli = apply_group_by_override(&cli_config, group_by);
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

    let (files_to_scan, modified_lines) = if staged {
        let staged_files = git::get_staged_files(&root)?;
        let staged_lines = git::get_staged_modified_lines(&root)?;
        if !is_json {
            println!(
                "{}",
                format!("Scanning {} staged files", staged_files.len())
                    .cyan()
                    .bold()
            );
        }
        log_info(&format!(
            "cmd_check: Found {} staged files",
            staged_files.len()
        ));
        let files = filters::get_files_to_scan_multi(
            &scan_paths,
            glob_filter.as_deref(),
            Some(staged_files),
        );
        (files, Some(staged_lines))
    } else {
        let (changed_files, modified_lines) = get_branch_changes(&root, &branch, is_json)?;
        let files =
            filters::get_files_to_scan_multi(&scan_paths, glob_filter.as_deref(), changed_files);
        (files, modified_lines)
    };

    let mut result = scanner.scan_multi(&scan_paths, files_to_scan.as_ref());

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

fn apply_group_by_override(cli_config: &CliConfig, group_by: Option<GroupMode>) -> CliConfig {
    CliConfig {
        group_by: group_by
            .as_ref()
            .map(|g| match g {
                GroupMode::Rule => CliGroupBy::Rule,
                GroupMode::File => CliGroupBy::File,
            })
            .unwrap_or(cli_config.group_by),
        ..cli_config.clone()
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
