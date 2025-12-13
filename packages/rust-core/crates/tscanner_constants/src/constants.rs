use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};
use tscanner_types::Severity;

const CONSTANTS_JSON: &str = include_str!("../../../../../assets/constants.json");

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Constants {
    shared: SharedConfig,
    core_rust: CoreRustConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SharedConfig {
    package_name: String,
    #[allow(dead_code)]
    package_display_name: String,
    #[allow(dead_code)]
    package_description: String,
    config_dir_name: String,
    config_file_name: String,
    #[allow(dead_code)]
    default_target_branch: String,
    log_basename: String,
    log_timezone_offset_hours: i8,
    log_context_width: usize,
    ignore_comment: String,
    ignore_next_line_comment: String,
    config_error_prefix: String,
    extensions: ExtensionsConfig,
    icons: IconsConfig,
    urls: UrlsConfig,
    lsp: LspConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct UrlsConfig {
    #[allow(dead_code)]
    repo: String,
    #[allow(dead_code)]
    repo_blob: String,
    rules_base: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LspConfig {
    #[allow(dead_code)]
    client_id: String,
    methods: LspMethodsConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LspMethodsConfig {
    scan: String,
    scan_file: String,
    scan_content: String,
    clear_cache: String,
    get_rules_metadata: String,
    format_results: String,
    validate_config: String,
    ai_progress: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CoreRustConfig {
    defaults: Defaults,
    cache: CacheConfig,
    ai: AiConstantsConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Defaults {
    code_editor: CodeEditorDefaults,
    directories: DirectoryDefaults,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CodeEditorDefaults {
    highlight_errors: bool,
    highlight_warnings: bool,
    highlight_infos: bool,
    highlight_hints: bool,
    auto_scan_interval: u32,
    auto_ai_scan_interval: u32,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DirectoryDefaults {
    script_rules: String,
    ai_rules: String,
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

lazy_static::lazy_static! {
    static ref CONSTANTS: Constants = serde_json::from_str(CONSTANTS_JSON)
        .expect("Failed to parse constants.json");
}

pub fn app_name() -> &'static str {
    &CONSTANTS.shared.package_name
}

pub fn config_dir_name() -> &'static str {
    &CONSTANTS.shared.config_dir_name
}

pub fn config_file_name() -> &'static str {
    &CONSTANTS.shared.config_file_name
}

pub fn log_basename() -> &'static str {
    &CONSTANTS.shared.log_basename
}

pub fn log_timezone_offset_hours() -> i8 {
    CONSTANTS.shared.log_timezone_offset_hours
}

pub fn log_context_width() -> usize {
    CONSTANTS.shared.log_context_width
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
    &CONSTANTS.shared.ignore_comment
}

pub fn ignore_next_line_comment() -> &'static str {
    &CONSTANTS.shared.ignore_next_line_comment
}

pub fn config_error_prefix() -> &'static str {
    &CONSTANTS.shared.config_error_prefix
}

pub fn script_rules_dir() -> &'static str {
    &CONSTANTS.core_rust.defaults.directories.script_rules
}

pub fn ai_rules_dir() -> &'static str {
    &CONSTANTS.core_rust.defaults.directories.ai_rules
}

pub fn default_highlight_errors() -> bool {
    CONSTANTS.core_rust.defaults.code_editor.highlight_errors
}

pub fn default_highlight_warnings() -> bool {
    CONSTANTS.core_rust.defaults.code_editor.highlight_warnings
}

pub fn default_highlight_infos() -> bool {
    CONSTANTS.core_rust.defaults.code_editor.highlight_infos
}

pub fn default_highlight_hints() -> bool {
    CONSTANTS.core_rust.defaults.code_editor.highlight_hints
}

pub fn default_auto_scan_interval() -> u32 {
    CONSTANTS.core_rust.defaults.code_editor.auto_scan_interval
}

pub fn default_severity() -> Severity {
    Severity::Warning
}

pub fn default_auto_ai_scan_interval() -> u32 {
    CONSTANTS
        .core_rust
        .defaults
        .code_editor
        .auto_ai_scan_interval
}

pub fn icon_builtin() -> &'static str {
    &CONSTANTS.shared.icons.builtin
}

pub fn icon_regex() -> &'static str {
    &CONSTANTS.shared.icons.regex
}

pub fn icon_script() -> &'static str {
    &CONSTANTS.shared.icons.script
}

pub fn icon_ai() -> &'static str {
    &CONSTANTS.shared.icons.ai
}

pub fn icon_error() -> &'static str {
    &CONSTANTS.shared.icons.error
}

pub fn icon_warning() -> &'static str {
    &CONSTANTS.shared.icons.warning
}

pub fn icon_info() -> &'static str {
    &CONSTANTS.shared.icons.info
}

pub fn icon_hint() -> &'static str {
    &CONSTANTS.shared.icons.hint
}

pub fn icon_progress() -> &'static str {
    &CONSTANTS.shared.icons.progress
}

pub fn icon_success() -> &'static str {
    &CONSTANTS.shared.icons.success
}

pub fn icon_skipped() -> &'static str {
    &CONSTANTS.shared.icons.skipped
}

pub fn is_js_ts_extension(ext: &str) -> bool {
    CONSTANTS
        .shared
        .extensions
        .javascript
        .iter()
        .any(|e| e == ext)
}

pub fn cache_dir_name() -> &'static str {
    &CONSTANTS.core_rust.cache.dir_name
}

pub fn ai_temp_dir() -> &'static str {
    &CONSTANTS.core_rust.ai.temp_dir
}

pub fn ai_placeholder_files() -> &'static str {
    &CONSTANTS.core_rust.ai.placeholders.files
}

pub fn ai_placeholder_content() -> &'static str {
    &CONSTANTS.core_rust.ai.placeholders.content
}

pub fn ai_placeholder_options() -> &'static str {
    &CONSTANTS.core_rust.ai.placeholders.options
}

pub fn claude_command() -> &'static str {
    &CONSTANTS.core_rust.ai.providers.claude.command
}

pub fn claude_args() -> &'static [String] {
    &CONSTANTS.core_rust.ai.providers.claude.args
}

pub fn gemini_command() -> &'static str {
    &CONSTANTS.core_rust.ai.providers.gemini.command
}

pub fn gemini_args() -> &'static [String] {
    &CONSTANTS.core_rust.ai.providers.gemini.args
}

pub fn rules_base_url() -> &'static str {
    &CONSTANTS.shared.urls.rules_base
}

pub fn lsp_method_scan() -> &'static str {
    &CONSTANTS.shared.lsp.methods.scan
}

pub fn lsp_method_scan_file() -> &'static str {
    &CONSTANTS.shared.lsp.methods.scan_file
}

pub fn lsp_method_scan_content() -> &'static str {
    &CONSTANTS.shared.lsp.methods.scan_content
}

pub fn lsp_method_clear_cache() -> &'static str {
    &CONSTANTS.shared.lsp.methods.clear_cache
}

pub fn lsp_method_get_rules_metadata() -> &'static str {
    &CONSTANTS.shared.lsp.methods.get_rules_metadata
}

pub fn lsp_method_format_results() -> &'static str {
    &CONSTANTS.shared.lsp.methods.format_results
}

pub fn lsp_method_validate_config() -> &'static str {
    &CONSTANTS.shared.lsp.methods.validate_config
}

pub fn lsp_method_ai_progress() -> &'static str {
    &CONSTANTS.shared.lsp.methods.ai_progress
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
