use anyhow::Result;
use colored::*;
use tscanner_config::{
    env_config, read_user_config, user_config_path, write_user_config, AiConfig, AiProvider,
    UserConfig,
};

pub fn set(provider: AiProvider, model: Option<String>) -> Result<()> {
    let config = UserConfig {
        ai: Some(AiConfig { provider, model }),
    };
    write_user_config(&config)?;

    println!("{} AI provider set to {}", "✓".green(), provider.as_str());
    if has_ai_env_override() {
        println!(
            "{} TSCANNER_AI_* environment variables are set and will override this user config.",
            "Note:".yellow()
        );
    }
    if let Some(ai) = config.ai {
        if let Some(model) = ai.model {
            println!("  model: {}", model);
        }
        println!("  config: {}", user_config_path().display());
    }

    Ok(())
}

fn has_ai_env_override() -> bool {
    std::env::var("TSCANNER_AI_PROVIDER").is_ok() || std::env::var("TSCANNER_AI_MODEL").is_ok()
}

pub fn show() -> Result<()> {
    let env_config = env_config()?;
    if let Some(config) = env_config {
        print_ai_config("env", &config);
        return Ok(());
    }

    let user_config = read_user_config()?;
    if let Some(config) = user_config.ai {
        print_ai_config("user-config", &config);
        println!("config: {}", user_config_path().display());
        return Ok(());
    }

    println!("AI provider: not configured");
    println!("Set one with: {}", "tscanner ai set <provider>".cyan());
    Ok(())
}

pub fn unset() -> Result<()> {
    let path = user_config_path();
    if !path.exists() {
        println!("AI provider already unset");
        return Ok(());
    }

    let mut config = read_user_config()?;
    config.ai = None;
    write_user_config(&config)?;
    println!("{} AI provider unset", "✓".green());
    Ok(())
}

pub(super) fn resolve_ai_config(
    flag_provider: Option<AiProvider>,
    flag_model: Option<String>,
) -> Result<Option<AiConfig>> {
    tscanner_config::resolve_ai_config(flag_provider, flag_model)
}

fn print_ai_config(source: &str, config: &AiConfig) {
    println!("AI provider: {}", config.provider.as_str());
    println!("source: {}", source);
    if let Some(model) = &config.model {
        println!("model: {}", model);
    }
}
