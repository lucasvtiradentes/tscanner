use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config_loader::load_config_with_custom;
use tscanner_config::{app_display_name, app_name, TscannerConfig, ValidationResult};
use tscanner_diagnostics::Severity;
use tscanner_rules::get_all_rule_metadata;
use tscanner_service::{log_error, log_info};

pub fn cmd_config(
    path: &Path,
    rules: bool,
    validate: bool,
    show: bool,
    config_path: Option<PathBuf>,
) -> Result<()> {
    log_info(&format!("cmd_config: path={}", path.display()));

    if !rules && !validate && !show {
        eprintln!(
            "{}",
            "Error: No flag specified. Use --rules, --validate, or --show"
                .red()
                .bold()
        );
        eprintln!();
        eprintln!("Usage:");
        eprintln!(
            "  {} config --rules     List all rules and their status",
            app_name()
        );
        eprintln!(
            "  {} config --validate  Validate the configuration file",
            app_name()
        );
        eprintln!(
            "  {} config --show      Show the resolved configuration",
            app_name()
        );
        std::process::exit(1);
    }

    let root = fs::canonicalize(path).context("Failed to resolve path")?;

    let (config, config_file_path) = match load_config_with_custom(&root, config_path)? {
        Some((cfg, path)) => {
            log_info(&format!("cmd_config: Config loaded from: {}", path));
            (cfg, path)
        }
        None => {
            log_error("cmd_config: No config found");
            eprintln!(
                "{}",
                format!("Error: No {} configuration found!", app_name())
                    .red()
                    .bold()
            );
            eprintln!();
            eprintln!(
                "Run {} to create a default configuration.",
                format!("{} init", app_name()).cyan()
            );
            std::process::exit(1);
        }
    };

    if validate {
        cmd_validate(&config_file_path)?;
    }

    if rules {
        cmd_rules(&config, &config_file_path)?;
    }

    if show {
        cmd_show(&config, &config_file_path)?;
    }

    Ok(())
}

fn cmd_validate(config_file_path: &str) -> Result<()> {
    let content = fs::read_to_string(config_file_path)
        .context(format!("Failed to read config file: {}", config_file_path))?;

    let config_path = Path::new(config_file_path);
    let workspace = config_path.parent().and_then(|p| p.parent());

    let config_dir_name = tscanner_config::config_dir_name();
    let (_, result) = match TscannerConfig::full_validate(&content, workspace, config_dir_name) {
        Ok(r) => r,
        Err(e) => {
            println!(
                "{} {}",
                "✗".red().bold(),
                format!("Parse error: {}", e).red()
            );
            println!("  {}", config_file_path.dimmed());
            std::process::exit(1);
        }
    };

    print_validation_result(&result, config_file_path);

    if !result.is_valid() {
        std::process::exit(1);
    }

    Ok(())
}

fn print_validation_result(result: &ValidationResult, config_file_path: &str) {
    for error in &result.errors {
        println!("{} {}", "✗".red().bold(), error.red());
    }

    for warning in &result.warnings {
        println!("{} {}", "⚠".yellow().bold(), warning.yellow());
    }

    if result.is_valid() {
        if result.has_warnings() {
            println!(
                "{} {}",
                "✓".green().bold(),
                "Configuration is valid (with warnings)".green()
            );
        } else {
            println!(
                "{} {}",
                "✓".green().bold(),
                "Configuration is valid".green()
            );
        }
    }

    println!("  {}", config_file_path.dimmed());
}

fn cmd_rules(config: &TscannerConfig, config_file_path: &str) -> Result<()> {
    println!(
        "{}",
        format!("{} Rules Configuration", app_display_name())
            .cyan()
            .bold()
    );
    println!("Config: {}\n", config_file_path.dimmed());

    let all_metadata = get_all_rule_metadata();

    let mut builtin_rules: Vec<_> = all_metadata
        .iter()
        .map(|meta| {
            let user_config = config.rules.builtin.get(meta.name);
            let enabled = match user_config {
                Some(cfg) => cfg.enabled.unwrap_or(true),
                None => meta.default_enabled,
            };
            let severity = user_config
                .and_then(|c| c.severity)
                .unwrap_or(meta.default_severity);
            (meta.name, enabled, severity, "builtin")
        })
        .collect();

    builtin_rules.sort_by(|a, b| a.0.cmp(b.0));

    let mut regex_rules: Vec<_> = config
        .rules
        .regex
        .iter()
        .map(|(name, cfg)| (name.as_str(), cfg.enabled, cfg.severity, "regex"))
        .collect();

    regex_rules.sort_by(|a, b| a.0.cmp(b.0));

    let mut script_rules: Vec<_> = config
        .rules
        .script
        .iter()
        .map(|(name, cfg)| (name.as_str(), cfg.enabled, cfg.severity, "script"))
        .collect();

    script_rules.sort_by(|a, b| a.0.cmp(b.0));

    let mut ai_rules: Vec<_> = config
        .ai_rules
        .iter()
        .map(|(name, cfg)| (name.as_str(), cfg.enabled, cfg.severity, "ai"))
        .collect();

    ai_rules.sort_by(|a, b| a.0.cmp(b.0));

    let enabled_builtin: Vec<_> = builtin_rules.iter().filter(|r| r.1).collect();
    let enabled_regex: Vec<_> = regex_rules.iter().filter(|r| r.1).collect();
    let enabled_script: Vec<_> = script_rules.iter().filter(|r| r.1).collect();
    let enabled_ai: Vec<_> = ai_rules.iter().filter(|r| r.1).collect();

    let total_enabled =
        enabled_builtin.len() + enabled_regex.len() + enabled_script.len() + enabled_ai.len();
    println!("{} enabled rules:\n", total_enabled);

    for (name, _, severity, rule_type) in enabled_builtin
        .iter()
        .chain(enabled_regex.iter())
        .chain(enabled_script.iter())
        .chain(enabled_ai.iter())
    {
        print_rule(name, *severity, rule_type);
    }

    let disabled_builtin = builtin_rules.iter().filter(|r| !r.1).count();
    let disabled_regex = regex_rules.iter().filter(|r| !r.1).count();
    let disabled_script = script_rules.iter().filter(|r| !r.1).count();
    let disabled_ai = ai_rules.iter().filter(|r| !r.1).count();
    let total_disabled = disabled_builtin + disabled_regex + disabled_script + disabled_ai;

    if total_disabled > 0 {
        println!("\n{} disabled rules", total_disabled.to_string().dimmed());
    }

    Ok(())
}

fn print_rule(name: &str, severity: Severity, rule_type: &str) {
    let severity_badge = match severity {
        Severity::Error => "ERROR".red(),
        Severity::Warning => "WARN".yellow(),
    };

    let type_badge = match rule_type {
        "builtin" => "AST".cyan(),
        "regex" => "REGEX".blue(),
        "script" => "SCRIPT".magenta(),
        "ai" => "AI".green(),
        _ => rule_type.normal(),
    };

    print!("  {} ", "•".cyan());
    print!("{} ", name.bold());
    print!("[{}] ", type_badge);
    println!("{}", severity_badge);
}

fn cmd_show(config: &TscannerConfig, config_file_path: &str) -> Result<()> {
    println!(
        "{}",
        format!("{} Resolved Configuration", app_display_name())
            .cyan()
            .bold()
    );
    println!("Config: {}\n", config_file_path.dimmed());

    println!("{}", "Files:".bold());
    println!("  include: {:?}", config.files.include);
    println!("  exclude: {:?}", config.files.exclude);

    if let Some(ref cli) = config.cli {
        println!("\n{}", "CLI:".bold());
        println!("  groupBy: {:?}", cli.group_by);
        println!("  noCache: {}", cli.no_cache);
        println!("  showSettings: {}", cli.show_settings);
        println!("  showIssueSeverity: {}", cli.show_issue_severity);
        println!("  showIssueSourceLine: {}", cli.show_issue_source_line);
        println!("  showIssueRuleName: {}", cli.show_issue_rule_name);
        println!("  showIssueDescription: {}", cli.show_issue_description);
        println!("  showSummary: {}", cli.show_summary);
    }

    if let Some(ref code_editor) = config.code_editor {
        println!("\n{}", "Code Editor:".bold());
        println!("  highlightErrors: {}", code_editor.highlight_errors);
        println!("  highlightWarnings: {}", code_editor.highlight_warnings);
        println!(
            "  scanIntervalSeconds: {}",
            code_editor.scan_interval_seconds
        );
    }

    if !config.rules.builtin.is_empty() {
        println!("\n{}", "Builtin Rules (configured):".bold());
        let mut sorted: Vec<_> = config.rules.builtin.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        for (name, cfg) in sorted {
            let enabled_str = cfg
                .enabled
                .map(|e| if e { "enabled" } else { "disabled" })
                .unwrap_or("default");
            let severity_str = cfg
                .severity
                .map(|s| format!("{:?}", s).to_lowercase())
                .unwrap_or_else(|| "default".to_string());
            print!(
                "  {}: enabled={}, severity={}",
                name, enabled_str, severity_str
            );
            if !cfg.options.is_empty() {
                print!(", options={:?}", cfg.options);
            }
            println!();
        }
    }

    if !config.rules.regex.is_empty() {
        println!("\n{}", "Regex Rules:".bold());
        let mut sorted: Vec<_> = config.rules.regex.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        for (name, cfg) in sorted {
            println!(
                "  {}: enabled={}, severity={:?}",
                name, cfg.enabled, cfg.severity
            );
        }
    }

    if !config.rules.script.is_empty() {
        println!("\n{}", "Script Rules:".bold());
        let mut sorted: Vec<_> = config.rules.script.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        for (name, cfg) in sorted {
            println!(
                "  {}: enabled={}, severity={:?}",
                name, cfg.enabled, cfg.severity
            );
        }
    }

    if !config.ai_rules.is_empty() {
        println!("\n{}", "AI Rules:".bold());
        let mut sorted: Vec<_> = config.ai_rules.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        for (name, cfg) in sorted {
            println!(
                "  {}: enabled={}, severity={:?}, mode={:?}",
                name, cfg.enabled, cfg.severity, cfg.mode
            );
        }
    }

    Ok(())
}
