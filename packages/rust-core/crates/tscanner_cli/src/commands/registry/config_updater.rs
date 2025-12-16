use anyhow::{Context, Result};
use regex::Regex;
use serde_json::{json, Value};
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
    let json: Value = serde_json::from_reader(stripped).context("Failed to parse config JSON")?;

    check_rule_exists(&json, rule, force)?;

    let new_content = insert_rule_text(&content, rule, config)?;
    fs::write(&config_path, new_content).context("Failed to write config")?;

    Ok(())
}

fn check_rule_exists(json: &Value, rule: &RegistryRule, force: bool) -> Result<()> {
    if force {
        return Ok(());
    }

    let exists = match rule.kind.as_str() {
        "ai" => json
            .get("aiRules")
            .and_then(|v| v.get(&rule.name))
            .is_some(),
        "script" => json
            .get("rules")
            .and_then(|v| v.get("script"))
            .and_then(|v| v.get(&rule.name))
            .is_some(),
        "regex" => json
            .get("rules")
            .and_then(|v| v.get("regex"))
            .and_then(|v| v.get(&rule.name))
            .is_some(),
        _ => false,
    };

    if exists {
        anyhow::bail!(
            "{} rule '{}' already exists in config. Use --force to overwrite.",
            rule.kind,
            rule.name
        );
    }

    Ok(())
}

fn insert_rule_text(content: &str, rule: &RegistryRule, config: &RuleConfig) -> Result<String> {
    let section_pattern = match rule.kind.as_str() {
        "ai" => r#""aiRules"\s*:\s*\{"#,
        "script" => r#""script"\s*:\s*\{"#,
        "regex" => r#""regex"\s*:\s*\{"#,
        _ => anyhow::bail!("Unknown rule kind: {}", rule.kind),
    };

    let re = Regex::new(section_pattern).context("Failed to compile regex")?;
    let section_match = re.find(content).context(format!(
        "Section for '{}' rules not found in config",
        rule.kind
    ))?;

    let section_start = section_match.end();
    let closing_brace_pos = find_matching_brace(content, section_start)?;

    let rule_json = build_rule_json(rule, config)?;
    let indent = detect_indent(content);
    let formatted_rule = format_rule_entry(&rule.name, &rule_json, &indent);

    let before_brace = &content[..closing_brace_pos];
    let after_brace = &content[closing_brace_pos..];

    let needs_comma = before_brace
        .trim_end()
        .chars()
        .last()
        .map(|c| c != '{' && c != ',')
        .unwrap_or(false);

    let separator = if needs_comma { ",\n" } else { "\n" };

    let new_content = format!(
        "{}{}{}{}",
        before_brace.trim_end(),
        separator,
        formatted_rule,
        after_brace
    );

    Ok(new_content)
}

fn find_matching_brace(content: &str, start: usize) -> Result<usize> {
    let mut depth = 1;
    let mut in_string = false;
    let mut escape_next = false;

    for (i, c) in content[start..].char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match c {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return Ok(start + i);
                }
            }
            _ => {}
        }
    }

    anyhow::bail!("Could not find matching closing brace")
}

fn detect_indent(content: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('"') {
            let indent_len = line.len() - trimmed.len();
            return line[..indent_len].to_string();
        }
    }
    "  ".to_string()
}

fn build_rule_json(rule: &RegistryRule, config: &RuleConfig) -> Result<Value> {
    let mut obj = serde_json::Map::new();

    match rule.kind.as_str() {
        "ai" => {
            obj.insert("prompt".to_string(), json!(format!("{}.md", rule.name)));
            if let Some(mode) = &config.mode {
                obj.insert("mode".to_string(), json!(mode));
            }
            obj.insert("message".to_string(), json!(&config.message));
            if let Some(severity) = &config.severity {
                obj.insert("severity".to_string(), json!(severity));
            }
            if let Some(include) = &config.include {
                obj.insert("include".to_string(), json!(include));
            }
            if let Some(exclude) = &config.exclude {
                obj.insert("exclude".to_string(), json!(exclude));
            }
        }
        "script" => {
            let command = config
                .command
                .clone()
                .context("Script rule config must have a 'command' field")?;
            obj.insert("command".to_string(), json!(command));
            obj.insert("message".to_string(), json!(&config.message));
            if let Some(severity) = &config.severity {
                obj.insert("severity".to_string(), json!(severity));
            }
            if let Some(include) = &config.include {
                obj.insert("include".to_string(), json!(include));
            }
            if let Some(exclude) = &config.exclude {
                obj.insert("exclude".to_string(), json!(exclude));
            }
        }
        "regex" => {
            if let Some(pattern) = &config.pattern {
                obj.insert("pattern".to_string(), json!(pattern));
            }
            obj.insert("message".to_string(), json!(&config.message));
            if let Some(severity) = &config.severity {
                obj.insert("severity".to_string(), json!(severity));
            }
            if let Some(include) = &config.include {
                obj.insert("include".to_string(), json!(include));
            }
            if let Some(exclude) = &config.exclude {
                obj.insert("exclude".to_string(), json!(exclude));
            }
        }
        _ => anyhow::bail!("Unknown rule kind: {}", rule.kind),
    }

    Ok(Value::Object(obj))
}

fn format_rule_entry(name: &str, value: &Value, base_indent: &str) -> String {
    let inner_indent = format!("{}{}", base_indent, base_indent);
    let array_indent = format!("{}{}", inner_indent, base_indent);

    let mut lines = vec![format!("{}\"{}\": {{", inner_indent, name)];

    if let Value::Object(obj) = value {
        let entries: Vec<_> = obj.iter().collect();
        for (i, (key, val)) in entries.iter().enumerate() {
            let comma = if i < entries.len() - 1 { "," } else { "" };
            let formatted = match val {
                Value::Array(arr) => {
                    let items: Vec<String> = arr
                        .iter()
                        .map(|v| format!("{}{}", array_indent, v))
                        .collect();
                    format!(
                        "{}\"{}\": [\n{}\n{}]{}",
                        array_indent,
                        key,
                        items.join(",\n"),
                        array_indent,
                        comma
                    )
                }
                _ => format!("{}\"{}\": {}{}", array_indent, key, val, comma),
            };
            lines.push(formatted);
        }
    }

    lines.push(format!("{}}}", inner_indent));
    lines.join("\n")
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
