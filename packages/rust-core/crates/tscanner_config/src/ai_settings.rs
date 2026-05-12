use crate::{AiConfig, AiProvider};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai: Option<AiConfig>,
}

pub fn resolve_ai_config(
    flag_provider: Option<AiProvider>,
    flag_model: Option<String>,
) -> Result<Option<AiConfig>> {
    let base_config = env_config()?.or(read_user_config()?.ai);

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

pub fn read_user_config() -> Result<UserConfig> {
    let path = user_config_path();
    if !path.exists() {
        return Ok(UserConfig::default());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read user config at {}", path.display()))?;
    let stripped = json_comments::StripComments::new(content.as_bytes());
    serde_json::from_reader(stripped)
        .with_context(|| format!("Failed to parse user config at {}", path.display()))
}

pub fn write_user_config(config: &UserConfig) -> Result<()> {
    let path = user_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create user config dir {}", parent.display()))?;
    }

    let json = serde_json::to_string_pretty(config)?;
    let mut file = fs::File::create(&path)
        .with_context(|| format!("Failed to write user config at {}", path.display()))?;
    file.write_all(json.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

pub fn user_config_path() -> PathBuf {
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
