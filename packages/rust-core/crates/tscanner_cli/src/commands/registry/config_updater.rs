use anyhow::{Context, Result};
use serde_json::{json, Map, Value};
use std::fs;
use std::path::Path;

use tscanner_constants::{config_dir_name, config_file_name};

use super::fetcher::{RegistryRule, RuleConfig};

pub fn update_config_with_rule(
    workspace_root: &Path,
    rule: &RegistryRule,
    config: &RuleConfig,
    force: bool,
) -> Result<()> {
    let config_path = workspace_root
        .join(config_dir_name())
        .join(config_file_name());

    if !config_path.exists() {
        create_minimal_config(&config_path)?;
    }

    let content = fs::read_to_string(&config_path)
        .context(format!("Failed to read config: {}", config_path.display()))?;

    let stripped = json_comments::StripComments::new(content.as_bytes());
    let mut json: Value =
        serde_json::from_reader(stripped).context("Failed to parse config JSON")?;

    add_rule_to_config(&mut json, rule, config, force)?;

    let output = serde_json::to_string_pretty(&json).context("Failed to serialize config")?;
    fs::write(&config_path, output).context("Failed to write config")?;

    Ok(())
}

fn create_minimal_config(config_path: &Path) -> Result<()> {
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).context("Failed to create config directory")?;
    }

    let minimal = json!({
        "$schema": "https://unpkg.com/tscanner@latest/schema.json",
        "rules": {
            "builtin": {},
            "regex": {},
            "script": {}
        },
        "aiRules": {},
        "files": {
            "include": ["**/*.ts", "**/*.tsx", "**/*.js", "**/*.jsx", "**/*.mjs", "**/*.cjs"],
            "exclude": ["**/node_modules/**", "**/dist/**", "**/build/**", "**/.git/**"]
        }
    });

    let output = serde_json::to_string_pretty(&minimal).context("Failed to serialize config")?;
    fs::write(config_path, output).context("Failed to write minimal config")?;

    Ok(())
}

fn add_rule_to_config(
    json: &mut Value,
    rule: &RegistryRule,
    config: &RuleConfig,
    force: bool,
) -> Result<()> {
    match rule.kind.as_str() {
        "ai" => add_ai_rule(json, rule, config, force),
        "script" => add_script_rule(json, rule, config, force),
        "regex" => add_regex_rule(json, rule, config, force),
        _ => anyhow::bail!("Unknown rule kind: {}", rule.kind),
    }
}

fn add_ai_rule(
    json: &mut Value,
    rule: &RegistryRule,
    config: &RuleConfig,
    force: bool,
) -> Result<()> {
    let ai_rules = json
        .as_object_mut()
        .context("Config is not an object")?
        .entry("aiRules")
        .or_insert(json!({}));

    let ai_rules_obj = ai_rules
        .as_object_mut()
        .context("aiRules is not an object")?;

    if ai_rules_obj.contains_key(&rule.name) && !force {
        anyhow::bail!(
            "AI rule '{}' already exists in config. Use --force to overwrite.",
            rule.name
        );
    }

    let mut rule_config = Map::new();
    rule_config.insert("prompt".to_string(), json!(format!("{}.md", rule.name)));
    if let Some(mode) = &config.mode {
        rule_config.insert("mode".to_string(), json!(mode));
    }
    rule_config.insert("message".to_string(), json!(config.message));
    if let Some(severity) = &config.severity {
        rule_config.insert("severity".to_string(), json!(severity));
    }
    if let Some(include) = &config.include {
        rule_config.insert("include".to_string(), json!(include));
    }
    if let Some(exclude) = &config.exclude {
        rule_config.insert("exclude".to_string(), json!(exclude));
    }

    ai_rules_obj.insert(rule.name.clone(), Value::Object(rule_config));

    Ok(())
}

fn add_script_rule(
    json: &mut Value,
    rule: &RegistryRule,
    config: &RuleConfig,
    force: bool,
) -> Result<()> {
    let rules = json
        .as_object_mut()
        .context("Config is not an object")?
        .entry("rules")
        .or_insert(json!({}));

    let script_rules = rules
        .as_object_mut()
        .context("rules is not an object")?
        .entry("script")
        .or_insert(json!({}));

    let script_rules_obj = script_rules
        .as_object_mut()
        .context("rules.script is not an object")?;

    if script_rules_obj.contains_key(&rule.name) && !force {
        anyhow::bail!(
            "Script rule '{}' already exists in config. Use --force to overwrite.",
            rule.name
        );
    }

    let mut rule_config = Map::new();
    rule_config.insert(
        "command".to_string(),
        json!(format!("npx tsx script-rules/{}.ts", rule.name)),
    );
    rule_config.insert("message".to_string(), json!(config.message));
    if let Some(severity) = &config.severity {
        rule_config.insert("severity".to_string(), json!(severity));
    }
    if let Some(include) = &config.include {
        rule_config.insert("include".to_string(), json!(include));
    }
    if let Some(exclude) = &config.exclude {
        rule_config.insert("exclude".to_string(), json!(exclude));
    }

    script_rules_obj.insert(rule.name.clone(), Value::Object(rule_config));

    Ok(())
}

fn add_regex_rule(
    json: &mut Value,
    rule: &RegistryRule,
    config: &RuleConfig,
    force: bool,
) -> Result<()> {
    let rules = json
        .as_object_mut()
        .context("Config is not an object")?
        .entry("rules")
        .or_insert(json!({}));

    let regex_rules = rules
        .as_object_mut()
        .context("rules is not an object")?
        .entry("regex")
        .or_insert(json!({}));

    let regex_rules_obj = regex_rules
        .as_object_mut()
        .context("rules.regex is not an object")?;

    if regex_rules_obj.contains_key(&rule.name) && !force {
        anyhow::bail!(
            "Regex rule '{}' already exists in config. Use --force to overwrite.",
            rule.name
        );
    }

    let mut rule_config = Map::new();
    if let Some(pattern) = &config.pattern {
        rule_config.insert("pattern".to_string(), json!(pattern));
    }
    rule_config.insert("message".to_string(), json!(config.message));
    if let Some(severity) = &config.severity {
        rule_config.insert("severity".to_string(), json!(severity));
    }
    if let Some(include) = &config.include {
        rule_config.insert("include".to_string(), json!(include));
    }
    if let Some(exclude) = &config.exclude {
        rule_config.insert("exclude".to_string(), json!(exclude));
    }

    regex_rules_obj.insert(rule.name.clone(), Value::Object(rule_config));

    Ok(())
}
