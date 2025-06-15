//! Top-level configuration structure that bundles all individual option
//! groups together.
//!
//! This file was moved from `src/config/tree_options.rs` to the *core* layer.

use super::filtering::FilteringOptions;
use super::html::HtmlOptions;
use super::input_source::InputSourceOptions;
use super::listing::ListingOptions;
use super::llm::LlmOptions;
use super::metadata::MetadataOptions;
use super::misc::MiscOptions;
use super::sorting::SortingOptions;

/// Configuration for the `rustree` library.
#[derive(Debug, Clone, Default)]
pub struct RustreeLibConfig {
    /// Configuration for input source handling (root display name, etc.)
    pub input_source: InputSourceOptions,

    /// Configuration for directory listing behaviour (depth, hidden files, …)
    pub listing: ListingOptions,

    /// Configuration for filtering (include/exclude patterns, gitignore, …)
    pub filtering: FilteringOptions,

    /// Configuration for sorting behaviour.
    pub sorting: SortingOptions,

    /// Configuration for metadata collection and display.
    pub metadata: MetadataOptions,

    /// Miscellaneous configuration options.
    pub misc: MiscOptions,

    /// HTML output specific options (only used when `output-format = html`).
    pub html: HtmlOptions,

    /// LLM options parsed from configuration files (not set via CLI here).
    pub llm: LlmOptions,
}
