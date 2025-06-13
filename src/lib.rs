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
    base::TreeFormatter, json::JsonFormatter, markdown::MarkdownFormatter,
    text_tree::TextTreeFormatter,
};

// Internal imports
use crate::core::{metadata::file_info, sorter, tree::builder::TempNode, walker};
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

    // 1b. Apply size-based file filtering prior to any tree manipulations
    if config.filtering.min_file_size.is_some() || config.filtering.max_file_size.is_some() {
        let min_opt = config.filtering.min_file_size;
        let max_opt = config.filtering.max_file_size;

        nodes.retain(|node| {
            // Apply filter only to regular files; always keep directories and symlinks.
            if node.node_type != NodeType::File {
                return true;
            }

            // If the walker did not collect size information (either due to I/O
            // error or because `show_size_bytes` was disabled), `node.size` will
            // be `None`.  In that case we keep the file instead of treating the
            // size as 0, otherwise legitimate files could be filtered out when a
            // minimum size constraint is specified.

            match node.size {
                None => true, // unknown size – keep the entry
                Some(size) => {
                    if let Some(min) = min_opt {
                        if size < min {
                            return false;
                        }
                    }

                    if let Some(max) = max_opt {
                        if size > max {
                            return false;
                        }
                    }

                    true
                }
            }
        });
    }

    // 2. Apply directory functions if needed or prune empty directories if requested
    if ((config.metadata.apply_function.is_some() && needs_directory_function_processing(config))
        || config.filtering.prune_empty_directories)
        && !nodes.is_empty()
    {
        // Build the tree structure from the flat list of nodes
        let mut temp_roots = core::tree::builder::build_tree(std::mem::take(&mut nodes))
            .map_err(RustreeError::TreeBuildError)?;

        // Apply directory functions if configured
        if let Some(apply_func) = &config.metadata.apply_function {
            if is_directory_function(apply_func) {
                apply_directory_functions_to_tree(&mut temp_roots, apply_func, config);
            }
        }

        // Prune empty directories if requested
        if config.filtering.prune_empty_directories {
            // Define the filter for pruning: keep only files.
            // TreeManipulator::prune_tree will then keep directories that (recursively) contain files.
            let prune_filter = |node_info: &NodeInfo| node_info.node_type == NodeType::File;

            // Apply prune_tree to each root. Retain roots that are not empty after pruning.
            temp_roots.retain_mut(|root_node| {
                core::tree::manipulator::TreeManipulator::prune_tree(root_node, &prune_filter)
            });
        }

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
        LibOutputFormat::Json => Box::new(core::formatter::JsonFormatter),
    };
    let tree_output = formatter_instance.format(nodes, config)?;

    let mut is_cat_like = false;
    if config.metadata.apply_function == Some(BuiltInFunction::Cat) {
        is_cat_like = true;
    } else if let Some(ext_fn) = &config.metadata.external_function {
        if matches!(
            ext_fn.kind,
            crate::config::metadata::FunctionOutputKind::Text
        ) {
            is_cat_like = true;
        }
    }

    if is_cat_like && !matches!(format, LibOutputFormat::Json) {
        let mut result = tree_output;

        // Only show file contents section if there are files with content
        let file_nodes_with_content: Vec<_> = nodes
            .iter()
            .filter(|node| {
                node.node_type == NodeType::File
                    && node.custom_function_output.is_some()
                    && matches!(node.custom_function_output, Some(Ok(_)))
            })
            .collect();

        if !file_nodes_with_content.is_empty() {
            // Determine section header text
            let header = if config.metadata.apply_function == Some(BuiltInFunction::Cat) {
                "File Contents".to_string()
            } else if let Some(ext_fn) = &config.metadata.external_function {
                format!(
                    "Results of applying '{}' to relevant files",
                    ext_fn.cmd_template
                )
            } else {
                "Results".to_string()
            };

            result.push_str(&format!("\n\n--- {} ---\n", header));

            for node in file_nodes_with_content {
                if let Some(Ok(content)) = &node.custom_function_output {
                    result.push_str(&format!("\n=== {} ===\n", node.path.display()));
                    result.push_str(content);
                    result.push('\n');
                }
            }
        }
        Ok(result)
    } else {
        Ok(tree_output)
    }
}

/// Checks if the current configuration needs directory function processing.
fn needs_directory_function_processing(config: &RustreeLibConfig) -> bool {
    if let Some(func) = &config.metadata.apply_function {
        is_directory_function(func)
    } else {
        false
    }
}

/// Checks if a function is a directory-specific function.
fn is_directory_function(func: &BuiltInFunction) -> bool {
    matches!(
        func,
        BuiltInFunction::CountFiles
            | BuiltInFunction::CountDirs
            | BuiltInFunction::SizeTotal
            | BuiltInFunction::DirStats
    )
}

/// Recursively applies directory functions to all directories in the tree.
fn apply_directory_functions_to_tree(
    roots: &mut [TempNode],
    func: &BuiltInFunction,
    config: &RustreeLibConfig,
) {
    for root in roots {
        apply_directory_functions_to_node(root, func, config);
    }
}

/// Recursively applies directory functions to a single node and its children.
fn apply_directory_functions_to_node(
    node: &mut TempNode,
    func: &BuiltInFunction,
    config: &RustreeLibConfig,
) {
    // First, recursively process all children
    for child in &mut node.children {
        apply_directory_functions_to_node(child, func, config);
    }

    // Then process this node if it's a directory and should have the function applied
    if node.node_info.node_type == NodeType::Directory
        && should_apply_function_to_node(&node.node_info, config)
    {
        // Collect child NodeInfo objects for the function
        let child_infos: Vec<NodeInfo> = node
            .children
            .iter()
            .map(|child| child.node_info.clone())
            .collect();

        // Apply the directory function
        let result = file_info::apply_builtin_to_directory(&child_infos, func);
        node.node_info.custom_function_output = Some(result);
    }
}

/// Checks if a function should be applied to a specific node based on filtering patterns.
fn should_apply_function_to_node(node: &NodeInfo, config: &RustreeLibConfig) -> bool {
    use crate::core::filter::pattern::{compile_glob_patterns, entry_matches_path_with_patterns};

    // Check apply_exclude_patterns first - if it matches, skip
    if let Some(exclude_patterns) = &config.filtering.apply_exclude_patterns {
        if !exclude_patterns.is_empty() {
            if let Ok(Some(patterns)) = compile_glob_patterns(
                &Some(exclude_patterns.clone()),
                config.filtering.case_insensitive_filter,
                config.listing.show_hidden,
            ) {
                if entry_matches_path_with_patterns(&node.path, &patterns) {
                    return false; // Skip this node
                }
            }
        }
    }

    // Check apply_include_patterns - if specified, node must match
    if let Some(include_patterns) = &config.filtering.apply_include_patterns {
        if !include_patterns.is_empty() {
            if let Ok(Some(patterns)) = compile_glob_patterns(
                &Some(include_patterns.clone()),
                config.filtering.case_insensitive_filter,
                config.listing.show_hidden,
            ) {
                return entry_matches_path_with_patterns(&node.path, &patterns);
            }
            // If we have include patterns but compilation failed, don't apply
            return false;
        }
    }

    // If no include patterns specified, or node passed all checks, apply the function
    true
}
