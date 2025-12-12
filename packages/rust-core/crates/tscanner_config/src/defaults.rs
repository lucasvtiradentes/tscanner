use tscanner_constants::{
    default_auto_ai_scan_interval, default_auto_scan_interval, default_highlight_errors,
    default_highlight_hints, default_highlight_infos, default_highlight_warnings,
    default_use_ai_scan_cache, default_use_scan_cache,
};

use crate::types::CodeEditorConfig;

pub fn default_code_editor_config() -> CodeEditorConfig {
    CodeEditorConfig {
        highlight_errors: default_highlight_errors(),
        highlight_warnings: default_highlight_warnings(),
        highlight_infos: default_highlight_infos(),
        highlight_hints: default_highlight_hints(),
        auto_scan_interval: default_auto_scan_interval(),
        auto_ai_scan_interval: default_auto_ai_scan_interval(),
        use_scan_cache: default_use_scan_cache(),
        use_ai_scan_cache: default_use_ai_scan_cache(),
    }
}
