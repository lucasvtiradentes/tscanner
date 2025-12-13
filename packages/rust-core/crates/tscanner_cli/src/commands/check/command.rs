use anyhow::{Context, Result};
use colored::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::config_loader::load_config_with_custom;
use crate::shared::{
    format_duration, print_section_header, print_section_title, render_header, FormattedOutput,
    RulesBreakdown, ScanConfig, ScanMode, SummaryStats,
};
use tscanner_cache::{AiCache, FileCache, ScriptCache};
use tscanner_cli::{CliGroupMode, CliRuleKind, CliSeverity, OutputFormat};
use tscanner_cli_output::GroupMode;
use tscanner_config::{AiExecutionMode, AiProvider};
use tscanner_constants::{
    app_name, config_dir_name, config_file_name, icon_progress, icon_skipped, icon_success,
    icon_warning,
};
use tscanner_scanner::{
    AiProgressCallback, AiProgressEvent, AiRuleStatus, ConfigExt, RegularRulesCompleteCallback,
    ScanCallbacks, Scanner,
};
use tscanner_service::{log_error, log_info};
use tscanner_types::enums::IssueRuleType;
use tscanner_types::enums::Severity;

use super::context::CheckContext;
use super::filters;
use super::git;
use super::output;

type ModifiedLinesMap = HashMap<PathBuf, HashSet<usize>>;

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
        eprintln!(
            "{}",
            "Error: --staged, --uncommitted, and --branch are mutually exclusive".red()
        );
        std::process::exit(1);
    }

    let output_format = format.unwrap_or_default();

    let root = fs::canonicalize(".").context("Failed to resolve current directory")?;
    let scan_paths: Vec<PathBuf> = if staged || uncommitted {
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
        "cmd_check: Root: {}, Scan paths: {:?} (no_cache: {}, group_by: {:?}, format: {:?}, staged: {}, uncommitted: {})",
        root.display(),
        scan_paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
        no_cache,
        group_by,
        output_format,
        staged,
        uncommitted
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
    let (cache, ai_cache, script_cache) = if no_cache {
        (FileCache::new(), AiCache::new(), ScriptCache::new())
    } else {
        (
            FileCache::with_config_hash(config_hash),
            AiCache::with_config_hash(config_hash),
            ScriptCache::with_config_hash(config_hash),
        )
    };

    let config_dir = Path::new(&resolved_config_path)
        .parent()
        .map(|p| p.to_path_buf());
    let scanner = match config_dir {
        Some(dir) => Scanner::with_caches_and_config_dir(
            config,
            Arc::new(cache),
            Arc::new(ai_cache),
            Arc::new(script_cache),
            root.clone(),
            dir,
        ),
        None => Scanner::with_caches(
            config,
            Arc::new(cache),
            Arc::new(ai_cache),
            Arc::new(script_cache),
            root.clone(),
        ),
    }
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
    } else if uncommitted {
        let uncommitted_files = git::get_uncommitted_files(&root)?;
        let uncommitted_lines = git::get_uncommitted_modified_lines(&root)?;
        let file_count = uncommitted_files.len();
        log_info(&format!(
            "cmd_check: Found {} uncommitted files",
            file_count
        ));
        let files = filters::get_files_to_scan_multi(
            &scan_paths,
            glob_filter.as_deref(),
            Some(uncommitted_files),
        );
        (
            files,
            Some(uncommitted_lines),
            ScanMode::Uncommitted { file_count },
        )
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
            severity_filter: severity_filter.as_ref().map(|s| s.as_str().to_string()),
            kind_filter: kind_filter.as_ref().map(|k| k.as_str().to_string()),
        };
        render_header(&scan_config);
        print_section_header("Scanning...");
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

    if let Some(ref sev) = severity_filter {
        let severity = match sev {
            CliSeverity::Error => Severity::Error,
            CliSeverity::Warning => Severity::Warning,
            CliSeverity::Info => Severity::Info,
            CliSeverity::Hint => Severity::Hint,
        };
        filters::apply_severity_filter(&mut result, severity);
    }

    if let Some(ref kind) = kind_filter {
        let rule_type = match kind {
            CliRuleKind::Builtin => IssueRuleType::Builtin,
            CliRuleKind::Regex => IssueRuleType::CustomRegex,
            CliRuleKind::Script => IssueRuleType::CustomScript,
            CliRuleKind::Ai => IssueRuleType::Ai,
        };
        filters::apply_rule_type_filter(&mut result, rule_type);
    }

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

        if scan_skipped {
            println!();
            print_section_header("Notes:");
            println!(
                "  {} {}",
                "ℹ".blue(),
                "Scan skipped: no files to analyze (staged/branch has no matching files)".dimmed()
            );
        }

        if !result.warnings.is_empty() {
            println!();
            print_section_header("Warnings:");
            for warning in &result.warnings {
                println!("  {} {}", icon_warning().yellow(), warning.yellow());
            }
        }

        if result.warnings.is_empty() {
            println!();
        }
        if cli_options.show_summary {
            output::render_summary(formatted_output.summary());
        }

        if let Some(ref json_path) = json_output {
            write_json_output(json_path, &formatted_output)?;
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
        write_json_output(json_path, &formatted_output)?;
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

fn write_json_output(json_path: &Path, output: &FormattedOutput) -> Result<()> {
    if let Some(json_str) = output.to_json() {
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
            "{} {} {}",
            icon_progress(),
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
                icon_success().green(),
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
                icon_skipped().dimmed(),
                format!("{} ({}) {}", label, count, "skipped".dimmed()).dimmed()
            );
        }
    }
}
