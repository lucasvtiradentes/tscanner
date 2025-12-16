use crate::enums::AiExecutionMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanParams {
    pub root: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_mode: Option<AiExecutionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanFileParams {
    pub root: String,
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanContentParams {
    pub root: String,
    pub file: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncommitted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifiedLineRange {
    pub start_line: usize,
    pub line_count: usize,
}
