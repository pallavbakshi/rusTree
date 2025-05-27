// src/core/formatter/mod.rs
pub mod base;
pub mod text_tree;
pub mod markdown;

/// Enumerates the available output formats for the `rustree` library.
/// This is used by the library internally and re-exported as `LibOutputFormat`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    /// Plain text, tree-like structure.
    Text,
    /// Markdown list format.
    Markdown,
}