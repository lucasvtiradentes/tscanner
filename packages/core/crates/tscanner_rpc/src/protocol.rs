use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tscanner_config::TscannerConfig;
use tscanner_diagnostics::ScanResult;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: u64,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl Response {
    pub fn success(id: u64, result: impl Serialize) -> Self {
        Self {
            id,
            result: Some(serde_json::to_value(result).unwrap()),
            error: None,
        }
    }

    pub fn error(id: u64, message: String) -> Self {
        Self {
            id,
            result: None,
            error: Some(message),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Notification {
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ScanParams {
    pub root: PathBuf,
    pub config: Option<TscannerConfig>,
    pub branch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WatchParams {
    pub root: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct ScanFileParams {
    pub root: PathBuf,
    pub file: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct ScanContentParams {
    pub root: PathBuf,
    pub file: PathBuf,
    pub content: String,
    pub config: Option<TscannerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct FormatResultsParams {
    pub root: PathBuf,
    pub results: ScanResult,
    pub group_mode: String,
}
