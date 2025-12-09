use anyhow::{Context, Result};
use colored::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::config_loader::load_config_with_custom;
use crate::shared::{
    compute_triggered_breakdown, format_duration, render_header, render_summary, RulesBreakdown,
    ScanConfig, ScanMode, SummaryStats,
};
use tscanner_cache::FileCache;
use tscanner_cli::{CliGroupMode, OutputFormat};
use tscanner_config::{app_name, config_dir_name, config_file_name, AiExecutionMode, AiProvider};
use tscanner_diagnostics::GroupMode;
use tscanner_scanner::{
    AiProgressCallback, AiProgressEvent, AiRuleStatus, ConfigExt, RegularRulesCompleteCallback,
    ScanCallbacks, Scanner,
};
use tscanner_service::{log_error, log_info};

use super::context::CheckContext;
use super::filters;
use super::git;
use super::output;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CliGroupBy {
    #[default]
    File,
    Rule,
}

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub group_by: CliGroupBy,
    pub show_settings: bool,
    pub show_summary: bool,
}

impl Default for CliOptions {
    fn default() -> Self {
        Self {
            group_by: CliGroupBy::File,
            show_settings: true,
            show_summary: true,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn cmd_check(
    paths: &[PathBuf],
    no_cache: bool,
    group_by: Option<CliGroupMode>,
    format: Option<OutputFormat>,
    json_output: Option<PathBuf>,
    branch: Option<String>,
    staged: bool,
    glob_filter: Option<String>,
    rule_filter: Option<String>,
    continue_on_error: bool,
    include_ai: bool,
    only_ai: bool,
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

    let (config, resolved_config_path) = match load_config_with_custom(&root, config_path)? {
        Some((cfg, config_file_path)) => {
            log_info(&format!(
                "cmd_check: Config loaded successfully from: {}",
                config_file_path
            ));
            (cfg, config_file_path)
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

    let cli_options = build_cli_options(group_by);
    let effective_group_mode = resolve_group_mode(&cli_options);
    let effective_ai_mode = resolve_ai_mode(include_ai, only_ai);

    let config_hash = config.compute_hash();
    let ai_provider = config.ai.as_ref().and_then(|ai| ai.provider);

    if ai_provider.is_none() && effective_ai_mode != AiExecutionMode::Ignore {
        eprintln!(
            "{}",
            "Error: AI rules enabled but no provider configured"
                .red()
                .bold()
        );
        eprintln!();
        eprintln!("Add a provider to your config file:");
        eprintln!("  {}", "\"ai\": { \"provider\": \"claude\" }".yellow());
        eprintln!();
        eprintln!("Available providers: {}", AiProvider::all_names().cyan());
        std::process::exit(1);
    }

    let (builtin_count, regex_count, script_count, ai_count) =
        config.count_enabled_rules_breakdown();
    let rules_breakdown = match effective_ai_mode {
        AiExecutionMode::Only => RulesBreakdown {
            builtin: 0,
            regex: 0,
            script: 0,
            ai: ai_count,
        },
        AiExecutionMode::Include => RulesBreakdown {
            builtin: builtin_count,
            regex: regex_count,
            script: script_count,
            ai: ai_count,
        },
        AiExecutionMode::Ignore => RulesBreakdown {
            builtin: builtin_count,
            regex: regex_count,
            script: script_count,
            ai: 0,
        },
    };
    let total_enabled_rules = rules_breakdown.builtin
        + rules_breakdown.regex
        + rules_breakdown.script
        + rules_breakdown.ai;
    let cache = if no_cache {
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
            show_settings: cli_options.show_settings,
            mode: scan_mode,
            format: output_format.clone(),
            group_by: effective_group_mode.clone(),
            ai_mode: effective_ai_mode,
            ai_provider,
            cache_enabled: !no_cache,
            continue_on_error,
            config_path: relative_config_path,
            glob_filter: glob_filter.clone(),
            rule_filter: rule_filter.clone(),
        };
        render_header(&scan_config);
        println!("{}", "Scanning...\n".cyan().bold());
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
                render_rules_status("Regular rules", count, RuleStatus::Completed(duration_ms));
                let _ = io::stdout().flush();
            }))
        } else {
            None
        };

    let ai_progress_callback: Option<AiProgressCallback> =
        if effective_ai_mode != AiExecutionMode::Ignore && !is_json && !scan_skipped {
            let rule_states: Arc<Mutex<HashMap<usize, (String, AiRuleStatus)>>> =
                Arc::new(Mutex::new(HashMap::new()));
            let has_rendered = Arc::new(Mutex::new(false));
            let start_time = Arc::new(Mutex::new(None::<std::time::Instant>));
            let states_clone = rule_states.clone();
            let has_rendered_clone = has_rendered.clone();
            let start_time_clone = start_time.clone();
            Some(Arc::new(move |event: AiProgressEvent| {
                let mut states = states_clone.lock().unwrap();
                let mut rendered = has_rendered_clone.lock().unwrap();
                let mut start = start_time_clone.lock().unwrap();
                if start.is_none() {
                    *start = Some(std::time::Instant::now());
                }
                states.insert(
                    event.rule_index,
                    (event.rule_name.clone(), event.status.clone()),
                );
                if states.len() == event.total_rules {
                    let elapsed_ms = start.map(|s| s.elapsed().as_millis()).unwrap_or(0);
                    render_ai_progress(&states, event.total_rules, !*rendered, elapsed_ms);
                    *rendered = true;
                }
            }))
        } else {
            None
        };

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

    if !is_json && scan_skipped {
        if regular_rules_count > 0 {
            render_rules_status("Regular rules", regular_rules_count, RuleStatus::Skipped);
        }
        if rules_breakdown.ai > 0 {
            render_rules_status("AI rules", rules_breakdown.ai, RuleStatus::Skipped);
        }
    } else if !is_json && rules_breakdown.ai > 0 {
        render_rules_status(
            "AI rules",
            rules_breakdown.ai,
            RuleStatus::Completed(result.ai_rules_duration_ms),
        );
    }

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

    if !is_json && !result.warnings.is_empty() {
        println!();
        for warning in &result.warnings {
            println!("{} {}", "⚠".yellow(), warning.yellow());
        }
    }

    let stats = SummaryStats::from_result(&result, total_enabled_rules, rules_breakdown);

    if result.files.is_empty() && !is_json {
        println!();
        println!("{}", "Results:".cyan().bold());
        println!();
        println!("{}", "✓ No issues found!".green().bold());

        if scan_skipped {
            println!();
            println!("{}", "Notes:".cyan().bold());
            println!();
            println!(
                "  {} {}",
                "ℹ".blue(),
                "Scan skipped: no files to analyze (staged/branch has no matching files)".dimmed()
            );
        }

        println!();
        if cli_options.show_summary {
            let triggered_breakdown = compute_triggered_breakdown(&result);
            render_summary(&result, &stats, &triggered_breakdown);
        }

        if let Some(ref json_path) = json_output {
            write_json_output(json_path, &root, &effective_group_mode, &result, &stats)?;
        }

        return Ok(());
    }

    let ctx = CheckContext::new(root.clone(), effective_group_mode.clone(), cli_options);

    let renderer = output::get_renderer(&output_format);
    renderer.render(&ctx, &result, &stats);

    if let Some(ref json_path) = json_output {
        write_json_output(json_path, &root, &effective_group_mode, &result, &stats)?;
    }

    log_info(&format!(
        "cmd_check: Found {} errors, {} warnings",
        stats.error_count, stats.warning_count
    ));

    if stats.error_count > 0 && !continue_on_error {
        std::process::exit(1);
    }

    Ok(())
}

fn write_json_output(
    json_path: &Path,
    root: &Path,
    group_mode: &GroupMode,
    result: &tscanner_diagnostics::ScanResult,
    stats: &SummaryStats,
) -> Result<()> {
    if let Some(json_str) = output::JsonRenderer::to_json_string(root, group_mode, result, stats) {
        fs::write(json_path, json_str)
            .context(format!("Failed to write JSON output to {:?}", json_path))?;
        log_info(&format!(
            "cmd_check: JSON output written to {:?}",
            json_path
        ));
    }
    Ok(())
}

fn build_cli_options(group_by: Option<CliGroupMode>) -> CliOptions {
    let mut options = CliOptions::default();
    if let Some(g) = group_by {
        options.group_by = match g {
            CliGroupMode::Rule => CliGroupBy::Rule,
            CliGroupMode::File => CliGroupBy::File,
        };
    }
    options
}

fn resolve_group_mode(cli_options: &CliOptions) -> GroupMode {
    match cli_options.group_by {
        CliGroupBy::Rule => GroupMode::Rule,
        CliGroupBy::File => GroupMode::File,
    }
}

fn resolve_ai_mode(include_ai_flag: bool, only_ai_flag: bool) -> AiExecutionMode {
    if only_ai_flag {
        AiExecutionMode::Only
    } else if include_ai_flag {
        AiExecutionMode::Include
    } else {
        AiExecutionMode::Ignore
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

fn render_ai_progress(
    states: &HashMap<usize, (String, AiRuleStatus)>,
    total: usize,
    is_first: bool,
    elapsed_ms: u128,
) {
    let completed = states
        .values()
        .filter(|(_, s)| {
            matches!(
                s,
                AiRuleStatus::Completed { .. } | AiRuleStatus::Failed { .. }
            )
        })
        .count();

    let all_completed = completed == total;

    if all_completed {
        if !is_first {
            eprint!("\x1B[1A");
            eprint!("\x1B[0J");
        }
    } else {
        if !is_first {
            eprint!("\x1B[1A");
            eprint!("\x1B[0J");
        }
        eprintln!(
            "⧗ {} {}",
            format!("AI rules ({}/{})", completed, total).cyan().bold(),
            format_duration(elapsed_ms).dimmed()
        );
    }

    let _ = io::stderr().flush();
}

enum RuleStatus {
    Completed(u128),
    Skipped,
}

fn render_rules_status(label: &str, count: usize, status: RuleStatus) {
    match status {
        RuleStatus::Completed(duration_ms) => {
            println!(
                "{} {}",
                "✓".green(),
                format!(
                    "{} ({}) {}",
                    label,
                    count,
                    format_duration(duration_ms).dimmed()
                )
                .cyan()
                .bold()
            );
        }
        RuleStatus::Skipped => {
            println!(
                "{} {}",
                "⊘".dimmed(),
                format!("{} ({}) {}", label, count, "skipped".dimmed()).dimmed()
            );
        }
    }
}
