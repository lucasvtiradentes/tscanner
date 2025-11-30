use crate::types::Severity;
use globset::{Glob, GlobSet, GlobSetBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::Path;

const TSCANNER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CONFIG_ERROR_PREFIX: &str = "TSCANNER_CONFIG_ERROR:";

const ALLOWED_TOP_LEVEL: &[&str] = &[
    "$schema",
    "lsp",
    "cli",
    "builtinRules",
    "customRules",
    "files",
];
const ALLOWED_CLI: &[&str] = &[
    "groupBy",
    "noCache",
    "showSettings",
    "showSeverity",
    "showSourceLine",
    "showRuleName",
    "showDescription",
    "showSummary",
];
const ALLOWED_FILES: &[&str] = &["include", "exclude"];
const ALLOWED_LSP: &[&str] = &["errors", "warnings"];
const ALLOWED_BUILTIN_RULE: &[&str] = &["enabled", "severity", "include", "exclude"];
const ALLOWED_CUSTOM_RULE: &[&str] = &[
    "type", "pattern", "script", "prompt", "message", "severity", "enabled", "include", "exclude",
];

fn collect_invalid_fields(
    obj: &serde_json::Map<String, serde_json::Value>,
    allowed: &[&str],
    prefix: &str,
) -> Vec<String> {
    let allowed_set: HashSet<&str> = allowed.iter().copied().collect();
    obj.keys()
        .filter(|key| !allowed_set.contains(key.as_str()))
        .map(|key| {
            if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            }
        })
        .collect()
}

fn validate_nested_rules(
    rules: &serde_json::Map<String, serde_json::Value>,
    allowed: &[&str],
    section_name: &str,
) -> Vec<String> {
    let mut invalid = Vec::new();
    for (rule_name, rule_config) in rules {
        if let Some(rule_obj) = rule_config.as_object() {
            let prefix = format!("{}.{}", section_name, rule_name);
            invalid.extend(collect_invalid_fields(rule_obj, allowed, &prefix));
        }
    }
    invalid
}

fn validate_json_fields(json: &serde_json::Value) -> Result<(), String> {
    let Some(obj) = json.as_object() else {
        return Ok(());
    };

    let mut invalid_fields = collect_invalid_fields(obj, ALLOWED_TOP_LEVEL, "");

    if let Some(files) = obj.get("files").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(files, ALLOWED_FILES, "files"));
    }

    if let Some(lsp) = obj.get("lsp").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(lsp, ALLOWED_LSP, "lsp"));
    }

    if let Some(cli) = obj.get("cli").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(cli, ALLOWED_CLI, "cli"));
    }

    if let Some(builtin_rules) = obj.get("builtinRules").and_then(|v| v.as_object()) {
        invalid_fields.extend(validate_nested_rules(
            builtin_rules,
            ALLOWED_BUILTIN_RULE,
            "builtinRules",
        ));
    }

    if let Some(custom_rules) = obj.get("customRules").and_then(|v| v.as_object()) {
        invalid_fields.extend(validate_nested_rules(
            custom_rules,
            ALLOWED_CUSTOM_RULE,
            "customRules",
        ));
    }

    if invalid_fields.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{}invalid_fields=[{}];version={}",
            CONFIG_ERROR_PREFIX,
            invalid_fields.join(","),
            TSCANNER_VERSION
        ))
    }
}

const DEFAULT_CONFIG_JSON: &str = include_str!("../../../../../assets/default-config.json");

fn default_lsp_config() -> LspConfig {
    let config: TscannerConfig =
        serde_json::from_str(DEFAULT_CONFIG_JSON).expect("Failed to parse default-config.json");
    config
        .lsp
        .expect("default-config.json must have 'lsp' section")
}

fn default_lsp_errors() -> bool {
    default_lsp_config().errors
}

fn default_lsp_warnings() -> bool {
    default_lsp_config().warnings
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LspConfig {
    #[serde(default = "default_lsp_errors")]
    #[schemars(
        default = "default_lsp_errors",
        description = "Show error diagnostics in LSP"
    )]
    pub errors: bool,

    #[serde(default = "default_lsp_warnings")]
    #[schemars(
        default = "default_lsp_warnings",
        description = "Show warning diagnostics in LSP"
    )]
    pub warnings: bool,
}

impl Default for LspConfig {
    fn default() -> Self {
        default_lsp_config()
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

fn default_cli_show_severity() -> bool {
    default_cli_config().show_severity
}

fn default_cli_show_source_line() -> bool {
    default_cli_config().show_source_line
}

fn default_cli_show_rule_name() -> bool {
    default_cli_config().show_rule_name
}

fn default_cli_show_settings() -> bool {
    default_cli_config().show_settings
}

fn default_cli_show_description() -> bool {
    default_cli_config().show_description
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
        description = "Show check settings header"
    )]
    pub show_settings: bool,

    #[serde(default = "default_cli_show_severity")]
    #[schemars(
        default = "default_cli_show_severity",
        description = "Show severity icon"
    )]
    pub show_severity: bool,

    #[serde(default = "default_cli_show_source_line")]
    #[schemars(
        default = "default_cli_show_source_line",
        description = "Show source line text"
    )]
    pub show_source_line: bool,

    #[serde(default = "default_cli_show_rule_name")]
    #[schemars(default = "default_cli_show_rule_name", description = "Show rule name")]
    pub show_rule_name: bool,

    #[serde(default = "default_cli_show_description")]
    #[schemars(
        default = "default_cli_show_description",
        description = "Show rule description/message"
    )]
    pub show_description: bool,

    #[serde(default = "default_cli_show_summary")]
    #[schemars(
        default = "default_cli_show_summary",
        description = "Show summary footer"
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
    #[schemars(description = "LSP server configuration")]
    pub lsp: Option<LspConfig>,

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
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;

        let json_without_comments = json_comments::StripComments::new(content.as_bytes());
        let json_value: serde_json::Value = serde_json::from_reader(json_without_comments)?;

        validate_json_fields(&json_value)?;

        let config: Self = serde_json::from_value(json_value)?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for (name, rule_config) in &self.custom_rules {
            match rule_config.rule_type {
                CustomRuleType::Regex => {
                    if rule_config.pattern.is_none() {
                        errors.push(format!(
                            "Custom rule '{}' is type 'regex' but has no 'pattern' field",
                            name
                        ));
                    } else if let Some(pattern) = &rule_config.pattern {
                        if let Err(e) = regex::Regex::new(pattern) {
                            errors
                                .push(format!("Rule '{}' has invalid regex pattern: {}", name, e));
                        }
                    }
                }
                CustomRuleType::Script => {
                    if rule_config.script.is_none() {
                        errors.push(format!(
                            "Custom rule '{}' is type 'script' but has no 'script' field",
                            name
                        ));
                    }
                }
                CustomRuleType::Ai => {
                    if rule_config.prompt.is_none() {
                        errors.push(format!(
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
                warnings.push(format!(
                    "Warning: Conflicting rules enabled: '{}' and '{}'. Both rules will run but contradict each other.",
                    rule1, rule2
                ));
            }
        }

        if !warnings.is_empty() {
            eprintln!("{}", warnings.join("\n"));
        }

        if !errors.is_empty() {
            return Err(format!("Config validation failed:\n  - {}", errors.join("\n  - ")).into());
        }

        Ok(())
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

        Ok(CompiledRuleConfig {
            enabled: rule_config.enabled.unwrap_or(true),
            severity: rule_config.severity.unwrap_or(default_severity),
            global_include: compile_globset(&self.files.include)?,
            global_exclude: compile_globset(&self.files.exclude)?,
            rule_include: compile_optional_globset(&rule_config.include)?,
            rule_exclude: compile_optional_globset(&rule_config.exclude)?,
            message: None,
            pattern: None,
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

fn compile_globset(patterns: &[String]) -> Result<GlobSet, Box<dyn std::error::Error>> {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let glob = Glob::new(pattern)?;
        builder.add(glob);
    }

    Ok(builder.build()?)
}
