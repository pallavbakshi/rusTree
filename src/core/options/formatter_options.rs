//! Options specifically for formatting operations
//!
//! This module provides the `FormatterOptions` struct which contains only the
//! configuration needed by formatter implementations, extracted from the full
//! `RustreeLibConfig`. This is part of Phase 1 of the core independence refactoring.

use super::{HtmlOptions, InputSourceOptions, ListingOptions, MetadataOptions, MiscOptions};

/// Options specifically for formatting operations
///
/// This struct contains only the configuration fields that formatters actually need,
/// providing a focused interface that doesn't couple formatters to the entire
/// configuration structure.
#[derive(Debug, Clone)]
pub struct FormatterOptions<'a> {
    /// Input source configuration (root display name, directory flag, etc.)
    pub input_source: &'a InputSourceOptions,

    /// Listing configuration (affects what gets displayed and how)
    pub listing: &'a ListingOptions,

    /// Metadata configuration (what metadata to show and how)
    pub metadata: &'a MetadataOptions,

    /// Miscellaneous options (colors, summary, etc.)
    pub misc: &'a MiscOptions,

    /// HTML-specific options (only used by HTML formatter)
    pub html: &'a HtmlOptions,
}

impl<'a> FormatterOptions<'a> {
    /// Create formatter options from a full config
    ///
    /// This extracts only the fields needed by formatters from the complete
    /// configuration structure.
    pub fn from_config(config: &'a crate::core::options::RustreeLibConfig) -> Self {
        Self {
            input_source: &config.input_source,
            listing: &config.listing,
            metadata: &config.metadata,
            misc: &config.misc,
            html: &config.html,
        }
    }
}
