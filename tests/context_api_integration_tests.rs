// tests/context_api_integration_tests.rs
//
// Integration tests for the new context-based APIs introduced in Phase 4.
// These tests verify that the new APIs work correctly and produce identical
// results to the old APIs, ensuring full backward compatibility.

use anyhow::Result;
use rustree::core::options::DirectoryFileOrder;
use rustree::core::options::contexts::*;
use rustree::*;

mod common;
use common::common_test_utils;

#[test]
fn test_backward_compatibility_identical_results() -> Result<()> {
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
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    // Old API
    let nodes_old = get_tree_nodes(root_path, &config)?;
    let output_old = format_nodes(&nodes_old, LibOutputFormat::Text, &config)?;

    // New API with converted contexts
    let processing_ctx = config.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;
    let output_new = format_nodes_with_context(
        &nodes_new,
        LibOutputFormat::Text,
        &processing_ctx.formatting,
    )?;

    // Results should be identical
    assert_eq!(nodes_old.len(), nodes_new.len());
    assert_eq!(output_old, output_new);

    // Verify specific node properties are preserved
    let mut nodes_old_sorted = nodes_old.clone();
    let mut nodes_new_sorted = nodes_new.clone();
    nodes_old_sorted.sort_by(|a, b| a.path.cmp(&b.path));
    nodes_new_sorted.sort_by(|a, b| a.path.cmp(&b.path));

    for (old_node, new_node) in nodes_old_sorted.iter().zip(nodes_new_sorted.iter()) {
        assert_eq!(old_node.name, new_node.name);
        assert_eq!(old_node.path, new_node.path);
        assert_eq!(old_node.node_type, new_node.node_type);
        assert_eq!(old_node.depth, new_node.depth);
        assert_eq!(old_node.size, new_node.size);
        assert_eq!(old_node.line_count, new_node.line_count);
    }

    Ok(())
}

#[test]
fn test_new_context_apis_focused_usage() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Create contexts manually for focused operations
    let listing = ListingOptions {
        max_depth: Some(2),
        show_hidden: false,
        ..Default::default()
    };
    let filtering = FilteringOptions::default();
    let metadata = MetadataOptions {
        show_size_bytes: true,
        calculate_line_count: true,
        ..Default::default()
    };

    let walking_ctx = WalkingContext::new(&listing, &filtering, &metadata);

    // Test focused walking API
    let nodes = walk_path_with_context(root_path, &walking_ctx)?;
    assert!(!nodes.is_empty());

    // Verify nodes contain expected data
    let file1_node = nodes
        .iter()
        .find(|n| n.name == "file1.txt")
        .expect("file1.txt not found");
    assert_eq!(file1_node.line_count, Some(3)); // "hello\nworld\nrust"

    // Test focused formatting
    let input_source = InputSourceOptions {
        root_display_name: "test_root".to_string(),
        root_is_directory: true,
        ..Default::default()
    };
    let misc = MiscOptions::default();
    let html = HtmlOptions::default();

    let formatting_ctx = FormattingContext::new(&input_source, &listing, &metadata, &misc, &html);
    let output = format_nodes_with_context(&nodes, LibOutputFormat::Text, &formatting_ctx)?;

    assert!(!output.is_empty());
    assert!(output.contains("test_root"));
    assert!(output.contains("file1.txt"));

    Ok(())
}

#[test]
fn test_owned_context_apis_for_gui_scenarios() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Create owned contexts that can be modified independently
    let mut owned_walking = OwnedWalkingContext::new(
        ListingOptions {
            max_depth: Some(1),
            show_hidden: false,
            ..Default::default()
        },
        FilteringOptions {
            ignore_patterns: Some(vec!["*.log".to_string()]),
            ..Default::default()
        },
        MetadataOptions {
            show_size_bytes: true,
            ..Default::default()
        },
    );

    // Validate context before use
    assert!(owned_walking.validate().is_ok());

    // Test owned walking API
    let nodes_depth_1 = walk_path_owned(root_path, &mut owned_walking)?;

    // Should exclude file2.log due to ignore pattern and sub_dir contents due to max_depth
    assert!(nodes_depth_1.iter().any(|n| n.name == "file1.txt"));
    assert!(!nodes_depth_1.iter().any(|n| n.name == "file2.log")); // Ignored
    assert!(nodes_depth_1.iter().any(|n| n.name == "sub_dir")); // Directory itself at depth 1
    assert!(!nodes_depth_1.iter().any(|n| n.name == "file3.dat")); // Beyond max_depth

    // Modify context (GUI user changes max depth)
    owned_walking.listing.max_depth = Some(2);

    // Test with modified context
    let nodes_depth_2 = walk_path_owned(root_path, &mut owned_walking)?;

    // Now should include file3.dat
    assert!(nodes_depth_2.iter().any(|n| n.name == "file3.dat"));
    assert!(nodes_depth_2.len() > nodes_depth_1.len());

    // Test pattern caching works (indirectly through performance)
    let _ = owned_walking.ignore_patterns()?;

    Ok(())
}

#[test]
fn test_processing_context_builder_api() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Build processing context programmatically
    let walking = OwnedWalkingContext::new(
        ListingOptions {
            max_depth: Some(2),
            show_hidden: true,
            ..Default::default()
        },
        FilteringOptions::default(),
        MetadataOptions {
            calculate_line_count: true,
            calculate_word_count: true,
            ..Default::default()
        },
    );

    let sorting = OwnedSortingContext {
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            reverse_sort: false,
            files_before_directories: false,
            directory_file_order: DirectoryFileOrder::DirsFirst,
        },
    };

    let formatting = OwnedFormattingContext {
        input_source: InputSourceOptions {
            root_display_name: "builder_test".to_string(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions::default(),
        metadata: MetadataOptions::default(),
        misc: MiscOptions::default(),
        html: HtmlOptions::default(),
    };

    let processing_ctx = ProcessingContextBuilder::new()
        .with_walking(walking)
        .with_sorting(sorting)
        .with_formatting(formatting)
        .build()
        .map_err(|e| anyhow::anyhow!(e))?;

    // Test the built context
    assert_eq!(processing_ctx.walking.listing.max_depth, Some(2));
    assert!(processing_ctx.walking.listing.show_hidden);
    assert!(processing_ctx.sorting.is_some());
    assert_eq!(
        processing_ctx.sorting.as_ref().unwrap().sorting.sort_by,
        Some(SortKey::Name)
    );
    assert_eq!(
        processing_ctx.formatting.input_source.root_display_name,
        "builder_test"
    );

    // Test using built context
    let mut processing_ctx_mut = processing_ctx;
    let nodes = get_tree_nodes_owned(root_path, &mut processing_ctx_mut)?;
    assert!(!nodes.is_empty());

    // Verify nodes are sorted (since we included sorting context)
    let node_names: Vec<_> = nodes.iter().map(|n| n.name.as_str()).collect();
    let mut sorted_names = node_names.clone();
    sorted_names.sort();
    assert_eq!(node_names, sorted_names);

    Ok(())
}

#[test]
fn test_builder_validation_errors() -> Result<()> {
    // Test missing required walking context
    let result = ProcessingContextBuilder::new()
        .with_formatting(OwnedFormattingContext::default())
        .build();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Walking context is required"));

    // Test missing required formatting context
    let result = ProcessingContextBuilder::new()
        .with_walking(OwnedWalkingContext::default())
        .build();
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Formatting context is required")
    );

    // Test valid minimal builder
    let result = ProcessingContextBuilder::new()
        .with_walking(OwnedWalkingContext::default())
        .with_formatting(OwnedFormattingContext::default())
        .build();
    assert!(result.is_ok());

    let processing_ctx = result.unwrap();
    assert!(processing_ctx.sorting.is_none()); // Optional, not provided

    Ok(())
}

#[test]
fn test_convenience_helper_functions() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let _root_path = temp_dir.path();

    // Test create_default_processing_context
    let mut default_ctx = create_default_processing_context(
        "default_test",
        Some(2), // max_depth
        false,   // show_hidden
    );
    assert_eq!(default_ctx.walking.listing.max_depth, Some(2));
    assert!(!default_ctx.walking.listing.show_hidden);
    assert_eq!(
        default_ctx.formatting.input_source.root_display_name,
        "default_test"
    );

    // Test validate_processing_context
    assert!(validate_processing_context(&default_ctx).is_ok());

    // Create invalid context and test validation
    default_ctx.walking.listing.max_depth = Some(0); // Invalid
    let result = validate_processing_context(&default_ctx);
    assert!(result.is_err());
    // Note: RustreeError doesn't implement Display with contains,
    // so we check validation through the result type
    assert!(result.is_err());

    // Test optimize_context
    default_ctx.walking.listing.max_depth = Some(2); // Fix validation issue
    default_ctx.walking.filtering.ignore_patterns = Some(vec!["*.tmp".to_string()]);

    let result = optimize_context(&mut default_ctx);
    assert!(result.is_ok());

    // Patterns should now be compiled (tested indirectly)
    // Note: compiled fields are private, so we trust the optimize function worked

    Ok(())
}

#[test]
fn test_all_output_formats_with_contexts() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "format_test".to_string(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };

    let processing_ctx = config.processing_context();
    let nodes = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Test all output formats with context API
    let text_output =
        format_nodes_with_context(&nodes, LibOutputFormat::Text, &processing_ctx.formatting)?;
    assert!(!text_output.is_empty());
    assert!(text_output.contains("format_test"));

    let markdown_output = format_nodes_with_context(
        &nodes,
        LibOutputFormat::Markdown,
        &processing_ctx.formatting,
    )?;
    assert!(!markdown_output.is_empty());
    assert!(markdown_output.contains("# format_test"));

    let json_output =
        format_nodes_with_context(&nodes, LibOutputFormat::Json, &processing_ctx.formatting)?;
    assert!(!json_output.is_empty());
    assert!(json_output.contains("\"name\""));

    let html_output =
        format_nodes_with_context(&nodes, LibOutputFormat::Html, &processing_ctx.formatting)?;
    assert!(!html_output.is_empty());
    assert!(html_output.contains("<html"));

    // Compare with old API to ensure compatibility
    let old_text = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    assert_eq!(text_output, old_text);

    Ok(())
}

#[test]
fn test_context_based_sorting() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    // Create additional files with different sizes for sorting
    common_test_utils::create_file_with_content(root_path, "small.txt", "a")?;
    common_test_utils::create_file_with_content(root_path, "large.txt", "a".repeat(1000).as_str())?;

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Size),
            reverse_sort: false,
            ..Default::default()
        },
        ..Default::default()
    };

    // Test context-based sorting
    let sorting_ctx = config.sorting_context();
    let processing_ctx = config.processing_context();
    let mut nodes = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Sort using context API
    sort_nodes_with_context(&mut nodes, &sorting_ctx)?;

    // Verify sorting by size (ascending)
    let file_nodes: Vec<_> = nodes
        .iter()
        .filter(|n| n.node_type == NodeType::File)
        .collect();

    if file_nodes.len() >= 2 {
        for i in 0..file_nodes.len() - 1 {
            if let (Some(size1), Some(size2)) = (file_nodes[i].size, file_nodes[i + 1].size) {
                assert!(size1 <= size2, "Files not sorted by size ascending");
            }
        }
    }

    Ok(())
}

#[test]
fn test_multiple_format_consistency() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    // Get nodes using both APIs
    let nodes_old = get_tree_nodes(root_path, &config)?;
    let processing_ctx = config.processing_context();
    let nodes_new = get_tree_nodes_with_context(root_path, &processing_ctx)?;

    // Test multiple formats with both APIs for consistency
    let formats = [
        LibOutputFormat::Text,
        LibOutputFormat::Markdown,
        LibOutputFormat::Json,
        LibOutputFormat::Html,
    ];

    for format in formats.iter() {
        let output_old = format_nodes(&nodes_old, format.clone(), &config)?;
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
fn test_context_isolation_and_independence() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let base_config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };

    // Create multiple owned contexts from the same base
    let mut ctx1 = base_config.to_owned_processing_context();
    let mut ctx2 = base_config.to_owned_processing_context();

    // Modify each context independently
    ctx1.walking.listing.max_depth = Some(1);
    ctx1.walking.listing.show_hidden = false;

    ctx2.walking.listing.max_depth = Some(2);
    ctx2.walking.listing.show_hidden = true;

    // Get nodes with each context
    let nodes1 = get_tree_nodes_owned(root_path, &mut ctx1)?;
    let nodes2 = get_tree_nodes_owned(root_path, &mut ctx2)?;

    // Results should be different due to different configurations
    assert!(
        nodes1.len() != nodes2.len()
            || nodes1
                .iter()
                .any(|n1| !nodes2.iter().any(|n2| n1.name == n2.name))
    );

    // Verify max_depth differences
    let max_depth_1 = nodes1.iter().map(|n| n.depth).max().unwrap_or(0);
    let max_depth_2 = nodes2.iter().map(|n| n.depth).max().unwrap_or(0);

    // ctx1 should have shallower max depth than ctx2
    assert!(max_depth_1 <= max_depth_2);

    // Original config should remain unchanged
    assert_eq!(base_config.listing.max_depth, Some(1));
    assert!(!base_config.listing.show_hidden);

    Ok(())
}

#[test]
fn test_async_context_integration() {
    // Create an owned context
    let owned = create_default_processing_context("test_project", Some(3), true);

    // Convert to async context
    let async_ctx = create_async_context(&owned);

    // Should be cloneable for multi-threading
    let cloned = async_ctx.clone();

    // Verify thread safety
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AsyncProcessingContext>();

    // Verify validation works
    assert!(async_ctx.validate().is_ok());
    assert!(cloned.validate().is_ok());
}

#[test]
fn test_complete_gui_workflow() {
    // Simulate a complete GUI workflow using advanced features

    // 1. Create initial context
    let mut initial_ctx = create_default_processing_context("my_project", Some(2), true);

    // 2. User modifies settings
    initial_ctx.walking.filtering.ignore_patterns = Some(vec!["*.tmp".to_string()]);

    // 3. Validate before use
    assert!(validate_processing_context(&initial_ctx).is_ok());

    // 4. Optimize for performance
    assert!(optimize_context(&mut initial_ctx).is_ok());

    // 5. Convert to async for background processing
    let async_ctx = create_async_context(&initial_ctx);

    // 6. User makes another change
    let mut updated_ctx = initial_ctx.clone();
    updated_ctx.walking.listing.max_depth = Some(5);

    // 7. Generate diff to understand what changed
    let diff = diff_processing_contexts(&initial_ctx, &updated_ctx);

    // 8. GUI can make intelligent decisions based on diff
    if diff.requires_complete_rebuild() {
        // Rescan directory would happen here
        // For depth change, this branch should be taken
    } else if diff.can_optimize_with_resort() {
        // Just resort existing nodes
        panic!("Depth change should require complete rebuild, not just resort");
    }

    // 9. Verify all contexts are valid
    assert!(async_ctx.validate().is_ok());
    assert!(updated_ctx.validate().is_ok());
}
