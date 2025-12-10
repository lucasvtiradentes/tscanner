use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tscanner_config::TscannerConfig;
use tscanner_types::Issue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenProjectParams {
    pub root: PathBuf,
    pub config: Option<TscannerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenFileParams {
    pub path: PathBuf,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseFileParams {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeFileParams {
    pub path: PathBuf,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullDiagnosticsParams {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullDiagnosticsResult {
    pub diagnostics: Vec<Issue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanParams {
    pub root: PathBuf,
    pub config: Option<TscannerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanFileParams {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanContentParams {
    pub path: PathBuf,
    pub content: String,
}
