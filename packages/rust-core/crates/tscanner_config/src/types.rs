use globset::GlobSet;
use std::path::Path;

pub use tscanner_types::{
    AiConfig, AiExecutionMode, AiMode, AiProvider, AiRuleConfig, BuiltinRuleConfig,
    CodeEditorConfig, FilesConfig, RegexRuleConfig, RulesConfig, ScriptRuleConfig, Severity,
    TscannerConfig,
};

pub struct CompiledRuleConfig {
    pub severity: Severity,
    pub global_include: GlobSet,
    pub global_exclude: GlobSet,
    pub rule_include: Option<GlobSet>,
    pub rule_exclude: Option<GlobSet>,
    pub message: Option<String>,
    pub pattern: Option<String>,
    pub options: Option<serde_json::Value>,
}

impl CompiledRuleConfig {
    pub fn matches(&self, relative_path: &Path) -> bool {
        let matches_include = if let Some(ref rule_include) = self.rule_include {
            rule_include.is_match(relative_path)
        } else {
            self.global_include.is_match(relative_path)
        };
        if !matches_include {
            return false;
        }
        if self.global_exclude.is_match(relative_path) {
            return false;
        }
        if let Some(ref rule_exclude) = self.rule_exclude {
            if rule_exclude.is_match(relative_path) {
                return false;
            }
        }
        true
    }
}
