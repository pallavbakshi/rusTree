// src/config/tree_options.rs

use crate::config::filtering::FilteringOptions;
use crate::config::html::HtmlOptions;
use crate::config::input_source::InputSourceOptions;
use crate::config::listing::ListingOptions;
use crate::config::metadata::MetadataOptions;
use crate::config::misc::MiscOptions;
use crate::config::sorting::SortingOptions;

/// Configuration for the `rustree` library.
///
/// This struct holds all the options that control how `rustree` processes
/// and displays directory trees. It uses a hierarchical structure where
/// related configuration options are grouped together for better organization
/// and maintainability.
///
/// # Structure
///
/// The configuration is organized into logical groups:
///
/// - [`input_source`](Self::input_source): Controls how the root path is displayed
/// - [`listing`](Self::listing): Controls directory traversal behavior
/// - [`filtering`](Self::filtering): Controls inclusion/exclusion patterns
/// - [`sorting`](Self::sorting): Controls sorting behavior
/// - [`metadata`](Self::metadata): Controls what metadata to collect and display
/// - [`misc`](Self::misc): Additional miscellaneous options
///
/// # Examples
///
/// Basic usage with default options:
/// ```
/// use rustree::RustreeLibConfig;
///
/// let config = RustreeLibConfig::default();
/// ```
///
/// Customizing specific aspects:
/// ```
/// use rustree::{RustreeLibConfig, ListingOptions, MetadataOptions, SortingOptions, SortKey};
///
/// let config = RustreeLibConfig {
///     listing: ListingOptions {
///         max_depth: Some(3),
///         show_hidden: true,
///         ..Default::default()
///     },
///     metadata: MetadataOptions {
///         show_size_bytes: true,
///         calculate_line_count: true,
///         ..Default::default()
///     },
///     sorting: SortingOptions {
///         sort_by: Some(SortKey::Size),
///         reverse_sort: true,
///         ..Default::default()
///     },
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct RustreeLibConfig {
    /// Configuration for input source handling (root display name, etc.)
    pub input_source: InputSourceOptions,

    /// Configuration for directory listing behavior (depth, hidden files, etc.)
    pub listing: ListingOptions,

    /// Configuration for filtering (include/exclude patterns, gitignore, etc.)
    pub filtering: FilteringOptions,

    /// Configuration for sorting behavior
    pub sorting: SortingOptions,

    /// Configuration for metadata collection and display
    pub metadata: MetadataOptions,

    /// Miscellaneous configuration options
    pub misc: MiscOptions,

    /// HTML output specific options (only used when `output-format`=html)
    pub html: HtmlOptions,

    /// LLM options parsed from configuration files (not set via CLI here)
    pub llm: crate::config::llm::LlmOptions,
}
