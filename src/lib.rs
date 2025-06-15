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
//! - Context-based APIs for better modularity and GUI support.
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
//! # API Overview
//!
//! The library provides two API styles to suit different use cases:
//!
//! ## Traditional API (Backward Compatible)
//!
//! Simple configuration-based API ideal for CLI applications:
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
//!
//! ## Context-Based API (Advanced)
//!
//! Modular context-based API ideal for GUI applications and advanced integrations:
//!
//! ```no_run
//! use rustree::{get_tree_nodes_with_context, format_nodes_with_context, RustreeLibConfig, LibOutputFormat};
//! use std::path::Path;
//!
//! fn main() -> Result<(), rustree::RustreeError> {
//!     let config = RustreeLibConfig::default();
//!     let path = Path::new(".");
//!     
//!     // Create processing context for tree operations
//!     let processing_ctx = config.processing_context();
//!     let nodes = get_tree_nodes_with_context(path, &processing_ctx)?;
//!     
//!     // Create formatting context for output
//!     let formatting_ctx = config.formatting_context();
//!     let output = format_nodes_with_context(&nodes, LibOutputFormat::Text, &formatting_ctx)?;
//!     println!("{}", output);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Owned Context API (GUI-Friendly)
//!
//! For applications that need to modify contexts independently:
//!
//! ```no_run
//! use rustree::{walk_path_owned, RustreeLibConfig};
//! use std::path::Path;
//!
//! fn main() -> Result<(), rustree::RustreeError> {
//!     let config = RustreeLibConfig::default();
//!     let mut walking_ctx = config.to_owned_walking_context();
//!     
//!     // Modify context independently (e.g., from GUI controls)
//!     walking_ctx.listing.max_depth = Some(5);
//!     walking_ctx.listing.show_hidden = true;
//!     
//!     // Use optimized owned context with pattern caching
//!     let nodes = walk_path_owned(Path::new("."), &mut walking_ctx)?;
//!     println!("Found {} nodes", nodes.len());
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
    HtmlOptions,
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
pub use crate::core::input::InputFormat;
pub use crate::core::tree::node::{NodeInfo, NodeType};

// Diff functionality
pub use crate::core::diff::changes::{DiffMetadata, DiffOptions};
pub use crate::core::diff::{Change, ChangeType, DiffEngine, DiffResult, DiffSummary};

// Formatter types (for advanced usage)
pub use crate::core::formatter::{
    base::{TreeFormatter, TreeFormatterCompat},
    json::JsonFormatter,
    markdown::MarkdownFormatter,
    text_tree::TextTreeFormatter,
};

// Context types for advanced users and GUI applications
pub use crate::core::options::contexts::{
    // Async/thread-safe contexts for multi-threaded applications
    AsyncFormattingContext,
    AsyncProcessingContext,
    AsyncSortingContext,
    AsyncWalkingContext,

    // Diff functionality for GUI state management
    ContextDiff,
    // Error handling with context-aware messages
    ContextType,
    ContextValidation,
    ContextValidationError,
    ContextValidationErrors,

    // Borrowed contexts (CLI/short-lived operations)
    FormattingContext,
    FormattingContextDiff,
    // Lazy initialization for performance
    LazyPatternCompilation,
    LazyValue,
    // Owned contexts (GUI/long-lived operations)
    OwnedFormattingContext,
    OwnedProcessingContext,
    OwnedSortingContext,
    OwnedWalkingContext,

    ProcessingContext,

    // Builder pattern for programmatic context creation
    ProcessingContextBuilder,
    ProcessingContextDiff,
    SortingContext,
    SortingContextDiff,
    ThreadSafeLazyPatternCompilation,
    ThreadSafeLazyValue,

    WalkingContext,
    WalkingContextDiff,
};

// Internal imports
use crate::core::options::ApplyFunction;
use crate::core::{metadata::file_info, sorter, tree::builder::TempNode, walker};
use std::path::Path;

/// Gets tree nodes from either filesystem scanning or input file parsing.
///
/// This is the main entry point for gathering tree information. It can either:
/// 1. Walk a directory structure (if `input_file` is `None`)
/// 2. Parse a previously generated tree file (if `input_file` is `Some`)
///
/// # Arguments
///
/// * `root_path` - The starting path for directory traversal (ignored when reading from file).
/// * `config` - Configuration options that control traversal, analysis, and sorting.
/// * `input_file` - Optional path to a tree file to parse instead of scanning filesystem.
/// * `input_format` - Format of the input file (ignored when scanning filesystem).
///
/// # Returns
///
/// A `Result` containing a `Vec<NodeInfo>` on success, representing the processed
/// entries, or a `RustreeError` on failure.
pub fn get_tree_nodes_from_source(
    root_path: &Path,
    config: &RustreeLibConfig,
    input_file: Option<&Path>,
    input_format: Option<crate::core::input::InputFormat>,
) -> Result<Vec<NodeInfo>, RustreeError> {
    match input_file {
        Some(file_path) => {
            // Parse from input file
            let format = input_format.unwrap_or(crate::core::input::InputFormat::Auto);
            let mut nodes = crate::core::input::TreeFileParser::parse_file(file_path, format)?;

            // Apply any post-processing that would normally be done by get_tree_nodes
            apply_post_processing(&mut nodes, config)?;
            Ok(nodes)
        }
        None => {
            // Use existing filesystem scanning
            get_tree_nodes(root_path, config)
        }
    }
}

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
/// * [`get_tree_nodes_with_context`] - For the new context-based API.
pub fn get_tree_nodes(
    root_path: &Path,
    config: &RustreeLibConfig,
) -> Result<Vec<NodeInfo>, RustreeError> {
    // 1. Walk and analyze using parameter objects (Phase 1 approach)
    let mut nodes = walker::walk_directory_with_options(
        root_path,
        &config.listing,
        &config.filtering,
        &config.metadata,
    )?;

    // 2. Apply shared post-processing
    apply_post_processing(&mut nodes, config)?;
    Ok(nodes)
}

/// Applies post-processing steps to nodes (shared between filesystem and file input).
fn apply_post_processing(
    nodes: &mut Vec<NodeInfo>,
    config: &RustreeLibConfig,
) -> Result<(), RustreeError> {
    // 1. Apply size-based file filtering prior to any tree manipulations
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
        let mut temp_roots = core::tree::builder::build_tree(std::mem::take(nodes))
            .map_err(RustreeError::TreeBuildError)?;

        // Apply directory functions if configured
        if let Some(ApplyFunction::BuiltIn(apply_func)) = &config.metadata.apply_function {
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
        core::tree::builder::flatten_tree_to_dfs_consuming(temp_roots, nodes);
    }

    // 3. Apply list_directories_only filter if enabled
    // This happens *after* pruning, so pruning decisions are based on full content.
    if config.listing.list_directories_only {
        nodes.retain(|node| node.node_type == NodeType::Directory);
    }

    // 4. Sort if requested in config
    if config.sorting.sort_by.is_some() {
        // sort_nodes_with_options internally handles building tree from `nodes` for sorting
        if let Err(e) = sorter::strategies::sort_nodes_with_options(nodes, &config.sorting) {
            return Err(RustreeError::TreeBuildError(format!(
                "Sorting failed: {}",
                e
            )));
        }
    }

    Ok(())
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
/// * [`format_nodes_with_context`] - For the new context-based API.
pub fn format_nodes(
    nodes: &[NodeInfo],
    format: LibOutputFormat,
    config: &RustreeLibConfig,
) -> Result<String, RustreeError> {
    let tree_output = match format {
        LibOutputFormat::Text => {
            let formatter = TextTreeFormatter;
            formatter.format_compat(nodes, config)?
        }
        LibOutputFormat::Markdown => {
            let formatter = core::formatter::MarkdownFormatter;
            formatter.format_compat(nodes, config)?
        }
        LibOutputFormat::Json => {
            let formatter = core::formatter::JsonFormatter;
            formatter.format_compat(nodes, config)?
        }
        LibOutputFormat::Html => {
            let formatter = core::formatter::HtmlFormatter;
            formatter.format_compat(nodes, config)?
        }
    };

    let mut is_cat_like = false;
    if let Some(apply_fn) = &config.metadata.apply_function {
        match apply_fn {
            ApplyFunction::BuiltIn(BuiltInFunction::Cat) => {
                is_cat_like = true;
            }
            ApplyFunction::External(ext_fn) => {
                if matches!(ext_fn.kind, crate::core::options::FunctionOutputKind::Text) {
                    is_cat_like = true;
                }
            }
            _ => {}
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
            let header = match &config.metadata.apply_function {
                Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)) => "File Contents".to_string(),
                Some(ApplyFunction::External(ext_fn)) => {
                    format!(
                        "Results of applying '{}' to relevant files",
                        ext_fn.cmd_template
                    )
                }
                _ => "Results".to_string(),
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

/// Formats a diff result into a string representation.
///
/// This function takes a `DiffResult` containing change information and formats it
/// according to the specified output format. Unlike the regular `format_nodes` function,
/// this is specialized for displaying directory change comparisons.
///
/// # Arguments
///
/// * `diff_result` - The diff result containing changes, summary, and metadata.
/// * `format` - The output format to use (Text, Markdown, Json, Html).
/// * `config` - Configuration options that affect formatting.
///
/// # Returns
///
/// A formatted string representation of the diff, or an error if formatting fails.
///
/// # Examples
///
/// ```no_run
/// use rustree::{DiffResult, LibOutputFormat, RustreeLibConfig, format_diff};
///
/// fn example_format_diff(diff_result: DiffResult, config: RustreeLibConfig) -> Result<String, rustree::RustreeError> {
///     format_diff(&diff_result, LibOutputFormat::Text, &config)
/// }
/// ```
pub fn format_diff(
    diff_result: &DiffResult,
    format: LibOutputFormat,
    config: &RustreeLibConfig,
) -> Result<String, RustreeError> {
    use crate::config::OutputFormat;
    let output_format = match format {
        LibOutputFormat::Text => OutputFormat::Text,
        LibOutputFormat::Markdown => OutputFormat::Markdown,
        LibOutputFormat::Json => OutputFormat::Json,
        LibOutputFormat::Html => OutputFormat::Html,
    };
    crate::core::diff::formatter::format_diff(diff_result, output_format, config)
}

// ===============================
// Context-based Public APIs
// ===============================

/// Context-based tree processing for advanced users and GUI applications.
///
/// This function uses contexts instead of monolithic config for cleaner APIs
/// and better modularity. It's ideal for GUI applications and advanced library usage.
///
/// # Arguments
/// * `root_path` - The starting path for directory traversal
/// * `processing_ctx` - Complete processing context containing walking, sorting, and formatting contexts
///
/// # Returns
/// A `Result` containing processed nodes or an error
///
/// # Examples
/// ```rust,no_run
/// use rustree::{RustreeLibConfig, get_tree_nodes_with_context};
/// use std::path::Path;
///
/// let config = RustreeLibConfig::default();
/// let processing_ctx = config.processing_context();
/// let nodes = get_tree_nodes_with_context(Path::new("."), &processing_ctx)?;
/// # Ok::<(), rustree::RustreeError>(())
/// ```
pub fn get_tree_nodes_with_context(
    root_path: &Path,
    processing_ctx: &ProcessingContext,
) -> Result<Vec<NodeInfo>, RustreeError> {
    // Use walking context
    let mut nodes = walker::walk_directory_with_context(root_path, &processing_ctx.walking)?;

    // Apply post-processing with contexts
    apply_post_processing_with_contexts(&mut nodes, processing_ctx)?;

    // Use sorting context if provided
    if let Some(sorting_ctx) = &processing_ctx.sorting {
        // Use the *options*-based sorter here to maintain identical behaviour
        // with the original, non-context API.  This is important for backwards
        // compatibility tests that compare the output of both public
        // functions.
        sorter::strategies::sort_nodes_with_options(&mut nodes, sorting_ctx.sorting)
            .map_err(|e| RustreeError::TreeBuildError(format!("Sorting failed: {}", e)))?;
    }

    Ok(nodes)
}

/// Focused API for directory walking using WalkingContext.
///
/// This function only performs directory traversal and metadata collection,
/// without sorting or other post-processing. Ideal for scenarios where you
/// need just the walking functionality.
///
/// # Arguments
/// * `root_path` - The starting path for directory traversal
/// * `walking_ctx` - Context containing walking-specific options
///
/// # Returns
/// A `Result` containing raw walked nodes or an error
pub fn walk_path_with_context(
    root_path: &Path,
    walking_ctx: &WalkingContext,
) -> Result<Vec<NodeInfo>, RustreeError> {
    walker::walk_directory_with_context(root_path, walking_ctx)
}

/// Focused API for directory walking using owned context (GUI-friendly).
///
/// This function is optimized for scenarios where contexts are owned and modified,
/// such as GUI applications. It includes pattern compilation caching for better performance.
///
/// # Arguments
/// * `root_path` - The starting path for directory traversal
/// * `walking_ctx` - Mutable owned context for caching and modification
///
/// # Returns
/// A `Result` containing raw walked nodes or an error
pub fn walk_path_owned(
    root_path: &Path,
    walking_ctx: &mut OwnedWalkingContext,
) -> Result<Vec<NodeInfo>, RustreeError> {
    walker::walk_directory_owned(root_path, walking_ctx)
}

/// Focused API for directory walking with borrowed context (CLI-friendly).
///
/// This function provides direct access to the directory walking functionality
/// using borrowed contexts, which is ideal for CLI applications and scenarios
/// where you don't need to modify the walking parameters.
///
/// # Arguments
/// * `root_path` - The starting path for directory traversal
/// * `walking_ctx` - Borrowed walking context
///
/// # Returns
/// A `Result` containing the walked nodes or an error
///
/// # Examples
/// ```rust,no_run
/// use rustree::{walk_path, RustreeLibConfig};
/// use std::path::Path;
///
/// let config = RustreeLibConfig::default();
/// let walking_ctx = config.walking_context();
/// let nodes = walk_path(Path::new("."), &walking_ctx)?;
/// # Ok::<(), rustree::RustreeError>(())
/// ```
pub fn walk_path(
    root_path: &Path,
    walking_ctx: &WalkingContext,
) -> Result<Vec<NodeInfo>, RustreeError> {
    walker::walk_directory_with_context(root_path, walking_ctx)
}

/// Context-based formatting for nodes.
///
/// This function formats nodes using a FormattingContext instead of full config,
/// providing a cleaner API for advanced users.
///
/// # Arguments
/// * `nodes` - Slice of NodeInfo objects to format
/// * `format` - The desired output format
/// * `formatting_ctx` - Context containing formatting-specific options
///
/// # Returns
/// A `Result` containing the formatted string or an error
pub fn format_nodes_with_context(
    nodes: &[NodeInfo],
    format: LibOutputFormat,
    formatting_ctx: &FormattingContext,
) -> Result<String, RustreeError> {
    let formatter_instance: Box<dyn TreeFormatter> = match format {
        LibOutputFormat::Text => Box::new(TextTreeFormatter),
        LibOutputFormat::Markdown => Box::new(core::formatter::MarkdownFormatter),
        LibOutputFormat::Json => Box::new(core::formatter::JsonFormatter),
        LibOutputFormat::Html => Box::new(core::formatter::HtmlFormatter),
    };
    formatter_instance.format(nodes, formatting_ctx)
}

/// Focused sorting API using SortingContext.
///
/// This function only performs sorting without other operations.
/// Useful when you need just sorting functionality.
///
/// # Arguments
/// * `nodes` - Mutable reference to nodes to be sorted
/// * `sorting_ctx` - Context containing sorting-specific options
///
/// # Returns
/// A `Result` indicating success or failure
pub fn sort_nodes_with_context(
    nodes: &mut Vec<NodeInfo>,
    sorting_ctx: &SortingContext,
) -> Result<(), RustreeError> {
    sorter::strategies::sort_nodes_with_context(nodes, sorting_ctx)
        .map_err(|e| RustreeError::TreeBuildError(format!("Sorting failed: {}", e)))
}

/// Context-aware post-processing using focused contexts.
///
/// This function applies the same post-processing logic as the original version
/// but uses context structures instead of monolithic config.
fn apply_post_processing_with_contexts(
    nodes: &mut Vec<NodeInfo>,
    processing_ctx: &ProcessingContext,
) -> Result<(), RustreeError> {
    // 1. Apply size-based file filtering prior to any tree manipulations
    if processing_ctx.walking.filtering.min_file_size.is_some()
        || processing_ctx.walking.filtering.max_file_size.is_some()
    {
        let min_opt = processing_ctx.walking.filtering.min_file_size;
        let max_opt = processing_ctx.walking.filtering.max_file_size;

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
    if ((processing_ctx.walking.metadata.apply_function.is_some()
        && needs_directory_function_processing_ctx(processing_ctx))
        || processing_ctx.walking.filtering.prune_empty_directories)
        && !nodes.is_empty()
    {
        // Build the tree structure from the flat list of nodes
        let mut temp_roots = core::tree::builder::build_tree(std::mem::take(nodes))
            .map_err(RustreeError::TreeBuildError)?;

        // Apply directory functions if configured
        if let Some(ApplyFunction::BuiltIn(apply_func)) =
            &processing_ctx.walking.metadata.apply_function
        {
            if is_directory_function(apply_func) {
                apply_directory_functions_to_tree_ctx(&mut temp_roots, apply_func, processing_ctx);
            }
        }

        // Prune empty directories if requested
        if processing_ctx.walking.filtering.prune_empty_directories {
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
        core::tree::builder::flatten_tree_to_dfs_consuming(temp_roots, nodes);
    }

    // 3. Apply list_directories_only filter if enabled
    // This happens *after* pruning, so pruning decisions are based on full content.
    if processing_ctx.walking.listing.list_directories_only {
        nodes.retain(|node| node.node_type == NodeType::Directory);
    }

    Ok(())
}

/// Context-aware check for directory function processing needs.
fn needs_directory_function_processing_ctx(processing_ctx: &ProcessingContext) -> bool {
    if let Some(ApplyFunction::BuiltIn(func)) = &processing_ctx.walking.metadata.apply_function {
        is_directory_function(func)
    } else {
        false
    }
}

/// Context-aware version of apply_directory_functions_to_tree.
fn apply_directory_functions_to_tree_ctx(
    roots: &mut [TempNode],
    func: &BuiltInFunction,
    processing_ctx: &ProcessingContext,
) {
    for root in roots {
        apply_directory_functions_to_node_ctx(root, func, processing_ctx);
    }
}

/// Context-aware version of apply_directory_functions_to_node.
fn apply_directory_functions_to_node_ctx(
    node: &mut TempNode,
    func: &BuiltInFunction,
    processing_ctx: &ProcessingContext,
) {
    // First, recursively process all children
    for child in &mut node.children {
        apply_directory_functions_to_node_ctx(child, func, processing_ctx);
    }

    // Then process this node if it's a directory and should have the function applied
    if node.node_info.node_type == NodeType::Directory
        && should_apply_function_to_node_ctx(&node.node_info, processing_ctx)
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

/// Context-aware version of should_apply_function_to_node.
fn should_apply_function_to_node_ctx(node: &NodeInfo, processing_ctx: &ProcessingContext) -> bool {
    use crate::core::filter::pattern::{compile_glob_patterns, entry_matches_path_with_patterns};

    // Check apply_exclude_patterns first - if it matches, skip
    if let Some(exclude_patterns) = &processing_ctx.walking.filtering.apply_exclude_patterns {
        if !exclude_patterns.is_empty() {
            if let Ok(Some(patterns)) = compile_glob_patterns(
                &Some(exclude_patterns.clone()),
                processing_ctx.walking.filtering.case_insensitive_filter,
                processing_ctx.walking.listing.show_hidden,
            ) {
                if entry_matches_path_with_patterns(&node.path, &patterns) {
                    return false; // Skip this node
                }
            }
        }
    }

    // Check apply_include_patterns - if specified, node must match
    if let Some(include_patterns) = &processing_ctx.walking.filtering.apply_include_patterns {
        if !include_patterns.is_empty() {
            if let Ok(Some(patterns)) = compile_glob_patterns(
                &Some(include_patterns.clone()),
                processing_ctx.walking.filtering.case_insensitive_filter,
                processing_ctx.walking.listing.show_hidden,
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

/// Checks if the current configuration needs directory function processing.
fn needs_directory_function_processing(config: &RustreeLibConfig) -> bool {
    if let Some(ApplyFunction::BuiltIn(func)) = &config.metadata.apply_function {
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

// ===============================
// Enhanced Public APIs (Phase 4)
// ===============================

/// GUI-friendly tree processing using owned context.
///
/// This function is optimized for GUI applications that need to own and modify
/// contexts independently. It includes pattern compilation caching and validation.
///
/// # Arguments
/// * `root_path` - The starting path for directory traversal
/// * `processing_ctx` - Mutable owned processing context for caching and modification
///
/// # Returns
/// A `Result` containing processed nodes or an error
///
/// # Examples
/// ```rust,no_run
/// use rustree::{RustreeLibConfig, get_tree_nodes_owned};
/// use std::path::Path;
///
/// let config = RustreeLibConfig::default();
/// let mut processing_ctx = config.to_owned_processing_context();
///
/// // User modifies context in GUI
/// processing_ctx.walking.listing.max_depth = Some(5);
///
/// let nodes = get_tree_nodes_owned(Path::new("."), &mut processing_ctx)?;
/// # Ok::<(), rustree::RustreeError>(())
/// ```
pub fn get_tree_nodes_owned(
    root_path: &Path,
    processing_ctx: &mut OwnedProcessingContext,
) -> Result<Vec<NodeInfo>, RustreeError> {
    // Validate the context before processing
    processing_ctx
        .validate()
        .map_err(RustreeError::ConfigError)?;

    // Optimize context for performance (compile patterns, etc.)
    processing_ctx.optimize()?;

    // Use owned walking context
    let mut nodes = walker::walk_directory_owned(root_path, &mut processing_ctx.walking)?;

    // Apply post-processing with contexts
    let borrowed_ctx = processing_ctx.as_borrowed();
    apply_post_processing_with_contexts(&mut nodes, &borrowed_ctx)?;

    // Use sorting context if provided
    if let Some(sorting_ctx) = &processing_ctx.sorting {
        let borrowed_sorting = sorting_ctx.as_borrowed();
        sorter::strategies::sort_nodes_with_context(&mut nodes, &borrowed_sorting)
            .map_err(|e| RustreeError::TreeBuildError(format!("Sorting failed: {}", e)))?;
    }

    Ok(nodes)
}

/// Fluent API for tree processing using builder pattern.
///
/// This function provides a builder-based API for constructing processing contexts
/// programmatically, ideal for library integration and complex GUI applications.
///
/// # Arguments
/// * `root_path` - The starting path for directory traversal
/// * `builder` - Configured ProcessingContextBuilder
///
/// # Returns
/// A `Result` containing processed nodes or an error
///
/// # Examples
/// ```rust,no_run
/// use rustree::{ProcessingContextBuilder, OwnedWalkingContext, OwnedFormattingContext};
/// use rustree::{process_tree_with_builder, ListingOptions, FilteringOptions, MetadataOptions};
/// use rustree::{InputSourceOptions, MiscOptions, HtmlOptions};
/// use std::path::Path;
///
/// let walking = OwnedWalkingContext::new(
///     ListingOptions { max_depth: Some(3), ..Default::default() },
///     FilteringOptions::default(),
///     MetadataOptions { show_size_bytes: true, ..Default::default() }
/// );
///
/// let formatting = OwnedFormattingContext::new(
///     InputSourceOptions::default(),
///     ListingOptions { max_depth: Some(3), ..Default::default() },
///     MetadataOptions { show_size_bytes: true, ..Default::default() },
///     MiscOptions::default(),
///     HtmlOptions::default(),
/// );
///
/// let builder = ProcessingContextBuilder::new()
///     .with_walking(walking)
///     .with_formatting(formatting)
///     .with_default_sorting();
///
/// let nodes = process_tree_with_builder(Path::new("."), builder)?;
/// # Ok::<(), rustree::RustreeError>(())
/// ```
pub fn process_tree_with_builder(
    root_path: &Path,
    builder: ProcessingContextBuilder,
) -> Result<Vec<NodeInfo>, RustreeError> {
    let mut processing_ctx = builder.build().map_err(RustreeError::ConfigError)?;
    get_tree_nodes_owned(root_path, &mut processing_ctx)
}

/// Create a default processing context with sensible defaults.
///
/// This is a convenience function for quickly creating a processing context
/// with common settings. Useful for simple integrations and quick prototyping.
///
/// # Arguments
/// * `root_display_name` - Display name for the root node
/// * `max_depth` - Optional maximum traversal depth
/// * `show_size` - Whether to collect and display file sizes
///
/// # Returns
/// A configured `OwnedProcessingContext`
///
/// # Examples
/// ```rust,no_run
/// use rustree::{create_default_processing_context, get_tree_nodes_owned};
/// use std::path::Path;
///
/// let mut context = create_default_processing_context("my_project", Some(3), true);
/// let nodes = get_tree_nodes_owned(Path::new("."), &mut context)?;
/// # Ok::<(), rustree::RustreeError>(())
/// ```
pub fn create_default_processing_context(
    root_display_name: &str,
    max_depth: Option<usize>,
    show_size: bool,
) -> OwnedProcessingContext {
    let walking = OwnedWalkingContext::new(
        ListingOptions {
            max_depth,
            show_hidden: false,
            list_directories_only: false,
            show_full_path: false,
        },
        FilteringOptions::default(),
        MetadataOptions {
            show_size_bytes: show_size,
            human_readable_size: false,
            report_permissions: false,
            show_last_modified: false,
            calculate_line_count: false,
            calculate_word_count: false,
            apply_function: None,
            report_change_time: false,
            report_creation_time: false,
        },
    );

    let formatting = OwnedFormattingContext::new(
        InputSourceOptions {
            root_display_name: root_display_name.to_string(),
            root_is_directory: true,
            root_node_size: None,
        },
        ListingOptions {
            max_depth,
            show_hidden: false,
            list_directories_only: false,
            show_full_path: false,
        },
        MetadataOptions {
            show_size_bytes: show_size,
            human_readable_size: false,
            report_permissions: false,
            show_last_modified: false,
            calculate_line_count: false,
            calculate_word_count: false,
            apply_function: None,
            report_change_time: false,
            report_creation_time: false,
        },
        MiscOptions::default(),
        HtmlOptions::default(),
    );

    OwnedProcessingContext::new(walking, None, formatting)
}

/// Validate a processing context for consistency and correctness.
///
/// This standalone function validates that all contexts within a processing context
/// are consistent with each other and contain valid configuration values.
///
/// # Arguments
/// * `processing_ctx` - The processing context to validate
///
/// # Returns
/// `Ok(())` if valid, or an error describing the validation failure
///
/// # Examples
/// ```rust,no_run
/// use rustree::{validate_processing_context, create_default_processing_context};
///
/// let context = create_default_processing_context("test", Some(3), true);
/// validate_processing_context(&context)?;
/// # Ok::<(), rustree::RustreeError>(())
/// ```
pub fn validate_processing_context(
    processing_ctx: &OwnedProcessingContext,
) -> Result<(), RustreeError> {
    processing_ctx.validate().map_err(RustreeError::ConfigError)
}

/// Optimize a processing context for repeated operations.
///
/// This function pre-compiles patterns and performs other optimizations
/// that benefit repeated tree processing operations. Useful for GUI applications
/// that will perform multiple tree operations with the same context.
///
/// # Arguments
/// * `processing_ctx` - Mutable processing context to optimize
///
/// # Returns
/// `Ok(())` if optimization succeeded, or an error if compilation failed
///
/// # Examples
/// ```rust,no_run
/// use rustree::{optimize_context, create_default_processing_context};
///
/// let mut context = create_default_processing_context("test", Some(3), true);
/// context.walking.filtering.ignore_patterns = Some(vec!["*.tmp".to_string()]);
///
/// optimize_context(&mut context)?;
/// // Patterns are now pre-compiled for better performance
/// # Ok::<(), rustree::RustreeError>(())
/// ```
pub fn optimize_context(processing_ctx: &mut OwnedProcessingContext) -> Result<(), RustreeError> {
    processing_ctx.optimize()
}

/// Create a processing context from individual option structs.
///
/// This function provides a convenient way to create processing contexts
/// when you already have individual option structs, avoiding the need
/// to use the builder pattern for simple cases.
///
/// # Arguments
/// * `listing` - Directory listing options
/// * `filtering` - File filtering options  
/// * `metadata` - Metadata collection options
/// * `input_source` - Input source options for formatting
/// * `misc` - Miscellaneous options
/// * `html` - HTML-specific options
/// * `sorting` - Optional sorting options
///
/// # Returns
/// A configured `OwnedProcessingContext`
///
/// # Examples
/// ```rust,no_run
/// use rustree::{create_context_from_options, ListingOptions, FilteringOptions};
/// use rustree::{MetadataOptions, InputSourceOptions, MiscOptions, HtmlOptions, SortingOptions};
///
/// let listing = ListingOptions { max_depth: Some(2), ..Default::default() };
/// let filtering = FilteringOptions::default();
/// let metadata = MetadataOptions { show_size_bytes: true, ..Default::default() };
/// let input_source = InputSourceOptions { root_display_name: "test".to_string(), ..Default::default() };
/// let misc = MiscOptions::default();
/// let html = HtmlOptions::default();
/// let sorting = Some(SortingOptions::default());
///
/// let context = create_context_from_options(
///     listing, filtering, metadata, input_source, misc, html, sorting
/// );
/// # let _: rustree::OwnedProcessingContext = context;
/// ```
pub fn create_context_from_options(
    listing: ListingOptions,
    filtering: FilteringOptions,
    metadata: MetadataOptions,
    input_source: InputSourceOptions,
    misc: MiscOptions,
    html: HtmlOptions,
    sorting: Option<SortingOptions>,
) -> OwnedProcessingContext {
    let walking = OwnedWalkingContext::new(listing.clone(), filtering, metadata.clone());

    let formatting = OwnedFormattingContext::new(input_source, listing, metadata, misc, html);

    let sorting_context = sorting.map(OwnedSortingContext::new);

    OwnedProcessingContext::new(walking, sorting_context, formatting)
}

// ===============================
// Async and Advanced APIs (Phase 8)
// ===============================

/// Create an async-safe processing context for multi-threaded applications.
///
/// This function converts an owned processing context into a thread-safe version
/// that can be shared between threads and used in async contexts. All internal
/// data structures use Arc for efficient sharing.
///
/// # Arguments
/// * `owned_ctx` - The owned processing context to convert
///
/// # Returns
/// A thread-safe async processing context
///
/// # Examples
/// ```rust,no_run
/// use rustree::{create_async_context, create_default_processing_context};
///
/// let owned = create_default_processing_context("my_project", Some(3), true);
/// let async_ctx = create_async_context(&owned);
///
/// // async_ctx can now be cloned and shared between threads
/// let cloned = async_ctx.clone();
/// # let _: rustree::AsyncProcessingContext = async_ctx;
/// ```
pub fn create_async_context(owned_ctx: &OwnedProcessingContext) -> AsyncProcessingContext {
    AsyncProcessingContext::from_owned(owned_ctx)
}

/// Compare two owned processing contexts and generate a detailed diff.
///
/// This function is essential for GUI applications that need to understand
/// exactly what changed between two context states, enabling optimized
/// updates instead of full rebuilds.
///
/// # Arguments
/// * `old_ctx` - The previous context state
/// * `new_ctx` - The new context state
///
/// # Returns
/// A detailed diff describing all changes
///
/// # Examples
/// ```rust,no_run
/// use rustree::{diff_processing_contexts, create_default_processing_context};
///
/// let mut old_ctx = create_default_processing_context("project", Some(2), true);
/// let mut new_ctx = old_ctx.clone();
///
/// // User changes max depth in GUI
/// new_ctx.walking.listing.max_depth = Some(5);
///
/// let diff = diff_processing_contexts(&old_ctx, &new_ctx);
///
/// if diff.requires_complete_rebuild() {
///     // Need to rescan directory
/// } else if diff.can_optimize_with_resort() {
///     // Just resort existing nodes
/// }
/// # let _: rustree::ProcessingContextDiff = diff;
/// ```
pub fn diff_processing_contexts(
    old_ctx: &OwnedProcessingContext,
    new_ctx: &OwnedProcessingContext,
) -> ProcessingContextDiff {
    old_ctx.diff(new_ctx)
}

/// Validate multiple contexts for consistency and provide detailed error messages.
///
/// This function performs comprehensive validation of contexts and returns
/// context-aware error messages with specific field references and suggestions
/// for fixing issues.
///
/// # Arguments
/// * `walking_ctx` - Walking context to validate
/// * `formatting_ctx` - Formatting context to validate
/// * `sorting_ctx` - Optional sorting context to validate
///
/// # Returns
/// `Ok(())` if all contexts are valid and consistent, or detailed validation errors
///
/// # Examples
/// ```rust,no_run
/// use rustree::{validate_contexts, OwnedWalkingContext, OwnedFormattingContext};
/// use rustree::{ListingOptions, FilteringOptions, MetadataOptions, InputSourceOptions};
/// use rustree::{MiscOptions, HtmlOptions};
///
/// let walking = OwnedWalkingContext::new(
///     ListingOptions { max_depth: Some(3), ..Default::default() },
///     FilteringOptions::default(),
///     MetadataOptions::default(),
/// );
///
/// let formatting = OwnedFormattingContext::new(
///     InputSourceOptions::default(),
///     ListingOptions::default(),
///     MetadataOptions::default(),
///     MiscOptions::default(),
///     HtmlOptions::default(),
/// );
///
/// match validate_contexts(&walking, &formatting, None) {
///     Ok(()) => println!("All contexts are valid"),
///     Err(errors) => println!("Validation errors: {}", errors),
/// }
/// # Ok::<(), String>(())
/// ```
pub fn validate_contexts(
    walking_ctx: &OwnedWalkingContext,
    formatting_ctx: &OwnedFormattingContext,
    sorting_ctx: Option<&OwnedSortingContext>,
) -> Result<(), ContextValidationErrors> {
    let mut errors = ContextValidationErrors::new(ContextType::Processing);

    // Validate individual contexts
    if let Err(walking_errors) = walking_ctx.validate() {
        errors.add_error(ContextValidationError::new(
            "walking",
            "invalid",
            walking_errors,
            ContextType::Walking,
        ));
    }

    if let Err(formatting_errors) = formatting_ctx.validate() {
        errors.add_error(ContextValidationError::new(
            "formatting",
            "invalid",
            formatting_errors,
            ContextType::Formatting,
        ));
    }

    if let Some(sort_ctx) = sorting_ctx {
        if let Err(sorting_errors) = sort_ctx.validate() {
            errors.add_error(ContextValidationError::new(
                "sorting",
                "invalid",
                sorting_errors,
                ContextType::Sorting,
            ));
        }
    }

    // Cross-context validation
    // Check metadata consistency
    if formatting_ctx.metadata.show_size_bytes && !walking_ctx.metadata.show_size_bytes {
        errors.add_error(ContextValidationError::inconsistent_metadata(
            "show_size_bytes",
            "show_size_bytes",
            "size information",
        ));
    }

    if formatting_ctx.metadata.show_last_modified && !walking_ctx.metadata.show_last_modified {
        errors.add_error(ContextValidationError::inconsistent_metadata(
            "show_last_modified",
            "show_last_modified",
            "modification time",
        ));
    }

    // Check depth consistency
    if let (Some(walking_depth), Some(formatting_depth)) = (
        walking_ctx.listing.max_depth,
        formatting_ctx.listing.max_depth,
    ) {
        if formatting_depth > walking_depth {
            errors.add_error(ContextValidationError::inconsistent_depth(
                walking_depth as u32,
                formatting_depth as u32,
            ));
        }
    }

    if errors.has_errors() {
        Err(errors)
    } else {
        Ok(())
    }
}

/// Create a lazy pattern compilation for expensive glob patterns.
///
/// This function pre-compiles glob patterns with lazy initialization,
/// which is particularly useful for GUI applications that might change
/// patterns frequently but don't always need the compiled results immediately.
///
/// # Arguments
/// * `patterns` - Vector of glob pattern strings
/// * `case_insensitive` - Whether to compile patterns case-insensitively
/// * `show_hidden` - Whether patterns should match hidden files
///
/// # Returns
/// A lazy pattern compilation that compiles patterns on first access
///
/// # Examples
/// ```rust,no_run
/// use rustree::create_lazy_patterns;
///
/// let patterns = vec!["*.rs".to_string(), "*.txt".to_string()];
/// let lazy_patterns = create_lazy_patterns(patterns, false, false);
///
/// // Patterns are compiled on first access
/// let compiled = lazy_patterns.get_compiled()?;
/// println!("Compiled {} patterns", compiled.len());
/// # Ok::<(), String>(())
/// ```
pub fn create_lazy_patterns(
    patterns: Vec<String>,
    case_insensitive: bool,
    show_hidden: bool,
) -> LazyPatternCompilation {
    LazyPatternCompilation::new(patterns, case_insensitive, show_hidden)
}

/// Create a thread-safe lazy pattern compilation for multi-threaded contexts.
///
/// This function creates a thread-safe version of lazy pattern compilation
/// that can be shared between threads and used in async contexts.
///
/// # Arguments  
/// * `patterns` - Vector of glob pattern strings
/// * `case_insensitive` - Whether to compile patterns case-insensitively
/// * `show_hidden` - Whether patterns should match hidden files
///
/// # Returns
/// A thread-safe lazy pattern compilation
///
/// # Examples
/// ```rust,no_run
/// use rustree::create_thread_safe_lazy_patterns;
///
/// let patterns = vec!["*.rs".to_string(), "target/**".to_string()];
/// let lazy_patterns = create_thread_safe_lazy_patterns(patterns, false, false);
///
/// // Can be cloned and shared between threads
/// let cloned = lazy_patterns.clone();
/// # let _: rustree::ThreadSafeLazyPatternCompilation = lazy_patterns;
/// ```
pub fn create_thread_safe_lazy_patterns(
    patterns: Vec<String>,
    case_insensitive: bool,
    show_hidden: bool,
) -> ThreadSafeLazyPatternCompilation {
    ThreadSafeLazyPatternCompilation::new(patterns, case_insensitive, show_hidden)
}

// Note: Core context-based APIs are already defined above in this file
