mod globset;
mod loader;
mod types;
mod validation;

pub use globset::{compile_globset, compile_optional_globset};
pub use loader::{get_config_error_prefix, TscannerConfigExt};
pub use types::{
    AiConfig, AiExecutionMode, AiMode, AiProvider, AiRuleConfig, BuiltinRuleConfig,
    CodeEditorConfig, CompiledRuleConfig, FilesConfig, RegexRuleConfig, RulesConfig,
    ScriptRuleConfig, TscannerConfig,
};
pub use validation::{validate_json_fields, ValidationResult};

pub use tscanner_types::Severity;
