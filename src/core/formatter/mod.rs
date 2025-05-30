// src/core/formatter/mod.rs
pub mod base;
pub mod text_tree;
pub mod markdown;

// Re-export the OutputFormat from config
pub use crate::config::output::OutputFormat;