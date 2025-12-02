use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::Path;

use crate::output::Severity;

use super::defaults::get_default_config_json;
use super::globset::{compile_globset, compile_optional_globset};
use super::types::{CompiledRuleConfig, CustomRuleConfig, TscannerConfig};
use super::validation::{validate_json_fields, ValidationResult};

const TSCANNER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CONFIG_ERROR_PREFIX: &str = "TSCANNER_CONFIG_ERROR:";

impl TscannerConfig {
    pub fn parse_json(content: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let json_without_comments = json_comments::StripComments::new(content.as_bytes());
        let json_value: serde_json::Value = serde_json::from_reader(json_without_comments)?;
        Ok(json_value)
    }

    pub fn full_validate(
        content: &str,
        workspace: Option<&Path>,
    ) -> Result<(Self, ValidationResult), Box<dyn std::error::Error>> {
        let json_value = Self::parse_json(content)?;

        let mut result = validate_json_fields(&json_value);
        if !result.is_valid() {
            return Ok((Self::default(), result));
        }

        let config: Self = serde_json::from_value(json_value)?;
        result.merge(config.validate_with_workspace(workspace));

        Ok((config, result))
    }

    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let workspace = path.parent().and_then(|p| p.parent());
        let content = std::fs::read_to_string(path)?;
        let (config, result) = Self::full_validate(&content, workspace)?;

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
        self.validate_with_workspace(None)
    }

    pub fn validate_with_workspace(&self, workspace: Option<&Path>) -> ValidationResult {
        use crate::constants::config_dir_name;

        let mut result = ValidationResult::new();

        for (name, rule_config) in &self.custom_rules {
            if let CustomRuleConfig::Regex(regex_config) = rule_config {
                if let Err(e) = regex::Regex::new(&regex_config.pattern) {
                    result.add_error(format!("Rule '{}' has invalid regex pattern: {}", name, e));
                }
            }

            if let CustomRuleConfig::Script(script_config) = rule_config {
                if let Some(ws) = workspace {
                    let script_path = ws
                        .join(config_dir_name())
                        .join("scripts")
                        .join(&script_config.script);
                    if !script_path.exists() {
                        result.add_error(format!(
                            "Rule '{}' references non-existent script: {}",
                            name, script_config.script
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
            enabled: rule_config.enabled(),
            severity: rule_config.severity(),
            global_include: compile_globset(&self.files.include)?,
            global_exclude: compile_globset(&self.files.exclude)?,
            rule_include: compile_optional_globset(rule_config.include())?,
            rule_exclude: compile_optional_globset(rule_config.exclude())?,
            message: Some(rule_config.message().to_string()),
            pattern: rule_config.pattern().map(|s| s.to_string()),
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

        let enabled_custom = self.custom_rules.values().filter(|r| r.enabled()).count();

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
            config.enabled().hash(&mut hasher);
            if let Some(pattern) = config.pattern() {
                pattern.hash(&mut hasher);
            }
            for pattern in config.include() {
                pattern.hash(&mut hasher);
            }
            for pattern in config.exclude() {
                pattern.hash(&mut hasher);
            }
        }

        hasher.finish()
    }
}

impl Default for TscannerConfig {
    fn default() -> Self {
        serde_json::from_str(get_default_config_json())
            .expect("Failed to parse embedded default-config.json")
    }
}
