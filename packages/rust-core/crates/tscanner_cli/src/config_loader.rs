use anyhow::Result;
use std::path::{Path, PathBuf};

use tscanner_config::TscannerConfig;
use tscanner_constants::{config_dir_name, config_file_name};
use tscanner_scanner::load_config;
use tscanner_service::log_info;

pub fn load_config_with_custom(
    root: &Path,
    custom_path: Option<PathBuf>,
) -> Result<Option<(TscannerConfig, String, Vec<String>)>> {
    if let Some(custom_config_dir) = custom_path {
        let resolved_dir = if custom_config_dir.is_absolute() {
            custom_config_dir
        } else {
            root.join(custom_config_dir)
        };

        if !resolved_dir.is_dir() {
            anyhow::bail!(
                "Config path must be a directory, got: {}",
                resolved_dir.display()
            );
        }

        let config_path = resolved_dir.join(config_file_name());

        if config_path.exists() {
            log_info(&format!(
                "config_loader: Loading custom config: {}",
                config_path.display()
            ));
            let (config, warnings) =
                load_config(&config_path, config_dir_name(), config_file_name())
                    .map_err(|e| anyhow::anyhow!("{}", e))?;
            return Ok(Some((config, config_path.display().to_string(), warnings)));
        } else {
            anyhow::bail!(
                "Config file not found: {} (expected {} in directory)",
                config_path.display(),
                config_file_name()
            );
        }
    }

    let local_path = root.join(config_dir_name()).join(config_file_name());
    if local_path.exists() {
        log_info(&format!(
            "config_loader: Loading local config: {}",
            local_path.display()
        ));
        let (config, warnings) = load_config(&local_path, config_dir_name(), config_file_name())
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        return Ok(Some((config, local_path.display().to_string(), warnings)));
    }

    log_info("config_loader: No config found");
    Ok(None)
}
