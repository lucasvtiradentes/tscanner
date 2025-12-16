use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use tscanner_constants::{ai_rules_dir, config_dir_name, script_rules_dir};

use super::registry::RegistryRule;

pub fn install_rule_file(
    workspace_root: &Path,
    rule: &RegistryRule,
    filename: &str,
    content: &str,
    force: bool,
) -> Result<String> {
    let config_dir = workspace_root.join(config_dir_name());

    let target_dir = match rule.kind.as_str() {
        "ai" => config_dir.join(ai_rules_dir()),
        "script" => config_dir.join(script_rules_dir()),
        "regex" => return Ok(String::new()),
        _ => anyhow::bail!("Unknown rule kind: {}", rule.kind),
    };

    fs::create_dir_all(&target_dir).context(format!(
        "Failed to create directory: {}",
        target_dir.display()
    ))?;

    let local_filename = get_local_filename(rule, filename);
    let target_path = target_dir.join(&local_filename);

    if target_path.exists() && !force {
        anyhow::bail!(
            "Rule file already exists: {}. Use --force to overwrite.",
            target_path.display()
        );
    }

    fs::write(&target_path, content).context(format!(
        "Failed to write rule file: {}",
        target_path.display()
    ))?;

    Ok(target_path.display().to_string())
}

fn get_local_filename(rule: &RegistryRule, registry_filename: &str) -> String {
    let extension = registry_filename.rsplit('.').next().unwrap_or("ts");

    match rule.kind.as_str() {
        "ai" => format!("{}.md", rule.name),
        "script" => format!("{}.{}", rule.name, extension),
        _ => registry_filename.to_string(),
    }
}

pub fn get_rule_local_path(rule: &RegistryRule) -> String {
    let extension = rule
        .file
        .as_ref()
        .and_then(|f| f.rsplit('.').next())
        .unwrap_or("ts");

    match rule.kind.as_str() {
        "ai" => format!("{}/{}.md", ai_rules_dir(), rule.name),
        "script" => format!("{}/{}.{}", script_rules_dir(), rule.name, extension),
        "regex" => String::new(),
        _ => String::new(),
    }
}
