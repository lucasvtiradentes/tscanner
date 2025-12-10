use serde::{Deserialize, Serialize};
use std::env;
use tscanner_diagnostics::Severity;

use crate::types::{CodeEditorConfig, FilesConfig, TscannerConfig};

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
    defaults: Defaults,
    display: DisplayConfig,
    extensions: ExtensionsConfig,
    cache: CacheConfig,
    ai: AiConstantsConfig,
    validation: ValidationConfig,
    urls: UrlsConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Defaults {
    files: FilesDefaults,
    code_editor: CodeEditorDefaults,
    intervals: IntervalDefaults,
    directories: DirectoryDefaults,
    examples: ExampleDefaults,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct FilesDefaults {
    include: Vec<String>,
    exclude: Vec<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CodeEditorDefaults {
    highlight_errors: bool,
    highlight_warnings: bool,
    scan_interval: u32,
    ai_scan_interval: u32,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct IntervalDefaults {
    lsp_debounce_ms: u64,
    scheduler_polling_ms: u64,
    script_polling_ms: u64,
    ai_polling_ms: u64,
    ai_inflight_wait_ms: u64,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DirectoryDefaults {
    script_rules: String,
    ai_rules: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExampleDefaults {
    script_rule: String,
    ai_rule: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DisplayConfig {
    icons: IconsConfig,
    rule_types: Vec<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct IconsConfig {
    builtin: String,
    regex: String,
    script: String,
    ai: String,
    error: String,
    warning: String,
    progress: String,
    success: String,
    skipped: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExtensionsConfig {
    javascript: Vec<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CacheConfig {
    dir_name: String,
    file_pattern: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AiConstantsConfig {
    temp_dir: String,
    placeholders: AiPlaceholdersConfig,
    providers: AiProvidersConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AiPlaceholdersConfig {
    files: String,
    content: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AiProvidersConfig {
    claude: AiProviderConfig,
    gemini: AiProviderConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AiProviderConfig {
    command: String,
    args: Vec<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ValidationConfig {
    conflicting_rules: Vec<Vec<String>>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct UrlsConfig {
    rules_base: String,
}

lazy_static::lazy_static! {
    static ref CONSTANTS: Constants = serde_json::from_str(CONSTANTS_JSON)
        .expect("Failed to parse constants.json");

    static ref DEFAULT_CONFIG_JSON_STRING: String = generate_default_config_json();
}

fn generate_default_config_json() -> String {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct DefaultConfig {
        files: DefaultFilesConfig,
        code_editor: DefaultCodeEditorConfig,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct DefaultFilesConfig {
        include: Vec<String>,
        exclude: Vec<String>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct DefaultCodeEditorConfig {
        highlight_errors: bool,
        highlight_warnings: bool,
        scan_interval: u32,
        ai_scan_interval: u32,
    }

    let config = DefaultConfig {
        files: DefaultFilesConfig {
            include: CONSTANTS.defaults.files.include.clone(),
            exclude: CONSTANTS.defaults.files.exclude.clone(),
        },
        code_editor: DefaultCodeEditorConfig {
            highlight_errors: CONSTANTS.defaults.code_editor.highlight_errors,
            highlight_warnings: CONSTANTS.defaults.code_editor.highlight_warnings,
            scan_interval: CONSTANTS.defaults.code_editor.scan_interval,
            ai_scan_interval: CONSTANTS.defaults.code_editor.ai_scan_interval,
        },
    };

    serde_json::to_string(&config).expect("Failed to serialize default config")
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

pub fn script_rules_dir() -> &'static str {
    &CONSTANTS.defaults.directories.script_rules
}

pub fn ai_rules_dir() -> &'static str {
    &CONSTANTS.defaults.directories.ai_rules
}

pub fn example_script_rule() -> &'static str {
    &CONSTANTS.defaults.examples.script_rule
}

pub fn example_ai_rule() -> &'static str {
    &CONSTANTS.defaults.examples.ai_rule
}

pub fn default_code_editor_config() -> CodeEditorConfig {
    CodeEditorConfig {
        highlight_errors: CONSTANTS.defaults.code_editor.highlight_errors,
        highlight_warnings: CONSTANTS.defaults.code_editor.highlight_warnings,
        scan_interval: CONSTANTS.defaults.code_editor.scan_interval,
        ai_scan_interval: CONSTANTS.defaults.code_editor.ai_scan_interval,
    }
}

pub fn default_highlight_errors() -> bool {
    CONSTANTS.defaults.code_editor.highlight_errors
}

pub fn default_highlight_warnings() -> bool {
    CONSTANTS.defaults.code_editor.highlight_warnings
}

pub fn default_scan_interval() -> u32 {
    CONSTANTS.defaults.code_editor.scan_interval
}

pub fn default_files_config() -> FilesConfig {
    FilesConfig {
        include: CONSTANTS.defaults.files.include.clone(),
        exclude: CONSTANTS.defaults.files.exclude.clone(),
    }
}

pub fn default_include() -> Vec<String> {
    CONSTANTS.defaults.files.include.clone()
}

pub fn default_exclude() -> Vec<String> {
    CONSTANTS.defaults.files.exclude.clone()
}

pub fn default_severity() -> Severity {
    Severity::Warning
}

pub fn default_ai_scan_interval() -> u32 {
    CONSTANTS.defaults.code_editor.ai_scan_interval
}

pub fn lsp_debounce_ms() -> u64 {
    CONSTANTS.defaults.intervals.lsp_debounce_ms
}

pub fn scheduler_polling_ms() -> u64 {
    CONSTANTS.defaults.intervals.scheduler_polling_ms
}

pub fn script_polling_ms() -> u64 {
    CONSTANTS.defaults.intervals.script_polling_ms
}

pub fn ai_polling_ms() -> u64 {
    CONSTANTS.defaults.intervals.ai_polling_ms
}

pub fn ai_inflight_wait_ms() -> u64 {
    CONSTANTS.defaults.intervals.ai_inflight_wait_ms
}

pub fn icon_builtin() -> &'static str {
    &CONSTANTS.display.icons.builtin
}

pub fn icon_regex() -> &'static str {
    &CONSTANTS.display.icons.regex
}

pub fn icon_script() -> &'static str {
    &CONSTANTS.display.icons.script
}

pub fn icon_ai() -> &'static str {
    &CONSTANTS.display.icons.ai
}

pub fn icon_error() -> &'static str {
    &CONSTANTS.display.icons.error
}

pub fn icon_warning() -> &'static str {
    &CONSTANTS.display.icons.warning
}

pub fn icon_progress() -> &'static str {
    &CONSTANTS.display.icons.progress
}

pub fn icon_success() -> &'static str {
    &CONSTANTS.display.icons.success
}

pub fn icon_skipped() -> &'static str {
    &CONSTANTS.display.icons.skipped
}

pub fn rule_types() -> &'static [String] {
    &CONSTANTS.display.rule_types
}

pub fn js_extensions() -> &'static [String] {
    &CONSTANTS.extensions.javascript
}

pub fn cache_dir_name() -> &'static str {
    &CONSTANTS.cache.dir_name
}

pub fn cache_file_pattern() -> &'static str {
    &CONSTANTS.cache.file_pattern
}

pub fn ai_temp_dir() -> &'static str {
    &CONSTANTS.ai.temp_dir
}

pub fn ai_placeholder_files() -> &'static str {
    &CONSTANTS.ai.placeholders.files
}

pub fn ai_placeholder_content() -> &'static str {
    &CONSTANTS.ai.placeholders.content
}

pub fn claude_command() -> &'static str {
    &CONSTANTS.ai.providers.claude.command
}

pub fn claude_args() -> &'static [String] {
    &CONSTANTS.ai.providers.claude.args
}

pub fn gemini_command() -> &'static str {
    &CONSTANTS.ai.providers.gemini.command
}

pub fn gemini_args() -> &'static [String] {
    &CONSTANTS.ai.providers.gemini.args
}

pub fn conflicting_rules() -> &'static [Vec<String>] {
    &CONSTANTS.validation.conflicting_rules
}

pub fn rules_base_url() -> &'static str {
    &CONSTANTS.urls.rules_base
}

pub fn get_default_config_json() -> &'static str {
    &DEFAULT_CONFIG_JSON_STRING
}

impl Default for TscannerConfig {
    fn default() -> Self {
        serde_json::from_str(get_default_config_json())
            .expect("Failed to parse generated default config")
    }
}
