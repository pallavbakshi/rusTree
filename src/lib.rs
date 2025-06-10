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
//! # Configuration
//!
//! The library uses a hierarchical configuration structure through [`RustreeLibConfig`],
//! which is composed of several specialized option groups:
//!
//! - [`InputSourceOptions`] - Controls how the root path is displayed
//! - [`ListingOptions`] - Controls directory traversal (depth, hidden files, etc.)
//! - [`FilteringOptions`] - Controls which files/directories to include/exclude
//! - [`SortingOptions`] - Controls sorting behavior
//! - [`MetadataOptions`] - Controls what metadata to collect and display
//! - [`MiscOptions`] - Additional miscellaneous options
//!
//! # Examples
//!
//! ```no_run
//! use rustree::{get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat};
//! use rustree::{InputSourceOptions, ListingOptions, MetadataOptions, SortingOptions, SortKey};
//! use std::path::Path;
//!
//! fn main() -> Result<(), rustree::RustreeError> {
//!     let path = Path::new(".");
//!     let config = RustreeLibConfig {
//!         input_source: InputSourceOptions {
//!             root_display_name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
//!             root_is_directory: true,
//!             ..Default::default()
//!         },
//!         listing: ListingOptions {
//!             max_depth: Some(2),
//!             show_hidden: false,
//!             ..Default::default()
//!         },
//!         metadata: MetadataOptions {
//!             show_size_bytes: true,
//!             show_last_modified: true,
//!             ..Default::default()
//!         },
//!         sorting: SortingOptions {
//!             sort_by: Some(SortKey::Name),
//!             ..Default::default()
//!         },
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

// CLI module - WARNING: This is not part of the stable public API!
// This module is only exposed publicly because the main binary needs access to it.
// External library users should NOT depend on this module as it may change
// without notice in future versions. Use the public API functions like
// get_tree_nodes() and format_nodes() instead.
#[doc(hidden)] // Hide from documentation
pub mod cli;

pub mod config;
pub mod core;

// Re-export key types for the public API

// Configuration types - organized by category
pub use crate::config::{
    // Enums and related types
    ApplyFnError,
    BuiltInFunction,
    // Configuration option groups
    FilteringOptions,
    InputSourceOptions,
    ListingOptions,
    MetadataOptions,
    MiscOptions,
    // Main config struct
    RustreeLibConfig,

    SortKey,
    SortingOptions,
};

// Output format
pub use crate::config::output_format::OutputFormat as LibOutputFormat;

// Core types for working with nodes
pub use crate::core::error::RustreeError;
pub use crate::core::tree::node::{NodeInfo, NodeType};

// Formatter types (for advanced usage)
pub use crate::core::formatter::{
    base::TreeFormatter, markdown::MarkdownFormatter, text_tree::TextTreeFormatter,
};

// Internal imports
use crate::core::{sorter, walker};
use std::path::Path;

/// Walks the directory, analyzes files, and sorts them based on the provided configuration.
///
/// This is the main entry point for gathering information about a directory structure.
/// It performs the following steps:
/// 1. Traverses the directory structure starting from `root_path` according to `config.listing` settings
///    (e.g., `config.listing.max_depth`, `config.listing.show_hidden`).
/// 2. Collects metadata and performs analysis (e.g., `config.metadata.show_size_bytes`,
///    `config.metadata.calculate_line_count`) for each file and directory.
/// 3. If a sort key is specified in `config.sorting.sort_by`, sorts the collected nodes.
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

    // 2. Prune empty directories if requested
    if config.filtering.prune_empty_directories && !nodes.is_empty() {
        // Build the tree structure from the flat list of nodes
        let mut temp_roots = core::tree::builder::build_tree(std::mem::take(&mut nodes))
            .map_err(RustreeError::TreeBuildError)?;

        // Define the filter for pruning: keep only files.
        // TreeManipulator::prune_tree will then keep directories that (recursively) contain files.
        let prune_filter = |node_info: &NodeInfo| node_info.node_type == NodeType::File;

        // Apply prune_tree to each root. Retain roots that are not empty after pruning.
        temp_roots.retain_mut(|root_node| {
            core::tree::manipulator::TreeManipulator::prune_tree(root_node, &prune_filter)
        });

        // Flatten the modified tree back into a flat list of NodeInfo
        // `nodes` is empty at this point due to `std::mem::take`.
        core::tree::builder::flatten_tree_to_dfs_consuming(temp_roots, &mut nodes);
    }

    // 3. Apply list_directories_only filter if enabled
    // This happens *after* pruning, so pruning decisions are based on full content.
    if config.listing.list_directories_only {
        nodes.retain(|node| node.node_type == NodeType::Directory);
    }

    // 4. Sort if requested in config
    if config.sorting.sort_by.is_some() {
        // sort_nodes_with_options internally handles building tree from `nodes` for sorting
        if let Err(e) = sorter::strategies::sort_nodes_with_options(&mut nodes, &config.sorting) {
            return Err(RustreeError::TreeBuildError(format!(
                "Sorting failed: {}",
                e
            )));
        }
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
///   (e.g., which metadata to display via `config.metadata`).
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
