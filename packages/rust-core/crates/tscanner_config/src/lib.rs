mod defaults;
mod globset;
mod loader;
mod types;
mod validation;

pub use defaults::get_default_config_json;
pub use globset::{compile_globset, compile_optional_globset};
pub use loader::CONFIG_ERROR_PREFIX;
pub use types::{
    AiConfig, AiExecutionMode, AiMode, AiProvider, AiRuleConfig, BuiltinRuleConfig,
    CodeEditorConfig, CompiledRuleConfig, FilesConfig, RegexRuleConfig, RulesConfig,
    ScriptRuleConfig, TscannerConfig,
};
pub use validation::{validate_json_fields, ValidationResult};

pub use tscanner_types::Severity;
