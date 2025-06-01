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
    /// An error originating from the `ignore` crate during directory traversal or gitignore processing.
    #[error("Ignore crate error: {0}")]
    IgnoreError(#[from] ignore::Error),
    /// An error during the construction of the internal tree representation.
    #[error("Tree building error: {0}")]
    TreeBuildError(String),
    // Add other specific error types as needed
    /// An unspecified or unknown error.
    #[error("Unknown error")]
    Unknown,
}
