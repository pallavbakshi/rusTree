// src/core/formatter/mod.rs
pub mod base;
pub mod markdown;
pub mod text_tree;

// Re-export the OutputFormat from config
pub use crate::config::output_format::OutputFormat;
