use lsp_types::notification::Notification;
use lsp_types::request::Request;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tscanner_config::{AiExecutionMode, TscannerConfig};
use tscanner_diagnostics::{ContentScanResult, FileResult, ScanResult};
use tscanner_scanner::{AiProgressEvent, AiRuleStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanParams {
    pub root: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<TscannerConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai_mode: Option<AiExecutionMode>,
}

pub enum ScanRequest {}
impl Request for ScanRequest {
    type Params = ScanParams;
    type Result = ScanResult;
    const METHOD: &'static str = "tscanner/scan";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanFileParams {
    pub root: PathBuf,
    pub file: PathBuf,
}

pub enum ScanFileRequest {}
impl Request for ScanFileRequest {
    type Params = ScanFileParams;
    type Result = FileResult;
    const METHOD: &'static str = "tscanner/scanFile";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanContentParams {
    pub root: PathBuf,
    pub file: PathBuf,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<TscannerConfig>,
}

pub enum ScanContentRequest {}
impl Request for ScanContentRequest {
    type Params = ScanContentParams;
    type Result = ContentScanResult;
    const METHOD: &'static str = "tscanner/scanContent";
}

pub enum ClearCacheRequest {}
impl Request for ClearCacheRequest {
    type Params = ();
    type Result = ClearCacheResult;
    const METHOD: &'static str = "tscanner/clearCache";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearCacheResult {
    pub cleared: bool,
}

pub enum GetRulesMetadataRequest {}
impl Request for GetRulesMetadataRequest {
    type Params = ();
    type Result = serde_json::Value;
    const METHOD: &'static str = "tscanner/getRulesMetadata";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatResultsParams {
    pub root: PathBuf,
    pub results: ScanResult,
    pub group_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatPrettyResult {
    pub output: String,
    pub summary: FormatSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatSummary {
    pub total_issues: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub hint_count: usize,
    pub file_count: usize,
    pub rule_count: usize,
}

pub enum FormatResultsRequest {}
impl Request for FormatResultsRequest {
    type Params = FormatResultsParams;
    type Result = FormatPrettyResult;
    const METHOD: &'static str = "tscanner/formatResults";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProgressParams {
    pub rule_name: String,
    pub rule_index: usize,
    pub total_rules: usize,
    pub status: AiRuleStatus,
}

impl From<AiProgressEvent> for AiProgressParams {
    fn from(event: AiProgressEvent) -> Self {
        Self {
            rule_name: event.rule_name,
            rule_index: event.rule_index,
            total_rules: event.total_rules,
            status: event.status,
        }
    }
}

pub enum AiProgressNotification {}
impl Notification for AiProgressNotification {
    type Params = AiProgressParams;
    const METHOD: &'static str = "tscanner/aiProgress";
}
