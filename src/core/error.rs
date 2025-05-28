// src/core/error.rs
use thiserror::Error;

/// Represents errors that can occur within the `rustree` library.
#[derive(Error, Debug)]
pub enum RustreeError {
    /// An error related to file system I/O operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// An error originating from the `walkdir` crate during directory traversal.
    #[error("WalkDir error: {0}")]
    WalkDir(#[from] walkdir::Error),
    /// An error related to formatting output.
    #[error("Formatting error: {0}")]
    Fmt(#[from] std::fmt::Error),
    /// An error related to glob pattern compilation or matching.
    #[error("Glob pattern error: {0}")]
    GlobPattern(#[from] glob::PatternError),
    // Add other specific error types as needed
    /// An unspecified or unknown error.
    #[error("Unknown error")]
    Unknown,
}