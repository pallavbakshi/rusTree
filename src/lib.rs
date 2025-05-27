// src/lib.rs
pub mod core;

// Re-export key types for the public API
pub use crate::core::node::{NodeInfo, NodeType};
pub use crate::core::config::RustreeLibConfig;
pub use crate::core::error::RustreeError;
pub use crate::core::sorter::SortKey; // And other enums needed by RustreeLibConfig
pub use crate::core::analyzer::apply_fn::BuiltInFunction; // Enum for built-in functions
pub use crate::core::formatter::{
    base::TreeFormatter, // Trait
    text_tree::TextTreeFormatter,
    markdown::MarkdownFormatter,
    OutputFormat as LibOutputFormat // Enum for output formats the lib can produce
};

use std::path::Path;
use crate::core::{walker, sorter}; // Internal use

/// Core function: Walks the directory, analyzes files, and sorts them based on config.
pub fn get_tree_nodes(
    root_path: &Path,
    config: &RustreeLibConfig,
) -> Result<Vec<NodeInfo>, RustreeError> {
    // 1. Walk and analyze (analyzer is called within walker)
    let mut nodes = walker::walk_directory(root_path, config)?;

    // 2. Sort if requested in config
    if let Some(sort_key) = &config.sort_by {
        sorter::sort_nodes(&mut nodes, sort_key, config.reverse_sort);
    }
    Ok(nodes)
}

/// Formats a slice of `NodeInfo` objects into a string using the specified format.
pub fn format_nodes(
    nodes: &[NodeInfo],
    format: LibOutputFormat,
    // config might be needed for formatter to know which details to include
    config: &RustreeLibConfig,
) -> Result<String, RustreeError> {
    let formatter_instance: Box<dyn TreeFormatter> = match format {
        LibOutputFormat::Text => Box::new(TextTreeFormatter),
        LibOutputFormat::Markdown => Box::new(MarkdownFormatter),
    };
    formatter_instance.format(nodes, config) // Pass config to formatter
}