mod constants;
mod errors;
mod logger;
mod server;
mod types;
mod workspace;

pub use constants::{
    app_description, app_display_name, app_name, config_dir_name, config_file_name,
    default_target_branch, disable_file_comment, disable_next_line_comment, get_log_filename,
    is_dev_mode, log_basename,
};
pub use errors::WorkspaceError;
pub use logger::{get_logger, init_logger, log_debug, log_error, log_info, log_warn};
pub use server::WorkspaceServer;
pub use types::*;
pub use workspace::Workspace;

pub fn create_workspace() -> Box<dyn Workspace> {
    Box::new(WorkspaceServer::new())
}
