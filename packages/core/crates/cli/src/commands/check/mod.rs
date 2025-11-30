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
use crate::shared::{render_header, ScanConfig, ScanMode, SummaryStats};
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

    let (config, cli_config, resolved_config_path) =
        match load_config_with_custom(&root, config_path)? {
            Some((cfg, config_file_path)) => {
                log_info(&format!(
                    "cmd_check: Config loaded successfully from: {}",
                    config_file_path
                ));
                let file_cli_config = cfg.cli.clone().unwrap_or_default();
                (cfg, file_cli_config, config_file_path)
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
    let total_enabled_rules = config.count_enabled_rules();
    let cache = if effective_no_cache {
        FileCache::new()
    } else {
        FileCache::with_config_hash(config_hash)
    };

    let scanner = Scanner::with_cache(config, Arc::new(cache), root.clone())
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    let is_json = matches!(output_format, OutputFormat::Json);

    let (files_to_scan, modified_lines, scan_mode) = if staged {
        let staged_files = git::get_staged_files(&root)?;
        let staged_lines = git::get_staged_modified_lines(&root)?;
        let file_count = staged_files.len();
        log_info(&format!("cmd_check: Found {} staged files", file_count));
        let files = filters::get_files_to_scan_multi(
            &scan_paths,
            glob_filter.as_deref(),
            Some(staged_files),
        );
        (files, Some(staged_lines), ScanMode::Staged { file_count })
    } else if let Some(ref branch_name) = branch {
        let (changed_files, modified_lines) = get_branch_changes(&root, branch_name)?;
        let file_count = changed_files.as_ref().map_or(0, |f| f.len());
        let files =
            filters::get_files_to_scan_multi(&scan_paths, glob_filter.as_deref(), changed_files);
        (
            files,
            modified_lines,
            ScanMode::Branch {
                name: branch_name.clone(),
                file_count,
            },
        )
    } else {
        let files = filters::get_files_to_scan_multi(&scan_paths, glob_filter.as_deref(), None);
        (files, None, ScanMode::Codebase)
    };

    if !is_json {
        let relative_config_path = pathdiff::diff_paths(&resolved_config_path, &root)
            .map(|p| p.display().to_string())
            .unwrap_or(resolved_config_path.clone());
        let scan_config = ScanConfig {
            show_settings: resolved_cli.show_settings,
            mode: scan_mode,
            format: output_format.clone(),
            group_by: effective_group_mode.clone(),
            cache_enabled: !effective_no_cache,
            continue_on_error,
            config_path: relative_config_path,
            glob_filter: glob_filter.clone(),
            rule_filter: rule_filter.clone(),
        };
        render_header(&scan_config);
        println!("{}", "Scanning...".cyan().bold());
    }

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

    let stats = SummaryStats::from_result(&result, total_enabled_rules);

    if result.files.is_empty() && !is_json {
        println!();
        println!("{}", "✓ No issues found!".green().bold());
        println!();
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

type ModifiedLinesMap = std::collections::HashMap<PathBuf, std::collections::HashSet<usize>>;

fn get_branch_changes(
    root: &Path,
    branch_name: &str,
) -> Result<(Option<HashSet<PathBuf>>, Option<ModifiedLinesMap>)> {
    match (
        git::get_changed_files(root, branch_name),
        git::get_modified_lines(root, branch_name),
    ) {
        (Ok(files), Ok(lines)) => {
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
