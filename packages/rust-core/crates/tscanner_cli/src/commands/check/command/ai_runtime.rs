use std::path::Path;

use crate::commands::ai;
use crate::shared::fatal_error_and_exit;
use tscanner_config::{AiConfig, AiExecutionMode, AiProvider};

pub(super) fn resolve_effective_ai_config(
    effective_ai_mode: AiExecutionMode,
    ai_provider_flag: Option<AiProvider>,
    ai_model_flag: Option<String>,
    config_dir: &Path,
) -> Option<AiConfig> {
    if effective_ai_mode == AiExecutionMode::Ignore
        && ai_provider_flag.is_none()
        && ai_model_flag.is_none()
    {
        return None;
    }

    match ai::resolve_ai_config(ai_provider_flag, ai_model_flag, config_dir) {
        Ok(config) => config,
        Err(error) => fatal_error_and_exit(&error.to_string(), &[]),
    }
}
