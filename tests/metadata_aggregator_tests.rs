use rustree::config::{RustreeLibConfig, metadata::BuiltInFunction};
use rustree::core::metadata::MetadataAggregator;
use rustree::core::tree::node::{NodeInfo, NodeType};
use std::path::PathBuf;

// Helper function to create a NodeInfo with minimal fields
fn create_node_info(name: &str, node_type: NodeType) -> NodeInfo {
    NodeInfo {
        name: name.to_string(),
        path: PathBuf::from(name),
        node_type,
        depth: 1,
        size: None,
        permissions: None,
        mtime: None,
        change_time: None,
        create_time: None,
        line_count: None,
        word_count: None,
        custom_function_output: None,
    }
}

#[test]
fn test_aggregate_line_counts() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_line_count = true;

    let nodes = vec![
        NodeInfo {
            name: "file1.txt".to_string(),
            path: PathBuf::from("file1.txt"),
            node_type: NodeType::File,
            depth: 1,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: Some(100),
            word_count: None,
            custom_function_output: None,
        },
        NodeInfo {
            name: "file2.txt".to_string(),
            path: PathBuf::from("file2.txt"),
            node_type: NodeType::File,
            depth: 1,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: Some(200),
            word_count: None,
            custom_function_output: None,
        },
        NodeInfo {
            name: "dir".to_string(),
            path: PathBuf::from("dir"),
            node_type: NodeType::Directory,
            depth: 1,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None, // Directories don't have line counts
            word_count: None,
            custom_function_output: None,
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.line_total, Some(300));

    let summary = aggregator.format_summary_additions();
    assert!(summary.contains("300 total lines"));
}

#[test]
fn test_aggregate_word_counts() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_word_count = true;

    let nodes = vec![
        NodeInfo {
            name: "file1.txt".to_string(),
            path: PathBuf::from("file1.txt"),
            node_type: NodeType::File,
            depth: 1,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None,
            word_count: Some(1000),
            custom_function_output: None,
        },
        NodeInfo {
            name: "file2.txt".to_string(),
            path: PathBuf::from("file2.txt"),
            node_type: NodeType::File,
            depth: 1,
            size: None,
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None,
            word_count: Some(2500),
            custom_function_output: None,
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.word_total, Some(3500));

    let summary = aggregator.format_summary_additions();
    assert!(summary.contains("3,500 total words"));
}

#[test]
fn test_aggregate_sizes() {
    let mut config = RustreeLibConfig::default();
    config.metadata.show_size_bytes = true;

    let nodes = vec![
        NodeInfo {
            name: "file1.txt".to_string(),
            path: PathBuf::from("file1.txt"),
            node_type: NodeType::File,
            depth: 1,
            size: Some(1024), // 1 KB
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        },
        NodeInfo {
            name: "file2.txt".to_string(),
            path: PathBuf::from("file2.txt"),
            node_type: NodeType::File,
            depth: 1,
            size: Some(2048), // 2 KB
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.size_total, Some(3072));

    let summary = aggregator.format_summary_additions();
    assert!(summary.contains("3.0 KB total"));
}

#[test]
fn test_aggregate_multiple_metadata() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_line_count = true;
    config.metadata.calculate_word_count = true;
    config.metadata.show_size_bytes = true;

    let nodes = vec![
        {
            let mut node = create_node_info("file1.txt", NodeType::File);
            node.line_count = Some(100);
            node.word_count = Some(500);
            node.size = Some(2048);
            node
        },
        {
            let mut node = create_node_info("file2.txt", NodeType::File);
            node.line_count = Some(50);
            node.word_count = Some(250);
            node.size = Some(1024);
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.line_total, Some(150));
    assert_eq!(aggregator.word_total, Some(750));
    assert_eq!(aggregator.size_total, Some(3072));

    let summary = aggregator.format_summary_additions();
    assert!(summary.contains("150 total lines"));
    assert!(summary.contains("750 total words"));
    assert!(summary.contains("3.0 KB total"));
}

#[test]
fn test_aggregate_dir_stats_function() {
    let mut config = RustreeLibConfig::default();
    config.metadata.apply_function = Some(BuiltInFunction::DirStats);

    let nodes = vec![
        {
            let mut node = create_node_info("dir1", NodeType::Directory);
            node.custom_function_output = Some(Ok("5f,2d,1024B".to_string()));
            node
        },
        {
            let mut node = create_node_info("dir2", NodeType::Directory);
            node.custom_function_output = Some(Ok("3f,1d,2048B".to_string()));
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.file_count_from_function, Some(8)); // 5 + 3
    assert_eq!(aggregator.dir_count_from_function, Some(3)); // 2 + 1
    assert_eq!(aggregator.size_from_function, Some(3072)); // 1024 + 2048
}

#[test]
fn test_aggregate_count_files_function() {
    let mut config = RustreeLibConfig::default();
    config.metadata.apply_function = Some(BuiltInFunction::CountFiles);

    let nodes = vec![
        {
            let mut node = create_node_info("dir1", NodeType::Directory);
            node.custom_function_output = Some(Ok("10".to_string()));
            node
        },
        {
            let mut node = create_node_info("dir2", NodeType::Directory);
            node.custom_function_output = Some(Ok("15".to_string()));
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.file_count_from_function, Some(25)); // 10 + 15
}

#[test]
fn test_format_number_with_commas() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_line_count = true;

    let nodes = vec![{
        let mut node = create_node_info("file.txt", NodeType::File);
        node.line_count = Some(1234567);
        node
    }];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    let summary = aggregator.format_summary_additions();
    assert!(summary.contains("1,234,567"));
}

#[test]
fn test_format_size_units() {
    let mut config = RustreeLibConfig::default();
    config.metadata.show_size_bytes = true;

    // Test bytes
    let nodes = vec![{
        let mut node = create_node_info("small.txt", NodeType::File);
        node.size = Some(500);
        node
    }];
    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    let summary = aggregator.format_summary_additions();
    assert!(summary.contains("500 B"));

    // Test KB
    let nodes = vec![{
        let mut node = create_node_info("medium.txt", NodeType::File);
        node.size = Some(2560); // 2.5 KB
        node
    }];
    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    let summary = aggregator.format_summary_additions();
    assert!(summary.contains("2.5 KB"));

    // Test MB
    let nodes = vec![{
        let mut node = create_node_info("large.txt", NodeType::File);
        node.size = Some(5242880); // 5 MB
        node
    }];
    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    let summary = aggregator.format_summary_additions();
    assert!(summary.contains("5.0 MB"));
}

#[test]
fn test_empty_aggregation() {
    let config = RustreeLibConfig::default();
    let nodes = vec![];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.line_total, None);
    assert_eq!(aggregator.word_total, None);
    assert_eq!(aggregator.size_total, None);

    let summary = aggregator.format_summary_additions();
    assert_eq!(summary, "");
}

#[test]
fn test_ignore_directories_for_file_metadata() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_line_count = true;
    config.metadata.calculate_word_count = true;

    let nodes = vec![
        {
            let mut node = create_node_info("file.txt", NodeType::File);
            node.line_count = Some(100);
            node.word_count = Some(500);
            node
        },
        {
            let mut node = create_node_info("dir", NodeType::Directory);
            node.line_count = Some(999); // Should be ignored
            node.word_count = Some(999); // Should be ignored
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.line_total, Some(100));
    assert_eq!(aggregator.word_total, Some(500));
}

// Edge case tests
#[test]
fn test_malformed_dir_stats_output() {
    let mut config = RustreeLibConfig::default();
    config.metadata.apply_function = Some(BuiltInFunction::DirStats);

    let nodes = vec![
        {
            let mut node = create_node_info("dir1", NodeType::Directory);
            node.custom_function_output = Some(Ok("invalid_format".to_string()));
            node
        },
        {
            let mut node = create_node_info("dir2", NodeType::Directory);
            node.custom_function_output = Some(Ok("5f,2d,1024B".to_string()));
            node
        },
        {
            let mut node = create_node_info("dir3", NodeType::Directory);
            node.custom_function_output = Some(Ok("not,enough,parts".to_string()));
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    // Should only count the valid dir2 entry
    assert_eq!(aggregator.file_count_from_function, Some(5));
    assert_eq!(aggregator.dir_count_from_function, Some(2));
    assert_eq!(aggregator.size_from_function, Some(1024));
}

#[test]
fn test_function_errors_ignored() {
    let mut config = RustreeLibConfig::default();
    config.metadata.apply_function = Some(BuiltInFunction::CountFiles);

    let nodes = vec![
        {
            let mut node = create_node_info("dir1", NodeType::Directory);
            node.custom_function_output = Some(Err(
                rustree::config::metadata::ApplyFnError::CalculationFailed("Error".to_string()),
            ));
            node
        },
        {
            let mut node = create_node_info("dir2", NodeType::Directory);
            node.custom_function_output = Some(Ok("10".to_string()));
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    // Should only count the successful dir2 entry
    assert_eq!(aggregator.file_count_from_function, Some(10));
}

#[test]
fn test_non_numeric_function_output() {
    let mut config = RustreeLibConfig::default();
    config.metadata.apply_function = Some(BuiltInFunction::CountFiles);

    let nodes = vec![
        {
            let mut node = create_node_info("dir1", NodeType::Directory);
            node.custom_function_output = Some(Ok("not_a_number".to_string()));
            node
        },
        {
            let mut node = create_node_info("dir2", NodeType::Directory);
            node.custom_function_output = Some(Ok("15".to_string()));
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    // Should only count the valid numeric entry
    assert_eq!(aggregator.file_count_from_function, Some(15));
}

#[test]
fn test_zero_values_aggregation() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_line_count = true;
    config.metadata.calculate_word_count = true;
    config.metadata.show_size_bytes = true;

    let nodes = vec![
        {
            let mut node = create_node_info("empty1.txt", NodeType::File);
            node.line_count = Some(0);
            node.word_count = Some(0);
            node.size = Some(0);
            node
        },
        {
            let mut node = create_node_info("empty2.txt", NodeType::File);
            node.line_count = Some(0);
            node.word_count = Some(0);
            node.size = Some(0);
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.line_total, Some(0));
    assert_eq!(aggregator.word_total, Some(0));
    assert_eq!(aggregator.size_total, Some(0));

    let summary = aggregator.format_summary_additions();
    assert!(summary.contains("0 total lines"));
    assert!(summary.contains("0 total words"));
    assert!(summary.contains("0 B total"));
}

#[test]
fn test_very_large_numbers() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_line_count = true;
    config.metadata.show_size_bytes = true;

    let nodes = vec![{
        let mut node = create_node_info("huge.txt", NodeType::File);
        node.line_count = Some(usize::MAX);
        node.size = Some(u64::MAX);
        node
    }];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.line_total, Some(usize::MAX));
    assert_eq!(aggregator.size_total, Some(u64::MAX));

    // Should handle large numbers without panicking
    let summary = aggregator.format_summary_additions();
    assert!(!summary.is_empty());
}

#[test]
fn test_partial_metadata_availability() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_line_count = true;
    config.metadata.calculate_word_count = true;
    config.metadata.show_size_bytes = true;

    let nodes = vec![
        {
            let mut node = create_node_info("complete.txt", NodeType::File);
            node.line_count = Some(10);
            node.word_count = Some(50);
            node.size = Some(100);
            node
        },
        {
            let mut node = create_node_info("partial.txt", NodeType::File);
            node.line_count = Some(5); // Only lines available
            node.word_count = None;
            node.size = None;
            node
        },
        {
            let mut node = create_node_info("size_only.txt", NodeType::File);
            node.line_count = None;
            node.word_count = None;
            node.size = Some(200); // Only size available
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    assert_eq!(aggregator.line_total, Some(15)); // 10 + 5
    assert_eq!(aggregator.word_total, Some(50)); // Only from complete.txt
    assert_eq!(aggregator.size_total, Some(300)); // 100 + 200
}

#[test]
fn test_mixed_node_types_aggregation() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_line_count = true;
    config.metadata.show_size_bytes = true;
    config.metadata.apply_function = Some(BuiltInFunction::DirStats);

    let nodes = vec![
        {
            let mut node = create_node_info("file.txt", NodeType::File);
            node.line_count = Some(10);
            node.size = Some(100);
            node
        },
        {
            let mut node = create_node_info("dir", NodeType::Directory);
            node.line_count = Some(999); // Should be ignored for files
            node.size = Some(999); // Should be ignored for files
            node.custom_function_output = Some(Ok("3f,1d,500B".to_string()));
            node
        },
        {
            let mut node = create_node_info("symlink", NodeType::Symlink);
            node.line_count = Some(888); // Should be ignored for files
            node.size = Some(888); // Should be ignored for files
            node
        },
    ];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    // Should only count file metadata for files
    assert_eq!(aggregator.line_total, Some(10));
    assert_eq!(aggregator.size_total, Some(100));
    // Should count function output from directory
    assert_eq!(aggregator.file_count_from_function, Some(3));
    assert_eq!(aggregator.size_from_function, Some(500));
}

#[test]
fn test_metadata_disabled_no_aggregation() {
    let config = RustreeLibConfig::default(); // All metadata disabled

    let nodes = vec![{
        let mut node = create_node_info("file.txt", NodeType::File);
        node.line_count = Some(100);
        node.word_count = Some(500);
        node.size = Some(1000);
        node
    }];

    let aggregator = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    // Should not aggregate anything when features are disabled
    assert_eq!(aggregator.line_total, None);
    assert_eq!(aggregator.word_total, None);
    assert_eq!(aggregator.size_total, None);

    let summary = aggregator.format_summary_additions();
    assert_eq!(summary, "");
}

#[test]
fn test_number_formatting_edge_cases() {
    // Test various number formatting scenarios
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_number(0),
        "0"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_number(1),
        "1"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_number(12),
        "12"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_number(123),
        "123"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_number(1234),
        "1,234"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_number(12345),
        "12,345"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_number(123456),
        "123,456"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_number(1234567),
        "1,234,567"
    );
}

#[test]
fn test_size_formatting_edge_cases() {
    // Test various size formatting scenarios
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_size(0),
        "0 B"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_size(1),
        "1 B"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_size(1023),
        "1023 B"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_size(1024),
        "1.0 KB"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_size(1536),
        "1.5 KB"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_size(1048576),
        "1.0 MB"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_size(1073741824),
        "1.0 GB"
    );
    assert_eq!(
        rustree::core::metadata::MetadataAggregator::format_size(1099511627776),
        "1.0 TB"
    );
}

#[test]
fn test_aggregation_consistency_across_multiple_calls() {
    let mut config = RustreeLibConfig::default();
    config.metadata.calculate_line_count = true;
    config.metadata.calculate_word_count = true;

    let nodes = vec![
        {
            let mut node = create_node_info("file1.txt", NodeType::File);
            node.line_count = Some(10);
            node.word_count = Some(50);
            node
        },
        {
            let mut node = create_node_info("file2.txt", NodeType::File);
            node.line_count = Some(20);
            node.word_count = Some(100);
            node
        },
    ];

    // Call aggregation multiple times with same data
    let aggregator1 = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    let aggregator2 = MetadataAggregator::aggregate_from_nodes(&nodes, &config);
    let aggregator3 = MetadataAggregator::aggregate_from_nodes(&nodes, &config);

    // All should produce identical results
    assert_eq!(aggregator1.line_total, aggregator2.line_total);
    assert_eq!(aggregator2.line_total, aggregator3.line_total);
    assert_eq!(aggregator1.word_total, aggregator2.word_total);
    assert_eq!(aggregator2.word_total, aggregator3.word_total);

    assert_eq!(aggregator1.line_total, Some(30));
    assert_eq!(aggregator1.word_total, Some(150));
}
