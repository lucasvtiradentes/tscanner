use crate::config::{CompiledRuleConfig, LinoConfig};
use crate::rules::{Rule, NoAnyTypeRule, NoConsoleLogRule, NoRelativeImportsRule, PreferTypeOverInterfaceRule};
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

        rules.insert("no-any-type".to_string(), Arc::new(NoAnyTypeRule));
        rules.insert("no-console-log".to_string(), Arc::new(NoConsoleLogRule));
        rules.insert("no-relative-imports".to_string(), Arc::new(NoRelativeImportsRule));
        rules.insert("prefer-type-over-interface".to_string(), Arc::new(PreferTypeOverInterfaceRule));

        Self {
            rules,
            compiled_configs: HashMap::new(),
        }
    }

    pub fn with_config(config: &LinoConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Self::new();

        for rule_name in registry.rules.keys() {
            if let Ok(compiled) = config.compile_rule(rule_name) {
                registry.compiled_configs.insert(rule_name.clone(), compiled);
            }
        }

        Ok(registry)
    }

    pub fn register_rule(&mut self, name: String, rule: Arc<dyn Rule>) {
        self.rules.insert(name, rule);
    }

    pub fn get_rule(&self, name: &str) -> Option<Arc<dyn Rule>> {
        self.rules.get(name).cloned()
    }

    pub fn get_enabled_rules(&self, file_path: &Path, config: &LinoConfig) -> Vec<(Arc<dyn Rule>, Severity)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = RuleRegistry::new();
        assert!(registry.rules.contains_key("no-any-type"));
    }

    #[test]
    fn test_registry_with_config() {
        let config = LinoConfig::default();
        let registry = RuleRegistry::with_config(&config).unwrap();
        assert!(registry.is_enabled("no-any-type"));
    }

    #[test]
    fn test_get_enabled_rules() {
        let config = LinoConfig::default();
        let registry = RuleRegistry::with_config(&config).unwrap();
        let rules = registry.get_enabled_rules(Path::new("src/test.ts"), &config);
        assert!(!rules.is_empty());
    }
}
