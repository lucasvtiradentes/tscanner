use globset::GlobSet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tscanner_diagnostics::Severity;

use crate::defaults::{
    default_ai_execution_mode, default_ai_scan_interval_seconds, default_ai_timeout,
    default_cli_config, default_cli_group_by, default_cli_no_cache,
    default_cli_show_issue_description, default_cli_show_issue_rule_name,
    default_cli_show_issue_severity, default_cli_show_issue_source_line, default_cli_show_settings,
    default_cli_show_summary, default_code_editor_config, default_exclude, default_files_config,
    default_highlight_errors, default_highlight_warnings, default_include,
    default_scan_interval_seconds, default_script_timeout, default_severity, default_true,
};

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
    #[schemars(description = "AI provider to use (claude, gemini, custom)")]
    pub provider: AiProvider,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Custom command path (required only for 'custom' provider)")]
    pub command: Option<String>,

    #[serde(default = "default_ai_timeout")]
    #[schemars(description = "Timeout in milliseconds for AI calls (default: 120000)")]
    pub timeout: u64,
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

    #[serde(default = "default_scan_interval_seconds")]
    #[schemars(
        default = "default_scan_interval_seconds",
        description = "Auto-scan interval in seconds (0 = disabled, only manual/on-save scans)"
    )]
    pub scan_interval_seconds: u32,

    #[serde(default = "default_ai_scan_interval_seconds")]
    #[schemars(
        default = "default_ai_scan_interval_seconds",
        description = "Auto-scan interval for AI rules in seconds (0 = disabled). Runs only AI rules on a separate schedule."
    )]
    pub ai_scan_interval_seconds: u32,
}

impl Default for CodeEditorConfig {
    fn default() -> Self {
        default_code_editor_config()
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum CliGroupBy {
    #[default]
    File,
    Rule,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CliConfig {
    #[serde(default = "default_cli_group_by")]
    #[schemars(
        default = "default_cli_group_by",
        description = "Group issues by file or rule"
    )]
    pub group_by: CliGroupBy,

    #[serde(default = "default_cli_no_cache")]
    #[schemars(
        default = "default_cli_no_cache",
        description = "Skip cache and force full scan"
    )]
    pub no_cache: bool,

    #[serde(default = "default_cli_show_settings")]
    #[schemars(
        default = "default_cli_show_settings",
        description = "Show check settings"
    )]
    pub show_settings: bool,

    #[serde(default = "default_cli_show_issue_severity")]
    #[schemars(
        default = "default_cli_show_issue_severity",
        description = "Show issue severity icon"
    )]
    pub show_issue_severity: bool,

    #[serde(default = "default_cli_show_issue_source_line")]
    #[schemars(
        default = "default_cli_show_issue_source_line",
        description = "Show issue source line text"
    )]
    pub show_issue_source_line: bool,

    #[serde(default = "default_cli_show_issue_rule_name")]
    #[schemars(
        default = "default_cli_show_issue_rule_name",
        description = "Show issue rule name"
    )]
    pub show_issue_rule_name: bool,

    #[serde(default = "default_cli_show_issue_description")]
    #[schemars(
        default = "default_cli_show_issue_description",
        description = "Show issue rule description/message"
    )]
    pub show_issue_description: bool,

    #[serde(default = "default_cli_show_summary")]
    #[schemars(
        default = "default_cli_show_summary",
        description = "Show check summary"
    )]
    pub show_summary: bool,

    #[serde(default = "default_ai_execution_mode")]
    #[schemars(
        default = "default_ai_execution_mode",
        description = "AI rules execution mode: 'ignore' (default), 'include', or 'only'"
    )]
    pub ai_mode: AiExecutionMode,
}

impl Default for CliConfig {
    fn default() -> Self {
        default_cli_config()
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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "CLI output configuration")]
    pub cli: Option<CliConfig>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Enable or disable this rule")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Severity level for this rule")]
    pub severity: Option<Severity>,

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

    #[serde(default = "default_severity")]
    #[schemars(description = "Severity level (default: warning)")]
    pub severity: Severity,

    #[serde(default = "default_true")]
    #[schemars(description = "Enable or disable this rule (default: true)")]
    pub enabled: bool,

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
            enabled: true,
            include: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScriptRuleConfig {
    #[schemars(
        description = "Full command to execute the script (e.g., 'npx tsx scripts/my-script.ts --arg'). Path is relative to .tscanner/ directory."
    )]
    pub command: String,

    #[schemars(description = "Error message to display when rule is violated")]
    pub message: String,

    #[serde(default = "default_severity")]
    #[schemars(description = "Severity level (default: warning)")]
    pub severity: Severity,

    #[serde(default = "default_true")]
    #[schemars(description = "Enable or disable this rule (default: true)")]
    pub enabled: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to include")]
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to exclude")]
    pub exclude: Vec<String>,

    #[serde(default = "default_script_timeout")]
    #[schemars(description = "Script timeout in milliseconds (default: 10000)")]
    pub timeout: u64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional options to pass to the script")]
    pub options: Option<serde_json::Value>,
}

impl Default for ScriptRuleConfig {
    fn default() -> Self {
        Self {
            command: String::new(),
            message: String::new(),
            severity: Severity::Warning,
            enabled: true,
            include: Vec::new(),
            exclude: Vec::new(),
            timeout: 10000,
            options: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiRuleConfig {
    #[schemars(description = "Path to AI prompt markdown file (relative to .tscanner/prompts/)")]
    pub prompt: String,

    #[schemars(description = "Error message to display when rule is violated")]
    pub message: String,

    #[serde(default)]
    #[schemars(
        description = "How files are provided to the AI: 'paths' (default) sends only file paths, 'content' sends file contents in prompt, 'agentic' lets AI explore files autonomously"
    )]
    pub mode: AiMode,

    #[serde(default = "default_severity")]
    #[schemars(description = "Severity level (default: warning)")]
    pub severity: Severity,

    #[serde(default = "default_true")]
    #[schemars(description = "Enable or disable this rule (default: true)")]
    pub enabled: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to include")]
    pub include: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "File patterns to exclude")]
    pub exclude: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Override timeout for this rule in milliseconds")]
    pub timeout: Option<u64>,
}

impl Default for AiRuleConfig {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            message: String::new(),
            mode: AiMode::Paths,
            severity: Severity::Warning,
            enabled: true,
            include: Vec::new(),
            exclude: Vec::new(),
            timeout: None,
        }
    }
}

pub struct CompiledRuleConfig {
    pub enabled: bool,
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
