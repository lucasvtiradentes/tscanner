use crate::errors::WorkspaceError;
use crate::types::*;
use crate::workspace::Workspace;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;
use tscanner_cache::FileCache;
use tscanner_config::TscannerConfig;
use tscanner_constants::resolve_config_dir;
use tscanner_logger::{log_debug, log_info};
use tscanner_rules::{get_all_rule_metadata, RuleMetadata};
use tscanner_scanner::{ConfigExt, Scanner};
use tscanner_types::{ContentScanResult, FileResult, Issue, ScanResult};

struct ProjectState {
    root: PathBuf,
    config: TscannerConfig,
    scanner: Option<Scanner>,
}

struct OpenFile {
    path: PathBuf,
    content: String,
    diagnostics: Vec<Issue>,
}

pub struct WorkspaceServer {
    project: RwLock<Option<ProjectState>>,
    open_files: RwLock<Vec<OpenFile>>,
    cache: Arc<FileCache>,
}

impl WorkspaceServer {
    pub fn new() -> Self {
        Self {
            project: RwLock::new(None),
            open_files: RwLock::new(Vec::new()),
            cache: Arc::new(FileCache::new()),
        }
    }

    pub fn with_cache(cache: Arc<FileCache>) -> Self {
        Self {
            project: RwLock::new(None),
            open_files: RwLock::new(Vec::new()),
            cache,
        }
    }

    pub fn cache(&self) -> Arc<FileCache> {
        self.cache.clone()
    }

    pub fn get_config(&self) -> Option<TscannerConfig> {
        self.project.read().as_ref().map(|p| p.config.clone())
    }

    pub fn get_root(&self) -> Option<PathBuf> {
        self.project.read().as_ref().map(|p| p.root.clone())
    }
}

impl Default for WorkspaceServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Workspace for WorkspaceServer {
    fn open_project(&self, params: OpenProjectParams) -> Result<(), WorkspaceError> {
        let config = params
            .config
            .ok_or_else(|| WorkspaceError::ConfigError("Config is required".to_string()))?;
        log_info(&format!(
            "open_project: {} (builtin rules: {})",
            params.root.display(),
            config.rules.builtin.len()
        ));

        let resolved_config_dir = resolve_config_dir(&params.root, params.config_dir);

        let config_hash = config.compute_hash();
        let ai_cache = Arc::new(tscanner_cache::AiCache::with_config_hash(config_hash));
        let script_cache = Arc::new(tscanner_cache::ScriptCache::with_config_hash(config_hash));

        let scanner_result = Scanner::with_caches_and_config_dir(
            config.clone(),
            self.cache.clone(),
            ai_cache,
            script_cache,
            params.root.clone(),
            resolved_config_dir,
        );
        let scanner = match scanner_result {
            Ok(s) => {
                log_debug("Scanner created successfully");
                Some(s)
            }
            Err(e) => {
                log_info(&format!("Scanner creation failed: {:?}", e));
                None
            }
        };

        *self.project.write() = Some(ProjectState {
            root: params.root,
            config,
            scanner,
        });

        Ok(())
    }

    fn close_project(&self) -> Result<(), WorkspaceError> {
        *self.project.write() = None;
        self.open_files.write().clear();
        Ok(())
    }

    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        let content = match params.content {
            Some(c) => c,
            None => std::fs::read_to_string(&params.path)
                .map_err(|_| WorkspaceError::FileNotFound(params.path.display().to_string()))?,
        };

        let mut files = self.open_files.write();
        files.retain(|f| f.path != params.path);
        files.push(OpenFile {
            path: params.path,
            content,
            diagnostics: Vec::new(),
        });

        Ok(())
    }

    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.open_files.write().retain(|f| f.path != params.path);
        Ok(())
    }

    fn change_file(&self, params: ChangeFileParams) -> Result<(), WorkspaceError> {
        let mut files = self.open_files.write();
        if let Some(file) = files.iter_mut().find(|f| f.path == params.path) {
            file.content = params.content;
            file.diagnostics.clear();
            Ok(())
        } else {
            Err(WorkspaceError::FileNotOpen(
                params.path.display().to_string(),
            ))
        }
    }

    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        let _project = self.project.read();
        let _project = _project.as_ref().ok_or(WorkspaceError::NoProjectOpen)?;

        let files = self.open_files.read();
        let file = files
            .iter()
            .find(|f| f.path == params.path)
            .ok_or_else(|| WorkspaceError::FileNotOpen(params.path.display().to_string()))?;

        Ok(PullDiagnosticsResult {
            diagnostics: file.diagnostics.clone(),
        })
    }

    fn scan(&self, _params: ScanParams) -> Result<ScanResult, WorkspaceError> {
        Err(WorkspaceError::Internal(
            "scan() not yet implemented - use core::Scanner directly".to_string(),
        ))
    }

    fn scan_file(&self, _params: ScanFileParams) -> Result<FileResult, WorkspaceError> {
        Err(WorkspaceError::Internal(
            "scan_file() not yet implemented - use core::Scanner directly".to_string(),
        ))
    }

    fn scan_content(&self, params: ScanContentParams) -> Result<ContentScanResult, WorkspaceError> {
        log_debug(&format!(
            "scan_content: {} ({} bytes)",
            params.path.display(),
            params.content.len()
        ));

        let project = self.project.read();
        let project = project.as_ref().ok_or(WorkspaceError::NoProjectOpen)?;

        let scanner = project
            .scanner
            .as_ref()
            .ok_or_else(|| WorkspaceError::Internal("Scanner not initialized".to_string()))?;

        let result = scanner.scan_content(&params.path, &params.content);
        log_debug(&format!(
            "scan_content result: {:?} issues",
            result.as_ref().map(|r| r.issues.len())
        ));

        Ok(result.unwrap_or_else(|| ContentScanResult {
            file: params.path,
            issues: Vec::new(),
            related_files: Vec::new(),
        }))
    }

    fn get_rules_metadata(&self) -> Result<Vec<RuleMetadata>, WorkspaceError> {
        Ok(get_all_rule_metadata())
    }

    fn clear_cache(&self) -> Result<(), WorkspaceError> {
        self.cache.clear();
        Ok(())
    }
}
