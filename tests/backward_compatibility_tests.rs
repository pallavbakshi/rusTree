// tests/backward_compatibility_tests.rs
//
// Comprehensive backward compatibility tests that verify the new context-based APIs
// produce identical results to the existing APIs across all scenarios.

use anyhow::Result;
use rustree::config::metadata::ApplyFunction;
use rustree::core::options::DirectoryFileOrder;
use rustree::*;

mod common;
use common::common_test_utils;

#[test]
fn test_basic_tree_generation_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            show_hidden: false,
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            calculate_line_count: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Old API
    let nodes_old = get_tree_nodes(root_path, &config)?;

    // New context API
    let processing_ctx = config.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Compare results
    assert_eq!(nodes_old.len(), nodes_new.len(), "Node count mismatch");

    // Sort both for stable comparison
    let mut nodes_old_sorted = nodes_old.clone();
    let mut nodes_new_sorted = nodes_new.clone();
    nodes_old_sorted.sort_by(|a, b| a.path.cmp(&b.path));
    nodes_new_sorted.sort_by(|a, b| a.path.cmp(&b.path));

    for (old, new) in nodes_old_sorted.iter().zip(nodes_new_sorted.iter()) {
        assert_eq!(old.name, new.name, "Node name mismatch");
        assert_eq!(old.path, new.path, "Node path mismatch");
        assert_eq!(old.node_type, new.node_type, "Node type mismatch");
        assert_eq!(old.depth, new.depth, "Node depth mismatch");
        assert_eq!(old.size, new.size, "Node size mismatch");
        assert_eq!(old.line_count, new.line_count, "Line count mismatch");
    }

    Ok(())
}

#[test]
fn test_all_formatters_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "compat_test".to_string(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(2),
            show_hidden: false,
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            show_last_modified: false,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes_old = get_tree_nodes(root_path, &config)?;

    let processing_ctx = config.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    let formats = [
        LibOutputFormat::Text,
        LibOutputFormat::Markdown,
        LibOutputFormat::Json,
        LibOutputFormat::Html,
    ];

    for format in formats.iter() {
        // Old API
        let output_old = format_nodes(&nodes_old, format.clone(), &config)?;

        // New API
        let output_new =
            format_nodes_with_context(&nodes_new, format.clone(), &processing_ctx.formatting)?;

        assert_eq!(
            output_old, output_new,
            "Output mismatch for format {:?}",
            format
        );
    }

    Ok(())
}

#[test]
fn test_hidden_files_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            show_hidden: true, // Show hidden files
            ..Default::default()
        },
        ..Default::default()
    };

    // Old API
    let nodes_old = get_tree_nodes(root_path, &config)?;

    // New API
    let processing_ctx = config.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Should have same number of nodes including hidden
    assert_eq!(nodes_old.len(), nodes_new.len());

    // Should both include .hidden_file
    let has_hidden_old = nodes_old.iter().any(|n| n.name == ".hidden_file");
    let has_hidden_new = nodes_new.iter().any(|n| n.name == ".hidden_file");
    assert_eq!(has_hidden_old, has_hidden_new);
    assert!(has_hidden_old, "Should include hidden files");

    Ok(())
}

#[test]
fn test_depth_limiting_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_complex_test_directory()?;
    let root_path = temp_dir.path();

    for max_depth in [Some(1), Some(2), Some(3), None] {
        let config = RustreeLibConfig {
            listing: ListingOptions {
                max_depth,
                show_hidden: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // Old API
        let nodes_old = get_tree_nodes(root_path, &config)?;

        // New API
        let processing_ctx = config.processing_context();
        let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

        assert_eq!(
            nodes_old.len(),
            nodes_new.len(),
            "Node count mismatch for max_depth {:?}",
            max_depth
        );

        // Check depth constraints are same
        let max_depth_old = nodes_old.iter().map(|n| n.depth).max().unwrap_or(0);
        let max_depth_new = nodes_new.iter().map(|n| n.depth).max().unwrap_or(0);
        assert_eq!(max_depth_old, max_depth_new);

        if let Some(limit) = max_depth {
            assert!(max_depth_old <= limit);
            assert!(max_depth_new <= limit);
        }
    }

    Ok(())
}

#[test]
fn test_sorting_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Add files with different properties for sorting
    common_test_utils::create_file_with_content(root_path, "aaaa.txt", "small")?;
    common_test_utils::create_file_with_content(root_path, "zzzz.txt", "a".repeat(1000).as_str())?;

    let sort_keys = [
        Some(SortKey::Name),
        Some(SortKey::Size),
        Some(SortKey::MTime),
        None,
    ];

    for sort_by in sort_keys.iter() {
        let config = RustreeLibConfig {
            listing: ListingOptions {
                max_depth: Some(1),
                ..Default::default()
            },
            metadata: MetadataOptions {
                show_size_bytes: true,
                show_last_modified: true,
                ..Default::default()
            },
            sorting: SortingOptions {
                sort_by: sort_by.clone(),
                reverse_sort: false,
                ..Default::default()
            },
            ..Default::default()
        };

        // Old API
        let nodes_old = get_tree_nodes(root_path, &config)?;

        // New API
        let processing_ctx = config.processing_context();
        let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

        // Compare ordering
        let names_old: Vec<_> = nodes_old.iter().map(|n| &n.name).collect();
        let names_new: Vec<_> = nodes_new.iter().map(|n| &n.name).collect();

        assert_eq!(
            names_old, names_new,
            "Sort order mismatch for sort_by {:?}",
            sort_by
        );
    }

    Ok(())
}

#[test]
fn test_metadata_collection_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            show_last_modified: true,
            calculate_line_count: true,
            calculate_word_count: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Old API
    let nodes_old = get_tree_nodes(root_path, &config)?;

    // New API
    let processing_ctx = config.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Compare metadata for each node
    let mut nodes_old_sorted = nodes_old.clone();
    let mut nodes_new_sorted = nodes_new.clone();
    nodes_old_sorted.sort_by(|a, b| a.path.cmp(&b.path));
    nodes_new_sorted.sort_by(|a, b| a.path.cmp(&b.path));

    for (old, new) in nodes_old_sorted.iter().zip(nodes_new_sorted.iter()) {
        assert_eq!(
            old.size, new.size,
            "Size metadata mismatch for {}",
            old.name
        );
        assert_eq!(
            old.mtime, new.mtime,
            "Mtime metadata mismatch for {}",
            old.name
        );
        assert_eq!(
            old.line_count, new.line_count,
            "Line count mismatch for {}",
            old.name
        );
        assert_eq!(
            old.word_count, new.word_count,
            "Word count mismatch for {}",
            old.name
        );
    }

    Ok(())
}

#[test]
fn test_apply_function_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            ..Default::default()
        },
        metadata: MetadataOptions {
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::CountPluses)),
            ..Default::default()
        },
        ..Default::default()
    };

    // Old API
    let nodes_old = get_tree_nodes(root_path, &config)?;

    // New API
    let processing_ctx = config.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Compare custom function output
    let mut nodes_old_sorted = nodes_old.clone();
    let mut nodes_new_sorted = nodes_new.clone();
    nodes_old_sorted.sort_by(|a, b| a.path.cmp(&b.path));
    nodes_new_sorted.sort_by(|a, b| a.path.cmp(&b.path));

    for (old, new) in nodes_old_sorted.iter().zip(nodes_new_sorted.iter()) {
        assert_eq!(
            old.custom_function_output, new.custom_function_output,
            "Custom function output mismatch for {}",
            old.name
        );
    }

    // Verify specific expected outputs
    let file3_old = nodes_old.iter().find(|n| n.name == "file3.dat");
    let file3_new = nodes_new.iter().find(|n| n.name == "file3.dat");

    if let (Some(old), Some(new)) = (file3_old, file3_new) {
        assert_eq!(old.custom_function_output, Some(Ok("2".to_string())));
        assert_eq!(new.custom_function_output, Some(Ok("2".to_string())));
    }

    Ok(())
}

#[test]
fn test_filtering_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_complex_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            show_hidden: false,
            ..Default::default()
        },
        filtering: FilteringOptions {
            ignore_patterns: Some(vec!["*.log".to_string(), "*.JPG".to_string()]),
            match_patterns: Some(vec!["*.txt".to_string(), "*.rs".to_string()]),
            case_insensitive_filter: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Old API
    let nodes_old = get_tree_nodes(root_path, &config)?;

    // New API
    let processing_ctx = config.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Should have same number of filtered nodes
    assert_eq!(nodes_old.len(), nodes_new.len());

    // Should both exclude .log files
    let has_log_old = nodes_old.iter().any(|n| n.name.ends_with(".log"));
    let has_log_new = nodes_new.iter().any(|n| n.name.ends_with(".log"));
    assert_eq!(has_log_old, has_log_new);
    assert!(!has_log_old, "Should exclude .log files");

    // Should both include only .txt and .rs files (plus directories)
    for node in &nodes_old {
        if node.node_type == NodeType::File {
            assert!(
                node.name.ends_with(".txt") || node.name.ends_with(".rs"),
                "File {} should match include patterns",
                node.name
            );
        }
    }

    for node in &nodes_new {
        if node.node_type == NodeType::File {
            assert!(
                node.name.ends_with(".txt") || node.name.ends_with(".rs"),
                "File {} should match include patterns",
                node.name
            );
        }
    }

    Ok(())
}

#[test]
fn test_summary_report_formatting_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Test with summary enabled
    let config_with_summary = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            ..Default::default()
        },
        misc: MiscOptions {
            no_summary_report: false,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config_with_summary)?;
    let processing_ctx = config_with_summary.processing_context();
    let nodes_ctx = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Test both text and markdown formats
    let text_old = format_nodes(&nodes, LibOutputFormat::Text, &config_with_summary)?;
    let text_new = format_nodes_with_context(
        &nodes_ctx,
        LibOutputFormat::Text,
        &processing_ctx.formatting,
    )?;
    assert_eq!(text_old, text_new);

    let md_old = format_nodes(&nodes, LibOutputFormat::Markdown, &config_with_summary)?;
    let md_new = format_nodes_with_context(
        &nodes_ctx,
        LibOutputFormat::Markdown,
        &processing_ctx.formatting,
    )?;
    assert_eq!(md_old, md_new);

    // Test with summary disabled
    let config_no_summary = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            ..Default::default()
        },
        misc: MiscOptions {
            no_summary_report: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let processing_ctx_no_summary = config_no_summary.processing_context();

    let text_no_summary_old = format_nodes(&nodes, LibOutputFormat::Text, &config_no_summary)?;
    let text_no_summary_new = format_nodes_with_context(
        &nodes_ctx,
        LibOutputFormat::Text,
        &processing_ctx_no_summary.formatting,
    )?;
    assert_eq!(text_no_summary_old, text_no_summary_new);

    // Verify summary behavior differences
    assert!(text_old.contains("directories") || text_old.contains("files"));
    assert!(!text_no_summary_old.contains("directories") && !text_no_summary_old.contains("files"));

    Ok(())
}

#[test]
fn test_html_formatting_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "html_test".to_string(),
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        html: HtmlOptions {
            include_links: true,
            base_href: Some("https://example.com/base".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let processing_ctx = config.processing_context();
    let nodes_ctx = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    let html_old = format_nodes(&nodes, LibOutputFormat::Html, &config)?;
    let html_new = format_nodes_with_context(
        &nodes_ctx,
        LibOutputFormat::Html,
        &processing_ctx.formatting,
    )?;

    assert_eq!(html_old, html_new);

    // Verify HTML-specific features are preserved
    assert!(html_old.contains("<!DOCTYPE html>"));
    assert!(html_old.contains("html_test"));
    assert!(html_old.contains("https://example.com/base"));
    assert!(html_new.contains("<!DOCTYPE html>"));
    assert!(html_new.contains("html_test"));
    assert!(html_new.contains("https://example.com/base"));

    Ok(())
}

#[test]
fn test_complex_combined_configuration_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_complex_test_directory()?;
    let root_path = temp_dir.path();

    // Create a complex configuration combining multiple features
    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "complex_test".to_string(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(3),
            show_hidden: true,
            show_full_path: true,
            list_directories_only: false,
        },
        filtering: FilteringOptions {
            ignore_patterns: Some(vec!["*.JPG".to_string()]),
            match_patterns: Some(vec!["*.txt".to_string(), "*.rs".to_string()]),
            case_insensitive_filter: true,
            prune_empty_directories: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            show_last_modified: true,
            calculate_line_count: true,
            calculate_word_count: true,
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::CountPluses)),
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            reverse_sort: false,
            files_before_directories: false,
            directory_file_order: DirectoryFileOrder::DirsFirst,
        },
        misc: MiscOptions {
            no_summary_report: false,
            human_friendly: true,
            ..Default::default()
        },
        html: HtmlOptions {
            include_links: true,
            base_href: Some("https://example.com".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };

    // Test with old API
    let nodes_old = get_tree_nodes(root_path, &config)?;

    // Test with new API
    let processing_ctx = config.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Verify identical results
    assert_eq!(nodes_old.len(), nodes_new.len());

    // Test all output formats
    for format in [
        LibOutputFormat::Text,
        LibOutputFormat::Markdown,
        LibOutputFormat::Json,
        LibOutputFormat::Html,
    ]
    .iter()
    {
        let output_old = format_nodes(&nodes_old, format.clone(), &config)?;
        let output_new =
            format_nodes_with_context(&nodes_new, format.clone(), &processing_ctx.formatting)?;

        assert_eq!(
            output_old, output_new,
            "Complex configuration output mismatch for format {:?}",
            format
        );
    }

    Ok(())
}

#[test]
fn test_edge_cases_compatibility() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Test empty ignore patterns
    let config_empty_patterns = RustreeLibConfig {
        filtering: FilteringOptions {
            ignore_patterns: Some(vec![]),
            match_patterns: Some(vec![]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes_old = get_tree_nodes(root_path, &config_empty_patterns)?;
    let processing_ctx = config_empty_patterns.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;
    assert_eq!(nodes_old.len(), nodes_new.len());

    // Test None patterns
    let config_none_patterns = RustreeLibConfig {
        filtering: FilteringOptions {
            ignore_patterns: None,
            match_patterns: None,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes_old = get_tree_nodes(root_path, &config_none_patterns)?;
    let processing_ctx = config_none_patterns.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;
    assert_eq!(nodes_old.len(), nodes_new.len());

    // Test max_depth = 1 (minimal depth)
    let config_min_depth = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes_old = get_tree_nodes(root_path, &config_min_depth)?;
    let processing_ctx = config_min_depth.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;
    assert_eq!(nodes_old.len(), nodes_new.len());

    // All nodes should be at depth 1
    for node in &nodes_old {
        assert_eq!(node.depth, 1);
    }
    for node in &nodes_new {
        assert_eq!(node.depth, 1);
    }

    Ok(())
}
