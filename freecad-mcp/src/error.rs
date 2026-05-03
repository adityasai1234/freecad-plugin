use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FreeCADError {
    #[error("FreeCAD subprocess failed: {0}")]
    SubprocessFailed(String),

    #[error("Failed to parse FreeCAD output: {0}")]
    ParseError(String),

    #[error("FreeCAD subprocess timed out")]
    Timeout,

    #[error("FreeCAD binary not found at {0}")]
    BinaryNotFound(PathBuf),

    #[error("Object not found: {0}")]
    ObjectNotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
