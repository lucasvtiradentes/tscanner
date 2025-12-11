use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};
use tscanner_types::Severity;

const CONSTANTS_JSON: &str = include_str!("../../../../../assets/constants.json");

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Constants {
    package_name: String,
    config_dir_name: String,
    config_file_name: String,
    log_basename: String,
    log_timezone_offset_hours: i8,
    ignore_comment: String,
    ignore_next_line_comment: String,
    defaults: Defaults,
    display: DisplayConfig,
    extensions: ExtensionsConfig,
    cache: CacheConfig,
    ai: AiConstantsConfig,
    urls: UrlsConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Defaults {
    code_editor: CodeEditorDefaults,
    directories: DirectoryDefaults,
    examples: ExampleDefaults,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CodeEditorDefaults {
    highlight_errors: bool,
    highlight_warnings: bool,
    highlight_infos: bool,
    highlight_hints: bool,
    scan_interval: u32,
    ai_scan_interval: u32,
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
    info: String,
    hint: String,
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
    options: String,
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
struct UrlsConfig {
    rules_base: String,
}

lazy_static::lazy_static! {
    static ref CONSTANTS: Constants = serde_json::from_str(CONSTANTS_JSON)
        .expect("Failed to parse constants.json");
}

pub fn app_name() -> &'static str {
    &CONSTANTS.package_name
}

pub fn config_dir_name() -> &'static str {
    &CONSTANTS.config_dir_name
}

pub fn config_file_name() -> &'static str {
    &CONSTANTS.config_file_name
}

pub fn log_basename() -> &'static str {
    &CONSTANTS.log_basename
}

pub fn log_timezone_offset_hours() -> i8 {
    CONSTANTS.log_timezone_offset_hours
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

pub fn default_highlight_errors() -> bool {
    CONSTANTS.defaults.code_editor.highlight_errors
}

pub fn default_highlight_warnings() -> bool {
    CONSTANTS.defaults.code_editor.highlight_warnings
}

pub fn default_highlight_infos() -> bool {
    CONSTANTS.defaults.code_editor.highlight_infos
}

pub fn default_highlight_hints() -> bool {
    CONSTANTS.defaults.code_editor.highlight_hints
}

pub fn default_scan_interval() -> u32 {
    CONSTANTS.defaults.code_editor.scan_interval
}

pub fn default_severity() -> Severity {
    Severity::Warning
}

pub fn default_ai_scan_interval() -> u32 {
    CONSTANTS.defaults.code_editor.ai_scan_interval
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

pub fn icon_info() -> &'static str {
    &CONSTANTS.display.icons.info
}

pub fn icon_hint() -> &'static str {
    &CONSTANTS.display.icons.hint
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

pub fn is_js_ts_extension(ext: &str) -> bool {
    CONSTANTS.extensions.javascript.iter().any(|e| e == ext)
}

pub fn cache_dir_name() -> &'static str {
    &CONSTANTS.cache.dir_name
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

pub fn ai_placeholder_options() -> &'static str {
    &CONSTANTS.ai.placeholders.options
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

pub fn rules_base_url() -> &'static str {
    &CONSTANTS.urls.rules_base
}

pub fn resolve_config_dir(root: &Path, config_dir: Option<PathBuf>) -> PathBuf {
    match config_dir {
        Some(dir) => {
            if dir.is_absolute() {
                dir.join(config_dir_name())
            } else {
                root.join(dir).join(config_dir_name())
            }
        }
        None => root.join(config_dir_name()),
    }
}
