use tscanner_constants::{
    default_ai_scan_interval, default_ai_scan_use_cache, default_highlight_errors,
    default_highlight_hints, default_highlight_infos, default_highlight_warnings,
    default_scan_interval, default_scan_use_cache,
};

use crate::types::CodeEditorConfig;

pub fn default_code_editor_config() -> CodeEditorConfig {
    CodeEditorConfig {
        highlight_errors: default_highlight_errors(),
        highlight_warnings: default_highlight_warnings(),
        highlight_infos: default_highlight_infos(),
        highlight_hints: default_highlight_hints(),
        scan_interval: default_scan_interval(),
        ai_scan_interval: default_ai_scan_interval(),
        scan_use_cache: default_scan_use_cache(),
        ai_scan_use_cache: default_ai_scan_use_cache(),
    }
}
