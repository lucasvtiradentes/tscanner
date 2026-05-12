use anyhow::{Context, Result};
use colored::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::config_loader::load_config_with_custom;
use crate::shared::{fatal_error_and_exit, render_messages, FormattedOutput, RulesBreakdown};
use tscanner_cache::{AiCache, FileCache, ScriptCache};
use tscanner_cli::{CliGroupMode, CliRuleKind, CliSeverity};
use tscanner_cli_output::GroupMode;
use tscanner_config::{AiConfig, AiExecutionMode, TscannerConfig};
use tscanner_constants::{app_name, config_dir_name, config_file_name, is_dev_mode};
use tscanner_scanner::Scanner;
use tscanner_service::{log_error, log_info};
use tscanner_types::enums::{IssueRuleType, Severity};
use tscanner_types::ScanResult;

use super::types::{CliGroupBy, CliOptions};
use crate::commands::check::filters;

pub(super) fn load_check_config(
    root: &Path,
    config_path: Option<PathBuf>,
) -> (TscannerConfig, String, Vec<String>) {
    match load_config_with_custom(root, config_path) {
        Ok(Some((cfg, config_file_path, warnings))) => {
            log_info(&format!(
                "cmd_check: Config loaded successfully from: {}",
                config_file_path
            ));
            (cfg, config_file_path, warnings)
        }
        Ok(None) => {
            log_error("cmd_check: No config found");
            fatal_error_and_exit(
                &format!("No {} configuration found!", app_name()),
                &[
                    "Expected config at:",
                    &format!(
                        "  • {}",
                        format!(
                            "{}/{}/{}",
                            root.display(),
                            config_dir_name(),
                            config_file_name()
                        )
                        .yellow()
                    ),
                    "",
                    &format!(
                        "Run {} to create a default configuration,",
                        format!("{} init", app_name()).cyan()
                    ),
                    &format!(
                        "or use {} to specify a custom config directory.",
                        "--config <path>".cyan()
                    ),
                ],
            );
        }
        Err(e) => {
            log_error(&format!("cmd_check: Config load error: {}", e));
            fatal_error_and_exit(&format!("{}", e), &[]);
        }
    }
}

pub(super) fn build_rules_breakdown(
    mode: AiExecutionMode,
    builtin_count: usize,
    regex_count: usize,
    script_count: usize,
    ai_count: usize,
) -> (RulesBreakdown, usize) {
    let rules_breakdown = match mode {
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
    (rules_breakdown, total_enabled_rules)
}

pub(super) fn build_scanner(
    root: &Path,
    config: TscannerConfig,
    resolved_config_path: &str,
    no_cache: bool,
    config_hash: u64,
    ai_config: Option<AiConfig>,
) -> Result<Scanner> {
    let (cache, ai_cache, script_cache) = if no_cache {
        (
            Arc::new(FileCache::new()),
            Arc::new(AiCache::new()),
            Arc::new(ScriptCache::new()),
        )
    } else {
        (
            Arc::new(FileCache::with_config_hash(config_hash)),
            Arc::new(AiCache::with_config_hash(config_hash)),
            Arc::new(ScriptCache::with_config_hash(config_hash)),
        )
    };

    let config_dir = Path::new(resolved_config_path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| root.to_path_buf());
    Scanner::with_caches_and_config_dir(
        config,
        cache,
        ai_cache,
        script_cache,
        root.to_path_buf(),
        config_dir,
        ai_config,
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

pub(super) fn write_json_output(json_path: &Path, output: &FormattedOutput) -> Result<()> {
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

pub(super) fn render_scan_messages(result: &ScanResult) {
    render_messages(&result.notes, &result.warnings, &result.errors);
}

pub(super) fn build_cli_options(group_by: Option<CliGroupMode>) -> CliOptions {
    let mut options = CliOptions::default();
    if let Some(g) = group_by {
        options.group_by = match g {
            CliGroupMode::Rule => CliGroupBy::Rule,
            CliGroupMode::File => CliGroupBy::File,
        };
    }
    options
}

pub(super) fn resolve_group_mode(cli_options: &CliOptions) -> GroupMode {
    match cli_options.group_by {
        CliGroupBy::Rule => GroupMode::Rule,
        CliGroupBy::File => GroupMode::File,
    }
}

pub(super) fn resolve_ai_mode(include_ai_flag: bool, only_ai_flag: bool) -> AiExecutionMode {
    if only_ai_flag {
        AiExecutionMode::Only
    } else if include_ai_flag {
        AiExecutionMode::Include
    } else {
        AiExecutionMode::Ignore
    }
}

pub(super) fn apply_result_filters(
    result: &mut ScanResult,
    modified_lines: Option<&HashMap<PathBuf, HashSet<usize>>>,
    rule_filter: Option<&str>,
    severity_filter: Option<&CliSeverity>,
    kind_filter: Option<&CliRuleKind>,
) {
    if let Some(line_filter) = modified_lines {
        filters::apply_line_filter(result, line_filter);
    }

    if let Some(rule_name) = rule_filter {
        filters::apply_rule_filter(result, rule_name);
    }

    if let Some(sev) = severity_filter {
        let severity = match sev {
            CliSeverity::Error => Severity::Error,
            CliSeverity::Warning => Severity::Warning,
            CliSeverity::Info => Severity::Info,
            CliSeverity::Hint => Severity::Hint,
        };
        filters::apply_severity_filter(result, severity);
    }

    if let Some(kind) = kind_filter {
        let rule_type = match kind {
            CliRuleKind::Builtin => IssueRuleType::Builtin,
            CliRuleKind::Regex => IssueRuleType::CustomRegex,
            CliRuleKind::Script => IssueRuleType::CustomScript,
            CliRuleKind::Ai => IssueRuleType::Ai,
        };
        filters::apply_rule_type_filter(result, rule_type);
    }
}

pub(super) fn check_schema_version_mismatch(config_path: &str) -> Option<String> {
    const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

    if is_dev_mode() {
        return None;
    }

    let content = match fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(_) => return None,
    };

    let schema_pattern = r#""?\$schema"?\s*:\s*"https://unpkg\.com/tscanner@([^/]+)/schema\.json""#;
    let re = match regex::Regex::new(schema_pattern) {
        Ok(r) => r,
        Err(_) => return None,
    };

    if let Some(captures) = re.captures(&content) {
        if let Some(schema_version) = captures.get(1) {
            let schema_ver = schema_version.as_str();
            if schema_ver != CLI_VERSION {
                return Some(format!(
                    "Schema version mismatch: CLI v{} but config uses schema v{}. Run 'tscanner init' or update $schema in config.",
                    CLI_VERSION, schema_ver
                ));
            }
        }
    }

    None
}
