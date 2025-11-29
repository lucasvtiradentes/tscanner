use crate::types::Severity;
use globset::{Glob, GlobSet, GlobSetBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LspConfig {
    #[serde(default = "default_true")]
    #[schemars(description = "Show error diagnostics in LSP (default: true)")]
    pub errors: bool,

    #[serde(default = "default_true")]
    #[schemars(description = "Show warning diagnostics in LSP (default: true)")]
    pub warnings: bool,
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            errors: true,
            warnings: true,
        }
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

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[schemars(description = "Built-in AST rules configuration")]
    pub builtin_rules: HashMap<String, BuiltinRuleConfig>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[schemars(description = "Custom rules configuration (regex, script, or AI)")]
    pub custom_rules: HashMap<String, CustomRuleConfig>,

    #[serde(default = "default_include")]
    pub include: Vec<String>,

    #[serde(default = "default_exclude")]
    pub exclude: Vec<String>,
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

fn default_include() -> Vec<String> {
    vec!["**/*.{ts,tsx,js,jsx,mjs,cjs}".to_string()]
}

fn default_exclude() -> Vec<String> {
    vec![
        "node_modules/**".to_string(),
        "dist/**".to_string(),
        "build/**".to_string(),
        ".git/**".to_string(),
    ]
}

impl TscannerConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let json_without_comments = json_comments::StripComments::new(content.as_bytes());
        let config: Self = serde_json::from_reader(json_without_comments)?;
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
        use crate::constants::{CONFIG_DIR_NAME, CONFIG_FILE_NAME};

        let config_path = workspace.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME);

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

        let global_include = compile_globset(&self.include)?;
        let global_exclude = compile_globset(&self.exclude)?;
        let rule_include = if rule_config.include.is_empty() {
            None
        } else {
            Some(compile_globset(&rule_config.include)?)
        };
        let rule_exclude = if rule_config.exclude.is_empty() {
            None
        } else {
            Some(compile_globset(&rule_config.exclude)?)
        };

        Ok(CompiledRuleConfig {
            enabled: rule_config.enabled.unwrap_or(true),
            severity: rule_config.severity.unwrap_or(default_severity),
            global_include,
            global_exclude,
            rule_include,
            rule_exclude,
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

        let global_include = compile_globset(&self.include)?;
        let global_exclude = compile_globset(&self.exclude)?;
        let rule_include = if rule_config.include.is_empty() {
            None
        } else {
            Some(compile_globset(&rule_config.include)?)
        };
        let rule_exclude = if rule_config.exclude.is_empty() {
            None
        } else {
            Some(compile_globset(&rule_config.exclude)?)
        };

        Ok(CompiledRuleConfig {
            enabled: rule_config.enabled,
            severity: rule_config.severity,
            global_include,
            global_exclude,
            rule_include,
            rule_exclude,
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

    pub fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        for pattern in &self.include {
            pattern.hash(&mut hasher);
        }
        for pattern in &self.exclude {
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
        const DEFAULT_CONFIG_JSON: &str = include_str!("../../../../../assets/default-config.json");

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
