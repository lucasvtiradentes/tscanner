use lsp_types::{Diagnostic, Url};
use std::collections::HashMap;
use std::path::PathBuf;
use tscanner_service::{OpenProjectParams, Workspace, WorkspaceServer};

pub struct Session {
    workspace: WorkspaceServer,
    root: Option<PathBuf>,
    pub open_files: HashMap<Url, String>,
    pub diagnostics: HashMap<Url, Vec<(Diagnostic, String)>>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            workspace: WorkspaceServer::new(),
            root: None,
            open_files: HashMap::new(),
            diagnostics: HashMap::new(),
        }
    }

    pub fn workspace(&self) -> &WorkspaceServer {
        &self.workspace
    }

    pub fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root.clone());
        let _ = self
            .workspace
            .open_project(OpenProjectParams { root, config: None });
    }

    pub fn is_initialized(&self) -> bool {
        self.root.is_some()
    }

    pub fn reload_config(&mut self) -> Result<(), String> {
        if let Some(root) = &self.root {
            let _ = self.workspace.open_project(OpenProjectParams {
                root: root.clone(),
                config: None,
            });
            Ok(())
        } else {
            Err("No workspace root set".to_string())
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
