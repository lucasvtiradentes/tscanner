use serde::Serialize;
use tscanner_constants::{
    default_ai_scan_interval, default_exclude, default_highlight_errors, default_highlight_hints,
    default_highlight_infos, default_highlight_warnings, default_include, default_scan_interval,
};

use crate::types::{CodeEditorConfig, FilesConfig, TscannerConfig};

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
