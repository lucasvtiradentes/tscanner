use crate::config::{CompiledRuleConfig, CustomRuleConfig, TscannerConfig};
use crate::output::Severity;
use crate::rule::{Rule, RuleRegistration};
use crate::rules::RegexRule;
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
            rules.insert(registration.name.to_string(), (registration.factory)(None));
        }

        Self {
            rules,
            compiled_configs: HashMap::new(),
        }
    }

    pub fn with_config(config: &TscannerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rules: HashMap<String, Arc<dyn Rule>> = HashMap::new();
        let mut compiled_configs: HashMap<String, CompiledRuleConfig> = HashMap::new();

        for rule_name in config.builtin_rules.keys() {
            if let Ok(compiled) = config.compile_builtin_rule(rule_name) {
                compiled_configs.insert(rule_name.clone(), compiled);
            }
        }

        for registration in inventory::iter::<RuleRegistration> {
            let options = compiled_configs
                .get(registration.name)
                .and_then(|c| c.options.as_ref());
            rules.insert(
                registration.name.to_string(),
                (registration.factory)(options),
            );
        }

        for (rule_name, rule_config) in &config.custom_rules {
            if let CustomRuleConfig::Regex(regex_config) = rule_config {
                match RegexRule::new(
                    rule_name.clone(),
                    regex_config.pattern.clone(),
                    regex_config.base.message.clone(),
                    regex_config.base.severity,
                ) {
                    Ok(regex_rule) => {
                        rules.insert(rule_name.clone(), Arc::new(regex_rule));
                    }
                    Err(e) => {
                        crate::utils::log_error(&format!(
                            "Failed to compile regex rule '{}': {}",
                            rule_name, e
                        ));
                        continue;
                    }
                }
            }

            if let Ok(compiled) = config.compile_custom_rule(rule_name) {
                compiled_configs.insert(rule_name.clone(), compiled);
            }
        }

        let enabled_count = compiled_configs.values().filter(|c| c.enabled).count();
        crate::utils::log_info(&format!(
            "Loaded {} rules ({} enabled)",
            rules.len(),
            enabled_count
        ));

        Ok(Self {
            rules,
            compiled_configs,
        })
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
        root: &Path,
        config: &TscannerConfig,
    ) -> Vec<(Arc<dyn Rule>, Severity)> {
        self.rules
            .iter()
            .filter_map(|(name, rule)| {
                if let Some(compiled) = self.compiled_configs.get(name) {
                    if compiled.enabled && config.matches_file_with_root(file_path, root, compiled)
                    {
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
