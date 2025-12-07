use serde::Deserialize;
use std::env;
use tscanner_diagnostics::Severity;

use crate::types::{
    AiExecutionMode, CliConfig, CliGroupBy, CodeEditorConfig, FilesConfig, TscannerConfig,
};

const DEFAULT_CONFIG_JSON: &str = include_str!("../../../../../assets/configs/default.json");
const CONSTANTS_JSON: &str = include_str!("../../../../../assets/constants.json");

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Constants {
    package_name: String,
    package_display_name: String,
    package_description: String,
    config_dir_name: String,
    config_file_name: String,
    default_target_branch: String,
    log_basename: String,
    ignore_comment: String,
    ignore_next_line_comment: String,
}

lazy_static::lazy_static! {
    static ref CONSTANTS: Constants = serde_json::from_str(CONSTANTS_JSON)
        .expect("Failed to parse constants.json");
}

pub fn app_name() -> &'static str {
    &CONSTANTS.package_name
}

pub fn app_display_name() -> &'static str {
    &CONSTANTS.package_display_name
}

pub fn app_description() -> &'static str {
    &CONSTANTS.package_description
}

pub fn config_dir_name() -> &'static str {
    &CONSTANTS.config_dir_name
}

pub fn config_file_name() -> &'static str {
    &CONSTANTS.config_file_name
}

pub fn default_target_branch() -> &'static str {
    &CONSTANTS.default_target_branch
}

pub fn log_basename() -> &'static str {
    &CONSTANTS.log_basename
}

pub fn is_dev_mode() -> bool {
    env::var("CI").is_err() && env::var("GITHUB_ACTIONS").is_err()
}

pub fn get_log_filename() -> String {
    if is_dev_mode() {
        format!("{}-dev.txt", log_basename())
    } else {
        format!("{}.txt", log_basename())
    }
}

pub fn ignore_comment() -> &'static str {
    &CONSTANTS.ignore_comment
}

pub fn ignore_next_line_comment() -> &'static str {
    &CONSTANTS.ignore_next_line_comment
}

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

pub fn default_ai_timeout() -> u64 {
    120000
}

pub fn default_ai_execution_mode() -> AiExecutionMode {
    AiExecutionMode::Ignore
}

pub fn default_ai_scan_interval_seconds() -> u32 {
    0
}

pub fn get_default_config_json() -> &'static str {
    DEFAULT_CONFIG_JSON
}
