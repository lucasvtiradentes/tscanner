mod config_ext;
mod disable_comments;
pub mod executors;
mod parser;
mod scanner;

pub use config_ext::{load_config, ConfigExt};
pub use disable_comments::DisableDirectives;
pub use executors::{
    is_js_ts_file, AiExecutor, BuiltinExecutor, ExecuteResult, ScriptError, ScriptExecutor,
    ScriptFile, ScriptInput, ScriptOutput,
};
pub use parser::parse_file;
pub use scanner::{BranchScanResult, Scanner, StagedScanResult};
pub use tscanner_config::{disable_file_comment, disable_next_line_comment};
