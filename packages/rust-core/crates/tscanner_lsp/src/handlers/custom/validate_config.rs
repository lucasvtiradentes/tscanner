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
            let result = ValidateConfigResult {
                valid: validation_result.is_valid(),
                errors: validation_result.errors,
                warnings: validation_result.warnings,
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
