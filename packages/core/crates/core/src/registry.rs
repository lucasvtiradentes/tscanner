use crate::config::{CompiledRuleConfig, CustomRuleType, TscannerConfig};
use crate::rules::{RegexRule, Rule, RuleRegistration};
use crate::types::Severity;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

pub struct RuleRegistry {
    rules: HashMap<String, Arc<dyn Rule>>,
    compiled_configs: HashMap<String, CompiledRuleConfig>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        let mut rules: HashMap<String, Arc<dyn Rule>> = HashMap::new();

        for registration in inventory::iter::<RuleRegistration> {
            rules.insert(registration.name.to_string(), (registration.factory)());
        }

        Self {
            rules,
            compiled_configs: HashMap::new(),
        }
    }

    pub fn with_config(config: &TscannerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Self::new();

        for rule_name in config.builtin_rules.keys() {
            if let Ok(compiled) = config.compile_builtin_rule(rule_name) {
                registry
                    .compiled_configs
                    .insert(rule_name.clone(), compiled);
            }
        }

        for (rule_name, rule_config) in &config.custom_rules {
            if rule_config.rule_type == CustomRuleType::Regex {
                if let Some(pattern) = &rule_config.pattern {
                    match RegexRule::new(
                        rule_name.clone(),
                        pattern.clone(),
                        rule_config.message.clone(),
                        rule_config.severity,
                    ) {
                        Ok(regex_rule) => {
                            registry
                                .rules
                                .insert(rule_name.clone(), Arc::new(regex_rule));
                        }
                        Err(e) => {
                            crate::log_error(&format!(
                                "Failed to compile regex rule '{}': {}",
                                rule_name, e
                            ));
                            continue;
                        }
                    }
                }
            }

            if let Ok(compiled) = config.compile_custom_rule(rule_name) {
                registry
                    .compiled_configs
                    .insert(rule_name.clone(), compiled);
            }
        }

        let enabled_count = registry
            .compiled_configs
            .values()
            .filter(|c| c.enabled)
            .count();
        crate::log_info(&format!(
            "Loaded {} rules ({} enabled)",
            registry.rules.len(),
            enabled_count
        ));

        Ok(registry)
    }

    pub fn register_rule(&mut self, name: String, rule: Arc<dyn Rule>) {
        self.rules.insert(name, rule);
    }

    pub fn get_rule(&self, name: &str) -> Option<Arc<dyn Rule>> {
        self.rules.get(name).cloned()
    }

    pub fn get_enabled_rules(
        &self,
        file_path: &Path,
        config: &TscannerConfig,
    ) -> Vec<(Arc<dyn Rule>, Severity)> {
        self.rules
            .iter()
            .filter_map(|(name, rule)| {
                if let Some(compiled) = self.compiled_configs.get(name) {
                    if compiled.enabled && config.matches_file(file_path, compiled) {
                        return Some((rule.clone(), compiled.severity));
                    }
                }
                None
            })
            .collect()
    }

    pub fn list_rules(&self) -> Vec<String> {
        self.rules.keys().cloned().collect()
    }

    pub fn is_enabled(&self, name: &str) -> bool {
        self.compiled_configs
            .get(name)
            .map(|c| c.enabled)
            .unwrap_or(false)
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
