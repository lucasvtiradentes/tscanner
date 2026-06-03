mod ai_rule_sources;
mod ai_rules_validator;
mod ai_settings;
mod globset;
mod loader;
mod types;
mod validation;

pub use ai_rule_sources::{resolve_ai_rule_sources, strip_frontmatter};
pub use ai_rules_validator::validate_ai_rules;
pub use ai_settings::{
    compute_ai_runtime_hash, env_config, find_project_config_dir, legacy_user_config_path,
    local_config_path_for_config_dir, read_local_config, resolve_ai_config, write_local_config,
    LocalConfig,
};
pub use globset::{compile_globset, compile_optional_globset};
pub use loader::{get_config_error_prefix, TscannerConfigExt};
pub use types::{
    AiConfig, AiExecutionMode, AiMode, AiProvider, AiRuleClassification, AiRuleSourceConfig,
    AiRuleSourceType, AiRuleSummary, BuiltinRuleConfig, CompiledRuleConfig, FilesConfig,
    RegexRuleConfig, ResolvedAiRuleConfig, RulesConfig, ScriptRuleConfig, TscannerConfig,
};
pub use validation::{validate_json_fields, ValidationResult};

pub use tscanner_types::Severity;
