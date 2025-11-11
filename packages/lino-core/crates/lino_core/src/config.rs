use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::types::Severity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinoConfig {
    #[serde(default)]
    pub rules: HashMap<String, RuleConfig>,
    #[serde(default = "default_include")]
    pub include: Vec<String>,
    #[serde(default = "default_exclude")]
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(rename = "type")]
    pub rule_type: RuleType,
    #[serde(default = "default_severity")]
    pub severity: Severity,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default, flatten)]
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    Ast,
    Regex,
}

pub struct CompiledRuleConfig {
    pub enabled: bool,
    pub rule_type: RuleType,
    pub severity: Severity,
    pub include: GlobSet,
    pub exclude: GlobSet,
    pub message: Option<String>,
    pub pattern: Option<String>,
    pub options: HashMap<String, serde_json::Value>,
}

fn default_enabled() -> bool {
    true
}

fn default_severity() -> Severity {
    Severity::Error
}

fn default_include() -> Vec<String> {
    vec!["**/*.{ts,tsx}".to_string()]
}

fn default_exclude() -> Vec<String> {
    vec![
        "node_modules/**".to_string(),
        "dist/**".to_string(),
        "build/**".to_string(),
        ".git/**".to_string(),
    ]
}

impl LinoConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn load_from_workspace(workspace: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = workspace.join(".lino/rules.json");
        if config_path.exists() {
            Self::load_from_file(&config_path)
        } else {
            Ok(Self::default())
        }
    }

    pub fn compile_rule(&self, name: &str) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>> {
        let rule_config = self.rules.get(name)
            .ok_or_else(|| format!("Rule '{}' not found in configuration", name))?;

        let include = compile_globs(&rule_config.include, &self.include)?;
        let exclude = compile_globs(&rule_config.exclude, &self.exclude)?;

        Ok(CompiledRuleConfig {
            enabled: rule_config.enabled,
            rule_type: rule_config.rule_type,
            severity: rule_config.severity,
            include,
            exclude,
            message: rule_config.message.clone(),
            pattern: rule_config.pattern.clone(),
            options: rule_config.options.clone(),
        })
    }

    pub fn matches_file(&self, path: &Path, rule_config: &CompiledRuleConfig) -> bool {
        if !rule_config.enabled {
            return false;
        }
        rule_config.include.is_match(path) && !rule_config.exclude.is_match(path)
    }
}

impl Default for LinoConfig {
    fn default() -> Self {
        let mut rules = HashMap::new();

        rules.insert("no-any-type".to_string(), RuleConfig {
            enabled: true,
            rule_type: RuleType::Ast,
            severity: Severity::Error,
            include: vec![],
            exclude: vec![],
            message: Some("Found 'any' type annotation".to_string()),
            pattern: None,
            options: HashMap::new(),
        });

        Self {
            rules,
            include: default_include(),
            exclude: default_exclude(),
        }
    }
}

fn compile_globs(
    rule_patterns: &[String],
    global_patterns: &[String],
) -> Result<GlobSet, Box<dyn std::error::Error>> {
    let mut builder = GlobSetBuilder::new();

    let patterns = if rule_patterns.is_empty() {
        global_patterns
    } else {
        rule_patterns
    };

    for pattern in patterns {
        let glob = Glob::new(pattern)?;
        builder.add(glob);
    }

    Ok(builder.build()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LinoConfig::default();
        assert!(config.rules.contains_key("no-any-type"));
        assert_eq!(config.include.len(), 1);
        assert!(config.exclude.len() > 0);
    }

    #[test]
    fn test_glob_matching() {
        let config = LinoConfig::default();
        let compiled = config.compile_rule("no-any-type").unwrap();

        assert!(config.matches_file(Path::new("src/test.ts"), &compiled));
        assert!(config.matches_file(Path::new("src/test.tsx"), &compiled));
        assert!(!config.matches_file(Path::new("node_modules/test.ts"), &compiled));
    }
}
