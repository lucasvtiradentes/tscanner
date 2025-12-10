use tscanner_cli::OutputFormat;
use tscanner_config::{AiExecutionMode, AiProvider};
use tscanner_output::GroupMode;

use super::section::{print_section_header, print_setting, print_setting_value};

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
    pub ai_mode: AiExecutionMode,
    pub ai_provider: Option<AiProvider>,
    pub cache_enabled: bool,
    pub continue_on_error: bool,
    pub config_path: String,
    pub glob_filter: Option<String>,
    pub rule_filter: Option<String>,
}

pub fn format_duration(ms: u128) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        let total_seconds = ms / 1000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}m {}s", minutes, seconds)
    }
}

pub fn render_header(config: &ScanConfig) {
    if config.show_settings {
        print_section_header("Check settings:");

        let mode_str = match &config.mode {
            ScanMode::Codebase => "codebase",
            ScanMode::Staged { .. } => "staged",
            ScanMode::Branch { .. } => "branch",
        };
        let format_str = match config.format {
            OutputFormat::Text => "text",
            OutputFormat::Json => "json",
        };
        let group_str = match config.group_by {
            GroupMode::Rule => "rule",
            GroupMode::File => "file",
        };
        let ai_mode_str = match config.ai_mode {
            AiExecutionMode::Ignore => "ignore",
            AiExecutionMode::Include => "include",
            AiExecutionMode::Only => "only",
        };
        let cache_str = if config.cache_enabled {
            "enabled"
        } else {
            "disabled"
        };

        print_setting("Mode", mode_str);
        match &config.mode {
            ScanMode::Staged { file_count } => {
                print_setting_value("Staged files", file_count);
            }
            ScanMode::Branch { name, file_count } => {
                print_setting("Target branch", name);
                print_setting_value("Changed files", file_count);
            }
            ScanMode::Codebase => {}
        }

        print_setting("Format", format_str);
        print_setting("Group by", group_str);
        print_setting("AI mode", ai_mode_str);
        if let Some(ref provider) = config.ai_provider {
            let provider_str = match provider {
                AiProvider::Claude => "claude",
                AiProvider::Gemini => "gemini",
                AiProvider::Custom => "custom",
            };
            print_setting("AI provider", provider_str);
        }
        print_setting("Cache", cache_str);
        print_setting_value("Continue on error", config.continue_on_error);
        print_setting("Config", &config.config_path);

        if let Some(ref glob) = config.glob_filter {
            print_setting("Glob filter", glob);
        }
        if let Some(ref rule) = config.rule_filter {
            print_setting("Rule filter", rule);
        }

        println!();
    }
}
