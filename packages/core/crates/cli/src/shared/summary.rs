use cli::{GroupMode, OutputFormat};
use colored::*;
use core::types::{ScanResult, Severity};
use serde::Serialize;

#[derive(Clone)]
pub enum ScanMode {
    Codebase,
    Staged { file_count: usize },
    Branch { name: String, file_count: usize },
}

pub struct ScanConfig {
    pub show_settings: bool,
    pub mode: ScanMode,
    pub format: OutputFormat,
    pub group_by: GroupMode,
    pub cache_enabled: bool,
    pub continue_on_error: bool,
    pub config_path: String,
    pub glob_filter: Option<String>,
    pub rule_filter: Option<String>,
}

#[derive(Serialize)]
pub struct JsonSummary {
    pub total_files: usize,
    pub cached_files: usize,
    pub scanned_files: usize,
    pub total_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub duration_ms: u128,
    pub total_enabled_rules: usize,
}

pub struct SummaryStats {
    pub total_issues: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub unique_rules_count: usize,
    pub total_enabled_rules: usize,
}

impl SummaryStats {
    pub fn from_result(result: &ScanResult, total_enabled_rules: usize) -> Self {
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut unique_rules = std::collections::HashSet::new();

        for file_result in &result.files {
            for issue in &file_result.issues {
                match issue.severity {
                    Severity::Error => error_count += 1,
                    Severity::Warning => warning_count += 1,
                }
                unique_rules.insert(&issue.rule);
            }
        }

        Self {
            total_issues: error_count + warning_count,
            error_count,
            warning_count,
            unique_rules_count: unique_rules.len(),
            total_enabled_rules,
        }
    }
}

impl JsonSummary {
    pub fn new(result: &ScanResult, stats: &SummaryStats) -> Self {
        Self {
            total_files: result.total_files,
            cached_files: result.cached_files,
            scanned_files: result.scanned_files,
            total_issues: stats.total_issues,
            errors: stats.error_count,
            warnings: stats.warning_count,
            duration_ms: result.duration_ms,
            total_enabled_rules: stats.total_enabled_rules,
        }
    }
}

pub fn render_header(config: &ScanConfig) {
    if config.show_settings {
        println!("{}", "Check settings:".cyan().bold());
        println!();

        let mode_str = match &config.mode {
            ScanMode::Codebase => "codebase",
            ScanMode::Staged { .. } => "staged",
            ScanMode::Branch { .. } => "branch",
        };
        let format_str = match config.format {
            OutputFormat::Text => "text",
            OutputFormat::Json => "json",
            OutputFormat::Pretty => "pretty",
        };
        let group_str = match config.group_by {
            GroupMode::Rule => "rule",
            GroupMode::File => "file",
        };
        let cache_str = if config.cache_enabled {
            "enabled"
        } else {
            "disabled"
        };

        println!("  {} {}", "Mode:".dimmed(), mode_str);
        match &config.mode {
            ScanMode::Staged { file_count } => {
                println!("  {} {}", "Staged files:".dimmed(), file_count);
            }
            ScanMode::Branch { name, file_count } => {
                println!("  {} {}", "Target branch:".dimmed(), name);
                println!("  {} {}", "Changed files:".dimmed(), file_count);
            }
            ScanMode::Codebase => {}
        }

        println!("  {} {}", "Format:".dimmed(), format_str);
        println!("  {} {}", "Group by:".dimmed(), group_str);
        println!("  {} {}", "Cache:".dimmed(), cache_str);
        println!(
            "  {} {}",
            "Continue on error:".dimmed(),
            config.continue_on_error
        );
        println!("  {} {}", "Config:".dimmed(), config.config_path);

        if let Some(ref glob) = config.glob_filter {
            println!("  {} {}", "Glob filter:".dimmed(), glob);
        }
        if let Some(ref rule) = config.rule_filter {
            println!("  {} {}", "Rule filter:".dimmed(), rule);
        }

        println!();
    }
}

pub fn render_summary(result: &ScanResult, stats: &SummaryStats) {
    let files_with_issues = result.files.iter().filter(|f| !f.issues.is_empty()).count();

    println!();
    println!(
        "{} {} ({} errors, {} warnings)",
        "Issues:".dimmed(),
        stats.total_issues.to_string().cyan(),
        stats.error_count.to_string().red(),
        stats.warning_count.to_string().yellow()
    );
    println!(
        "{} {}/{} ({} cached, {} scanned)",
        "Files with issues:".dimmed(),
        files_with_issues.to_string().cyan(),
        result.total_files,
        result.cached_files.to_string().green(),
        result.scanned_files.to_string().yellow()
    );
    println!(
        "{} {}/{}",
        "Triggered rules:".dimmed(),
        stats.unique_rules_count.to_string().cyan(),
        stats.total_enabled_rules
    );
    println!("{} {}ms", "Duration:".dimmed(), result.duration_ms);
    println!();
}
