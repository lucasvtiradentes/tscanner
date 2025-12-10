use serde::Serialize;

use crate::types::{CodeEditorConfig, FilesConfig, TscannerConfig};

pub use tscanner_constants::{
    ai_inflight_wait_ms, ai_placeholder_content, ai_placeholder_files, ai_polling_ms, ai_rules_dir,
    ai_temp_dir, app_description, app_display_name, app_name, cache_dir_name, cache_file_pattern,
    claude_args, claude_command, config_dir_name, config_file_name, conflicting_rules,
    default_ai_scan_interval, default_exclude, default_highlight_errors, default_highlight_hints,
    default_highlight_infos, default_highlight_warnings, default_include, default_scan_interval,
    default_target_branch, example_ai_rule, example_script_rule, gemini_args, gemini_command,
    get_log_filename, icon_ai, icon_builtin, icon_error, icon_hint, icon_info, icon_progress,
    icon_regex, icon_script, icon_skipped, icon_success, icon_warning, ignore_comment,
    ignore_next_line_comment, is_dev_mode, js_extensions, log_basename, lsp_debounce_ms,
    rule_types, rules_base_url, scheduler_polling_ms, script_polling_ms, script_rules_dir,
};

lazy_static::lazy_static! {
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
        highlight_infos: bool,
        highlight_hints: bool,
        scan_interval: u32,
        ai_scan_interval: u32,
    }

    let config = DefaultConfig {
        files: DefaultFilesConfig {
            include: default_include(),
            exclude: default_exclude(),
        },
        code_editor: DefaultCodeEditorConfig {
            highlight_errors: default_highlight_errors(),
            highlight_warnings: default_highlight_warnings(),
            highlight_infos: default_highlight_infos(),
            highlight_hints: default_highlight_hints(),
            scan_interval: default_scan_interval(),
            ai_scan_interval: default_ai_scan_interval(),
        },
    };

    serde_json::to_string(&config).expect("Failed to serialize default config")
}

pub fn default_code_editor_config() -> CodeEditorConfig {
    CodeEditorConfig {
        highlight_errors: default_highlight_errors(),
        highlight_warnings: default_highlight_warnings(),
        highlight_infos: default_highlight_infos(),
        highlight_hints: default_highlight_hints(),
        scan_interval: default_scan_interval(),
        ai_scan_interval: default_ai_scan_interval(),
    }
}

pub fn default_files_config() -> FilesConfig {
    FilesConfig {
        include: default_include(),
        exclude: default_exclude(),
    }
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
