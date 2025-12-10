mod errors;
mod server;
mod types;
mod workspace;

pub use errors::WorkspaceError;
pub use server::WorkspaceServer;
pub use tscanner_logger::{get_logger, init_logger, log_debug, log_error, log_info, log_warn};
pub use types::*;
pub use workspace::Workspace;

pub fn create_workspace() -> Box<dyn Workspace> {
    Box::new(WorkspaceServer::new())
}
