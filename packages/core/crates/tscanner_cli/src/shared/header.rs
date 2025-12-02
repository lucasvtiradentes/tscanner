use colored::*;
use tscanner_cli::OutputFormat;
use tscanner_diagnostics::GroupMode;

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
