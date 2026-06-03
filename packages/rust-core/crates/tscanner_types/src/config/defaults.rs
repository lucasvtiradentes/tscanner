use super::{AiRuleSourceConfig, RegexRuleConfig, ResolvedAiRuleConfig, ScriptRuleConfig};
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

impl Default for AiRuleSourceConfig {
    fn default() -> Self {
        Self {
            path: String::new(),
            ignore: Vec::new(),
            id: None,
            message: None,
            mode: None,
            severity: None,
            include: Vec::new(),
            exclude: Vec::new(),
            timeout: None,
            options: serde_json::Value::Null,
            classification: None,
        }
    }
}

impl Default for ResolvedAiRuleConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            prompt_path: std::path::PathBuf::new(),
            prompt_hash: 0,
            message: String::new(),
            mode: AiMode::Paths,
            severity: Severity::Warning,
            include: Vec::new(),
            exclude: Vec::new(),
            timeout: 0,
            options: serde_json::Value::Null,
            source_path: String::new(),
            file_path: String::new(),
            source_type: super::AiRuleSourceType::Generic,
            classification: super::AiRuleClassification::GuidanceOnly,
        }
    }
}
