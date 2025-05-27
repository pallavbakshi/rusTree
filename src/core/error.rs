// src/core/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustreeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("WalkDir error: {0}")]
    WalkDir(#[from] walkdir::Error),
    #[error("Formatting error: {0}")]
    Fmt(#[from] std::fmt::Error),
    // Add other specific error types as needed
    #[error("Unknown error")]
    Unknown,
}