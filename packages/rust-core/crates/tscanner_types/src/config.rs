use crate::enums::{AiMode, AiProvider, Severity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod defaults;

fn default_severity() -> Severity {
    Severity::Warning
}

fn is_default_severity(s: &Severity) -> bool {
    *s == Severity::Warning
}

fn is_default_mode(m: &AiMode) -> bool {
    *m == AiMode::Paths
}

fn is_zero(v: &u64) -> bool {
    *v == 0
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiConfig {
    #[schemars(description = "AI provider to use (claude, codex, gemini)")]
    pub provider: AiProvider,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "AI model to use")]
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FilesConfig {
    #[schemars(description = "File patterns to include (required)")]
    pub include: Vec<String>,

    #[schemars(description = "File patterns to exclude (required)")]
    pub exclude: Vec<String>,
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

    #[serde(default)]
    #[schemars(description = "Rules configuration (builtin, regex, script)")]
    pub rules: RulesConfig,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[schemars(description = "AI-powered rules configuration")]
    pub ai_rules: HashMap<String, AiRuleConfig>,

    #[schemars(description = "File patterns configuration (required)")]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScriptRuleConfig {
    #[schemars(description = "Full command to execute the script")]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiRuleConfig {
    #[schemars(description = "Path to AI prompt markdown file")]
    pub prompt: String,

    #[schemars(description = "Error message to display when rule is violated")]
    pub message: String,

    #[serde(default, skip_serializing_if = "is_default_mode")]
    #[schemars(description = "How files are provided to the AI")]
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
    #[schemars(description = "Timeout in seconds (default: 0 = no limit)")]
    pub timeout: u64,

    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    #[schemars(description = "Additional options")]
    pub options: serde_json::Value,
}
