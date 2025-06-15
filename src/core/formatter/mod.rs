// src/core/formatter/mod.rs
//! Output formatting functionality.
//!
//! This module provides various formatters for converting `NodeInfo` collections
//! into human-readable output formats. Each formatter implements the `TreeFormatter`
//! trait to ensure a consistent interface.
//!
//! # Available Formatters
//!
//! - [`TextTreeFormatter`] - ASCII tree-style output (similar to the `tree` command)
//! - [`MarkdownFormatter`] - Nested Markdown list output
//!
//! # Examples
//!
//! ```rust
//! # use rustree::core::formatter::{TextTreeFormatter, TreeFormatter};
//! # use rustree::{RustreeLibConfig, NodeInfo};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let formatter = TextTreeFormatter;
//! let config = RustreeLibConfig::default();
//! let nodes: Vec<NodeInfo> = vec![];
//! let output = formatter.format(&nodes, &config)?;
//! # Ok(())
//! # }
//! ```

pub mod base;
pub mod html;
pub mod json;
pub mod markdown;
pub mod text_tree;

// Re-export the OutputFormat from config for convenience
pub use crate::core::options::OutputFormat;

// Re-export the core types for external use
pub use base::TreeFormatter;
pub use html::HtmlFormatter;
pub use json::JsonFormatter;
pub use markdown::MarkdownFormatter;
pub use text_tree::TextTreeFormatter;
