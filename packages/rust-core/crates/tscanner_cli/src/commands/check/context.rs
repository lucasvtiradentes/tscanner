use std::path::PathBuf;
use tscanner_diagnostics::GroupMode;

use super::command::CliOptions;

#[derive(Clone)]
pub struct CheckContext {
    pub root: PathBuf,
    pub group_mode: GroupMode,
    pub cli_options: CliOptions,
}

impl CheckContext {
    pub fn new(root: PathBuf, group_mode: GroupMode, cli_options: CliOptions) -> Self {
        Self {
            root,
            group_mode,
            cli_options,
        }
    }
}
