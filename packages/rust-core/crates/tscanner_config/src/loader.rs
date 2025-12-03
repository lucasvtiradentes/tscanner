use std::path::Path;

use crate::defaults::get_default_config_json;
use crate::types::{CustomRuleConfig, TscannerConfig};
use crate::validation::{validate_json_fields, AllowedOptionsGetter, ValidationResult};

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
        config_dir_name: &str,
        get_allowed_options: Option<AllowedOptionsGetter>,
    ) -> Result<(Self, ValidationResult), Box<dyn std::error::Error>> {
        let json_value = Self::parse_json(content)?;

        let mut result = validate_json_fields(&json_value, get_allowed_options);
        if !result.is_valid() {
            return Ok((Self::default(), result));
        }

        let config: Self = serde_json::from_value(json_value)?;
        result.merge(config.validate_with_workspace(workspace, config_dir_name));

        Ok((config, result))
    }

    pub fn validate(&self) -> ValidationResult {
        self.validate_with_workspace(None, ".tscanner")
    }

    pub fn validate_with_workspace(
        &self,
        _workspace: Option<&Path>,
        _config_dir_name: &str,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();

        for (name, rule_config) in &self.custom_rules {
            if let CustomRuleConfig::Regex(regex_config) = rule_config {
                if let Err(e) = regex::Regex::new(&regex_config.pattern) {
                    result.add_error(format!("Rule '{}' has invalid regex pattern: {}", name, e));
                }
            }

            if let CustomRuleConfig::Script(script_config) = rule_config {
                if script_config.command.trim().is_empty() {
                    result.add_error(format!("Rule '{}' has empty command", name));
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

    pub fn matches_file(
        &self,
        path: &Path,
        rule_config: &crate::types::CompiledRuleConfig,
    ) -> bool {
        if !rule_config.enabled {
            return false;
        }
        rule_config.matches(path)
    }

    pub fn matches_file_with_root(
        &self,
        path: &Path,
        root: &Path,
        rule_config: &crate::types::CompiledRuleConfig,
    ) -> bool {
        if !rule_config.enabled {
            return false;
        }

        let relative_path = path.strip_prefix(root).unwrap_or(path);
        rule_config.matches(relative_path)
    }

    pub fn get_rule_specific_include_patterns(&self) -> Vec<String> {
        let builtin_patterns = self
            .builtin_rules
            .values()
            .filter(|rule| rule.enabled.unwrap_or(true))
            .flat_map(|rule| rule.include.clone());

        let custom_patterns = self
            .custom_rules
            .values()
            .filter(|rule| rule.base().enabled)
            .flat_map(|rule| rule.base().include.clone());

        builtin_patterns.chain(custom_patterns).collect()
    }
}

impl Default for TscannerConfig {
    fn default() -> Self {
        serde_json::from_str(get_default_config_json())
            .expect("Failed to parse embedded default-config.json")
    }
}
