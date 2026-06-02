use std::path::Path;

use tscanner_config::{resolve_ai_rule_sources, TscannerConfig, TscannerConfigExt};

pub fn load_config(
    path: &Path,
    config_dir_name: &str,
    config_file_name: &str,
) -> Result<(TscannerConfig, Vec<String>), Box<dyn std::error::Error>> {
    let config_path = if path.is_file() {
        path.to_path_buf()
    } else {
        path.join(config_dir_name).join(config_file_name)
    };

    if !config_path.exists() {
        return Err(format!(
            "Config file not found: {}. Run 'tscanner init' to create one.",
            config_path.display()
        )
        .into());
    }

    let workspace = config_path.parent().and_then(|p| p.parent());
    let content = std::fs::read_to_string(&config_path)?;
    let (config, result) = TscannerConfig::full_validate(&content, workspace, config_dir_name)?;

    let mut warnings = result.warnings.clone();

    let invalid_fields: Vec<_> = result
        .errors
        .iter()
        .filter_map(|e| e.strip_prefix("Invalid field: "))
        .map(|s| s.to_string())
        .collect();

    if !result.is_valid() {
        if !invalid_fields.is_empty() {
            warnings.push(format!(
                "Config contains invalid fields [{}] which will be ignored",
                invalid_fields.join(", ")
            ));

            let remaining_errors: Vec<_> = result
                .errors
                .iter()
                .filter(|e| !e.starts_with("Invalid field: "))
                .cloned()
                .collect();

            if !remaining_errors.is_empty() {
                return Err(format!(
                    "Config validation failed:\n  - {}",
                    remaining_errors.join("\n  - ")
                )
                .into());
            }
        } else {
            return Err(format!(
                "Config validation failed:\n  - {}",
                result.errors.join("\n  - ")
            )
            .into());
        }
    }

    let config = if let Some(cfg) = config {
        cfg
    } else if !invalid_fields.is_empty() {
        let json_value = TscannerConfig::parse_json(&content)?;
        let mut config = serde_json::from_value::<TscannerConfig>(json_value)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        let mut fallback_result = resolve_ai_rule_sources(&mut config, workspace);
        fallback_result.merge(config.validate_with_workspace(workspace, config_dir_name));
        warnings.extend(fallback_result.warnings);

        if !fallback_result.errors.is_empty() {
            return Err(format!(
                "Config validation failed:\n  - {}",
                fallback_result.errors.join("\n  - ")
            )
            .into());
        }

        config
    } else {
        return Err("Config parsing failed".into());
    };

    Ok((config, warnings))
}
