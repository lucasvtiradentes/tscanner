use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;
use tscanner_config::{CompiledRuleConfig, TscannerConfig};
use tscanner_types::Severity;

use crate::executors::RegexExecutor;
use crate::metadata::{get_all_rule_metadata, RuleCategory};
use crate::traits::{DynRule, RuleRegistration};

fn category_to_folder(category: RuleCategory) -> &'static str {
    match category {
        RuleCategory::TypeSafety => "type_safety",
        RuleCategory::CodeQuality => "code_quality",
        RuleCategory::Style => "style",
        RuleCategory::Performance => "performance",
        RuleCategory::BugPrevention => "bug_prevention",
        RuleCategory::Variables => "variables",
        RuleCategory::Imports => "imports",
    }
}

pub struct RuleRegistry {
    rules: HashMap<String, Arc<dyn DynRule>>,
    compiled_configs: HashMap<String, CompiledRuleConfig>,
    rule_categories: HashMap<String, String>,
    custom_regex_rules: HashSet<String>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        let mut rules: HashMap<String, Arc<dyn DynRule>> = HashMap::new();
        let mut rule_categories: HashMap<String, String> = HashMap::new();

        for registration in inventory::iter::<RuleRegistration> {
            rules.insert(registration.name.to_string(), (registration.factory)(None));
        }

        for metadata in get_all_rule_metadata() {
            rule_categories.insert(
                metadata.name.to_string(),
                category_to_folder(metadata.category).to_string(),
            );
        }

        Self {
            rules,
            compiled_configs: HashMap::new(),
            rule_categories,
            custom_regex_rules: HashSet::new(),
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
        let mut rules: HashMap<String, Arc<dyn DynRule>> = HashMap::new();
        let mut compiled_configs: HashMap<String, CompiledRuleConfig> = HashMap::new();
        let mut rule_categories: HashMap<String, String> = HashMap::new();
        let mut custom_regex_rules: HashSet<String> = HashSet::new();

        for metadata in get_all_rule_metadata() {
            rule_categories.insert(
                metadata.name.to_string(),
                category_to_folder(metadata.category).to_string(),
            );
        }

        for rule_name in config.rules.builtin.keys() {
            if let Ok(compiled) = compile_builtin(config, rule_name) {
                compiled_configs.insert(rule_name.to_string(), compiled);
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

        for (rule_name, regex_config) in &config.rules.regex {
            match RegexExecutor::new(
                rule_name.clone(),
                regex_config.pattern.clone(),
                regex_config.message.clone(),
                regex_config.severity,
            ) {
                Ok(regex_executor) => {
                    rules.insert(rule_name.clone(), Arc::new(regex_executor));
                    custom_regex_rules.insert(rule_name.clone());
                }
                Err(e) => {
                    log_error(&format!(
                        "Failed to compile regex rule '{}': {}",
                        rule_name, e
                    ));
                    continue;
                }
            }

            if let Ok(compiled) = compile_custom(config, rule_name) {
                compiled_configs.insert(rule_name.clone(), compiled);
            }
        }

        for rule_name in config.rules.script.keys() {
            if let Ok(compiled) = compile_custom(config, rule_name) {
                compiled_configs.insert(rule_name.clone(), compiled);
            }
        }

        log_info(&format!(
            "Loaded {} rules ({} configured)",
            rules.len(),
            compiled_configs.len()
        ));

        Ok(Self {
            rules,
            compiled_configs,
            rule_categories,
            custom_regex_rules,
        })
    }

    pub fn is_custom_regex_rule(&self, name: &str) -> bool {
        self.custom_regex_rules.contains(name)
    }

    pub fn register_rule(&mut self, name: String, rule: Arc<dyn DynRule>) {
        self.rules.insert(name, rule);
    }

    pub fn get_rule(&self, name: &str) -> Option<Arc<dyn DynRule>> {
        self.rules.get(name).cloned()
    }

    pub fn get_rule_category(&self, name: &str) -> Option<&str> {
        self.rule_categories.get(name).map(|s| s.as_str())
    }

    pub fn get_enabled_rules<F>(
        &self,
        file_path: &Path,
        root: &Path,
        matches_file: F,
    ) -> Vec<(Arc<dyn DynRule>, Severity)>
    where
        F: Fn(&Path, &Path, &CompiledRuleConfig) -> bool,
    {
        self.rules
            .iter()
            .filter_map(|(name, rule)| {
                if let Some(compiled) = self.compiled_configs.get(name) {
                    if matches_file(file_path, root, compiled) {
                        return Some((rule.clone(), compiled.severity));
                    }
                }
                None
            })
            .collect()
    }

    pub fn get_enabled_regex_rules<F>(
        &self,
        file_path: &Path,
        root: &Path,
        matches_file: F,
    ) -> Vec<(Arc<dyn DynRule>, Severity)>
    where
        F: Fn(&Path, &Path, &CompiledRuleConfig) -> bool,
    {
        self.rules
            .iter()
            .filter_map(|(name, rule)| {
                if !rule.is_regex_only() {
                    return None;
                }
                if let Some(compiled) = self.compiled_configs.get(name) {
                    if matches_file(file_path, root, compiled) {
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
        self.compiled_configs.contains_key(name)
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
