mod defaults;
mod globset;
mod loader;
mod types;
mod validation;

pub use defaults::{
    app_description, app_display_name, app_name, config_dir_name, config_file_name,
    default_target_branch, disable_file_comment, disable_next_line_comment,
    get_default_config_json, get_log_filename, is_dev_mode, log_basename,
};
pub use globset::{compile_globset, compile_optional_globset};
pub use loader::CONFIG_ERROR_PREFIX;
pub use types::{
    AiRuleConfig, BuiltinRuleConfig, CliConfig, CliGroupBy, CodeEditorConfig, CompiledRuleConfig,
    CustomRuleBase, CustomRuleConfig, FilesConfig, RegexRuleConfig, ScriptMode, ScriptRuleConfig,
    TscannerConfig,
};
pub use validation::{validate_json_fields, AllowedOptionsGetter, ValidationResult};

pub use tscanner_diagnostics::Severity;
