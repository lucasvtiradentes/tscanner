use crate::server::WorkspaceServer;
use crate::workspace::Workspace;

pub fn create_workspace() -> Box<dyn Workspace> {
    Box::new(WorkspaceServer::new())
}
