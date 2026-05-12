mod ai_rules_validator;
mod ai_settings;
mod globset;
mod loader;
mod types;
mod validation;

pub use ai_rules_validator::validate_ai_rules;
pub use ai_settings::{
    compute_ai_runtime_hash, env_config, read_user_config, resolve_ai_config, user_config_path,
    write_user_config, UserConfig,
};
pub use globset::{compile_globset, compile_optional_globset};
pub use loader::{get_config_error_prefix, TscannerConfigExt};
pub use types::{
    AiConfig, AiExecutionMode, AiMode, AiProvider, AiRuleConfig, BuiltinRuleConfig,
    CodeEditorConfig, CompiledRuleConfig, FilesConfig, RegexRuleConfig, RulesConfig,
    ScriptRuleConfig, TscannerConfig,
};
pub use validation::{validate_json_fields, ValidationResult};

pub use tscanner_types::Severity;
