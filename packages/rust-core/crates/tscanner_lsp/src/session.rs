use lsp_types::{Diagnostic, Url};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tscanner_cache::FileCache;
use tscanner_constants::{config_dir_name, config_file_name};
use tscanner_scanner::{load_config, Scanner};
use tscanner_service::{OpenProjectParams, Workspace, WorkspaceServer};

#[derive(Clone)]
pub struct OpenDocument {
    pub content: String,
    pub version: i32,
}

pub struct Session {
    workspace: Arc<Mutex<WorkspaceServer>>,
    root: Option<PathBuf>,
    pub open_files: HashMap<Url, OpenDocument>,
    pub diagnostics: HashMap<Url, Vec<(Diagnostic, String)>>,
    pub scanner: Option<Scanner>,
    pub cache: Arc<FileCache>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            workspace: Arc::new(Mutex::new(WorkspaceServer::new())),
            root: None,
            open_files: HashMap::new(),
            diagnostics: HashMap::new(),
            scanner: None,
            cache: Arc::new(FileCache::new()),
        }
    }

    pub fn workspace(&self) -> Arc<Mutex<WorkspaceServer>> {
        self.workspace.clone()
    }

    pub fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root.clone());
        let config_result = load_config(&root, config_dir_name(), config_file_name());
        let config = config_result.ok();
        let ws = self.workspace.lock().unwrap();
        let _ = ws.open_project(OpenProjectParams {
            root,
            config,
            config_dir: None,
        });
    }

    pub fn is_initialized(&self) -> bool {
        self.root.is_some()
    }

    pub fn reload_config(&mut self) -> Result<(), String> {
        if let Some(root) = &self.root {
            let config = load_config(root, config_dir_name(), config_file_name()).ok();
            let ws = self.workspace.lock().unwrap();
            let _ = ws.open_project(OpenProjectParams {
                root: root.clone(),
                config,
                config_dir: None,
            });
            Ok(())
        } else {
            Err("No workspace root set".to_string())
        }
    }

    pub fn open_document(&mut self, uri: &Url, content: String, version: i32) {
        self.open_files
            .insert(uri.clone(), OpenDocument { content, version });
    }

    pub fn update_document(&mut self, uri: &Url, content: String, version: i32) {
        if let Some(doc) = self.open_files.get_mut(uri) {
            doc.content = content;
            doc.version = version;
        } else {
            self.open_files
                .insert(uri.clone(), OpenDocument { content, version });
        }
    }

    pub fn close_document(&mut self, uri: &Url) {
        self.open_files.remove(uri);
        self.diagnostics.remove(uri);
    }

    pub fn get_document(&self, uri: &Url) -> Option<&OpenDocument> {
        self.open_files.get(uri)
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
