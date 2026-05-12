use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Constants {
    pub(super) shared: SharedConfig,
    pub(super) core_rust: CoreRustConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SharedConfig {
    pub(super) package_name: String,
    pub(super) config_dir_name: String,
    pub(super) config_file_name: String,
    pub(super) log_basename: String,
    pub(super) log_timezone_offset_hours: i8,
    pub(super) log_context_width: usize,
    pub(super) ignore_comment: String,
    pub(super) ignore_next_line_comment: String,
    pub(super) config_error_prefix: String,
    pub(super) extensions: ExtensionsConfig,
    pub(super) icons: IconsConfig,
    pub(super) urls: UrlsConfig,
    pub(super) lsp: LspConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct UrlsConfig {
    pub(super) rules_base: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct LspConfig {
    pub(super) methods: LspMethodsConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct LspMethodsConfig {
    pub(super) scan: String,
    pub(super) scan_file: String,
    pub(super) scan_content: String,
    pub(super) clear_cache: String,
    pub(super) get_rules_metadata: String,
    pub(super) format_results: String,
    pub(super) validate_config: String,
    pub(super) ai_progress: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CoreRustConfig {
    pub(super) defaults: Defaults,
    pub(super) cache: CacheConfig,
    pub(super) ai: AiConstantsConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Defaults {
    pub(super) code_editor: CodeEditorDefaults,
    pub(super) directories: DirectoryDefaults,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct CodeEditorDefaults {
    pub(super) highlight_errors: bool,
    pub(super) highlight_warnings: bool,
    pub(super) highlight_infos: bool,
    pub(super) highlight_hints: bool,
    pub(super) auto_scan_interval: u32,
    pub(super) auto_ai_scan_interval: u32,
    pub(super) startup_scan: String,
    pub(super) startup_ai_scan: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct DirectoryDefaults {
    pub(super) script_rules: String,
    pub(super) ai_rules: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct IconsConfig {
    pub(super) builtin: String,
    pub(super) regex: String,
    pub(super) script: String,
    pub(super) ai: String,
    pub(super) error: String,
    pub(super) warning: String,
    pub(super) info: String,
    pub(super) hint: String,
    pub(super) progress: String,
    pub(super) success: String,
    pub(super) skipped: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct ExtensionsConfig {
    pub(super) javascript: Vec<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct CacheConfig {
    pub(super) dir_name: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct AiConstantsConfig {
    pub(super) temp_dir: String,
    pub(super) placeholders: AiPlaceholdersConfig,
    pub(super) providers: AiProvidersConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct AiPlaceholdersConfig {
    pub(super) files: String,
    pub(super) content: String,
    pub(super) options: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct AiProvidersConfig {
    pub(super) claude: AiProviderConfig,
    pub(super) gemini: AiProviderConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct AiProviderConfig {
    pub(super) command: String,
    pub(super) args: Vec<String>,
}
