// src/lib.rs

//! `rustree` is a library for generating directory tree listings, similar to the `tree` command,
//! but with extended capabilities for file analysis and customizable output.
//!
//! It allows for scanning directories, collecting information about files and subdirectories,
//! performing analysis like line/word counts, applying custom functions to file contents,
//! sorting the results, and formatting them into various output formats (e.g., text tree, Markdown).
//!
//! # Key Features
//!
//! - Directory traversal with depth control and hidden file filtering.
//! - File metadata reporting (size, modification time).
//! - Content analysis (line count, word count).
//! - Extensible via custom function application on file contents.
//! - Sorting of tree entries by various keys (name, size, mtime, etc.).
//! - Multiple output formats.
//!
//! # Examples
//!
//! ```no_run
//! use rustree::{get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat, SortKey};
//! use std::path::Path;
//!
//! fn main() -> Result<(), rustree::RustreeError> {
//!     let path = Path::new(".");
//!     let mut config = RustreeLibConfig {
//!         root_display_name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
//!         max_depth: Some(2),
//!         show_hidden: false,
//!         report_sizes: true,
//!         report_mtime: true,
//!         sort_by: Some(SortKey::Name),
//!         ..Default::default()
//!     };
//!
//!     // Get the processed nodes
//!     let nodes = get_tree_nodes(path, &config)?;
//!
//!     // Format the nodes into a string
//!     let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
//!     println!("{}", output);
//!
//!     Ok(())
//! }
//! ```

pub mod core;

// Re-export key types for the public API
pub use crate::core::node::{NodeInfo, NodeType};
pub use crate::core::config::RustreeLibConfig;
pub use crate::core::error::RustreeError;
pub use crate::core::sorter::SortKey;
pub use crate::core::analyzer::apply_fn::BuiltInFunction;
pub use crate::core::formatter::{
    base::TreeFormatter,
    text_tree::TextTreeFormatter,
    markdown::MarkdownFormatter,
    OutputFormat as LibOutputFormat
};

use std::path::Path;
use crate::core::{walker, sorter}; // Internal use

/// Walks the directory, analyzes files, and sorts them based on the provided configuration.
///
/// This is the main entry point for gathering information about a directory structure.
/// It performs the following steps:
/// 1. Traverses the directory structure starting from `root_path` according to `config` settings
///    (e.g., `max_depth`, `show_hidden`).
/// 2. Collects metadata and performs analysis (e.g., size, line count) for each file and directory.
/// 3. If a sort key is specified in `config`, sorts the collected nodes.
///
/// # Arguments
///
/// * `root_path` - The starting path for directory traversal.
/// * `config` - Configuration options that control traversal, analysis, and sorting.
///
/// # Returns
///
/// A `Result` containing a `Vec<NodeInfo>` on success, representing the processed
/// directory entries, or a `RustreeError` on failure.
///
/// # See Also
///
/// * [`NodeInfo`] - For the structure of information collected for each entry.
/// * [`RustreeLibConfig`] - For configuration options.
/// * [`format_nodes`] - For formatting the output of this function.
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
///
/// This function takes the processed nodes (typically from [`get_tree_nodes`]) and
/// renders them into a human-readable string representation based on the chosen
/// output format and configuration.
///
/// # Arguments
///
/// * `nodes` - A slice of `NodeInfo` objects to format.
/// * `format` - The desired output format (e.g., text tree, Markdown).
/// * `config` - The library configuration, which may influence formatting details
///   (e.g., which metadata to display).
///
/// # Returns
///
/// A `Result` containing the formatted `String` on success, or a `RustreeError` on failure.
///
/// # See Also
///
/// * [`LibOutputFormat`] - For available output formats.
/// * [`TextTreeFormatter`] - For the default text tree formatter.
/// * [`MarkdownFormatter`] - For Markdown list formatting.
pub fn format_nodes(
    nodes: &[NodeInfo],
    format: LibOutputFormat,
    config: &RustreeLibConfig,
) -> Result<String, RustreeError> {
    let formatter_instance: Box<dyn TreeFormatter> = match format {
        LibOutputFormat::Text => Box::new(TextTreeFormatter),
        LibOutputFormat::Markdown => Box::new(MarkdownFormatter),
    };
    formatter_instance.format(nodes, config) // Pass config to formatter
}