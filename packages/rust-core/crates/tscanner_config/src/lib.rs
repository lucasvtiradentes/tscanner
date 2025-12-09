mod defaults;
mod globset;
mod loader;
mod types;
mod validation;

pub use defaults::{
    app_description, app_display_name, app_name, config_dir_name, config_file_name,
    default_target_branch, get_default_config_json, get_log_filename, ignore_comment,
    ignore_next_line_comment, is_dev_mode, log_basename,
};
pub use globset::{compile_globset, compile_optional_globset};
pub use loader::CONFIG_ERROR_PREFIX;
pub use types::{
    AiConfig, AiExecutionMode, AiMode, AiProvider, AiRuleConfig, BuiltinRuleConfig,
    CodeEditorConfig, CompiledRuleConfig, FilesConfig, RegexRuleConfig, RulesConfig,
    ScriptRuleConfig, TscannerConfig,
};
pub use validation::{validate_json_fields, ValidationResult};

pub use tscanner_diagnostics::Severity;
