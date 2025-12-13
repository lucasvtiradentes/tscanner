use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::Path;

use tscanner_config::{
    compile_globset, compile_optional_globset, CompiledRuleConfig, TscannerConfig,
    TscannerConfigExt,
};
use tscanner_constants::config_error_prefix;

const TSCANNER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn load_config(
    path: &Path,
    config_dir_name: &str,
    config_file_name: &str,
) -> Result<TscannerConfig, Box<dyn std::error::Error>> {
    let config_path = if path.is_file() {
        path.to_path_buf()
    } else {
        path.join(config_dir_name).join(config_file_name)
    };

    if !config_path.exists() {
        return Err(format!(
            "Config file not found: {}. Run 'tscanner init' to create one.",
            config_path.display()
        )
        .into());
    }

    let workspace = config_path.parent().and_then(|p| p.parent());
    let content = std::fs::read_to_string(&config_path)?;
    let (config, result) = TscannerConfig::full_validate(&content, workspace, config_dir_name)?;

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
                config_error_prefix(),
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

    config.ok_or_else(|| "Config parsing failed".into())
}

pub trait ConfigExt {
    fn load_from_file(
        path: &Path,
        config_dir_name: &str,
        config_file_name: &str,
    ) -> Result<TscannerConfig, Box<dyn std::error::Error>>;
    fn load_from_workspace(
        workspace: &Path,
        config_dir_name: &str,
        config_file_name: &str,
    ) -> Result<TscannerConfig, Box<dyn std::error::Error>>;
    fn compile_builtin_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>>;
    fn compile_regex_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>>;
    fn compile_script_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>>;
    fn compile_custom_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>>;
    fn count_enabled_rules(&self) -> usize;
    fn count_enabled_rules_breakdown(&self) -> (usize, usize, usize, usize);
    fn compute_hash(&self) -> u64;
}

impl ConfigExt for TscannerConfig {
    fn load_from_file(
        path: &Path,
        config_dir_name: &str,
        config_file_name: &str,
    ) -> Result<TscannerConfig, Box<dyn std::error::Error>> {
        load_config(path, config_dir_name, config_file_name)
    }

    fn load_from_workspace(
        workspace: &Path,
        config_dir_name: &str,
        config_file_name: &str,
    ) -> Result<TscannerConfig, Box<dyn std::error::Error>> {
        load_config(workspace, config_dir_name, config_file_name)
    }

    fn compile_builtin_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>> {
        let rule_config = self
            .rules
            .builtin
            .get(name)
            .ok_or_else(|| format!("Builtin rule '{}' not found in configuration", name))?;

        let options = if rule_config.options.is_empty() {
            None
        } else {
            Some(serde_json::to_value(&rule_config.options)?)
        };

        Ok(CompiledRuleConfig {
            severity: rule_config.severity,
            global_include: compile_globset(&self.files.include)?,
            global_exclude: compile_globset(&self.files.exclude)?,
            rule_include: compile_optional_globset(&rule_config.include)?,
            rule_exclude: compile_optional_globset(&rule_config.exclude)?,
            message: None,
            pattern: None,
            options,
        })
    }

    fn compile_regex_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>> {
        let rule_config = self
            .rules
            .regex
            .get(name)
            .ok_or_else(|| format!("Regex rule '{}' not found in configuration", name))?;

        Ok(CompiledRuleConfig {
            severity: rule_config.severity,
            global_include: compile_globset(&self.files.include)?,
            global_exclude: compile_globset(&self.files.exclude)?,
            rule_include: compile_optional_globset(&rule_config.include)?,
            rule_exclude: compile_optional_globset(&rule_config.exclude)?,
            message: Some(rule_config.message.clone()),
            pattern: Some(rule_config.pattern.clone()),
            options: None,
        })
    }

    fn compile_script_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>> {
        let rule_config = self
            .rules
            .script
            .get(name)
            .ok_or_else(|| format!("Script rule '{}' not found in configuration", name))?;

        Ok(CompiledRuleConfig {
            severity: rule_config.severity,
            global_include: compile_globset(&self.files.include)?,
            global_exclude: compile_globset(&self.files.exclude)?,
            rule_include: compile_optional_globset(&rule_config.include)?,
            rule_exclude: compile_optional_globset(&rule_config.exclude)?,
            message: Some(rule_config.message.clone()),
            pattern: None,
            options: None,
        })
    }

    fn compile_custom_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>> {
        if let Some(rule_config) = self.rules.regex.get(name) {
            return Ok(CompiledRuleConfig {
                severity: rule_config.severity,
                global_include: compile_globset(&self.files.include)?,
                global_exclude: compile_globset(&self.files.exclude)?,
                rule_include: compile_optional_globset(&rule_config.include)?,
                rule_exclude: compile_optional_globset(&rule_config.exclude)?,
                message: Some(rule_config.message.clone()),
                pattern: Some(rule_config.pattern.clone()),
                options: None,
            });
        }

        if let Some(rule_config) = self.rules.script.get(name) {
            return Ok(CompiledRuleConfig {
                severity: rule_config.severity,
                global_include: compile_globset(&self.files.include)?,
                global_exclude: compile_globset(&self.files.exclude)?,
                rule_include: compile_optional_globset(&rule_config.include)?,
                rule_exclude: compile_optional_globset(&rule_config.exclude)?,
                message: Some(rule_config.message.clone()),
                pattern: None,
                options: None,
            });
        }

        Err(format!("Custom rule '{}' not found in configuration", name).into())
    }

    fn count_enabled_rules(&self) -> usize {
        let (builtin, regex, script, ai) = self.count_enabled_rules_breakdown();
        builtin + regex + script + ai
    }

    fn count_enabled_rules_breakdown(&self) -> (usize, usize, usize, usize) {
        let enabled_builtin = self.rules.builtin.len();
        let enabled_regex = self.rules.regex.len();
        let enabled_script = self.rules.script.len();
        let enabled_ai = self.ai_rules.len();

        (enabled_builtin, enabled_regex, enabled_script, enabled_ai)
    }

    fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        for pattern in &self.files.include {
            pattern.hash(&mut hasher);
        }
        for pattern in &self.files.exclude {
            pattern.hash(&mut hasher);
        }

        let sorted_builtin: BTreeMap<_, _> = self.rules.builtin.iter().collect();
        for (name, config) in sorted_builtin {
            name.hash(&mut hasher);
            format!("{:?}", config.severity).hash(&mut hasher);
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

        let sorted_regex: BTreeMap<_, _> = self.rules.regex.iter().collect();
        for (name, config) in sorted_regex {
            name.hash(&mut hasher);
            config.pattern.hash(&mut hasher);
            for pattern in &config.include {
                pattern.hash(&mut hasher);
            }
            for pattern in &config.exclude {
                pattern.hash(&mut hasher);
            }
        }

        let sorted_script: BTreeMap<_, _> = self.rules.script.iter().collect();
        for (name, config) in sorted_script {
            name.hash(&mut hasher);
            config.command.hash(&mut hasher);
            format!("{:?}", config.severity).hash(&mut hasher);
            for pattern in &config.include {
                pattern.hash(&mut hasher);
            }
            for pattern in &config.exclude {
                pattern.hash(&mut hasher);
            }
            if !config.options.is_null() {
                if let Ok(json) = serde_json::to_string(&config.options) {
                    json.hash(&mut hasher);
                }
            }
        }

        let sorted_ai: BTreeMap<_, _> = self.ai_rules.iter().collect();
        for (name, config) in sorted_ai {
            name.hash(&mut hasher);
            config.prompt.hash(&mut hasher);
            format!("{:?}", config.mode).hash(&mut hasher);
            format!("{:?}", config.severity).hash(&mut hasher);
            for pattern in &config.include {
                pattern.hash(&mut hasher);
            }
            for pattern in &config.exclude {
                pattern.hash(&mut hasher);
            }
            if !config.options.is_null() {
                if let Ok(json) = serde_json::to_string(&config.options) {
                    json.hash(&mut hasher);
                }
            }
        }

        if let Some(ref ai_config) = self.ai {
            if let Some(provider) = ai_config.provider {
                format!("{:?}", provider).hash(&mut hasher);
            }
            if let Some(ref command) = ai_config.command {
                command.hash(&mut hasher);
            }
        }

        hasher.finish()
    }
}
