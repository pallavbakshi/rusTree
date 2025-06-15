//! # Context Diff for State Management
//!
//! This module provides functionality to detect changes between context states,
//! which is essential for interactive applications that need to react to configuration
//! changes efficiently. Instead of rebuilding everything when any setting changes,
//! the diff can tell us exactly what changed and allow for optimized updates.

use super::{
    OwnedFormattingContext, OwnedProcessingContext, OwnedSortingContext, OwnedWalkingContext,
};

/// Represents all possible changes between two contexts
#[derive(Debug, Clone, PartialEq)]
pub enum ContextDiff {
    WalkingChanged(WalkingContextDiff),
    FormattingChanged(FormattingContextDiff),
    SortingChanged(SortingContextDiff),
    ProcessingChanged(ProcessingContextDiff),
}

/// Detailed changes in walking context
#[derive(Debug, Clone, PartialEq)]
pub struct WalkingContextDiff {
    // Listing changes
    pub max_depth_changed: bool,
    pub show_hidden_changed: bool,
    pub list_directories_only_changed: bool,
    pub show_full_path_changed: bool,

    // Filtering changes (these invalidate pattern compilation)
    pub ignore_patterns_changed: bool,
    pub match_patterns_changed: bool,
    pub case_insensitive_filter_changed: bool,
    pub prune_empty_directories_changed: bool,
    pub min_file_size_changed: bool,
    pub max_file_size_changed: bool,

    // Metadata changes
    pub show_size_bytes_changed: bool,
    pub show_last_modified_changed: bool,
    pub calculate_line_count_changed: bool,
    pub calculate_word_count_changed: bool,
    pub apply_function_changed: bool,
    pub human_readable_size_changed: bool,
    pub report_permissions_changed: bool,
    pub report_change_time_changed: bool,
    pub report_creation_time_changed: bool,
}

impl WalkingContextDiff {
    /// Check if any pattern-related changes occurred that would require recompilation
    pub fn requires_pattern_recompilation(&self) -> bool {
        self.ignore_patterns_changed
            || self.match_patterns_changed
            || self.case_insensitive_filter_changed
            || self.show_hidden_changed // show_hidden affects pattern compilation
    }

    /// Check if any changes require re-walking the directory tree
    pub fn requires_directory_rescan(&self) -> bool {
        self.max_depth_changed
            || self.show_hidden_changed
            || self.list_directories_only_changed
            || self.requires_pattern_recompilation()
            || self.prune_empty_directories_changed
            || self.min_file_size_changed
            || self.max_file_size_changed
    }

    /// Check if any changes only affect metadata collection (no rescan needed)
    pub fn affects_only_metadata(&self) -> bool {
        !self.requires_directory_rescan()
            && (self.show_size_bytes_changed
                || self.show_last_modified_changed
                || self.calculate_line_count_changed
                || self.calculate_word_count_changed
                || self.apply_function_changed
                || self.human_readable_size_changed
                || self.report_permissions_changed
                || self.report_change_time_changed
                || self.report_creation_time_changed)
    }

    /// Check if any changes occurred at all
    pub fn has_changes(&self) -> bool {
        self.max_depth_changed
            || self.show_hidden_changed
            || self.list_directories_only_changed
            || self.show_full_path_changed
            || self.ignore_patterns_changed
            || self.match_patterns_changed
            || self.case_insensitive_filter_changed
            || self.prune_empty_directories_changed
            || self.min_file_size_changed
            || self.max_file_size_changed
            || self.show_size_bytes_changed
            || self.show_last_modified_changed
            || self.calculate_line_count_changed
            || self.calculate_word_count_changed
            || self.apply_function_changed
            || self.human_readable_size_changed
            || self.report_permissions_changed
            || self.report_change_time_changed
            || self.report_creation_time_changed
    }
}

/// Detailed changes in formatting context
#[derive(Debug, Clone, PartialEq)]
pub struct FormattingContextDiff {
    // Input source changes
    pub root_display_name_changed: bool,
    pub root_is_directory_changed: bool,
    pub root_node_size_changed: bool,

    // Listing display changes
    pub max_depth_display_changed: bool,
    pub show_hidden_display_changed: bool,
    pub list_directories_only_display_changed: bool,
    pub show_full_path_display_changed: bool,

    // Metadata display changes
    pub show_size_bytes_display_changed: bool,
    pub show_last_modified_display_changed: bool,
    pub calculate_line_count_display_changed: bool,
    pub calculate_word_count_display_changed: bool,
    pub apply_function_display_changed: bool,
    pub human_readable_size_display_changed: bool,
    pub report_permissions_display_changed: bool,
    pub report_change_time_display_changed: bool,
    pub report_creation_time_display_changed: bool,

    // Misc output changes
    pub no_summary_report_changed: bool,
    pub human_friendly_changed: bool,
    pub no_color_changed: bool,
    pub verbose_changed: bool,

    // HTML-specific changes
    pub include_links_changed: bool,
    pub base_href_changed: bool,
    pub strip_first_component_changed: bool,
    pub custom_intro_changed: bool,
    pub custom_outro_changed: bool,
}

impl FormattingContextDiff {
    /// Check if changes require complete reformatting (vs just style changes)
    pub fn requires_reformatting(&self) -> bool {
        self.root_display_name_changed
            || self.root_is_directory_changed
            || self.root_node_size_changed
            || self.max_depth_display_changed
            || self.show_hidden_display_changed
            || self.list_directories_only_display_changed
            || self.show_full_path_display_changed
            || self.show_size_bytes_display_changed
            || self.show_last_modified_display_changed
            || self.calculate_line_count_display_changed
            || self.calculate_word_count_display_changed
            || self.apply_function_display_changed
            || self.human_readable_size_display_changed
            || self.report_permissions_display_changed
            || self.report_change_time_display_changed
            || self.report_creation_time_display_changed
            || self.no_summary_report_changed
            || self.verbose_changed
    }

    /// Check if changes only affect styling (color, HTML presentation, etc.)
    pub fn affects_only_styling(&self) -> bool {
        !self.requires_reformatting()
            && (self.human_friendly_changed
                || self.no_color_changed
                || self.include_links_changed
                || self.base_href_changed
                || self.strip_first_component_changed
                || self.custom_intro_changed
                || self.custom_outro_changed)
    }

    /// Check if any changes occurred
    pub fn has_changes(&self) -> bool {
        self.requires_reformatting() || self.affects_only_styling()
    }
}

/// Detailed changes in sorting context
#[derive(Debug, Clone, PartialEq)]
pub struct SortingContextDiff {
    pub sort_by_changed: bool,
    pub reverse_sort_changed: bool,
    pub files_before_directories_changed: bool,
    pub directory_file_order_changed: bool,
}

impl SortingContextDiff {
    /// Check if any sorting changes occurred
    pub fn has_changes(&self) -> bool {
        self.sort_by_changed
            || self.reverse_sort_changed
            || self.files_before_directories_changed
            || self.directory_file_order_changed
    }

    /// Check if changes require complete re-sorting (vs just order reversal)
    pub fn requires_resort(&self) -> bool {
        self.sort_by_changed
            || self.files_before_directories_changed
            || self.directory_file_order_changed
    }

    /// Check if only reverse order changed (can optimize by just reversing)
    pub fn only_reverse_changed(&self) -> bool {
        self.reverse_sort_changed
            && !self.sort_by_changed
            && !self.files_before_directories_changed
            && !self.directory_file_order_changed
    }
}

/// Composite diff for processing context
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessingContextDiff {
    pub walking: Option<WalkingContextDiff>,
    pub sorting: Option<SortingContextDiff>,
    pub formatting: Option<FormattingContextDiff>,
    pub sorting_added: bool,   // sorting was None, now Some
    pub sorting_removed: bool, // sorting was Some, now None
}

impl ProcessingContextDiff {
    /// Check if any changes require a complete rebuild from scratch
    pub fn requires_complete_rebuild(&self) -> bool {
        if let Some(ref walking_diff) = self.walking {
            if walking_diff.requires_directory_rescan() {
                return true;
            }
        }
        self.sorting_added || self.sorting_removed
    }

    /// Check if changes can be handled by re-sorting existing nodes
    pub fn can_optimize_with_resort(&self) -> bool {
        !self.requires_complete_rebuild() && self.sorting.as_ref().is_some_and(|s| s.has_changes())
    }

    /// Check if changes only require reformatting existing data
    pub fn only_requires_reformatting(&self) -> bool {
        !self.requires_complete_rebuild()
            && !self.can_optimize_with_resort()
            && self.formatting.as_ref().is_some_and(|f| f.has_changes())
    }

    /// Check if any changes occurred
    pub fn has_changes(&self) -> bool {
        self.walking.as_ref().is_some_and(|w| w.has_changes())
            || self.sorting.as_ref().is_some_and(|s| s.has_changes())
            || self.formatting.as_ref().is_some_and(|f| f.has_changes())
            || self.sorting_added
            || self.sorting_removed
    }
}

impl OwnedWalkingContext {
    /// Compare with another walking context and return detailed diff
    pub fn diff(&self, other: &Self) -> WalkingContextDiff {
        WalkingContextDiff {
            // Listing changes
            max_depth_changed: self.listing.max_depth != other.listing.max_depth,
            show_hidden_changed: self.listing.show_hidden != other.listing.show_hidden,
            list_directories_only_changed: self.listing.list_directories_only
                != other.listing.list_directories_only,
            show_full_path_changed: self.listing.show_full_path != other.listing.show_full_path,

            // Filtering changes
            ignore_patterns_changed: self.filtering.ignore_patterns
                != other.filtering.ignore_patterns,
            match_patterns_changed: self.filtering.match_patterns != other.filtering.match_patterns,
            case_insensitive_filter_changed: self.filtering.case_insensitive_filter
                != other.filtering.case_insensitive_filter,
            prune_empty_directories_changed: self.filtering.prune_empty_directories
                != other.filtering.prune_empty_directories,
            min_file_size_changed: self.filtering.min_file_size != other.filtering.min_file_size,
            max_file_size_changed: self.filtering.max_file_size != other.filtering.max_file_size,

            // Metadata changes
            show_size_bytes_changed: self.metadata.show_size_bytes
                != other.metadata.show_size_bytes,
            show_last_modified_changed: self.metadata.show_last_modified
                != other.metadata.show_last_modified,
            calculate_line_count_changed: self.metadata.calculate_line_count
                != other.metadata.calculate_line_count,
            calculate_word_count_changed: self.metadata.calculate_word_count
                != other.metadata.calculate_word_count,
            apply_function_changed: self.metadata.apply_function != other.metadata.apply_function,
            human_readable_size_changed: self.metadata.human_readable_size
                != other.metadata.human_readable_size,
            report_permissions_changed: self.metadata.report_permissions
                != other.metadata.report_permissions,
            report_change_time_changed: self.metadata.report_change_time
                != other.metadata.report_change_time,
            report_creation_time_changed: self.metadata.report_creation_time
                != other.metadata.report_creation_time,
        }
    }
}

impl OwnedFormattingContext {
    /// Compare with another formatting context and return detailed diff
    pub fn diff(&self, other: &Self) -> FormattingContextDiff {
        FormattingContextDiff {
            // Input source changes
            root_display_name_changed: self.input_source.root_display_name
                != other.input_source.root_display_name,
            root_is_directory_changed: self.input_source.root_is_directory
                != other.input_source.root_is_directory,
            root_node_size_changed: self.input_source.root_node_size
                != other.input_source.root_node_size,

            // Listing display changes
            max_depth_display_changed: self.listing.max_depth != other.listing.max_depth,
            show_hidden_display_changed: self.listing.show_hidden != other.listing.show_hidden,
            list_directories_only_display_changed: self.listing.list_directories_only
                != other.listing.list_directories_only,
            show_full_path_display_changed: self.listing.show_full_path
                != other.listing.show_full_path,

            // Metadata display changes
            show_size_bytes_display_changed: self.metadata.show_size_bytes
                != other.metadata.show_size_bytes,
            show_last_modified_display_changed: self.metadata.show_last_modified
                != other.metadata.show_last_modified,
            calculate_line_count_display_changed: self.metadata.calculate_line_count
                != other.metadata.calculate_line_count,
            calculate_word_count_display_changed: self.metadata.calculate_word_count
                != other.metadata.calculate_word_count,
            apply_function_display_changed: self.metadata.apply_function
                != other.metadata.apply_function,
            human_readable_size_display_changed: self.metadata.human_readable_size
                != other.metadata.human_readable_size,
            report_permissions_display_changed: self.metadata.report_permissions
                != other.metadata.report_permissions,
            report_change_time_display_changed: self.metadata.report_change_time
                != other.metadata.report_change_time,
            report_creation_time_display_changed: self.metadata.report_creation_time
                != other.metadata.report_creation_time,

            // Misc output changes
            no_summary_report_changed: self.misc.no_summary_report != other.misc.no_summary_report,
            human_friendly_changed: self.misc.human_friendly != other.misc.human_friendly,
            no_color_changed: self.misc.no_color != other.misc.no_color,
            verbose_changed: self.misc.verbose != other.misc.verbose,

            // HTML-specific changes
            include_links_changed: self.html.include_links != other.html.include_links,
            base_href_changed: self.html.base_href != other.html.base_href,
            strip_first_component_changed: self.html.strip_first_component
                != other.html.strip_first_component,
            custom_intro_changed: self.html.custom_intro != other.html.custom_intro,
            custom_outro_changed: self.html.custom_outro != other.html.custom_outro,
        }
    }
}

impl OwnedSortingContext {
    /// Compare with another sorting context and return detailed diff
    pub fn diff(&self, other: &Self) -> SortingContextDiff {
        SortingContextDiff {
            sort_by_changed: self.sorting.sort_by != other.sorting.sort_by,
            reverse_sort_changed: self.sorting.reverse_sort != other.sorting.reverse_sort,
            files_before_directories_changed: self.sorting.files_before_directories
                != other.sorting.files_before_directories,
            directory_file_order_changed: self.sorting.directory_file_order
                != other.sorting.directory_file_order,
        }
    }
}

impl OwnedProcessingContext {
    /// Compare with another processing context and return comprehensive diff
    pub fn diff(&self, other: &Self) -> ProcessingContextDiff {
        let walking_diff = Some(self.walking.diff(&other.walking));

        let sorting_diff = match (&self.sorting, &other.sorting) {
            (Some(self_sort), Some(other_sort)) => Some(self_sort.diff(other_sort)),
            _ => None,
        };

        let formatting_diff = Some(self.formatting.diff(&other.formatting));

        let sorting_added = self.sorting.is_none() && other.sorting.is_some();
        let sorting_removed = self.sorting.is_some() && other.sorting.is_none();

        ProcessingContextDiff {
            walking: walking_diff,
            sorting: sorting_diff,
            formatting: formatting_diff,
            sorting_added,
            sorting_removed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::options::*;

    #[test]
    fn test_walking_context_diff_basic() {
        let ctx1 = OwnedWalkingContext::new(
            ListingOptions {
                max_depth: Some(2),
                show_hidden: false,
                ..Default::default()
            },
            FilteringOptions {
                ignore_patterns: Some(vec!["*.tmp".to_string()]),
                ..Default::default()
            },
            MetadataOptions {
                show_size_bytes: true,
                ..Default::default()
            },
        );

        let ctx2 = OwnedWalkingContext::new(
            ListingOptions {
                max_depth: Some(3),
                show_hidden: true,
                ..Default::default()
            },
            FilteringOptions {
                ignore_patterns: Some(vec!["*.log".to_string()]),
                ..Default::default()
            },
            MetadataOptions {
                show_size_bytes: false,
                ..Default::default()
            },
        );

        let diff = ctx1.diff(&ctx2);

        assert!(diff.max_depth_changed);
        assert!(diff.show_hidden_changed);
        assert!(diff.ignore_patterns_changed);
        assert!(diff.show_size_bytes_changed);
        assert!(diff.requires_directory_rescan());
        assert!(diff.requires_pattern_recompilation());
    }

    #[test]
    fn test_walking_context_diff_metadata_only() {
        let ctx1 = OwnedWalkingContext::new(
            ListingOptions {
                max_depth: Some(2),
                ..Default::default()
            },
            FilteringOptions::default(),
            MetadataOptions {
                show_size_bytes: true,
                calculate_line_count: false,
                ..Default::default()
            },
        );

        let ctx2 = OwnedWalkingContext::new(
            ListingOptions {
                max_depth: Some(2),
                ..Default::default()
            },
            FilteringOptions::default(),
            MetadataOptions {
                show_size_bytes: false,
                calculate_line_count: true,
                ..Default::default()
            },
        );

        let diff = ctx1.diff(&ctx2);

        assert!(!diff.requires_directory_rescan());
        assert!(diff.affects_only_metadata());
        assert!(diff.show_size_bytes_changed);
        assert!(diff.calculate_line_count_changed);
    }

    #[test]
    fn test_sorting_context_diff() {
        let ctx1 = OwnedSortingContext {
            sorting: SortingOptions {
                sort_by: Some(SortKey::Name),
                reverse_sort: false,
                ..Default::default()
            },
        };

        let ctx2 = OwnedSortingContext {
            sorting: SortingOptions {
                sort_by: Some(SortKey::Name),
                reverse_sort: true,
                ..Default::default()
            },
        };

        let diff = ctx1.diff(&ctx2);

        assert!(diff.only_reverse_changed());
        assert!(!diff.requires_resort());
        assert!(diff.has_changes());
    }

    #[test]
    fn test_formatting_context_diff() {
        let ctx1 = OwnedFormattingContext {
            input_source: InputSourceOptions {
                root_display_name: "test1".to_string(),
                ..Default::default()
            },
            misc: MiscOptions {
                no_color: false,
                ..Default::default()
            },
            ..Default::default()
        };

        let ctx2 = OwnedFormattingContext {
            input_source: InputSourceOptions {
                root_display_name: "test2".to_string(),
                ..Default::default()
            },
            misc: MiscOptions {
                no_color: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let diff = ctx1.diff(&ctx2);

        assert!(diff.root_display_name_changed);
        assert!(diff.no_color_changed);
        assert!(diff.requires_reformatting()); // root name change requires reformatting
        assert!(!diff.affects_only_styling()); // because root name changed
    }

    #[test]
    fn test_processing_context_diff_optimization_hints() {
        let ctx1 = OwnedProcessingContext {
            walking: OwnedWalkingContext::new(
                ListingOptions {
                    max_depth: Some(2),
                    ..Default::default()
                },
                FilteringOptions::default(),
                MetadataOptions {
                    show_size_bytes: true,
                    ..Default::default()
                },
            ),
            sorting: Some(OwnedSortingContext {
                sorting: SortingOptions {
                    sort_by: Some(SortKey::Name),
                    reverse_sort: false,
                    ..Default::default()
                },
            }),
            formatting: OwnedFormattingContext::default(),
        };

        let ctx2 = OwnedProcessingContext {
            walking: ctx1.walking.clone(),
            sorting: Some(OwnedSortingContext {
                sorting: SortingOptions {
                    sort_by: Some(SortKey::Size), // Only sorting changed
                    reverse_sort: false,
                    ..Default::default()
                },
            }),
            formatting: ctx1.formatting.clone(),
        };

        let diff = ctx1.diff(&ctx2);

        assert!(!diff.requires_complete_rebuild());
        assert!(diff.can_optimize_with_resort());
        assert!(!diff.only_requires_reformatting());
    }
}
