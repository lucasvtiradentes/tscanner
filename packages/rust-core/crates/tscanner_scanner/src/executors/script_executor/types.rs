use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ScriptFile {
    pub path: String,
    pub content: String,
    pub lines: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptInput {
    pub files: Vec<ScriptFile>,
    pub options: Option<serde_json::Value>,
    pub workspace_root: String,
}

#[derive(Debug, Deserialize)]
pub struct ScriptIssue {
    pub file: String,
    pub line: usize,
    #[serde(default)]
    pub column: usize,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ScriptOutput {
    pub issues: Vec<ScriptIssue>,
}

#[derive(Debug)]
pub enum ScriptError {
    IoError(std::io::Error),
    Timeout(u64),
    NonZeroExit { code: Option<i32>, stderr: String },
    InvalidOutput(String),
    RunnerNotFound(String),
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptError::IoError(e) => write!(f, "IO error: {}", e),
            ScriptError::Timeout(secs) => write!(f, "Command timed out after {}s", secs),
            ScriptError::NonZeroExit { code, stderr } => {
                write!(f, "Command exited with code {:?}: {}", code, stderr)
            }
            ScriptError::InvalidOutput(msg) => write!(f, "Invalid output: {}", msg),
            ScriptError::RunnerNotFound(cmd) => {
                write!(f, "Command '{}' not found", cmd)
            }
        }
    }
}

impl From<std::io::Error> for ScriptError {
    fn from(e: std::io::Error) -> Self {
        ScriptError::IoError(e)
    }
}
