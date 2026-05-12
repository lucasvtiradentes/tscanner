use super::{AiRuleConfig, CodeEditorConfig, RegexRuleConfig, ScriptRuleConfig};
use crate::enums::{AiMode, Severity, StartupScanMode};

impl Default for CodeEditorConfig {
    fn default() -> Self {
        Self {
            highlight_errors: tscanner_constants::default_highlight_errors(),
            highlight_warnings: tscanner_constants::default_highlight_warnings(),
            highlight_infos: tscanner_constants::default_highlight_infos(),
            highlight_hints: tscanner_constants::default_highlight_hints(),
            auto_scan_interval: tscanner_constants::default_auto_scan_interval(),
            auto_ai_scan_interval: tscanner_constants::default_auto_ai_scan_interval(),
            startup_scan: StartupScanMode::from_str_or_panic(
                tscanner_constants::default_startup_scan(),
            ),
            startup_ai_scan: StartupScanMode::from_str_or_panic(
                tscanner_constants::default_startup_ai_scan(),
            ),
        }
    }
}

impl Default for RegexRuleConfig {
    fn default() -> Self {
        Self {
            pattern: String::new(),
            message: String::new(),
            severity: Severity::Warning,
            include: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

impl Default for ScriptRuleConfig {
    fn default() -> Self {
        Self {
            command: String::new(),
            message: String::new(),
            severity: Severity::Warning,
            include: Vec::new(),
            exclude: Vec::new(),
            timeout: 0,
            options: serde_json::Value::Null,
        }
    }
}

impl Default for AiRuleConfig {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            message: String::new(),
            mode: AiMode::Paths,
            severity: Severity::Warning,
            include: Vec::new(),
            exclude: Vec::new(),
            timeout: 0,
            options: serde_json::Value::Null,
        }
    }
}
