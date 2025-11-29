use core::CliConfig;
use std::path::PathBuf;

use crate::GroupMode;

#[derive(Clone)]
pub struct CheckContext {
    pub root: PathBuf,
    pub group_mode: GroupMode,
    pub cli_config: CliConfig,
}

impl CheckContext {
    pub fn new(root: PathBuf, group_mode: GroupMode, cli_config: CliConfig) -> Self {
        Self {
            root,
            group_mode,
            cli_config,
        }
    }
}
