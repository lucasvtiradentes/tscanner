use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::Path;

use tscanner_config::{
    compile_globset, compile_optional_globset, CompiledRuleConfig, TscannerConfig,
    CONFIG_ERROR_PREFIX,
};
use tscanner_diagnostics::Severity;
use tscanner_rules::{get_all_rule_metadata, get_allowed_options_for_rule};

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
        return Ok(TscannerConfig::default());
    }

    let workspace = config_path.parent().and_then(|p| p.parent());
    let content = std::fs::read_to_string(&config_path)?;
    let (config, result) = TscannerConfig::full_validate(
        &content,
        workspace,
        config_dir_name,
        Some(get_allowed_options_for_rule),
    )?;

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
    fn compile_custom_rule(
        &self,
        name: &str,
    ) -> Result<CompiledRuleConfig, Box<dyn std::error::Error>>;
    fn count_enabled_rules(&self) -> usize;
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

    fn compile_custom_rule(
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

    fn count_enabled_rules(&self) -> usize {
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

    fn compute_hash(&self) -> u64 {
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
