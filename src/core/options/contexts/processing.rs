use super::{FormattingContext, SortingContext, WalkingContext};
use super::{OwnedFormattingContext, OwnedSortingContext, OwnedWalkingContext};

/// Composite context for full tree processing operations (borrowed references)
///
/// This context combines all the individual contexts needed for a complete
/// tree processing pipeline: walking, optional sorting, and formatting.
/// It uses borrowed references for efficient CLI operations.
#[derive(Debug)]
pub struct ProcessingContext<'a> {
    pub walking: WalkingContext<'a>,
    pub sorting: Option<SortingContext<'a>>,
    pub formatting: FormattingContext<'a>,
}

impl<'a> ProcessingContext<'a> {
    /// Create a new processing context
    pub fn new(
        walking: WalkingContext<'a>,
        sorting: Option<SortingContext<'a>>,
        formatting: FormattingContext<'a>,
    ) -> Self {
        Self {
            walking,
            sorting,
            formatting,
        }
    }

    /// Check if sorting is enabled in this processing context
    pub fn has_sorting(&self) -> bool {
        self.sorting.is_some()
    }

    /// Get the sorting context if it exists
    pub fn sorting_context(&self) -> Option<&SortingContext<'a>> {
        self.sorting.as_ref()
    }
}

/// Owned version for complex operations
///
/// This context owns all its data and provides complete independence
/// for applications where contexts may need to be modified, cached,
/// or passed between threads.
#[derive(Debug, Clone)]
pub struct OwnedProcessingContext {
    pub walking: OwnedWalkingContext,
    pub sorting: Option<OwnedSortingContext>,
    pub formatting: OwnedFormattingContext,
}

impl OwnedProcessingContext {
    /// Create a new owned processing context
    pub fn new(
        walking: OwnedWalkingContext,
        sorting: Option<OwnedSortingContext>,
        formatting: OwnedFormattingContext,
    ) -> Self {
        Self {
            walking,
            sorting,
            formatting,
        }
    }

    /// Check if sorting is enabled in this processing context
    pub fn has_sorting(&self) -> bool {
        self.sorting.is_some()
    }

    /// Get the sorting context if it exists
    pub fn sorting_context(&self) -> Option<&OwnedSortingContext> {
        self.sorting.as_ref()
    }

    /// Get mutable access to the sorting context if it exists
    pub fn sorting_context_mut(&mut self) -> Option<&mut OwnedSortingContext> {
        self.sorting.as_mut()
    }

    /// Enable sorting with the given context
    pub fn enable_sorting(&mut self, sorting: OwnedSortingContext) {
        self.sorting = Some(sorting);
    }

    /// Disable sorting
    pub fn disable_sorting(&mut self) {
        self.sorting = None;
    }

    /// Validate all contexts for consistency and correctness
    ///
    /// This method validates all component contexts and checks for
    /// any cross-context inconsistencies.
    pub fn validate(&self) -> Result<(), String> {
        // Validate individual contexts
        self.walking.validate()?;
        self.formatting.validate()?;

        if let Some(ref sorting) = self.sorting {
            sorting.validate()?;
        }

        // Cross-context validation
        // Check that metadata requirements are consistent between walking and formatting
        if self.formatting.metadata.show_size_bytes && !self.walking.metadata.show_size_bytes {
            return Err(
                "Formatting context requires size display but walking context doesn't collect size"
                    .to_string(),
            );
        }

        if self.formatting.metadata.show_last_modified && !self.walking.metadata.show_last_modified
        {
            return Err("Formatting context requires modification time display but walking context doesn't collect it".to_string());
        }

        if self.formatting.metadata.calculate_line_count
            && !self.walking.metadata.calculate_line_count
        {
            return Err("Formatting context requires line count display but walking context doesn't calculate it".to_string());
        }

        if self.formatting.metadata.calculate_word_count
            && !self.walking.metadata.calculate_word_count
        {
            return Err("Formatting context requires word count display but walking context doesn't calculate it".to_string());
        }

        // Ensure that the formatting context does not require *more* depth than the walking
        // context is configured to provide.  It is perfectly valid for the walking context to
        // limit recursion (e.g. `max_depth = Some(2)`) while the formatting context keeps the
        // default of `None` (unlimited) – in that case the additional items simply won't be
        // available and no inconsistency arises.  We therefore only fail validation when **both
        // values are specified** and differ.
        if let (Some(walking_depth), Some(formatting_depth)) = (
            self.walking.listing.max_depth,
            self.formatting.listing.max_depth,
        ) {
            // Allow the formatting context to request *less* depth than the
            // walker collects – the additional information is simply ignored
            // during rendering.  Reject only if the formatter would need more
            // depth than is available.
            if formatting_depth > walking_depth {
                return Err(
                    "Walking and formatting contexts have inconsistent max_depth settings"
                        .to_string(),
                );
            }
        }

        Ok(())
    }

    /// Create borrowed contexts from this owned context
    ///
    /// This allows you to use the owned context with APIs that expect
    /// borrowed contexts, providing flexibility in how you manage context lifetimes.
    pub fn as_borrowed(&self) -> ProcessingContext<'_> {
        ProcessingContext {
            walking: self.walking.as_borrowed(),
            sorting: self.sorting.as_ref().map(|s| s.as_borrowed()),
            formatting: self.formatting.as_borrowed(),
        }
    }

    /// Optimize the context for repeated operations
    ///
    /// This method pre-compiles patterns and performs other optimizations
    /// that benefit repeated tree processing operations.
    pub fn optimize(&mut self) -> Result<(), crate::core::error::RustreeError> {
        // Pre-compile patterns in walking context
        let _ = self.walking.ignore_patterns()?;
        let _ = self.walking.match_patterns()?;

        Ok(())
    }
}

impl Default for OwnedProcessingContext {
    fn default() -> Self {
        Self::new(
            OwnedWalkingContext::default(),
            None, // No sorting by default
            OwnedFormattingContext::default(),
        )
    }
}

impl<'a> From<ProcessingContext<'a>> for OwnedProcessingContext {
    fn from(ctx: ProcessingContext<'a>) -> Self {
        Self::new(
            ctx.walking.into(),
            ctx.sorting.map(|s| s.into()),
            ctx.formatting.into(),
        )
    }
}

impl
    TryFrom<(
        OwnedWalkingContext,
        Option<OwnedSortingContext>,
        OwnedFormattingContext,
    )> for OwnedProcessingContext
{
    type Error = String;

    fn try_from(
        (walking, sorting, formatting): (
            OwnedWalkingContext,
            Option<OwnedSortingContext>,
            OwnedFormattingContext,
        ),
    ) -> Result<Self, Self::Error> {
        let owned = Self::new(walking, sorting, formatting);
        owned.validate()?;
        Ok(owned)
    }
}

/// Builder for creating processing contexts programmatically
///
/// This builder provides a fluent interface for constructing processing
/// contexts, which is especially useful for interactive applications where
/// users might configure options incrementally.
pub struct ProcessingContextBuilder {
    walking: Option<OwnedWalkingContext>,
    sorting: Option<OwnedSortingContext>,
    formatting: Option<OwnedFormattingContext>,
}

impl ProcessingContextBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            walking: None,
            sorting: None,
            formatting: None,
        }
    }

    /// Set the walking context
    pub fn with_walking(mut self, walking: OwnedWalkingContext) -> Self {
        self.walking = Some(walking);
        self
    }

    /// Set the sorting context
    pub fn with_sorting(mut self, sorting: OwnedSortingContext) -> Self {
        self.sorting = Some(sorting);
        self
    }

    /// Set the formatting context
    pub fn with_formatting(mut self, formatting: OwnedFormattingContext) -> Self {
        self.formatting = Some(formatting);
        self
    }

    /// Enable sorting with default options
    pub fn with_default_sorting(mut self) -> Self {
        self.sorting = Some(OwnedSortingContext::default());
        self
    }

    /// Build the processing context
    ///
    /// Returns an error if required contexts (walking and formatting) are missing.
    pub fn build(self) -> Result<OwnedProcessingContext, String> {
        let walking = self.walking.ok_or("Walking context is required")?;
        let formatting = self.formatting.ok_or("Formatting context is required")?;

        let context = OwnedProcessingContext::new(walking, self.sorting, formatting);

        // Validate the built context
        context.validate()?;

        Ok(context)
    }

    /// Build the processing context without validation
    ///
    /// Use this when you want to build a context that might not be
    /// immediately valid but will be corrected later.
    pub fn build_unchecked(self) -> Result<OwnedProcessingContext, String> {
        let walking = self.walking.ok_or("Walking context is required")?;
        let formatting = self.formatting.ok_or("Formatting context is required")?;

        Ok(OwnedProcessingContext::new(
            walking,
            self.sorting,
            formatting,
        ))
    }
}

impl Default for ProcessingContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::options::*;

    fn create_test_walking_context() -> OwnedWalkingContext {
        OwnedWalkingContext::new(
            ListingOptions {
                max_depth: Some(3),
                show_hidden: false,
                ..Default::default()
            },
            FilteringOptions::default(),
            MetadataOptions {
                show_size_bytes: true,
                calculate_line_count: true,
                ..Default::default()
            },
        )
    }

    fn create_test_formatting_context() -> OwnedFormattingContext {
        OwnedFormattingContext::new(
            InputSourceOptions {
                root_display_name: "test".to_string(),
                ..Default::default()
            },
            ListingOptions {
                max_depth: Some(3), // Consistent with walking
                ..Default::default()
            },
            MetadataOptions {
                show_size_bytes: true,      // Consistent with walking
                calculate_line_count: true, // Consistent with walking
                ..Default::default()
            },
            MiscOptions::default(),
            HtmlOptions::default(),
        )
    }

    #[test]
    fn test_owned_processing_context_creation() {
        let walking = create_test_walking_context();
        let sorting = Some(OwnedSortingContext::new(SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        }));
        let formatting = create_test_formatting_context();

        let ctx = OwnedProcessingContext::new(walking, sorting, formatting);

        assert!(ctx.has_sorting());
        assert_eq!(ctx.walking.listing.max_depth, Some(3));
        assert_eq!(ctx.formatting.input_source.root_display_name, "test");
        assert!(ctx.sorting_context().unwrap().is_sorting_enabled());
    }

    #[test]
    fn test_validation_consistent_contexts() {
        let walking = create_test_walking_context();
        let formatting = create_test_formatting_context();

        let ctx = OwnedProcessingContext::new(walking, None, formatting);
        assert!(ctx.validate().is_ok());
    }

    #[test]
    fn test_validation_inconsistent_metadata() {
        let walking = OwnedWalkingContext::new(
            ListingOptions::default(),
            FilteringOptions::default(),
            MetadataOptions {
                show_size_bytes: false, // Not collecting size
                ..Default::default()
            },
        );

        let formatting = OwnedFormattingContext::new(
            InputSourceOptions {
                root_display_name: "test".to_string(),
                ..Default::default()
            },
            ListingOptions::default(),
            MetadataOptions {
                show_size_bytes: true, // But wants to display size
                ..Default::default()
            },
            MiscOptions::default(),
            HtmlOptions::default(),
        );

        let ctx = OwnedProcessingContext::new(walking, None, formatting);
        let result = ctx.validate();

        assert!(result.is_err());
        let error = result.unwrap_err();
        println!("Actual error: {}", error);
        assert!(error.contains("size display"));
    }

    #[test]
    fn test_validation_inconsistent_depth() {
        let walking = OwnedWalkingContext::new(
            ListingOptions {
                max_depth: Some(2),
                ..Default::default()
            },
            FilteringOptions::default(),
            MetadataOptions::default(),
        );

        let formatting = OwnedFormattingContext::new(
            InputSourceOptions {
                root_display_name: "test".to_string(),
                ..Default::default()
            },
            ListingOptions {
                max_depth: Some(5), // Different depth
                ..Default::default()
            },
            MetadataOptions::default(),
            MiscOptions::default(),
            HtmlOptions::default(),
        );

        let ctx = OwnedProcessingContext::new(walking, None, formatting);
        let result = ctx.validate();

        assert!(result.is_err());
        let error = result.unwrap_err();
        println!("Actual error: {}", error);
        assert!(error.contains("inconsistent max_depth"));
    }

    #[test]
    fn test_builder_complete_build() {
        let walking = create_test_walking_context();
        let sorting = OwnedSortingContext::default();
        let formatting = create_test_formatting_context();

        let ctx = ProcessingContextBuilder::new()
            .with_walking(walking)
            .with_sorting(sorting)
            .with_formatting(formatting)
            .build();

        assert!(ctx.is_ok());
        let ctx = ctx.unwrap();
        assert!(ctx.has_sorting());
    }

    #[test]
    fn test_builder_missing_required_context() {
        let result = ProcessingContextBuilder::new()
            .with_walking(create_test_walking_context())
            // Missing formatting context
            .build();

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Formatting context is required")
        );
    }

    #[test]
    fn test_builder_with_default_sorting() {
        let walking = create_test_walking_context();
        let formatting = create_test_formatting_context();

        let ctx = ProcessingContextBuilder::new()
            .with_walking(walking)
            .with_formatting(formatting)
            .with_default_sorting()
            .build();

        assert!(ctx.is_ok());
        let ctx = ctx.unwrap();
        assert!(ctx.has_sorting());
    }

    #[test]
    fn test_borrowed_context_creation() {
        let walking_opts = ListingOptions::default();
        let filtering_opts = FilteringOptions::default();
        let metadata_opts = MetadataOptions::default();
        let input_opts = InputSourceOptions::default();
        let misc_opts = MiscOptions::default();
        let html_opts = HtmlOptions::default();

        let walking = WalkingContext::new(&walking_opts, &filtering_opts, &metadata_opts);
        let formatting = FormattingContext::new(
            &input_opts,
            &walking_opts,
            &metadata_opts,
            &misc_opts,
            &html_opts,
        );

        let ctx = ProcessingContext::new(walking, None, formatting);

        assert!(!ctx.has_sorting());
        assert!(ctx.sorting_context().is_none());
    }

    #[test]
    fn test_conversion_from_borrowed_to_owned() {
        let walking_opts = ListingOptions::default();
        let filtering_opts = FilteringOptions::default();
        let metadata_opts = MetadataOptions::default();
        let input_opts = InputSourceOptions::default();
        let misc_opts = MiscOptions::default();
        let html_opts = HtmlOptions::default();

        let walking = WalkingContext::new(&walking_opts, &filtering_opts, &metadata_opts);
        let formatting = FormattingContext::new(
            &input_opts,
            &walking_opts,
            &metadata_opts,
            &misc_opts,
            &html_opts,
        );

        let borrowed_ctx = ProcessingContext::new(walking, None, formatting);
        let owned_ctx: OwnedProcessingContext = borrowed_ctx.into();

        assert!(!owned_ctx.has_sorting());
    }

    #[test]
    fn test_as_borrowed_method() {
        let ctx = OwnedProcessingContext::default();
        let borrowed_ctx = ctx.as_borrowed();

        assert!(!borrowed_ctx.has_sorting());
    }

    #[test]
    fn test_enable_disable_sorting() {
        let mut ctx = OwnedProcessingContext::default();

        assert!(!ctx.has_sorting());

        ctx.enable_sorting(OwnedSortingContext::default());
        assert!(ctx.has_sorting());

        ctx.disable_sorting();
        assert!(!ctx.has_sorting());
    }

    #[test]
    fn test_optimization() {
        let mut walking = create_test_walking_context();
        walking.filtering.ignore_patterns = Some(vec!["*.tmp".to_string()]);

        let mut ctx = OwnedProcessingContext::new(walking, None, create_test_formatting_context());

        assert!(ctx.optimize().is_ok());

        // Patterns should now be compiled and cached
        // Note: compiled_ignore_patterns is private, but we can verify through behavior
        // The optimize() method should succeed, indicating patterns were compiled
        assert!(ctx.optimize().is_ok());
    }
}
