use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use tscanner_types::Issue;

pub type ChangedLinesMap = HashMap<PathBuf, HashSet<usize>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRuleStatus {
    Pending {},
    Running {},
    Completed { issues_found: usize },
    Failed { error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProgressEvent {
    pub rule_name: String,
    pub rule_index: usize,
    pub total_rules: usize,
    pub status: AiRuleStatus,
}

pub type AiProgressCallback = Arc<dyn Fn(AiProgressEvent) + Send + Sync>;
pub type RegularRulesCompleteCallback = Arc<dyn Fn(u128) + Send + Sync>;

#[derive(Debug, Clone)]
pub struct PreviousAiIssue {
    pub rule: String,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub line_text: Option<String>,
}

pub(super) struct AiRuleExecutionContext<'a> {
    pub files: &'a [&'a (PathBuf, String)],
    pub workspace_root: &'a std::path::Path,
    pub ai_config: &'a tscanner_config::AiConfig,
    pub changed_lines: Option<&'a ChangedLinesMap>,
    pub previous_issues: &'a [PreviousAiIssue],
}

#[derive(Debug, Deserialize)]
pub(super) struct AiResponse {
    pub(super) issues: Vec<AiIssue>,
}

#[derive(Debug, Deserialize)]
pub(super) struct AiIssue {
    pub(super) file: String,
    pub(super) line: usize,
    #[serde(default)]
    pub(super) column: usize,
    pub(super) message: String,
}

#[derive(Debug)]
pub enum AiError {
    IoError(std::io::Error),
    Timeout(u64),
    NonZeroExit { code: Option<i32>, stderr: String },
    InvalidOutput(String),
    ProviderNotFound(String),
    PromptNotFound(PathBuf),
}

impl std::fmt::Display for AiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiError::IoError(e) => write!(f, "IO error: {}", e),
            AiError::Timeout(secs) => write!(f, "AI call timed out after {}s", secs),
            AiError::NonZeroExit { code, stderr } => {
                write!(f, "AI command exited with code {:?}: {}", code, stderr)
            }
            AiError::InvalidOutput(msg) => write!(f, "Invalid AI output: {}", msg),
            AiError::ProviderNotFound(cmd) => {
                write!(
                    f,
                    "AI provider '{}' not found. Install it or check PATH",
                    cmd
                )
            }
            AiError::PromptNotFound(path) => write!(f, "Prompt file not found: {:?}", path),
        }
    }
}

impl From<std::io::Error> for AiError {
    fn from(e: std::io::Error) -> Self {
        AiError::IoError(e)
    }
}

#[derive(Debug, Default)]
pub struct AiExecutionResult {
    pub issues: Vec<Issue>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub cache_hits: usize,
}
