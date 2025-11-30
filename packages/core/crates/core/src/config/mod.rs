mod validation;

pub use validation::{validate_json_fields, ValidationResult};

use crate::output::Severity;
use globset::{Glob, GlobSet, GlobSetBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::path::Path;

const TSCANNER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CONFIG_ERROR_PREFIX: &str = "TSCANNER_CONFIG_ERROR:";

const DEFAULT_CONFIG_JSON: &str = include_str!("../../../../../../assets/default-config.json");

fn default_code_editor_config() -> CodeEditorConfig {
    let config: TscannerConfig =
        serde_json::from_str(DEFAULT_CONFIG_JSON).expect("Failed to parse default-config.json");
    config
        .code_editor
        .expect("default-config.json must have 'codeEditor' section")
}

fn default_highlight_errors() -> bool {
    default_code_editor_config().highlight_errors
}

fn default_highlight_warnings() -> bool {
    default_code_editor_config().highlight_warnings
}

fn default_scan_interval_seconds() -> u32 {
    default_code_editor_config().scan_interval_seconds
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

fn default_cli_config() -> CliConfig {
    let config: TscannerConfig =
        serde_json::from_str(DEFAULT_CONFIG_JSON).expect("Failed to parse default-config.json");
    config
        .cli
        .expect("default-config.json must have 'cli' section")
}

fn default_cli_group_by() -> CliGroupBy {
    default_cli_config().group_by
}

fn default_cli_no_cache() -> bool {
    default_cli_config().no_cache
}

fn default_cli_show_issue_severity() -> bool {
    default_cli_config().show_issue_severity
}

fn default_cli_show_issue_source_line() -> bool {
    default_cli_config().show_issue_source_line
}

fn default_cli_show_issue_rule_name() -> bool {
    default_cli_config().show_issue_rule_name
}

fn default_cli_show_settings() -> bool {
    default_cli_config().show_settings
}

fn default_cli_show_issue_description() -> bool {
    default_cli_config().show_issue_description
}

fn default_cli_show_summary() -> bool {
    default_cli_config().show_summary
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
}

impl Default for CliConfig {
    fn default() -> Self {
        default_cli_config()
    }
}

fn default_files_config() -> FilesConfig {
    let config: TscannerConfig =
        serde_json::from_str(DEFAULT_CONFIG_JSON).expect("Failed to parse default-config.json");
    config.files
}

fn default_include() -> Vec<String> {
    default_files_config().include
}

fn default_exclude() -> Vec<String> {
    default_files_config().exclude
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TscannerConfig {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "JSON schema URL for editor support")]
    pub schema: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Code editor configuration (highlighting, auto-scan)")]
    pub code_editor: Option<CodeEditorConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "CLI output configuration")]
    pub cli: Option<CliConfig>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[schemars(description = "Built-in AST rules configuration")]
    pub builtin_rules: HashMap<String, BuiltinRuleConfig>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[schemars(description = "Custom rules configuration (regex, script, or AI)")]
    pub custom_rules: HashMap<String, CustomRuleConfig>,

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
pub struct CustomRuleConfig {
    #[serde(rename = "type")]
    #[schemars(description = "Type of custom rule")]
    pub rule_type: CustomRuleType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Regex pattern (required for type: regex)")]
    pub pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Path to script file (required for type: script)")]
    pub script: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Path to AI prompt markdown file (required for type: ai)")]
    pub prompt: Option<String>,

    #[schemars(description = "Error message to display when rule is violated")]
    pub message: String,

    #[serde(default = "default_severity")]
    #[schemars(description = "Severity level (default: error)")]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum CustomRuleType {
    Regex,
    Script,
    Ai,
}

fn default_true() -> bool {
    true
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
        if !self.global_include.is_match(relative_path) {
            return false;
        }
        if self.global_exclude.is_match(relative_path) {
            return false;
        }
        if let Some(ref rule_include) = self.rule_include {
            if !rule_include.is_match(relative_path) {
                return false;
            }
        }
        if let Some(ref rule_exclude) = self.rule_exclude {
            if rule_exclude.is_match(relative_path) {
                return false;
            }
        }
        true
    }
}

fn default_severity() -> Severity {
    Severity::Warning
}

fn compile_optional_globset(
    patterns: &[String],
) -> Result<Option<GlobSet>, Box<dyn std::error::Error>> {
    if patterns.is_empty() {
        Ok(None)
    } else {
        Ok(Some(compile_globset(patterns)?))
    }
}

impl TscannerConfig {
    pub fn parse_json(content: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let json_without_comments = json_comments::StripComments::new(content.as_bytes());
        let json_value: serde_json::Value = serde_json::from_reader(json_without_comments)?;
        Ok(json_value)
    }

    pub fn full_validate(
        content: &str,
    ) -> Result<(Self, ValidationResult), Box<dyn std::error::Error>> {
        let json_value = Self::parse_json(content)?;

        let mut result = validate_json_fields(&json_value);
        if !result.is_valid() {
            return Ok((Self::default(), result));
        }

        let config: Self = serde_json::from_value(json_value)?;
        result.merge(config.validate());

        Ok((config, result))
    }

    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let (config, result) = Self::full_validate(&content)?;

        for warning in &result.warnings {
            eprintln!("Warning: {}", warning);
        }

        if !result.is_valid() {
            let invalid_fields: Vec<_> = result
                .errors
                .iter()
                .filter_map(|e| e.strip_prefix("Invalid field: "))
                .collect();

            if !invalid_fields.is_empty() {
                return Err(format!(
                    "{}invalid_fields=[{}];version={}",
                    CONFIG_ERROR_PREFIX,
                    invalid_fields.join(","),
                    TSCANNER_VERSION
                )
                .into());
            }

            return Err(format!(
                "Config validation failed:\n  - {}",
                result.errors.join("\n  - ")
            )
            .into());
        }

        Ok(config)
    }

    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        for (name, rule_config) in &self.custom_rules {
            match rule_config.rule_type {
                CustomRuleType::Regex => {
                    if rule_config.pattern.is_none() {
                        result.add_error(format!(
                            "Custom rule '{}' is type 'regex' but has no 'pattern' field",
                            name
                        ));
                    } else if let Some(pattern) = &rule_config.pattern {
                        if let Err(e) = regex::Regex::new(pattern) {
                            result.add_error(format!(
                                "Rule '{}' has invalid regex pattern: {}",
                                name, e
                            ));
                        }
                    }
                }
                CustomRuleType::Script => {
                    if rule_config.script.is_none() {
                        result.add_error(format!(
                            "Custom rule '{}' is type 'script' but has no 'script' field",
                            name
                        ));
                    }
                }
                CustomRuleType::Ai => {
                    if rule_config.prompt.is_none() {
                        result.add_error(format!(
                            "Custom rule '{}' is type 'ai' but has no 'prompt' field",
                            name
                        ));
                    }
                }
            }
        }

        let conflicting_builtin_rules = [
            ("prefer-type-over-interface", "prefer-interface-over-type"),
            ("no-relative-imports", "no-absolute-imports"),
        ];

        for (rule1, rule2) in &conflicting_builtin_rules {
            let rule1_enabled = self
                .builtin_rules
                .get(*rule1)
                .and_then(|r| r.enabled)
                .unwrap_or(false);
            let rule2_enabled = self
                .builtin_rules
                .get(*rule2)
                .and_then(|r| r.enabled)
                .unwrap_or(false);

            if rule1_enabled && rule2_enabled {
                result.add_warning(format!(
                    "Conflicting rules enabled: '{}' and '{}'",
                    rule1, rule2
                ));
            }
        }

        result
    }

    pub fn load_from_workspace(workspace: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        use crate::constants::{config_dir_name, config_file_name};

        let config_path = workspace.join(config_dir_name()).join(config_file_name());

        if config_path.exists() {
            Self::load_from_file(&config_path)
        } else {
            Ok(Self::default())
        }
    }

    pub fn compile_builtin_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>> {
        use crate::rules::get_all_rule_metadata;

        let rule_config = self
            .builtin_rules
            .get(name)
            .ok_or_else(|| format!("Builtin rule '{}' not found in configuration", name))?;

        let metadata = get_all_rule_metadata().into_iter().find(|m| m.name == name);

        let default_severity = metadata
            .as_ref()
            .map(|m| m.default_severity)
            .unwrap_or(Severity::Warning);

        let options = if rule_config.options.is_empty() {
            None
        } else {
            Some(serde_json::to_value(&rule_config.options)?)
        };

        Ok(CompiledRuleConfig {
            enabled: rule_config.enabled.unwrap_or(true),
            severity: rule_config.severity.unwrap_or(default_severity),
            global_include: compile_globset(&self.files.include)?,
            global_exclude: compile_globset(&self.files.exclude)?,
            rule_include: compile_optional_globset(&rule_config.include)?,
            rule_exclude: compile_optional_globset(&rule_config.exclude)?,
            message: None,
            pattern: None,
            options,
        })
    }

    pub fn compile_custom_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>> {
        let rule_config = self
            .custom_rules
            .get(name)
            .ok_or_else(|| format!("Custom rule '{}' not found in configuration", name))?;

        Ok(CompiledRuleConfig {
            enabled: rule_config.enabled,
            severity: rule_config.severity,
            global_include: compile_globset(&self.files.include)?,
            global_exclude: compile_globset(&self.files.exclude)?,
            rule_include: compile_optional_globset(&rule_config.include)?,
            rule_exclude: compile_optional_globset(&rule_config.exclude)?,
            message: Some(rule_config.message.clone()),
            pattern: rule_config.pattern.clone(),
            options: None,
        })
    }

    pub fn matches_file(&self, path: &Path, rule_config: &CompiledRuleConfig) -> bool {
        if !rule_config.enabled {
            return false;
        }
        rule_config.matches(path)
    }

    pub fn matches_file_with_root(
        &self,
        path: &Path,
        root: &Path,
        rule_config: &CompiledRuleConfig,
    ) -> bool {
        if !rule_config.enabled {
            return false;
        }

        let relative_path = path.strip_prefix(root).unwrap_or(path);
        rule_config.matches(relative_path)
    }

    pub fn count_enabled_rules(&self) -> usize {
        use crate::rules::get_all_rule_metadata;

        let all_builtin_metadata = get_all_rule_metadata();

        let enabled_builtin = all_builtin_metadata
            .iter()
            .filter(|meta| match self.builtin_rules.get(meta.name) {
                Some(rule_config) => rule_config.enabled.unwrap_or(true),
                None => meta.default_enabled,
            })
            .count();

        let enabled_custom = self.custom_rules.values().filter(|r| r.enabled).count();

        enabled_builtin + enabled_custom
    }

    pub fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        for pattern in &self.files.include {
            pattern.hash(&mut hasher);
        }
        for pattern in &self.files.exclude {
            pattern.hash(&mut hasher);
        }

        let sorted_builtin: BTreeMap<_, _> = self.builtin_rules.iter().collect();
        for (name, config) in sorted_builtin {
            name.hash(&mut hasher);
            config.enabled.hash(&mut hasher);
            if let Some(sev) = &config.severity {
                format!("{:?}", sev).hash(&mut hasher);
            }
            for pattern in &config.include {
                pattern.hash(&mut hasher);
            }
            for pattern in &config.exclude {
                pattern.hash(&mut hasher);
            }
            if !config.options.is_empty() {
                if let Ok(json) = serde_json::to_string(&config.options) {
                    json.hash(&mut hasher);
                }
            }
        }

        let sorted_custom: BTreeMap<_, _> = self.custom_rules.iter().collect();
        for (name, config) in sorted_custom {
            name.hash(&mut hasher);
            config.enabled.hash(&mut hasher);
            if let Some(pattern) = &config.pattern {
                pattern.hash(&mut hasher);
            }
            for pattern in &config.include {
                pattern.hash(&mut hasher);
            }
            for pattern in &config.exclude {
                pattern.hash(&mut hasher);
            }
        }

        hasher.finish()
    }
}

impl Default for TscannerConfig {
    fn default() -> Self {
        serde_json::from_str(DEFAULT_CONFIG_JSON)
            .expect("Failed to parse embedded default-config.json")
    }
}

pub fn compile_globset(patterns: &[String]) -> Result<GlobSet, Box<dyn std::error::Error>> {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let glob = Glob::new(pattern)?;
        builder.add(glob);
    }

    Ok(builder.build()?)
}
