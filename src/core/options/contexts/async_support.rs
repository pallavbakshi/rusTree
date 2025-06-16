//! # Async Support for Context Operations
//!
//! This module provides thread-safe and async-compatible context structures for
//! applications and multi-threaded scenarios. These contexts use Arc for shared
//! ownership and pre-compiled patterns for optimal performance.

use super::lazy::ThreadSafeLazyPatternCompilation;
use super::{
    OwnedFormattingContext, OwnedProcessingContext, OwnedSortingContext, OwnedWalkingContext,
};
use crate::core::filter::pattern::CompiledGlobPattern;
use crate::core::options::{
    FilteringOptions, HtmlOptions, InputSourceOptions, ListingOptions, MetadataOptions,
    MiscOptions, SortingOptions,
};
use std::sync::Arc;

/// Thread-safe walking context for async operations
///
/// This context is optimized for scenarios where:
/// - Multiple threads need access to the same walking configuration
/// - Async operations require Send + Sync contexts
/// - Pattern compilation results need to be shared efficiently
/// - Interactive applications need reactive updates to walking parameters
#[derive(Debug, Clone)]
pub struct AsyncWalkingContext {
    pub listing: Arc<ListingOptions>,
    pub filtering: Arc<FilteringOptions>,
    pub metadata: Arc<MetadataOptions>,

    /// Thread-safe lazy pattern compilation for optimal performance
    /// These use lazy initialization and are safe to access from multiple threads
    pub lazy_ignore_patterns: Option<ThreadSafeLazyPatternCompilation>,
    pub lazy_match_patterns: Option<ThreadSafeLazyPatternCompilation>,
}

impl AsyncWalkingContext {
    /// Create a new async walking context from individual options
    pub fn new(
        listing: ListingOptions,
        filtering: FilteringOptions,
        metadata: MetadataOptions,
    ) -> Self {
        // Initialize lazy pattern compilation if patterns exist
        let lazy_ignore_patterns = filtering.ignore_patterns.as_ref().map(|patterns| {
            ThreadSafeLazyPatternCompilation::new(
                patterns.clone(),
                filtering.case_insensitive_filter,
                listing.show_hidden,
            )
        });

        let lazy_match_patterns = filtering.match_patterns.as_ref().map(|patterns| {
            ThreadSafeLazyPatternCompilation::new(
                patterns.clone(),
                filtering.case_insensitive_filter,
                listing.show_hidden,
            )
        });

        Self {
            listing: Arc::new(listing),
            filtering: Arc::new(filtering),
            metadata: Arc::new(metadata),
            lazy_ignore_patterns,
            lazy_match_patterns,
        }
    }

    /// Convert from an owned context with async-safe lazy compilation
    pub fn from_owned(owned: &OwnedWalkingContext) -> Self {
        Self::new(
            owned.listing.clone(),
            owned.filtering.clone(),
            owned.metadata.clone(),
        )
    }

    /// Get ignore patterns (compiled lazily and thread-safely)
    pub fn ignore_patterns(&self) -> Result<Option<Vec<CompiledGlobPattern>>, String> {
        match &self.lazy_ignore_patterns {
            Some(lazy) => lazy.get_compiled().map(Some),
            None => Ok(None),
        }
    }

    /// Get match patterns (compiled lazily and thread-safely)
    pub fn match_patterns(&self) -> Result<Option<Vec<CompiledGlobPattern>>, String> {
        match &self.lazy_match_patterns {
            Some(lazy) => lazy.get_compiled().map(Some),
            None => Ok(None),
        }
    }

    /// Create a new context with updated listing options
    /// This preserves lazy compilation but updates patterns if show_hidden changed
    pub fn with_listing(&self, listing: ListingOptions) -> Self {
        // If show_hidden changed, we need new lazy pattern compilations
        let show_hidden_changed = self.listing.show_hidden != listing.show_hidden;

        let lazy_ignore_patterns = if show_hidden_changed {
            self.filtering.ignore_patterns.as_ref().map(|patterns| {
                ThreadSafeLazyPatternCompilation::new(
                    patterns.clone(),
                    self.filtering.case_insensitive_filter,
                    listing.show_hidden,
                )
            })
        } else {
            self.lazy_ignore_patterns.clone()
        };

        let lazy_match_patterns = if show_hidden_changed {
            self.filtering.match_patterns.as_ref().map(|patterns| {
                ThreadSafeLazyPatternCompilation::new(
                    patterns.clone(),
                    self.filtering.case_insensitive_filter,
                    listing.show_hidden,
                )
            })
        } else {
            self.lazy_match_patterns.clone()
        };

        Self {
            listing: Arc::new(listing),
            filtering: Arc::clone(&self.filtering),
            metadata: Arc::clone(&self.metadata),
            lazy_ignore_patterns,
            lazy_match_patterns,
        }
    }

    /// Create a new context with updated filtering options
    /// This creates new lazy pattern compilations as they depend on filtering
    pub fn with_filtering(&self, filtering: FilteringOptions) -> Self {
        Self::new((*self.listing).clone(), filtering, (*self.metadata).clone())
    }

    /// Create a new context with updated metadata options
    /// Metadata changes don't affect pattern compilation
    pub fn with_metadata(&self, metadata: MetadataOptions) -> Self {
        Self {
            listing: Arc::clone(&self.listing),
            filtering: Arc::clone(&self.filtering),
            metadata: Arc::new(metadata),
            lazy_ignore_patterns: self.lazy_ignore_patterns.clone(),
            lazy_match_patterns: self.lazy_match_patterns.clone(),
        }
    }

    /// Validate the context configuration
    pub fn validate(&self) -> Result<(), String> {
        if let Some(max_depth) = self.listing.max_depth {
            if max_depth == 0 {
                return Err("max_depth cannot be 0".to_string());
            }
        }

        if let Some(min_size) = self.filtering.min_file_size {
            if let Some(max_size) = self.filtering.max_file_size {
                if min_size > max_size {
                    return Err("min_file_size cannot be greater than max_file_size".to_string());
                }
            }
        }

        // Check for empty patterns
        if let Some(ref patterns) = self.filtering.ignore_patterns {
            if patterns.iter().any(|p| p.trim().is_empty()) {
                return Err("ignore_patterns cannot contain empty strings".to_string());
            }
        }

        if let Some(ref patterns) = self.filtering.match_patterns {
            if patterns.iter().any(|p| p.trim().is_empty()) {
                return Err("match_patterns cannot contain empty strings".to_string());
            }
        }

        Ok(())
    }
}

/// Thread-safe formatting context for async operations
#[derive(Debug, Clone)]
pub struct AsyncFormattingContext {
    pub input_source: Arc<InputSourceOptions>,
    pub listing: Arc<ListingOptions>,
    pub metadata: Arc<MetadataOptions>,
    pub misc: Arc<MiscOptions>,
    pub html: Arc<HtmlOptions>,
}

impl AsyncFormattingContext {
    /// Create a new async formatting context
    pub fn new(
        input_source: InputSourceOptions,
        listing: ListingOptions,
        metadata: MetadataOptions,
        misc: MiscOptions,
        html: HtmlOptions,
    ) -> Self {
        Self {
            input_source: Arc::new(input_source),
            listing: Arc::new(listing),
            metadata: Arc::new(metadata),
            misc: Arc::new(misc),
            html: Arc::new(html),
        }
    }

    /// Convert from owned formatting context
    pub fn from_owned(owned: &OwnedFormattingContext) -> Self {
        Self {
            input_source: Arc::new(owned.input_source.clone()),
            listing: Arc::new(owned.listing.clone()),
            metadata: Arc::new(owned.metadata.clone()),
            misc: Arc::new(owned.misc.clone()),
            html: Arc::new(owned.html.clone()),
        }
    }

    /// Check if this context has metadata display enabled
    pub fn has_metadata_display(&self) -> bool {
        self.metadata.show_size_bytes
            || self.metadata.show_last_modified
            || self.metadata.calculate_line_count
            || self.metadata.calculate_word_count
            || self.metadata.apply_function.is_some()
            || self.metadata.report_permissions
            || self.metadata.report_change_time
            || self.metadata.report_creation_time
    }

    /// Check if this context produces minimal output
    pub fn is_minimal_output(&self) -> bool {
        self.misc.no_summary_report && !self.has_metadata_display()
    }

    /// Validate the formatting context
    pub fn validate(&self) -> Result<(), String> {
        if self.input_source.root_display_name.trim().is_empty() {
            return Err("root_display_name cannot be empty or whitespace".to_string());
        }

        if let Some(ref base_href) = self.html.base_href {
            // Basic URL validation
            if !base_href.starts_with("http://")
                && !base_href.starts_with("https://")
                && !base_href.starts_with("file://")
            {
                return Err(
                    "base_href should be a valid URL starting with http://, https://, or file://"
                        .to_string(),
                );
            }
        }

        Ok(())
    }
}

/// Thread-safe sorting context for async operations
#[derive(Debug, Clone)]
pub struct AsyncSortingContext {
    pub sorting: Arc<SortingOptions>,
}

impl AsyncSortingContext {
    /// Create a new async sorting context
    pub fn new(sorting: SortingOptions) -> Self {
        Self {
            sorting: Arc::new(sorting),
        }
    }

    /// Convert from owned sorting context
    pub fn from_owned(owned: &OwnedSortingContext) -> Self {
        Self {
            sorting: Arc::new(owned.sorting.clone()),
        }
    }
}

/// Thread-safe processing context combining all async contexts
#[derive(Debug, Clone)]
pub struct AsyncProcessingContext {
    pub walking: AsyncWalkingContext,
    pub sorting: Option<AsyncSortingContext>,
    pub formatting: AsyncFormattingContext,
}

impl AsyncProcessingContext {
    /// Create from individual async contexts
    pub fn new(
        walking: AsyncWalkingContext,
        sorting: Option<AsyncSortingContext>,
        formatting: AsyncFormattingContext,
    ) -> Self {
        Self {
            walking,
            sorting,
            formatting,
        }
    }

    /// Convert from owned processing context
    pub fn from_owned(owned: &OwnedProcessingContext) -> Self {
        let walking = AsyncWalkingContext::from_owned(&owned.walking);
        let sorting = owned.sorting.as_ref().map(AsyncSortingContext::from_owned);
        let formatting = AsyncFormattingContext::from_owned(&owned.formatting);

        Self {
            walking,
            sorting,
            formatting,
        }
    }

    /// Validate all contexts
    pub fn validate(&self) -> Result<(), String> {
        self.walking.validate()?;
        self.formatting.validate()?;
        Ok(())
    }
}

// All our contexts use Arc internally and should automatically be Send + Sync
// since Arc<T> implements Send + Sync when T: Send + Sync.
// The standard option types (bool, Option<usize>, String, Vec<String>, etc.) are all Send + Sync.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::options::*;

    #[test]
    fn test_async_walking_context_creation() {
        let listing = ListingOptions {
            max_depth: Some(3),
            show_hidden: true,
            ..Default::default()
        };
        let filtering = FilteringOptions {
            ignore_patterns: Some(vec!["*.tmp".to_string()]),
            ..Default::default()
        };
        let metadata = MetadataOptions {
            show_size_bytes: true,
            ..Default::default()
        };

        let async_ctx = AsyncWalkingContext::new(listing, filtering, metadata);

        assert_eq!(async_ctx.listing.max_depth, Some(3));
        assert!(async_ctx.listing.show_hidden);
        assert_eq!(
            async_ctx.filtering.ignore_patterns,
            Some(vec!["*.tmp".to_string()])
        );
        assert!(async_ctx.metadata.show_size_bytes);
    }

    #[test]
    fn test_async_context_with_updated_options() {
        let original = AsyncWalkingContext::new(
            ListingOptions {
                max_depth: Some(2),
                show_hidden: false,
                ..Default::default()
            },
            FilteringOptions {
                ignore_patterns: Some(vec!["*.tmp".to_string()]), // Add patterns so there's something to compile
                ..Default::default()
            },
            MetadataOptions::default(),
        );

        // Verify original has patterns
        assert!(original.lazy_ignore_patterns.is_some());

        // Update listing - since show_hidden changes, patterns should be recreated
        let updated_listing = original.with_listing(ListingOptions {
            max_depth: Some(5),
            show_hidden: true,
            ..Default::default()
        });

        assert_eq!(updated_listing.listing.max_depth, Some(5));
        assert!(updated_listing.listing.show_hidden);

        // Patterns should still exist since we have ignore patterns in the filtering options
        assert!(updated_listing.lazy_ignore_patterns.is_some());

        // Test that updating with the same show_hidden preserves the pattern compilation
        let preserved_listing = original.with_listing(ListingOptions {
            max_depth: Some(3),
            show_hidden: false, // Same as original
            ..Default::default()
        });

        // Should preserve the original lazy patterns
        assert!(preserved_listing.lazy_ignore_patterns.is_some());
    }

    #[test]
    fn test_async_context_validation() {
        let invalid_ctx = AsyncWalkingContext::new(
            ListingOptions {
                max_depth: Some(0),
                ..Default::default()
            }, // Invalid
            FilteringOptions::default(),
            MetadataOptions::default(),
        );

        assert!(invalid_ctx.validate().is_err());
        assert!(
            invalid_ctx
                .validate()
                .unwrap_err()
                .contains("max_depth cannot be 0")
        );
    }

    #[test]
    fn test_async_formatting_context_validation() {
        let invalid_ctx = AsyncFormattingContext::new(
            InputSourceOptions {
                root_display_name: "   ".to_string(),
                ..Default::default()
            }, // Invalid
            ListingOptions::default(),
            MetadataOptions::default(),
            MiscOptions::default(),
            HtmlOptions::default(),
        );

        assert!(invalid_ctx.validate().is_err());
        assert!(
            invalid_ctx
                .validate()
                .unwrap_err()
                .contains("root_display_name cannot be empty")
        );
    }

    #[test]
    fn test_thread_safety() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<AsyncWalkingContext>();
        assert_send_sync::<AsyncFormattingContext>();
        assert_send_sync::<AsyncSortingContext>();
        assert_send_sync::<AsyncProcessingContext>();
    }
}
