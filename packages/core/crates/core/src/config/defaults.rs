use crate::output::Severity;

use super::types::{
    CliConfig, CliGroupBy, CodeEditorConfig, FilesConfig, ScriptMode, TscannerConfig,
};

const DEFAULT_CONFIG_JSON: &str = include_str!("../../../../../../assets/default-config.json");

pub fn default_code_editor_config() -> CodeEditorConfig {
    let config: TscannerConfig =
        serde_json::from_str(DEFAULT_CONFIG_JSON).expect("Failed to parse default-config.json");
    config
        .code_editor
        .expect("default-config.json must have 'codeEditor' section")
}

pub fn default_highlight_errors() -> bool {
    default_code_editor_config().highlight_errors
}

pub fn default_highlight_warnings() -> bool {
    default_code_editor_config().highlight_warnings
}

pub fn default_scan_interval_seconds() -> u32 {
    default_code_editor_config().scan_interval_seconds
}

pub fn default_cli_config() -> CliConfig {
    let config: TscannerConfig =
        serde_json::from_str(DEFAULT_CONFIG_JSON).expect("Failed to parse default-config.json");
    config
        .cli
        .expect("default-config.json must have 'cli' section")
}

pub fn default_cli_group_by() -> CliGroupBy {
    default_cli_config().group_by
}

pub fn default_cli_no_cache() -> bool {
    default_cli_config().no_cache
}

pub fn default_cli_show_issue_severity() -> bool {
    default_cli_config().show_issue_severity
}

pub fn default_cli_show_issue_source_line() -> bool {
    default_cli_config().show_issue_source_line
}

pub fn default_cli_show_issue_rule_name() -> bool {
    default_cli_config().show_issue_rule_name
}

pub fn default_cli_show_settings() -> bool {
    default_cli_config().show_settings
}

pub fn default_cli_show_issue_description() -> bool {
    default_cli_config().show_issue_description
}

pub fn default_cli_show_summary() -> bool {
    default_cli_config().show_summary
}

pub fn default_files_config() -> FilesConfig {
    let config: TscannerConfig =
        serde_json::from_str(DEFAULT_CONFIG_JSON).expect("Failed to parse default-config.json");
    config.files
}

pub fn default_include() -> Vec<String> {
    default_files_config().include
}

pub fn default_exclude() -> Vec<String> {
    default_files_config().exclude
}

pub fn default_true() -> bool {
    true
}

pub fn default_severity() -> Severity {
    Severity::Warning
}

pub fn default_script_timeout() -> u64 {
    10000
}

pub fn default_script_mode() -> ScriptMode {
    ScriptMode::Batch
}

pub fn get_default_config_json() -> &'static str {
    DEFAULT_CONFIG_JSON
}
