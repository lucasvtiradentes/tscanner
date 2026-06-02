use crate::enums::{AiMode, AiProvider, Severity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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

fn is_empty_string(s: &String) -> bool {
    s.is_empty()
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

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "AI-powered markdown rule sources")]
    pub ai_rules: Vec<AiRuleSourceConfig>,

    #[serde(default, skip)]
    #[schemars(skip)]
    pub resolved_ai_rules: Vec<ResolvedAiRuleConfig>,

    #[serde(default, skip)]
    #[schemars(skip)]
    pub ai_rule_summary: AiRuleSummary,

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AiRuleClassification {
    CodeCheckable,
    #[default]
    GuidanceOnly,
    Unsupported,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AiRuleSourceType {
    Cursor,
    Claude,
    #[default]
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiRuleSourceConfig {
    #[schemars(description = "Path to an AI markdown rule file or folder")]
    pub path: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File names, folder names, or rule ids to ignore under this source")]
    pub ignore: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Optional id override for a single-file source")]
    pub id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Finding message override")]
    pub message: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "How files are provided to the AI")]
    pub mode: Option<AiMode>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Severity level (default: warning)")]
    pub severity: Option<Severity>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to include")]
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to exclude")]
    pub exclude: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Timeout in seconds")]
    pub timeout: Option<u64>,

    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    #[schemars(description = "Additional options")]
    pub options: serde_json::Value,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Whether this markdown rule should run as a code check")]
    pub classification: Option<AiRuleClassification>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedAiRuleConfig {
    #[serde(default, skip_serializing_if = "is_empty_string")]
    pub id: String,

    pub prompt_path: PathBuf,

    #[serde(default, skip_serializing_if = "is_zero")]
    pub prompt_hash: u64,

    pub message: String,

    #[serde(default, skip_serializing_if = "is_default_mode")]
    pub mode: AiMode,

    #[serde(
        default = "default_severity",
        skip_serializing_if = "is_default_severity"
    )]
    pub severity: Severity,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,

    #[serde(default, skip_serializing_if = "is_zero")]
    pub timeout: u64,

    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub options: serde_json::Value,

    pub source_path: String,

    pub file_path: String,

    pub source_type: AiRuleSourceType,

    pub classification: AiRuleClassification,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiRuleSummary {
    pub source_count: usize,
    pub markdown_count: usize,
    pub code_checkable_count: usize,
    pub guidance_only_count: usize,
    pub unsupported_count: usize,
    pub skipped_count: usize,
}
