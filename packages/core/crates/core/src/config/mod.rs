mod defaults;
mod globset;
mod loader;
mod types;
mod validation;

pub use globset::compile_globset;
pub use loader::CONFIG_ERROR_PREFIX;
pub use types::{
    AiRuleConfig, BuiltinRuleConfig, CliConfig, CliGroupBy, CodeEditorConfig, CompiledRuleConfig,
    CustomRuleBase, CustomRuleConfig, FilesConfig, RegexRuleConfig, ScriptMode, ScriptRuleConfig,
    TscannerConfig,
};
pub use validation::{validate_json_fields, ValidationResult};
