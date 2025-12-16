use anyhow::{Context, Result};
use serde::Deserialize;
use tscanner_constants::registry_base_url;

#[derive(Debug, Clone, Deserialize)]
pub struct RegistryIndex {
    pub rules: Vec<RegistryRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistryRule {
    pub name: String,
    pub kind: String,
    pub category: String,
    pub description: String,
    #[serde(default)]
    pub file: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuleConfig {
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    pub message: String,
    #[serde(default)]
    pub severity: Option<String>,
    #[serde(default)]
    pub include: Option<Vec<String>>,
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
}

pub fn fetch_registry_index() -> Result<RegistryIndex> {
    let base_url = registry_base_url();
    let url = format!("{}/index.json", base_url);
    let response = reqwest::blocking::get(&url)
        .context(format!("Failed to fetch registry index from {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to fetch registry: HTTP {} from {}",
            response.status(),
            url
        );
    }

    let index: RegistryIndex = response
        .json()
        .context("Failed to parse registry index JSON")?;

    Ok(index)
}

pub fn fetch_rule_config(rule: &RegistryRule) -> Result<RuleConfig> {
    let base_url = registry_base_url();
    let folder = get_rule_folder(&rule.kind)?;

    let url = format!("{}/{}/{}/config.jsonc", base_url, folder, rule.name);

    let response = reqwest::blocking::get(&url)
        .context(format!("Failed to fetch rule config from {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to fetch rule config: HTTP {} from {}",
            response.status(),
            url
        );
    }

    let text = response.text().context("Failed to read response text")?;
    let stripped = json_comments::StripComments::new(text.as_bytes());
    let config: RuleConfig =
        serde_json::from_reader(stripped).context("Failed to parse rule config JSON")?;

    Ok(config)
}

pub fn fetch_rule_file(rule: &RegistryRule) -> Result<Option<(String, String)>> {
    if rule.kind == "regex" {
        return Ok(None);
    }

    let filename = match &rule.file {
        Some(f) => f.clone(),
        None => get_default_filename(&rule.kind)?,
    };

    let base_url = registry_base_url();
    let folder = get_rule_folder(&rule.kind)?;

    let url = format!("{}/{}/{}/{}", base_url, folder, rule.name, filename);

    let response =
        reqwest::blocking::get(&url).context(format!("Failed to fetch rule file from {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to fetch rule file: HTTP {} from {}",
            response.status(),
            url
        );
    }

    let content = response
        .text()
        .context("Failed to read rule file content")?;

    Ok(Some((filename, content)))
}

fn get_rule_folder(kind: &str) -> Result<&'static str> {
    match kind {
        "ai" => Ok("ai-rules"),
        "script" => Ok("script-rules"),
        "regex" => Ok("regex-rules"),
        _ => anyhow::bail!("Unknown rule kind: {}", kind),
    }
}

fn get_default_filename(kind: &str) -> Result<String> {
    match kind {
        "ai" => Ok("prompt.md".to_string()),
        "script" => Ok("script.ts".to_string()),
        _ => anyhow::bail!("No default filename for kind: {}", kind),
    }
}

pub fn filter_rules(
    rules: Vec<RegistryRule>,
    kind: Option<&str>,
    category: Option<&str>,
) -> Vec<RegistryRule> {
    rules
        .into_iter()
        .filter(|r| kind.is_none_or(|k| r.kind == k))
        .filter(|r| category.is_none_or(|c| r.category == c))
        .collect()
}
