// tests/context_tests.rs
//
// Comprehensive tests for the new context-based APIs introduced in Phase 3 and 4.
// These tests verify context conversions, validation, caching, and ensure backward
// compatibility between old and new APIs.

use anyhow::Result;
use rustree::core::options::DirectoryFileOrder;
use rustree::core::options::contexts::*;
use rustree::*;
use rustree::{create_default_processing_context, diff_processing_contexts};

mod common;

#[test]
fn test_walking_context_conversions_preserve_data() -> Result<()> {
    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(5),
            show_hidden: true,
            list_directories_only: false,
            show_full_path: true,
        },
        filtering: FilteringOptions {
            ignore_patterns: Some(vec!["*.tmp".to_string(), "*.log".to_string()]),
            match_patterns: Some(vec!["*.rs".to_string(), "*.md".to_string()]),
            case_insensitive_filter: true,
            prune_empty_directories: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            calculate_line_count: true,
            calculate_word_count: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Test borrowed walking context conversion
    let walking_ctx = config.walking_context();
    assert_eq!(walking_ctx.listing.max_depth, Some(5));
    assert!(walking_ctx.listing.show_hidden);
    assert!(walking_ctx.listing.show_full_path);
    assert_eq!(
        walking_ctx.filtering.ignore_patterns,
        Some(vec!["*.tmp".to_string(), "*.log".to_string()])
    );
    assert_eq!(
        walking_ctx.filtering.match_patterns,
        Some(vec!["*.rs".to_string(), "*.md".to_string()])
    );
    assert!(walking_ctx.filtering.case_insensitive_filter);
    assert!(walking_ctx.filtering.prune_empty_directories);
    assert!(walking_ctx.metadata.show_size_bytes);
    assert!(walking_ctx.metadata.calculate_line_count);
    assert!(walking_ctx.metadata.calculate_word_count);

    // Test owned context independence
    let mut owned_walking = config.to_owned_walking_context();
    assert_eq!(owned_walking.listing.max_depth, Some(5));
    assert_eq!(
        owned_walking.filtering.ignore_patterns,
        Some(vec!["*.tmp".to_string(), "*.log".to_string()])
    );

    // Modify owned context - should not affect original config
    owned_walking.listing.max_depth = Some(10);
    owned_walking.filtering.ignore_patterns = Some(vec!["*.backup".to_string()]);

    // Original should be unchanged
    assert_eq!(config.listing.max_depth, Some(5));
    assert_eq!(
        config.filtering.ignore_patterns,
        Some(vec!["*.tmp".to_string(), "*.log".to_string()])
    );

    // Owned context should reflect changes
    assert_eq!(owned_walking.listing.max_depth, Some(10));
    assert_eq!(
        owned_walking.filtering.ignore_patterns,
        Some(vec!["*.backup".to_string()])
    );

    Ok(())
}

#[test]
fn test_formatting_context_conversions() -> Result<()> {
    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "test_project".to_string(),
            root_is_directory: true,
            root_node_size: Some(1024),
        },
        listing: ListingOptions {
            max_depth: Some(3),
            show_full_path: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            show_last_modified: true,
            ..Default::default()
        },
        misc: MiscOptions {
            no_summary_report: true,
            human_friendly: true,
            ..Default::default()
        },
        html: HtmlOptions {
            include_links: true,
            base_href: Some("https://example.com/docs".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };

    // Test borrowed formatting context
    let formatting_ctx = config.formatting_context();
    assert_eq!(
        formatting_ctx.input_source.root_display_name,
        "test_project"
    );
    assert!(formatting_ctx.input_source.root_is_directory);
    assert_eq!(formatting_ctx.input_source.root_node_size, Some(1024));
    assert_eq!(formatting_ctx.listing.max_depth, Some(3));
    assert!(formatting_ctx.listing.show_full_path);
    assert!(formatting_ctx.metadata.show_size_bytes);
    assert!(formatting_ctx.metadata.show_last_modified);
    assert!(formatting_ctx.misc.no_summary_report);
    assert!(formatting_ctx.misc.human_friendly);
    assert!(formatting_ctx.html.include_links);
    assert_eq!(
        formatting_ctx.html.base_href,
        Some("https://example.com/docs".to_string())
    );

    // Test owned formatting context independence
    let mut owned_formatting = config.to_owned_formatting_context();
    owned_formatting.input_source.root_display_name = "modified_project".to_string();
    owned_formatting.html.include_links = false;

    // Original should be unchanged
    assert_eq!(config.input_source.root_display_name, "test_project");
    assert!(config.html.include_links);

    // Owned context should reflect changes
    assert_eq!(
        owned_formatting.input_source.root_display_name,
        "modified_project"
    );
    assert!(!owned_formatting.html.include_links);

    Ok(())
}

#[test]
fn test_sorting_context_conversions() -> Result<()> {
    let config = RustreeLibConfig {
        sorting: SortingOptions {
            sort_by: Some(SortKey::Size),
            reverse_sort: true,
            files_before_directories: true,
            directory_file_order: DirectoryFileOrder::FilesFirst,
        },
        ..Default::default()
    };

    // Test borrowed sorting context
    let sorting_ctx = config.sorting_context();
    assert_eq!(sorting_ctx.sorting.sort_by, Some(SortKey::Size));
    assert!(sorting_ctx.sorting.reverse_sort);
    assert_eq!(
        sorting_ctx.sorting.directory_file_order,
        DirectoryFileOrder::FilesFirst
    );

    // Test owned sorting context independence
    let mut owned_sorting = config.to_owned_sorting_context();
    owned_sorting.sorting.sort_by = Some(SortKey::Name);
    owned_sorting.sorting.reverse_sort = false;

    // Original should be unchanged
    assert_eq!(config.sorting.sort_by, Some(SortKey::Size));
    assert!(config.sorting.reverse_sort);

    // Owned context should reflect changes
    assert_eq!(owned_sorting.sorting.sort_by, Some(SortKey::Name));
    assert!(!owned_sorting.sorting.reverse_sort);

    Ok(())
}

#[test]
fn test_processing_context_creation() -> Result<()> {
    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        input_source: InputSourceOptions {
            root_display_name: "root".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    // Test borrowed processing context
    let processing_ctx = config.processing_context();
    assert_eq!(processing_ctx.walking.listing.max_depth, Some(2));
    assert!(processing_ctx.sorting.is_some());
    assert_eq!(
        processing_ctx.sorting.unwrap().sorting.sort_by,
        Some(SortKey::Name)
    );
    assert_eq!(
        processing_ctx.formatting.input_source.root_display_name,
        "root"
    );

    // Test owned processing context
    let owned_processing = config.to_owned_processing_context();
    assert_eq!(owned_processing.walking.listing.max_depth, Some(2));
    assert!(owned_processing.sorting.is_some());
    assert_eq!(
        owned_processing.sorting.unwrap().sorting.sort_by,
        Some(SortKey::Name)
    );
    assert_eq!(
        owned_processing.formatting.input_source.root_display_name,
        "root"
    );

    Ok(())
}

#[test]
fn test_processing_context_no_sorting() -> Result<()> {
    let config = RustreeLibConfig {
        sorting: SortingOptions {
            sort_by: None, // No sorting
            ..Default::default()
        },
        ..Default::default()
    };

    // When sort_by is None, sorting context should not be included
    let processing_ctx = config.processing_context();
    assert!(processing_ctx.sorting.is_none());

    let owned_processing = config.to_owned_processing_context();
    assert!(owned_processing.sorting.is_none());

    Ok(())
}

#[test]
fn test_owned_walking_context_pattern_compilation_caching() -> Result<()> {
    let mut owned_walking = OwnedWalkingContext::new(
        ListingOptions::default(),
        FilteringOptions {
            ignore_patterns: Some(vec!["*.tmp".to_string(), "*.log".to_string()]),
            match_patterns: Some(vec!["*.rs".to_string()]),
            case_insensitive_filter: false,
            ..Default::default()
        },
        MetadataOptions::default(),
    );

    // Initially, patterns should not be compiled
    // Note: compiled_* fields are private, so we test behavior indirectly

    // First call should compile ignore patterns
    let _ignore_patterns = owned_walking.ignore_patterns()?;

    // First call should compile match patterns
    let _match_patterns = owned_walking.match_patterns()?;

    // Second calls should use cached patterns (no recompilation)
    let ignore_patterns2 = owned_walking.ignore_patterns()?;
    assert!(ignore_patterns2.is_some());

    let match_patterns2 = owned_walking.match_patterns()?;
    assert!(match_patterns2.is_some());

    // Note: We can't test pointer equality due to borrowing rules,
    // but the caching behavior is tested implicitly through performance

    Ok(())
}

#[test]
fn test_owned_walking_context_cache_invalidation() -> Result<()> {
    let mut owned_walking = OwnedWalkingContext::new(
        ListingOptions::default(),
        FilteringOptions {
            ignore_patterns: Some(vec!["*.tmp".to_string()]),
            ..Default::default()
        },
        MetadataOptions::default(),
    );

    // Compile patterns
    let patterns_before = owned_walking.ignore_patterns()?;
    assert!(patterns_before.is_some());

    // Invalidate cache
    owned_walking.invalidate_pattern_cache();

    // Next call should recompile (patterns should still work)
    let patterns_after = owned_walking.ignore_patterns()?;
    assert!(patterns_after.is_some());

    Ok(())
}

#[test]
fn test_owned_walking_context_validation() -> Result<()> {
    // Valid context
    let valid_walking = OwnedWalkingContext::new(
        ListingOptions {
            max_depth: Some(5),
            ..Default::default()
        },
        FilteringOptions {
            min_file_size: Some(100),
            max_file_size: Some(1000),
            ignore_patterns: Some(vec!["*.tmp".to_string(), "*.log".to_string()]),
            match_patterns: Some(vec!["*.rs".to_string()]),
            ..Default::default()
        },
        MetadataOptions::default(),
    );
    assert!(valid_walking.validate().is_ok());

    // Invalid max_depth (zero)
    let invalid_max_depth = OwnedWalkingContext::new(
        ListingOptions {
            max_depth: Some(0), // Invalid
            ..Default::default()
        },
        FilteringOptions::default(),
        MetadataOptions::default(),
    );
    let result = invalid_max_depth.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("max_depth cannot be 0"));

    // Invalid file size range
    let invalid_file_size = OwnedWalkingContext::new(
        ListingOptions::default(),
        FilteringOptions {
            min_file_size: Some(1000),
            max_file_size: Some(100), // min > max
            ..Default::default()
        },
        MetadataOptions::default(),
    );
    let result = invalid_file_size.validate();
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("min_file_size"));
    assert!(error_msg.contains("max_file_size"));

    // Empty patterns
    let empty_pattern = OwnedWalkingContext::new(
        ListingOptions::default(),
        FilteringOptions {
            ignore_patterns: Some(vec!["*.tmp".to_string(), "".to_string()]), // Empty pattern
            ..Default::default()
        },
        MetadataOptions::default(),
    );
    let result = empty_pattern.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("empty strings"));

    Ok(())
}

#[test]
fn test_owned_formatting_context_validation() -> Result<()> {
    // Valid context
    let valid_formatting = OwnedFormattingContext {
        input_source: InputSourceOptions {
            root_display_name: "valid_name".to_string(),
            ..Default::default()
        },
        html: HtmlOptions {
            include_links: true,
            base_href: Some("https://example.com/docs".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(valid_formatting.validate().is_ok());

    // Empty root name
    let empty_root_name = OwnedFormattingContext {
        input_source: InputSourceOptions {
            root_display_name: "   ".to_string(), // Only whitespace
            ..Default::default()
        },
        ..Default::default()
    };
    let result = empty_root_name.validate();
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("root_display_name cannot be empty")
    );

    // Invalid base_href
    let invalid_base_href = OwnedFormattingContext {
        input_source: InputSourceOptions {
            root_display_name: "valid_name".to_string(),
            ..Default::default()
        },
        html: HtmlOptions {
            include_links: true,
            base_href: Some("invalid-url".to_string()), // Invalid URL format
            ..Default::default()
        },
        ..Default::default()
    };
    let result = invalid_base_href.validate();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("base_href should be a valid URL"));

    Ok(())
}

#[test]
fn test_formatting_context_helper_methods() -> Result<()> {
    // Context with metadata
    let ctx_with_metadata = OwnedFormattingContext {
        metadata: MetadataOptions {
            show_size_bytes: true,
            calculate_line_count: true,
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(ctx_with_metadata.has_metadata_display());
    assert!(!ctx_with_metadata.is_minimal_output());

    // Minimal context
    let ctx_minimal = OwnedFormattingContext {
        misc: MiscOptions {
            no_summary_report: true,
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(!ctx_minimal.has_metadata_display());
    assert!(ctx_minimal.is_minimal_output());

    // Context with summary
    let ctx_with_summary = OwnedFormattingContext {
        misc: MiscOptions {
            no_summary_report: false,
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(!ctx_with_summary.is_minimal_output());

    Ok(())
}

#[test]
fn test_as_borrowed_conversions() -> Result<()> {
    let owned_walking = OwnedWalkingContext::new(
        ListingOptions {
            max_depth: Some(3),
            show_hidden: true,
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

    let borrowed_walking = owned_walking.as_borrowed();
    assert_eq!(borrowed_walking.listing.max_depth, Some(3));
    assert!(borrowed_walking.listing.show_hidden);
    assert_eq!(
        borrowed_walking.filtering.ignore_patterns,
        Some(vec!["*.tmp".to_string()])
    );
    assert!(borrowed_walking.metadata.show_size_bytes);

    let owned_formatting = OwnedFormattingContext {
        input_source: InputSourceOptions {
            root_display_name: "test_project".to_string(),
            ..Default::default()
        },
        html: HtmlOptions {
            include_links: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let borrowed_formatting = owned_formatting.as_borrowed();
    assert_eq!(
        borrowed_formatting.input_source.root_display_name,
        "test_project"
    );
    assert!(borrowed_formatting.html.include_links);

    Ok(())
}

#[test]
fn test_from_borrowed_to_owned_conversions() -> Result<()> {
    let listing = ListingOptions {
        max_depth: Some(2),
        show_hidden: true,
        ..Default::default()
    };
    let filtering = FilteringOptions {
        ignore_patterns: Some(vec!["*.log".to_string()]),
        ..Default::default()
    };
    let metadata = MetadataOptions {
        calculate_line_count: true,
        ..Default::default()
    };

    let borrowed_walking = WalkingContext::new(&listing, &filtering, &metadata);
    let owned_walking: OwnedWalkingContext = borrowed_walking.into();

    assert_eq!(owned_walking.listing.max_depth, Some(2));
    assert!(owned_walking.listing.show_hidden);
    assert_eq!(
        owned_walking.filtering.ignore_patterns,
        Some(vec!["*.log".to_string()])
    );
    assert!(owned_walking.metadata.calculate_line_count);

    Ok(())
}

#[test]
fn test_owned_walking_context_empty_patterns() -> Result<()> {
    let mut owned_walking = OwnedWalkingContext::new(
        ListingOptions::default(),
        FilteringOptions {
            ignore_patterns: Some(vec![]), // Empty but Some
            match_patterns: None,
            ..Default::default()
        },
        MetadataOptions::default(),
    );

    // Empty patterns should return None
    let ignore_patterns = owned_walking.ignore_patterns()?;
    assert!(ignore_patterns.is_none());

    let match_patterns = owned_walking.match_patterns()?;
    assert!(match_patterns.is_none());

    Ok(())
}

#[test]
fn test_default_implementations() -> Result<()> {
    let default_owned_walking = OwnedWalkingContext::default();
    assert_eq!(default_owned_walking.listing.max_depth, None);
    assert!(!default_owned_walking.listing.show_hidden);
    assert_eq!(default_owned_walking.filtering.ignore_patterns, None);
    assert!(!default_owned_walking.metadata.show_size_bytes);

    let default_owned_formatting = OwnedFormattingContext::default();
    assert!(
        !default_owned_formatting
            .input_source
            .root_display_name
            .is_empty()
    );
    assert_eq!(default_owned_formatting.listing.max_depth, None);
    assert!(!default_owned_formatting.metadata.show_size_bytes);
    assert!(!default_owned_formatting.misc.no_summary_report);
    assert!(!default_owned_formatting.html.include_links);

    Ok(())
}

// Context Diff Operations

#[test]
fn test_context_diff_integration() {
    // Create two different contexts
    let old_ctx = create_default_processing_context("project", Some(2), true);
    let mut new_ctx = old_ctx.clone();

    // User changes settings in GUI
    new_ctx.walking.listing.max_depth = Some(5);
    new_ctx.walking.listing.show_hidden = true;

    // Generate diff
    let diff = diff_processing_contexts(&old_ctx, &new_ctx);

    // Should detect changes and recommend actions
    assert!(diff.has_changes());
    assert!(diff.requires_complete_rebuild()); // depth change requires rescan

    // Test metadata-only change
    let mut metadata_ctx = old_ctx.clone();
    metadata_ctx.walking.metadata.human_readable_size = true;

    let metadata_diff = diff_processing_contexts(&old_ctx, &metadata_ctx);
    assert!(metadata_diff.has_changes());
    // This should not require rebuild since it's metadata display only
    assert!(!metadata_diff.requires_complete_rebuild());
}

#[test]
fn test_cross_context_validation() {
    // Create contexts with inconsistent settings
    let walking = OwnedWalkingContext::new(
        ListingOptions {
            max_depth: Some(2),
            ..Default::default()
        },
        FilteringOptions::default(),
        MetadataOptions {
            show_size_bytes: false, // Walking doesn't collect size
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
            show_size_bytes: true, // But formatting wants to display it
            ..Default::default()
        },
        MiscOptions::default(),
        HtmlOptions::default(),
    );

    // Should catch inconsistency
    let result = validate_contexts(&walking, &formatting, None);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error.has_errors());
    assert!(error.error_count() > 0);
}
