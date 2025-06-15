//! Top-level configuration structure that bundles all individual option
//! groups together.
//!
//! This file was moved from `src/config/tree_options.rs` to the *core* layer.

use super::contexts::{
    FormattingContext, OwnedFormattingContext, OwnedProcessingContext, OwnedSortingContext,
    OwnedWalkingContext, ProcessingContext, SortingContext, WalkingContext,
};
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

impl RustreeLibConfig {
    /// Create a borrowed walking context
    ///
    /// This method creates a walking context that borrows from this config,
    /// suitable for CLI operations where the config lifetime is sufficient.
    pub fn walking_context(&self) -> WalkingContext<'_> {
        WalkingContext {
            listing: &self.listing,
            filtering: &self.filtering,
            metadata: &self.metadata,
        }
    }

    /// Create an owned walking context (for GUI/async use)
    ///
    /// This method creates an independent walking context that owns its data,
    /// suitable for GUI applications where contexts may need to be modified
    /// or live beyond the original config's lifetime.
    pub fn to_owned_walking_context(&self) -> OwnedWalkingContext {
        OwnedWalkingContext::new(
            self.listing.clone(),
            self.filtering.clone(),
            self.metadata.clone(),
        )
    }

    /// Create a borrowed formatting context
    ///
    /// This method creates a formatting context that borrows from this config,
    /// suitable for CLI operations where the config lifetime is sufficient.
    pub fn formatting_context(&self) -> FormattingContext<'_> {
        FormattingContext {
            input_source: &self.input_source,
            listing: &self.listing,
            metadata: &self.metadata,
            misc: &self.misc,
            html: &self.html,
        }
    }

    /// Create an owned formatting context
    ///
    /// This method creates an independent formatting context that owns its data,
    /// suitable for GUI applications where contexts may need to be modified
    /// or live beyond the original config's lifetime.
    pub fn to_owned_formatting_context(&self) -> OwnedFormattingContext {
        OwnedFormattingContext {
            input_source: self.input_source.clone(),
            listing: self.listing.clone(),
            metadata: self.metadata.clone(),
            misc: self.misc.clone(),
            html: self.html.clone(),
        }
    }

    /// Create a borrowed sorting context
    ///
    /// This method creates a sorting context that borrows from this config,
    /// suitable for CLI operations where the config lifetime is sufficient.
    pub fn sorting_context(&self) -> SortingContext<'_> {
        SortingContext {
            sorting: &self.sorting,
        }
    }

    /// Create an owned sorting context
    ///
    /// This method creates an independent sorting context that owns its data,
    /// suitable for GUI applications where contexts may need to be modified
    /// or live beyond the original config's lifetime.
    pub fn to_owned_sorting_context(&self) -> OwnedSortingContext {
        OwnedSortingContext {
            sorting: self.sorting.clone(),
        }
    }

    /// Create a complete processing context
    ///
    /// This method creates a complete processing context with all the
    /// necessary contexts for a full tree processing pipeline.
    /// Sorting is included only if sorting is enabled in the config.
    pub fn processing_context(&self) -> ProcessingContext<'_> {
        ProcessingContext {
            walking: self.walking_context(),
            sorting: if self.sorting.sort_by.is_some() {
                Some(self.sorting_context())
            } else {
                None
            },
            formatting: self.formatting_context(),
        }
    }

    /// Create an owned processing context
    ///
    /// This method creates an independent processing context that owns all its data,
    /// suitable for GUI applications where contexts may need to be modified
    /// or live beyond the original config's lifetime.
    pub fn to_owned_processing_context(&self) -> OwnedProcessingContext {
        OwnedProcessingContext {
            walking: self.to_owned_walking_context(),
            sorting: if self.sorting.sort_by.is_some() {
                Some(self.to_owned_sorting_context())
            } else {
                None
            },
            formatting: self.to_owned_formatting_context(),
        }
    }

    /// Create contexts optimized for CLI usage (borrowed)
    ///
    /// This is a convenience method that returns the individual contexts
    /// most commonly needed for CLI operations in a single call.
    pub fn cli_contexts(
        &self,
    ) -> (
        WalkingContext<'_>,
        Option<SortingContext<'_>>,
        FormattingContext<'_>,
    ) {
        let walking = self.walking_context();
        let sorting = if self.sorting.sort_by.is_some() {
            Some(self.sorting_context())
        } else {
            None
        };
        let formatting = self.formatting_context();

        (walking, sorting, formatting)
    }
}
