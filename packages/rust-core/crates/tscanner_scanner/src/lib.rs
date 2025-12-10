pub mod ai_providers;
mod config_ext;
mod disable_comments;
pub mod executors;
mod parser;
mod scanner;

pub use ai_providers::resolve_provider_command;
pub use config_ext::{load_config, ConfigExt};
pub use disable_comments::DisableDirectives;
pub use executors::{
    is_js_ts_file, AiExecutor, AiProgressCallback, AiProgressEvent, AiRuleStatus, BuiltinExecutor,
    ExecuteResult, RegularRulesCompleteCallback, ScriptError, ScriptExecutor, ScriptFile,
    ScriptInput, ScriptOutput,
};
pub use parser::parse_file;
pub use scanner::{BranchScanResult, ScanCallbacks, Scanner, StagedScanResult};
pub use tscanner_constants::{ignore_comment, ignore_next_line_comment};
