use crate::errors::WorkspaceError;
use crate::types::*;
use tscanner_rules::RuleMetadata;
use tscanner_types::{ContentScanResult, FileResult, ScanResult};

pub trait Workspace: Send + Sync {
    fn open_project(&self, params: OpenProjectParams) -> Result<(), WorkspaceError>;
    fn close_project(&self) -> Result<(), WorkspaceError>;

    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError>;
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError>;
    fn change_file(&self, params: ChangeFileParams) -> Result<(), WorkspaceError>;

    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError>;

    fn scan(&self, params: ScanParams) -> Result<ScanResult, WorkspaceError>;
    fn scan_file(&self, params: ScanFileParams) -> Result<FileResult, WorkspaceError>;
    fn scan_content(&self, params: ScanContentParams) -> Result<ContentScanResult, WorkspaceError>;

    fn get_rules_metadata(&self) -> Result<Vec<RuleMetadata>, WorkspaceError>;
    fn clear_cache(&self) -> Result<(), WorkspaceError>;
}
