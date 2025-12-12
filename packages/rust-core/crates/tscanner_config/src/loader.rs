use std::path::Path;

use crate::types::{AiProvider, TscannerConfig};
use crate::validation::{validate_json_fields, ValidationResult};
use tscanner_constants::{config_dir_name, config_error_prefix};

pub fn get_config_error_prefix() -> &'static str {
    config_error_prefix()
}

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
    ) -> Result<(Option<Self>, ValidationResult), Box<dyn std::error::Error>> {
        let json_value = Self::parse_json(content)?;

        let mut result = validate_json_fields(&json_value);
        if !result.is_valid() {
            return Ok((None, result));
        }

        let config: Self = match serde_json::from_value(json_value) {
            Ok(c) => c,
            Err(e) => {
                result.add_error(format!("Failed to parse config: {}", e));
                return Ok((None, result));
            }
        };
        result.merge(config.validate_with_workspace(workspace, config_dir_name));

        Ok((Some(config), result))
    }

    pub fn validate(&self) -> ValidationResult {
        self.validate_with_workspace(None, config_dir_name())
    }

    pub fn validate_with_workspace(
        &self,
        _workspace: Option<&Path>,
        _config_dir_name: &str,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();

        if let Some(ref ai_config) = self.ai {
            if ai_config.provider == Some(AiProvider::Custom)
                && (ai_config.command.is_none()
                    || ai_config.command.as_ref().map(|c| c.trim().is_empty()) == Some(true))
            {
                result.add_error("ai.command is required when ai.provider is 'custom'".to_string());
            }
        }

        for (name, regex_config) in &self.rules.regex {
            if let Err(e) = regex::Regex::new(&regex_config.pattern) {
                result.add_error(format!("Rule '{}' has invalid regex pattern: {}", name, e));
            }
        }

        for (name, script_config) in &self.rules.script {
            if script_config.command.trim().is_empty() {
                result.add_error(format!("Rule '{}' has empty command", name));
            }
        }

        for (name, ai_config) in &self.ai_rules {
            if ai_config.prompt.trim().is_empty() {
                result.add_error(format!("AI rule '{}' has empty prompt", name));
            }
        }

        result
    }

    pub fn matches_file(
        &self,
        path: &Path,
        rule_config: &crate::types::CompiledRuleConfig,
    ) -> bool {
        rule_config.matches(path)
    }

    pub fn matches_file_with_root(
        &self,
        path: &Path,
        root: &Path,
        rule_config: &crate::types::CompiledRuleConfig,
    ) -> bool {
        let relative_path = path.strip_prefix(root).unwrap_or(path);
        rule_config.matches(relative_path)
    }

    pub fn get_rule_specific_include_patterns(&self) -> Vec<String> {
        let builtin_patterns = self
            .rules
            .builtin
            .values()
            .flat_map(|rule| rule.include.clone());

        let regex_patterns = self
            .rules
            .regex
            .values()
            .flat_map(|rule| rule.include.clone());

        let script_patterns = self
            .rules
            .script
            .values()
            .flat_map(|rule| rule.include.clone());

        let ai_patterns = self.ai_rules.values().flat_map(|rule| rule.include.clone());

        builtin_patterns
            .chain(regex_patterns)
            .chain(script_patterns)
            .chain(ai_patterns)
            .collect()
    }
}
