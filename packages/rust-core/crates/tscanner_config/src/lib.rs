mod defaults;
mod globset;
mod loader;
mod types;
mod validation;

pub use defaults::{
    ai_inflight_wait_ms, ai_placeholder_content, ai_placeholder_files, ai_polling_ms, ai_rules_dir,
    ai_temp_dir, app_description, app_display_name, app_name, cache_dir_name, cache_file_pattern,
    claude_args, claude_command, config_dir_name, config_file_name, conflicting_rules,
    default_target_branch, example_ai_rule, example_script_rule, gemini_args, gemini_command,
    get_default_config_json, get_log_filename, icon_ai, icon_builtin, icon_error, icon_progress,
    icon_regex, icon_script, icon_skipped, icon_success, icon_warning, ignore_comment,
    ignore_next_line_comment, is_dev_mode, js_extensions, log_basename, lsp_debounce_ms,
    rule_types, rules_base_url, scheduler_polling_ms, script_polling_ms, script_rules_dir,
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
