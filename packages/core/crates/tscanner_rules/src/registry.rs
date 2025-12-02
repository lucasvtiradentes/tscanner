use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tscanner_config::{CompiledRuleConfig, CustomRuleConfig, TscannerConfig};
use tscanner_diagnostics::Severity;

use crate::executors::RegexExecutor;
use crate::traits::{Rule, RuleRegistration};

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

    pub fn with_config<F, G>(
        config: &TscannerConfig,
        compile_builtin: F,
        compile_custom: G,
        log_info: fn(&str),
        log_error: fn(&str),
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        F: Fn(&TscannerConfig, &str) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>>,
        G: Fn(&TscannerConfig, &str) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>>,
    {
        let mut rules: HashMap<String, Arc<dyn Rule>> = HashMap::new();
        let mut compiled_configs: HashMap<String, CompiledRuleConfig> = HashMap::new();

        for rule_name in config.builtin_rules.keys() {
            if let Ok(compiled) = compile_builtin(config, rule_name) {
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
                match RegexExecutor::new(
                    rule_name.clone(),
                    regex_config.pattern.clone(),
                    regex_config.base.message.clone(),
                    regex_config.base.severity,
                ) {
                    Ok(regex_executor) => {
                        rules.insert(rule_name.clone(), Arc::new(regex_executor));
                    }
                    Err(e) => {
                        log_error(&format!(
                            "Failed to compile regex rule '{}': {}",
                            rule_name, e
                        ));
                        continue;
                    }
                }
            }

            if let Ok(compiled) = compile_custom(config, rule_name) {
                compiled_configs.insert(rule_name.clone(), compiled);
            }
        }

        let enabled_count = compiled_configs.values().filter(|c| c.enabled).count();
        log_info(&format!(
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

    pub fn get_enabled_rules<F>(
        &self,
        file_path: &Path,
        root: &Path,
        matches_file: F,
    ) -> Vec<(Arc<dyn Rule>, Severity)>
    where
        F: Fn(&Path, &Path, &CompiledRuleConfig) -> bool,
    {
        self.rules
            .iter()
            .filter_map(|(name, rule)| {
                if let Some(compiled) = self.compiled_configs.get(name) {
                    if compiled.enabled && matches_file(file_path, root, compiled) {
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
