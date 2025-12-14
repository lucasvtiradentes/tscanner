use crate::custom_requests::{ValidateConfigParams, ValidateConfigResult};
use crate::session::Session;
use lsp_server::{Connection, Message, Request, Response};
use std::fs;
use tscanner_config::TscannerConfigExt;
use tscanner_constants::{config_dir_name, config_file_name};

type LspError = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_validate_config(
    connection: &Connection,
    req: Request,
    _session: &mut Session,
) -> Result<(), LspError> {
    let params: ValidateConfigParams = serde_json::from_value(req.params)?;

    let config_file = if params.config_path.is_file() {
        params.config_path
    } else {
        params
            .config_path
            .join(config_dir_name())
            .join(config_file_name())
    };

    if !config_file.exists() {
        let result = ValidateConfigResult {
            valid: false,
            errors: vec![format!("Config file not found: {}", config_file.display())],
            warnings: vec![],
        };
        let response = Response::new_ok(req.id, serde_json::to_value(&result)?);
        connection.sender.send(Message::Response(response))?;
        return Ok(());
    }

    let content = fs::read_to_string(&config_file)?;
    let workspace = config_file.parent().and_then(|p| p.parent());

    match tscanner_config::TscannerConfig::full_validate(&content, workspace, config_dir_name()) {
        Ok((_config, validation_result)) => {
            let mut errors = Vec::new();
            let mut warnings = validation_result.warnings.clone();
            let mut invalid_fields = Vec::new();

            for error in &validation_result.errors {
                if error.starts_with("Invalid field: ") {
                    if let Some(field) = error.strip_prefix("Invalid field: ") {
                        invalid_fields.push(field.to_string());
                    }
                } else {
                    errors.push(error.clone());
                }
            }

            if !invalid_fields.is_empty() {
                warnings.push(format!(
                    "Config contains invalid fields [{}] which will be ignored",
                    invalid_fields.join(", ")
                ));
            }

            let result = ValidateConfigResult {
                valid: errors.is_empty(),
                errors,
                warnings,
            };
            let response = Response::new_ok(req.id, serde_json::to_value(&result)?);
            connection.sender.send(Message::Response(response))?;
        }
        Err(e) => {
            let result = ValidateConfigResult {
                valid: false,
                errors: vec![format!("Validation failed: {}", e)],
                warnings: vec![],
            };
            let response = Response::new_ok(req.id, serde_json::to_value(&result)?);
            connection.sender.send(Message::Response(response))?;
        }
    }

    Ok(())
}
