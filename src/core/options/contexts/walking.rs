use crate::core::error::RustreeError;
use crate::core::filter::pattern::CompiledGlobPattern;
use crate::core::options::{FilteringOptions, ListingOptions, MetadataOptions};

/// Context for directory walking operations (borrowed references)
///
/// This context provides all the information needed for directory traversal
/// operations in a focused, efficient manner. It uses borrowed references
/// to avoid unnecessary cloning for short-lived CLI operations.
#[derive(Debug)]
pub struct WalkingContext<'a> {
    pub listing: &'a ListingOptions,
    pub filtering: &'a FilteringOptions,
    pub metadata: &'a MetadataOptions,
}

impl<'a> WalkingContext<'a> {
    /// Create a new walking context from individual option references
    pub fn new(
        listing: &'a ListingOptions,
        filtering: &'a FilteringOptions,
        metadata: &'a MetadataOptions,
    ) -> Self {
        Self {
            listing,
            filtering,
            metadata,
        }
    }
}

/// Owned version for advanced and async scenarios
///
/// This context owns all its data and provides caching capabilities for
/// expensive operations like pattern compilation. It's designed for scenarios
/// where the context needs to live independently or be modified over time.
#[derive(Debug, Clone)]
pub struct OwnedWalkingContext {
    pub listing: ListingOptions,
    pub filtering: FilteringOptions,
    pub metadata: MetadataOptions,

    // Cached/derived data for performance
    compiled_ignore_patterns: Option<Option<Vec<CompiledGlobPattern>>>,
    compiled_match_patterns: Option<Option<Vec<CompiledGlobPattern>>>,
}

impl OwnedWalkingContext {
    /// Create a new owned walking context
    pub fn new(
        listing: ListingOptions,
        filtering: FilteringOptions,
        metadata: MetadataOptions,
    ) -> Self {
        Self {
            listing,
            filtering,
            metadata,
            compiled_ignore_patterns: None,
            compiled_match_patterns: None,
        }
    }

    /// Get or compile ignore patterns, caching the result
    ///
    /// This method compiles ignore patterns on first access and caches
    /// the result for subsequent calls, providing significant performance
    /// benefits for repeated operations.
    pub fn ignore_patterns(&mut self) -> Result<Option<&Vec<CompiledGlobPattern>>, RustreeError> {
        if self.compiled_ignore_patterns.is_none() {
            let patterns = if self
                .filtering
                .ignore_patterns
                .as_ref()
                .is_some_and(|p| !p.is_empty())
            {
                crate::core::filter::pattern::compile_glob_patterns(
                    &self.filtering.ignore_patterns,
                    self.filtering.case_insensitive_filter,
                    self.listing.show_hidden,
                )?
            } else {
                None
            };
            self.compiled_ignore_patterns = Some(patterns);
        }

        Ok(self.compiled_ignore_patterns.as_ref().unwrap().as_ref())
    }

    /// Get or compile match patterns, caching the result
    ///
    /// Similar to ignore_patterns, this provides cached compilation
    /// of include/match patterns for performance optimization.
    pub fn match_patterns(&mut self) -> Result<Option<&Vec<CompiledGlobPattern>>, RustreeError> {
        if self.compiled_match_patterns.is_none() {
            let patterns = if self
                .filtering
                .match_patterns
                .as_ref()
                .is_some_and(|p| !p.is_empty())
            {
                crate::core::filter::pattern::compile_glob_patterns(
                    &self.filtering.match_patterns,
                    self.filtering.case_insensitive_filter,
                    self.listing.show_hidden,
                )?
            } else {
                None
            };
            self.compiled_match_patterns = Some(patterns);
        }

        Ok(self.compiled_match_patterns.as_ref().unwrap().as_ref())
    }

    /// Invalidate cached patterns when filtering options change
    ///
    /// Call this method when you modify filtering options to ensure
    /// the cached patterns are recompiled on next access.
    pub fn invalidate_pattern_cache(&mut self) {
        self.compiled_ignore_patterns = None;
        self.compiled_match_patterns = None;
    }

    /// Validate context for consistency and correctness
    ///
    /// This method checks for invalid combinations of options and
    /// provides helpful error messages for fixing configuration issues.
    pub fn validate(&self) -> Result<(), String> {
        // Validate max_depth
        if let Some(max_depth) = self.listing.max_depth {
            if max_depth == 0 {
                return Err("max_depth cannot be 0 (use None for unlimited depth)".to_string());
            }
        }

        // Validate file size constraints
        if let (Some(min), Some(max)) = (self.filtering.min_file_size, self.filtering.max_file_size)
        {
            if min > max {
                return Err(format!(
                    "min_file_size ({}) cannot be greater than max_file_size ({})",
                    min, max
                ));
            }
        }

        // Validate patterns are not empty strings
        if let Some(ref patterns) = self.filtering.ignore_patterns {
            for pattern in patterns {
                if pattern.trim().is_empty() {
                    return Err("ignore patterns cannot be empty strings".to_string());
                }
            }
        }

        if let Some(ref patterns) = self.filtering.match_patterns {
            for pattern in patterns {
                if pattern.trim().is_empty() {
                    return Err("match patterns cannot be empty strings".to_string());
                }
            }
        }

        Ok(())
    }

    /// Create a borrowed context from this owned context
    ///
    /// This allows you to use the owned context with APIs that expect
    /// borrowed contexts, providing flexibility in how you manage context lifetimes.
    pub fn as_borrowed(&self) -> WalkingContext<'_> {
        WalkingContext {
            listing: &self.listing,
            filtering: &self.filtering,
            metadata: &self.metadata,
        }
    }
}

impl Default for OwnedWalkingContext {
    fn default() -> Self {
        Self::new(
            ListingOptions::default(),
            FilteringOptions::default(),
            MetadataOptions::default(),
        )
    }
}

impl<'a> From<WalkingContext<'a>> for OwnedWalkingContext {
    fn from(ctx: WalkingContext<'a>) -> Self {
        Self::new(
            ctx.listing.clone(),
            ctx.filtering.clone(),
            ctx.metadata.clone(),
        )
    }
}

impl TryFrom<(ListingOptions, FilteringOptions, MetadataOptions)> for OwnedWalkingContext {
    type Error = String;

    fn try_from(
        (listing, filtering, metadata): (ListingOptions, FilteringOptions, MetadataOptions),
    ) -> Result<Self, Self::Error> {
        let owned = Self::new(listing, filtering, metadata);
        owned.validate()?;
        Ok(owned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owned_walking_context_creation() {
        let listing = ListingOptions {
            max_depth: Some(3),
            show_hidden: true,
            ..Default::default()
        };

        let filtering = FilteringOptions {
            ignore_patterns: Some(vec!["*.tmp".to_string()]),
            match_patterns: Some(vec!["*.rs".to_string()]),
            case_insensitive_filter: true,
            ..Default::default()
        };

        let metadata = MetadataOptions {
            show_size_bytes: true,
            calculate_line_count: true,
            ..Default::default()
        };

        let ctx = OwnedWalkingContext::new(listing, filtering, metadata);

        assert_eq!(ctx.listing.max_depth, Some(3));
        assert!(ctx.listing.show_hidden);
        assert_eq!(
            ctx.filtering.ignore_patterns,
            Some(vec!["*.tmp".to_string()])
        );
        assert_eq!(ctx.filtering.match_patterns, Some(vec!["*.rs".to_string()]));
        assert!(ctx.filtering.case_insensitive_filter);
        assert!(ctx.metadata.show_size_bytes);
        assert!(ctx.metadata.calculate_line_count);
    }

    #[test]
    fn test_validation_valid_context() {
        let ctx = OwnedWalkingContext {
            listing: ListingOptions {
                max_depth: Some(5),
                ..Default::default()
            },
            filtering: FilteringOptions {
                min_file_size: Some(100),
                max_file_size: Some(1000),
                ignore_patterns: Some(vec!["*.tmp".to_string(), "*.log".to_string()]),
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(ctx.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_max_depth() {
        let ctx = OwnedWalkingContext {
            listing: ListingOptions {
                max_depth: Some(0),
                ..Default::default()
            },
            ..Default::default()
        };

        let result = ctx.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("max_depth cannot be 0"));
    }

    #[test]
    fn test_validation_invalid_file_size_range() {
        let ctx = OwnedWalkingContext {
            filtering: FilteringOptions {
                min_file_size: Some(1000),
                max_file_size: Some(100),
                ..Default::default()
            },
            ..Default::default()
        };

        let result = ctx.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("min_file_size"));
        assert!(error_msg.contains("max_file_size"));
    }

    #[test]
    fn test_validation_empty_patterns() {
        let ctx = OwnedWalkingContext {
            filtering: FilteringOptions {
                ignore_patterns: Some(vec!["*.tmp".to_string(), "".to_string()]),
                ..Default::default()
            },
            ..Default::default()
        };

        let result = ctx.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty strings"));
    }

    #[test]
    fn test_borrowed_context_creation() {
        let listing = ListingOptions::default();
        let filtering = FilteringOptions::default();
        let metadata = MetadataOptions::default();

        let ctx = WalkingContext::new(&listing, &filtering, &metadata);

        // Verify references work
        assert_eq!(ctx.listing.max_depth, None);
        assert!(!ctx.listing.show_hidden);
    }

    #[test]
    fn test_conversion_from_borrowed_to_owned() {
        let listing = ListingOptions {
            max_depth: Some(2),
            show_hidden: true,
            ..Default::default()
        };
        let filtering = FilteringOptions::default();
        let metadata = MetadataOptions::default();

        let borrowed_ctx = WalkingContext::new(&listing, &filtering, &metadata);
        let owned_ctx: OwnedWalkingContext = borrowed_ctx.into();

        assert_eq!(owned_ctx.listing.max_depth, Some(2));
        assert!(owned_ctx.listing.show_hidden);
    }

    #[test]
    fn test_as_borrowed_method() {
        let owned_ctx = OwnedWalkingContext {
            listing: ListingOptions {
                max_depth: Some(3),
                ..Default::default()
            },
            ..Default::default()
        };

        let borrowed_ctx = owned_ctx.as_borrowed();
        assert_eq!(borrowed_ctx.listing.max_depth, Some(3));
    }

    #[test]
    fn test_cache_invalidation() {
        let mut ctx = OwnedWalkingContext {
            filtering: FilteringOptions {
                ignore_patterns: Some(vec!["*.tmp".to_string()]),
                ..Default::default()
            },
            ..Default::default()
        };

        // Cache should be initially None
        assert!(ctx.compiled_ignore_patterns.is_none());

        // This should trigger compilation and caching
        let _ = ctx.ignore_patterns().unwrap();
        assert!(ctx.compiled_ignore_patterns.is_some());

        // Invalidate cache
        ctx.invalidate_pattern_cache();
        assert!(ctx.compiled_ignore_patterns.is_none());
    }
}
