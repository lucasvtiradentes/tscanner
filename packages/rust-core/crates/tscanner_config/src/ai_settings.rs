use crate::{AiConfig, AiProvider};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tscanner_constants::{config_dir_name, config_file_name, local_config_file_name};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai: Option<AiConfig>,

    #[serde(flatten)]
    pub other: serde_json::Map<String, serde_json::Value>,
}

impl LocalConfig {
    fn is_empty(&self) -> bool {
        self.ai.is_none() && self.other.is_empty()
    }
}

pub fn resolve_ai_config(
    flag_provider: Option<AiProvider>,
    flag_model: Option<String>,
    config_dir: &Path,
) -> Result<Option<AiConfig>> {
    let base_config = env_config()?.or(read_local_config(config_dir)?.ai);

    if let Some(provider) = flag_provider {
        let base_config = base_config.filter(|config| config.provider == provider);
        return Ok(Some(AiConfig {
            provider,
            model: flag_model
                .or_else(|| base_config.as_ref().and_then(|config| config.model.clone())),
        }));
    }

    if let Some(model) = flag_model {
        let Some(mut config) = base_config else {
            anyhow::bail!("--ai-model requires an AI provider from --ai-provider, TSCANNER_AI_PROVIDER, or 'tscanner ai set'");
        };
        config.model = Some(model);
        return Ok(Some(config));
    }

    Ok(base_config)
}

pub fn compute_ai_runtime_hash(base_hash: u64, ai_config: Option<&AiConfig>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    base_hash.hash(&mut hasher);
    if let Some(ai_config) = ai_config {
        ai_config.provider.as_str().hash(&mut hasher);
        ai_config.model.hash(&mut hasher);
    }
    hasher.finish()
}

pub fn env_config() -> Result<Option<AiConfig>> {
    let Some(provider_name) = std::env::var("TSCANNER_AI_PROVIDER").ok() else {
        return Ok(None);
    };

    let provider = AiProvider::from_str(&provider_name).map_err(|_| {
        anyhow::anyhow!(
            "unsupported TSCANNER_AI_PROVIDER '{}'. Available providers: {}",
            provider_name,
            AiProvider::all_names()
        )
    })?;
    let model = std::env::var("TSCANNER_AI_MODEL").ok();

    Ok(Some(AiConfig { provider, model }))
}

pub fn read_local_config(config_dir: &Path) -> Result<LocalConfig> {
    let path = local_config_path_for_config_dir(config_dir);
    if !path.exists() {
        return Ok(LocalConfig::default());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read local config at {}", path.display()))?;
    let stripped = json_comments::StripComments::new(content.as_bytes());
    serde_json::from_reader(stripped)
        .with_context(|| format!("Failed to parse local config at {}", path.display()))
}

pub fn write_local_config(config_dir: &Path, config: &LocalConfig) -> Result<()> {
    let path = local_config_path_for_config_dir(config_dir);
    if config.is_empty() {
        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("Failed to remove local config at {}", path.display()))?;
        }
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create local config dir {}", parent.display()))?;
    }
    ensure_local_config_ignored(config_dir)?;

    let json = serde_json::to_string_pretty(config)?;
    let temp_path = path.with_extension("jsonc.tmp");
    let mut file = fs::File::create(&temp_path)
        .with_context(|| format!("Failed to write local config at {}", temp_path.display()))?;
    file.write_all(json.as_bytes())?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    fs::rename(&temp_path, &path).with_context(|| {
        format!(
            "Failed to move local config from {} to {}",
            temp_path.display(),
            path.display()
        )
    })?;
    Ok(())
}

fn ensure_local_config_ignored(config_dir: &Path) -> Result<()> {
    fs::create_dir_all(config_dir)
        .with_context(|| format!("Failed to create config dir {}", config_dir.display()))?;

    let path = config_dir.join(".gitignore");
    let entry = local_config_file_name();
    if path.exists() {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        if content.lines().any(|line| line.trim() == entry) {
            return Ok(());
        }

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .with_context(|| format!("Failed to update {}", path.display()))?;
        if !content.ends_with('\n') {
            file.write_all(b"\n")?;
        }
        file.write_all(entry.as_bytes())?;
        file.write_all(b"\n")?;
        return Ok(());
    }

    fs::write(&path, format!("{entry}\n"))
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

pub fn local_config_path_for_config_dir(config_dir: &Path) -> PathBuf {
    config_dir.join(local_config_file_name())
}

pub fn find_project_config_dir(start: &Path) -> Result<PathBuf> {
    let start = if start.is_file() {
        start.parent().unwrap_or(start)
    } else {
        start
    };

    for ancestor in start.ancestors() {
        let config_dir = ancestor.join(config_dir_name());
        if config_dir.join(config_file_name()).exists() {
            return Ok(config_dir);
        }
    }

    anyhow::bail!(
        "No TScanner project found. Expected {}/{} in this directory or one of its parents.",
        config_dir_name(),
        config_file_name()
    )
}

pub fn legacy_user_config_path() -> PathBuf {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(config_home)
            .join("tscanner")
            .join("config.jsonc");
    }

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".config")
        .join("tscanner")
        .join("config.jsonc")
}
