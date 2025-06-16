use crate::core::options::SortingOptions;

/// Context for sorting operations (borrowed references)
///
/// This context provides all the information needed for sorting operations
/// in a focused, efficient manner. It uses borrowed references to avoid
/// unnecessary cloning for short-lived CLI operations.
#[derive(Debug)]
pub struct SortingContext<'a> {
    pub sorting: &'a SortingOptions,
}

impl<'a> SortingContext<'a> {
    /// Create a new sorting context from sorting options reference
    pub fn new(sorting: &'a SortingOptions) -> Self {
        Self { sorting }
    }

    /// Check if sorting is enabled
    pub fn is_sorting_enabled(&self) -> bool {
        self.sorting.sort_by.is_some()
    }
}

/// Owned version for advanced scenarios
///
/// This context owns its data and is designed for scenarios where
/// the context needs to live independently or be modified over time,
/// such as in interactive applications where users can change sorting options.
#[derive(Debug, Clone)]
pub struct OwnedSortingContext {
    pub sorting: SortingOptions,
}

impl OwnedSortingContext {
    /// Create a new owned sorting context
    pub fn new(sorting: SortingOptions) -> Self {
        Self { sorting }
    }

    /// Check if sorting is enabled
    pub fn is_sorting_enabled(&self) -> bool {
        self.sorting.sort_by.is_some()
    }

    /// Validate context for consistency and correctness
    ///
    /// This method checks for invalid combinations of sorting options
    /// and provides helpful error messages for fixing configuration issues.
    pub fn validate(&self) -> Result<(), String> {
        // Currently, all combinations of sorting options are valid
        // This method is here for future validation needs
        Ok(())
    }

    /// Create a borrowed context from this owned context
    ///
    /// This allows you to use the owned context with APIs that expect
    /// borrowed contexts, providing flexibility in how you manage context lifetimes.
    pub fn as_borrowed(&self) -> SortingContext<'_> {
        SortingContext {
            sorting: &self.sorting,
        }
    }
}

impl Default for OwnedSortingContext {
    fn default() -> Self {
        Self::new(SortingOptions::default())
    }
}

impl<'a> From<SortingContext<'a>> for OwnedSortingContext {
    fn from(ctx: SortingContext<'a>) -> Self {
        Self::new(ctx.sorting.clone())
    }
}

impl TryFrom<SortingOptions> for OwnedSortingContext {
    type Error = String;

    fn try_from(sorting: SortingOptions) -> Result<Self, Self::Error> {
        let owned = Self::new(sorting);
        owned.validate()?;
        Ok(owned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::options::{DirectoryFileOrder, SortKey};

    #[test]
    fn test_owned_sorting_context_creation() {
        let sorting = SortingOptions {
            sort_by: Some(SortKey::Name),
            reverse_sort: true,
            directory_file_order: DirectoryFileOrder::DirsFirst,
            ..Default::default()
        };

        let ctx = OwnedSortingContext::new(sorting);

        assert_eq!(ctx.sorting.sort_by, Some(SortKey::Name));
        assert!(ctx.sorting.reverse_sort);
        assert_eq!(
            ctx.sorting.directory_file_order,
            DirectoryFileOrder::DirsFirst
        );
    }

    #[test]
    fn test_is_sorting_enabled() {
        let ctx_with_sorting = OwnedSortingContext {
            sorting: SortingOptions {
                sort_by: Some(SortKey::Size),
                ..Default::default()
            },
        };
        assert!(ctx_with_sorting.is_sorting_enabled());

        let ctx_without_sorting = OwnedSortingContext {
            sorting: SortingOptions {
                sort_by: None,
                ..Default::default()
            },
        };
        assert!(!ctx_without_sorting.is_sorting_enabled());
    }

    #[test]
    fn test_validation() {
        let ctx = OwnedSortingContext::default();
        assert!(ctx.validate().is_ok());

        let ctx_with_sorting = OwnedSortingContext {
            sorting: SortingOptions {
                sort_by: Some(SortKey::MTime),
                reverse_sort: true,
                ..Default::default()
            },
        };
        assert!(ctx_with_sorting.validate().is_ok());
    }

    #[test]
    fn test_borrowed_context_creation() {
        let sorting = SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        };

        let ctx = SortingContext::new(&sorting);

        assert_eq!(ctx.sorting.sort_by, Some(SortKey::Name));
        assert!(ctx.is_sorting_enabled());
    }

    #[test]
    fn test_conversion_from_borrowed_to_owned() {
        let sorting = SortingOptions {
            sort_by: Some(SortKey::Size),
            reverse_sort: true,
            ..Default::default()
        };

        let borrowed_ctx = SortingContext::new(&sorting);
        let owned_ctx: OwnedSortingContext = borrowed_ctx.into();

        assert_eq!(owned_ctx.sorting.sort_by, Some(SortKey::Size));
        assert!(owned_ctx.sorting.reverse_sort);
    }

    #[test]
    fn test_as_borrowed_method() {
        let owned_ctx = OwnedSortingContext {
            sorting: SortingOptions {
                sort_by: Some(SortKey::Lines),
                ..Default::default()
            },
        };

        let borrowed_ctx = owned_ctx.as_borrowed();
        assert_eq!(borrowed_ctx.sorting.sort_by, Some(SortKey::Lines));
    }
}
