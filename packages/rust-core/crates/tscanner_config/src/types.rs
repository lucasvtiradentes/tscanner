use globset::GlobSet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tscanner_constants::{
    default_ai_scan_interval, default_exclude, default_highlight_errors, default_highlight_hints,
    default_highlight_infos, default_highlight_warnings, default_include, default_scan_interval,
    default_severity,
};
use tscanner_types::Severity;

use crate::defaults::{default_code_editor_config, default_files_config};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AiMode {
    #[default]
    Paths,
    Content,
    Agentic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AiProvider {
    Claude,
    Gemini,
    Custom,
}

impl AiProvider {
    pub const ALL: &'static [AiProvider] =
        &[AiProvider::Claude, AiProvider::Gemini, AiProvider::Custom];

    pub fn as_str(&self) -> &'static str {
        match self {
            AiProvider::Claude => "claude",
            AiProvider::Gemini => "gemini",
            AiProvider::Custom => "custom",
        }
    }

    pub fn all_names() -> String {
        Self::ALL
            .iter()
            .map(|p| p.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AiExecutionMode {
    #[default]
    Ignore,
    Include,
    Only,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "AI provider to use (claude, gemini, custom)")]
    pub provider: Option<AiProvider>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Custom command path (required only for 'custom' provider)")]
    pub command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CodeEditorConfig {
    #[serde(default = "default_highlight_errors")]
    #[schemars(
        default = "default_highlight_errors",
        description = "Highlight error issues in the code editor"
    )]
    pub highlight_errors: bool,

    #[serde(default = "default_highlight_warnings")]
    #[schemars(
        default = "default_highlight_warnings",
        description = "Highlight warning issues in the code editor"
    )]
    pub highlight_warnings: bool,

    #[serde(default = "default_highlight_infos")]
    #[schemars(
        default = "default_highlight_infos",
        description = "Highlight info issues in the code editor"
    )]
    pub highlight_infos: bool,

    #[serde(default = "default_highlight_hints")]
    #[schemars(
        default = "default_highlight_hints",
        description = "Highlight hint issues in the code editor"
    )]
    pub highlight_hints: bool,

    #[serde(default = "default_scan_interval")]
    #[schemars(
        default = "default_scan_interval",
        description = "Auto-scan interval in seconds (0 = disabled, only manual/on-save scans)"
    )]
    pub scan_interval: u32,

    #[serde(default = "default_ai_scan_interval")]
    #[schemars(
        default = "default_ai_scan_interval",
        description = "Auto-scan interval for AI rules in seconds (0 = disabled). Runs only AI rules on a separate schedule."
    )]
    pub ai_scan_interval: u32,
}

impl Default for CodeEditorConfig {
    fn default() -> Self {
        default_code_editor_config()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FilesConfig {
    #[serde(default = "default_include")]
    #[schemars(default = "default_include", description = "File patterns to include")]
    pub include: Vec<String>,

    #[serde(default = "default_exclude")]
    #[schemars(default = "default_exclude", description = "File patterns to exclude")]
    pub exclude: Vec<String>,
}

impl Default for FilesConfig {
    fn default() -> Self {
        default_files_config()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct RulesConfig {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[schemars(description = "Built-in AST rules configuration")]
    pub builtin: HashMap<String, BuiltinRuleConfig>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[schemars(description = "Regex-based rules configuration")]
    pub regex: HashMap<String, RegexRuleConfig>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[schemars(description = "Script-based rules configuration")]
    pub script: HashMap<String, ScriptRuleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TscannerConfig {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "JSON schema URL for editor support")]
    pub schema: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "AI provider configuration for AI-powered rules")]
    pub ai: Option<AiConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Code editor configuration (highlighting, auto-scan)")]
    pub code_editor: Option<CodeEditorConfig>,

    #[serde(default)]
    #[schemars(description = "Rules configuration (builtin, regex, script)")]
    pub rules: RulesConfig,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[schemars(description = "AI-powered rules configuration")]
    pub ai_rules: HashMap<String, AiRuleConfig>,

    #[serde(default)]
    #[schemars(description = "File patterns configuration")]
    pub files: FilesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BuiltinRuleConfig {
    #[serde(
        default = "default_severity",
        skip_serializing_if = "is_default_severity"
    )]
    #[schemars(description = "Severity level for this rule (default: warning)")]
    pub severity: Severity,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to include for this rule")]
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to exclude for this rule")]
    pub exclude: Vec<String>,

    #[serde(flatten)]
    #[schemars(skip)]
    pub options: HashMap<String, serde_json::Value>,
}

fn is_default_severity(s: &Severity) -> bool {
    *s == Severity::Warning
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegexRuleConfig {
    #[schemars(description = "Regex pattern to match")]
    pub pattern: String,

    #[schemars(description = "Error message to display when rule is violated")]
    pub message: String,

    #[serde(
        default = "default_severity",
        skip_serializing_if = "is_default_severity"
    )]
    #[schemars(description = "Severity level (default: warning)")]
    pub severity: Severity,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to include")]
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to exclude")]
    pub exclude: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScriptRuleConfig {
    #[schemars(
        description = "Full command to execute the script (e.g., 'npx tsx script-rules/my-script.ts --arg'). Path is relative to .tscanner/ directory."
    )]
    pub command: String,

    #[schemars(description = "Error message to display when rule is violated")]
    pub message: String,

    #[serde(
        default = "default_severity",
        skip_serializing_if = "is_default_severity"
    )]
    #[schemars(description = "Severity level (default: warning)")]
    pub severity: Severity,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to include")]
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to exclude")]
    pub exclude: Vec<String>,

    #[serde(default, skip_serializing_if = "is_zero")]
    #[schemars(description = "Script timeout in seconds (default: 0 = no limit)")]
    pub timeout: u64,

    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    #[schemars(description = "Additional options to pass to the script")]
    pub options: serde_json::Value,
}

fn is_zero(v: &u64) -> bool {
    *v == 0
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiRuleConfig {
    #[schemars(description = "Path to AI prompt markdown file (relative to .tscanner/ai-rules/)")]
    pub prompt: String,

    #[schemars(description = "Error message to display when rule is violated")]
    pub message: String,

    #[serde(default, skip_serializing_if = "is_default_mode")]
    #[schemars(
        description = "How files are provided to the AI: 'paths' (default) sends only file paths, 'content' sends file contents in prompt, 'agentic' lets AI explore files autonomously"
    )]
    pub mode: AiMode,

    #[serde(
        default = "default_severity",
        skip_serializing_if = "is_default_severity"
    )]
    #[schemars(description = "Severity level (default: warning)")]
    pub severity: Severity,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to include")]
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to exclude")]
    pub exclude: Vec<String>,

    #[serde(default, skip_serializing_if = "is_zero")]
    #[schemars(description = "Timeout for this rule in seconds (default: 0 = no limit)")]
    pub timeout: u64,

    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    #[schemars(description = "Additional options to pass to the AI prompt")]
    pub options: serde_json::Value,
}

fn is_default_mode(m: &AiMode) -> bool {
    *m == AiMode::Paths
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
