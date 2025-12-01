pub mod config;
pub mod constants;
pub mod output;
pub mod rule;
pub mod rules;
pub mod scanning;
pub mod utils;

pub use config::{
    compile_globset, validate_json_fields, AiRuleConfig, BuiltinRuleConfig, CliConfig, CliGroupBy,
    CompiledRuleConfig, CustomRuleBase, CustomRuleConfig, RegexRuleConfig, ScriptMode,
    ScriptRuleConfig, TscannerConfig, ValidationResult,
};
pub use constants::*;
pub use output::{FileResult, GroupMode, Issue, PrettyFormatter, ScanResult, Severity};
pub use rule::{Rule, RuleRegistration};
pub use rules::{get_all_rule_metadata, RegexRule, RuleCategory, RuleMetadata};
pub use scanning::{
    parse_file, FileCache, FileEvent, FileWatcher, RuleRegistry, Scanner, ScriptExecutor,
};
pub use utils::{
    get_changed_files, get_logger, get_modified_lines, get_staged_files, get_staged_modified_lines,
    init_logger, log_debug, log_error, log_info, log_warn, DisableDirectives, FileSource,
};
