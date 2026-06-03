use anyhow::Result;
use colored::*;
use std::path::{Path, PathBuf};
use tscanner_config::{
    env_config, find_project_config_dir, legacy_user_config_path, local_config_path_for_config_dir,
    read_local_config, write_local_config, AiConfig, AiProvider,
};

pub fn set(provider: AiProvider, model: Option<String>) -> Result<()> {
    let config_dir = current_project_config_dir()?;
    print_legacy_warning(&config_dir);

    let mut config = read_local_config(&config_dir)?;
    config.ai = Some(AiConfig { provider, model });
    write_local_config(&config_dir, &config)?;

    println!("{} AI provider set to {}", "✓".green(), provider.as_str());
    if has_ai_env_override() {
        println!(
            "{} TSCANNER_AI_* environment variables are set and will override this local config.",
            "Note:".yellow()
        );
    }
    if let Some(ai) = config.ai {
        if let Some(model) = ai.model {
            println!("  model: {}", model);
        }
        print_config_location(&config_dir);
    }

    Ok(())
}

fn has_ai_env_override() -> bool {
    std::env::var("TSCANNER_AI_PROVIDER").is_ok() || std::env::var("TSCANNER_AI_MODEL").is_ok()
}

pub fn show() -> Result<()> {
    let config_dir = current_project_config_dir()?;
    print_legacy_warning(&config_dir);

    let env_config = env_config()?;
    if let Some(config) = env_config {
        print_ai_config("env", &config);
        print_config_location(&config_dir);
        return Ok(());
    }

    let local_config = read_local_config(&config_dir)?;
    if let Some(config) = local_config.ai {
        print_ai_config("local", &config);
        print_config_location(&config_dir);
        return Ok(());
    }

    println!("AI provider: not configured");
    println!("Set one with: {}", "tscanner ai set <provider>".cyan());
    Ok(())
}

pub fn unset() -> Result<()> {
    let config_dir = current_project_config_dir()?;
    print_legacy_warning(&config_dir);

    let mut config = read_local_config(&config_dir)?;
    if config.ai.is_none() {
        println!("AI provider already unset");
        return Ok(());
    }

    config.ai = None;
    write_local_config(&config_dir, &config)?;
    println!("{} AI provider unset", "✓".green());
    print_config_location(&config_dir);
    Ok(())
}

pub(super) fn resolve_ai_config(
    flag_provider: Option<AiProvider>,
    flag_model: Option<String>,
    config_dir: &Path,
) -> Result<Option<AiConfig>> {
    tscanner_config::resolve_ai_config(flag_provider, flag_model, config_dir)
}

fn print_ai_config(source: &str, config: &AiConfig) {
    println!("AI provider: {}", config.provider.as_str());
    println!("source: {}", source);
    if let Some(model) = &config.model {
        println!("model: {}", model);
    }
}

fn current_project_config_dir() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    find_project_config_dir(&cwd)
}

fn print_config_location(config_dir: &Path) {
    let project_root = config_dir.parent().unwrap_or(config_dir);
    println!(
        "config: {}",
        local_config_path_for_config_dir(config_dir).display()
    );
    println!("project: {}", project_root.display());
}

fn print_legacy_warning(config_dir: &Path) {
    let legacy_path = legacy_user_config_path();
    let local_path = local_config_path_for_config_dir(config_dir);
    if legacy_path.exists() && !local_path.exists() {
        println!(
            "{} Legacy global AI config detected at {}. It is no longer read; run {} inside this project.",
            "Note:".yellow(),
            legacy_path.display(),
            "tscanner ai set <provider>".cyan()
        );
    }
}
