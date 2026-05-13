use super::{AiRuleConfig, RegexRuleConfig, ScriptRuleConfig};
use crate::enums::{AiMode, Severity};

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
