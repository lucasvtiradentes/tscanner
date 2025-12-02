use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("No project is open")]
    NoProjectOpen,

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("File not open: {0}")]
    FileNotOpen(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Scan error: {0}")]
    ScanError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}
