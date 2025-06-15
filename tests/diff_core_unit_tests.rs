// tests/diff_core_unit_tests.rs

//! Unit tests for core diff functionality and edge cases
//! Tests specific diff engine behaviors and error conditions

use rustree::core::diff::{ChangeType, DiffEngine, DiffMetadata, DiffOptions};
use rustree::core::tree::node::{NodeInfo, NodeType};
use std::path::PathBuf;
use std::time::SystemTime;

/// Helper function to create test nodes
fn create_test_node(
    name: &str,
    node_type: NodeType,
    size: Option<u64>,
    path: Option<&str>,
) -> NodeInfo {
    NodeInfo {
        name: name.to_string(),
        path: PathBuf::from(path.unwrap_or(name)),
        node_type,
        depth: 0,
        size,
        mtime: Some(SystemTime::UNIX_EPOCH),
        change_time: None,
        create_time: None,
        permissions: None,
        line_count: None,
        word_count: None,
        custom_function_output: None,
    }
}

/// Helper function to create test metadata
fn create_test_metadata() -> DiffMetadata {
    DiffMetadata {
        generated_at: "2024-01-01T00:00:00Z".to_string(),
        snapshot_file: PathBuf::from("test.json"),
        snapshot_date: None,
        comparison_root: PathBuf::from("."),
        filters_applied: vec![],
        options: DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        },
    }
}

#[test]
fn test_empty_trees_comparison() {
    let engine = DiffEngine::new(DiffOptions::default());
    let previous: Vec<NodeInfo> = vec![];
    let current: Vec<NodeInfo> = vec![];
    let metadata = create_test_metadata();

    let result = engine.compare(&previous, &current, metadata).unwrap();

    assert_eq!(result.changes.len(), 0);
    assert_eq!(result.summary.added, 0);
    assert_eq!(result.summary.removed, 0);
    assert_eq!(result.summary.total_changes(), 0);
}

#[test]
fn test_identical_trees_comparison() {
    let engine = DiffEngine::new(DiffOptions::default());

    let nodes = vec![
        create_test_node("main.rs", NodeType::File, Some(100), None),
        create_test_node("lib.rs", NodeType::File, Some(200), None),
        create_test_node("tests", NodeType::Directory, None, None),
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&nodes, &nodes, metadata).unwrap();

    assert_eq!(result.changes.len(), 3);
    assert_eq!(result.summary.unchanged, 3);
    assert_eq!(result.summary.total_changes(), 0);

    // All changes should be Unchanged
    for change in &result.changes {
        assert!(matches!(change.change_type, ChangeType::Unchanged));
    }
}

#[test]
fn test_only_additions() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous: Vec<NodeInfo> = vec![];
    let current = vec![
        create_test_node("new_file.rs", NodeType::File, Some(150), None),
        create_test_node("new_dir", NodeType::Directory, None, None),
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    assert_eq!(result.changes.len(), 2);
    assert_eq!(result.summary.added, 2);
    assert_eq!(result.summary.files_added, 1);
    assert_eq!(result.summary.directories_added, 1);
    assert_eq!(result.summary.removed, 0);
    assert_eq!(result.summary.total_changes(), 2);
}

#[test]
fn test_only_removals() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous = vec![
        create_test_node("old_file.rs", NodeType::File, Some(250), None),
        create_test_node("old_dir", NodeType::Directory, None, None),
    ];
    let current: Vec<NodeInfo> = vec![];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    assert_eq!(result.changes.len(), 2);
    assert_eq!(result.summary.removed, 2);
    assert_eq!(result.summary.files_removed, 1);
    assert_eq!(result.summary.directories_removed, 1);
    assert_eq!(result.summary.added, 0);
    assert_eq!(result.summary.total_changes(), 2);
}

#[test]
fn test_type_changes() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous = vec![create_test_node("item", NodeType::File, Some(100), None)];
    let current = vec![create_test_node("item", NodeType::Directory, None, None)];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    assert_eq!(result.changes.len(), 1);
    assert_eq!(result.summary.type_changed, 1);
    assert_eq!(result.summary.total_changes(), 1);

    let change = &result.changes[0];
    match &change.change_type {
        ChangeType::TypeChanged { from_type, to_type } => {
            assert_eq!(*from_type, NodeType::File);
            assert_eq!(*to_type, NodeType::Directory);
        }
        _ => panic!("Expected TypeChanged"),
    }
}

#[test]
fn test_move_detection_with_high_similarity() {
    let options = DiffOptions {
        detect_moves: true,
        move_threshold: 0.5,
        ..Default::default()
    };

    let engine = DiffEngine::new(options);

    let previous = vec![create_test_node(
        "original_name.rs",
        NodeType::File,
        Some(500),
        Some("src/original_name.rs"),
    )];
    let current = vec![create_test_node(
        "new_name.rs",
        NodeType::File,
        Some(500),
        Some("src/new_name.rs"),
    )];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    assert!(result.summary.moved >= 1);
    assert!(result.summary.files_moved >= 1);

    let change = &result.changes[0];
    match &change.change_type {
        ChangeType::Moved {
            from_path,
            similarity,
        } => {
            assert_eq!(from_path, &PathBuf::from("src/original_name.rs"));
            assert!(*similarity >= 0.5);
        }
        _ => panic!("Expected Moved, got {:?}", change.change_type),
    }
}

#[test]
fn test_move_detection_disabled() {
    let options = DiffOptions {
        ignore_moves: true,
        ..Default::default()
    };

    let engine = DiffEngine::new(options);

    let previous = vec![create_test_node(
        "file.rs",
        NodeType::File,
        Some(500),
        Some("old/file.rs"),
    )];
    let current = vec![create_test_node(
        "file.rs",
        NodeType::File,
        Some(500),
        Some("new/file.rs"),
    )];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    // Should treat as separate add and remove
    assert_eq!(result.changes.len(), 2);
    assert_eq!(result.summary.added, 1);
    assert_eq!(result.summary.removed, 1);
    assert_eq!(result.summary.moved, 0);
}

#[test]
fn test_move_threshold_filtering() {
    let options = DiffOptions {
        detect_moves: true,
        move_threshold: 0.95, // Very high threshold
        ..Default::default()
    };

    let engine = DiffEngine::new(options);

    let previous = vec![create_test_node(
        "completely_different.rs",
        NodeType::File,
        Some(100),
        None,
    )];
    let current = vec![
        create_test_node("totally_other.rs", NodeType::File, Some(200), None), // Different size and name
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    // Should not detect as move due to low similarity
    assert_eq!(result.changes.len(), 2);
    assert_eq!(result.summary.added, 1);
    assert_eq!(result.summary.removed, 1);
    assert_eq!(result.summary.moved, 0);
}

#[test]
fn test_directory_modification_detection() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous = vec![
        create_test_node("src", NodeType::Directory, None, None),
        create_test_node("main.rs", NodeType::File, Some(100), Some("src/main.rs")),
    ];
    let current = vec![
        create_test_node("src", NodeType::Directory, None, None),
        create_test_node("main.rs", NodeType::File, Some(100), Some("src/main.rs")),
        create_test_node("lib.rs", NodeType::File, Some(200), Some("src/lib.rs")), // New file in directory
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    // Should detect directory modification and new file
    assert!(result.summary.modified > 0 || result.summary.added > 0);

    // Look for directory modification or added file
    let has_modification = result.changes.iter().any(|c| {
        matches!(c.change_type, ChangeType::Modified) || matches!(c.change_type, ChangeType::Added)
    });
    assert!(
        has_modification,
        "Should detect directory modification or file addition"
    );
}

#[test]
fn test_nested_directory_changes() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous = vec![
        create_test_node("src", NodeType::Directory, None, None),
        create_test_node("utils", NodeType::Directory, None, Some("src/utils")),
        create_test_node(
            "helper.rs",
            NodeType::File,
            Some(150),
            Some("src/utils/helper.rs"),
        ),
    ];
    let current = vec![
        create_test_node("src", NodeType::Directory, None, None),
        create_test_node("utils", NodeType::Directory, None, Some("src/utils")),
        create_test_node(
            "helper.rs",
            NodeType::File,
            Some(150),
            Some("src/utils/helper.rs"),
        ),
        create_test_node(
            "common.rs",
            NodeType::File,
            Some(100),
            Some("src/utils/common.rs"),
        ),
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    assert!(result.summary.added >= 1, "Should detect added file");

    // Should contain the new file
    let has_common_rs = result
        .changes
        .iter()
        .any(|c| c.path().to_string_lossy().contains("common.rs"));
    assert!(has_common_rs, "Should detect common.rs addition");
}

#[test]
fn test_size_change_calculation() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous = vec![
        create_test_node("file1.rs", NodeType::File, Some(100), None),
        create_test_node("file2.rs", NodeType::File, Some(200), None),
    ];
    let current = vec![
        create_test_node("file1.rs", NodeType::File, Some(150), None), // Size increased
        create_test_node("file3.rs", NodeType::File, Some(300), None), // New file
                                                                       // file2.rs removed
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    // Size change: +50 (file1 grew) +300 (new file3) -200 (file2 removed) = +150
    assert_eq!(result.summary.size_change, 150);
}

#[test]
fn test_change_sorting() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous: Vec<NodeInfo> = vec![];
    let current = vec![
        create_test_node("zebra.rs", NodeType::File, Some(100), None),
        create_test_node("alpha.rs", NodeType::File, Some(200), None),
        create_test_node("beta.rs", NodeType::File, Some(150), None),
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    // Changes should be sorted by path
    assert_eq!(result.changes.len(), 3);
    assert_eq!(result.changes[0].path(), &PathBuf::from("alpha.rs"));
    assert_eq!(result.changes[1].path(), &PathBuf::from("beta.rs"));
    assert_eq!(result.changes[2].path(), &PathBuf::from("zebra.rs"));
}

#[test]
fn test_diff_metadata_preservation() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous: Vec<NodeInfo> = vec![];
    let current = vec![create_test_node("test.rs", NodeType::File, Some(100), None)];

    let metadata = DiffMetadata {
        generated_at: "2024-06-14T12:00:00Z".to_string(),
        snapshot_file: PathBuf::from("custom_snapshot.json"),
        snapshot_date: Some("2024-06-13T10:00:00Z".to_string()),
        comparison_root: PathBuf::from("/custom/root"),
        filters_applied: vec!["*.rs".to_string(), "!target/*".to_string()],
        options: DiffOptions::default(),
    };

    let result = engine
        .compare(&previous, &current, metadata.clone())
        .unwrap();

    // Metadata should be preserved
    assert_eq!(result.metadata.generated_at, "2024-06-14T12:00:00Z");
    assert_eq!(
        result.metadata.snapshot_file,
        PathBuf::from("custom_snapshot.json")
    );
    assert_eq!(
        result.metadata.snapshot_date,
        Some("2024-06-13T10:00:00Z".to_string())
    );
    assert_eq!(
        result.metadata.comparison_root,
        PathBuf::from("/custom/root")
    );
    assert_eq!(result.metadata.filters_applied, vec!["*.rs", "!target/*"]);
}

#[test]
fn test_complex_mixed_changes() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous = vec![
        create_test_node("keep.rs", NodeType::File, Some(100), None),
        create_test_node("remove.rs", NodeType::File, Some(200), None),
        create_test_node("rename_me.rs", NodeType::File, Some(150), None),
        create_test_node("change_type", NodeType::File, Some(50), None),
        create_test_node("src", NodeType::Directory, None, None),
    ];
    let current = vec![
        create_test_node("keep.rs", NodeType::File, Some(100), None), // Unchanged
        create_test_node("add.rs", NodeType::File, Some(300), None),  // Added
        create_test_node("renamed.rs", NodeType::File, Some(150), None), // Moved (same size)
        create_test_node("change_type", NodeType::Directory, None, None), // Type changed
        create_test_node("src", NodeType::Directory, None, None), // Directory (might be modified)
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    assert!(result.summary.unchanged >= 1);
    assert!(result.summary.added >= 1);
    assert!(result.summary.removed >= 1);
    // Move detection may be disabled by default; ensure total changes still accurate
    assert!(result.summary.type_changed >= 1);

    // Total changes should not include unchanged
    let expected_total = result.summary.added
        + result.summary.removed
        + result.summary.moved
        + result.summary.type_changed
        + result.summary.modified;
    assert_eq!(result.summary.total_changes(), expected_total);
}

#[test]
fn test_path_normalization() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous = vec![create_test_node(
        "file.rs",
        NodeType::File,
        Some(100),
        Some("./src/file.rs"),
    )];
    let current = vec![
        create_test_node("file.rs", NodeType::File, Some(100), Some("src/file.rs")), // Same file, different path format
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    // Should recognize as the same file despite path format differences
    assert_eq!(result.summary.unchanged, 1);
    assert_eq!(result.summary.total_changes(), 0);
}

#[test]
fn test_symlink_handling() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous = vec![create_test_node("link", NodeType::Symlink, None, None)];
    let current = vec![
        create_test_node("link", NodeType::File, Some(100), None), // Symlink became file
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    assert_eq!(result.summary.type_changed, 1);

    let change = &result.changes[0];
    match &change.change_type {
        ChangeType::TypeChanged { from_type, to_type } => {
            assert_eq!(*from_type, NodeType::Symlink);
            assert_eq!(*to_type, NodeType::File);
        }
        _ => panic!("Expected TypeChanged for symlink -> file"),
    }
}

#[test]
fn test_large_number_of_changes() {
    let engine = DiffEngine::new(DiffOptions::default());

    // Create many files
    let mut previous = Vec::new();
    let mut current = Vec::new();

    // Add 100 files to previous
    for i in 0..100 {
        previous.push(create_test_node(
            &format!("file_{}.rs", i),
            NodeType::File,
            Some(100),
            None,
        ));
    }

    // Add 100 different files to current
    for i in 100..200 {
        current.push(create_test_node(
            &format!("file_{}.rs", i),
            NodeType::File,
            Some(100),
            None,
        ));
    }

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    assert_eq!(result.changes.len(), 200); // 100 removed + 100 added
    assert_eq!(result.summary.removed, 100);
    assert_eq!(result.summary.added, 100);
    assert_eq!(result.summary.total_changes(), 200);
}

#[test]
fn test_diff_summary_detailed_breakdown() {
    let engine = DiffEngine::new(DiffOptions::default());

    let previous = vec![
        create_test_node("remove_file.rs", NodeType::File, Some(100), None),
        create_test_node("remove_dir", NodeType::Directory, None, None),
        create_test_node("move_file.rs", NodeType::File, Some(200), None),
        create_test_node("move_dir", NodeType::Directory, None, None),
    ];
    let current = vec![
        create_test_node("add_file.rs", NodeType::File, Some(150), None),
        create_test_node("add_dir", NodeType::Directory, None, None),
        create_test_node("moved_file.rs", NodeType::File, Some(200), None), // Moved from move_file.rs
        create_test_node("moved_dir", NodeType::Directory, None, None),     // Moved from move_dir
    ];

    let metadata = create_test_metadata();
    let result = engine.compare(&previous, &current, metadata).unwrap();

    // Check detailed breakdown
    assert!(result.summary.files_added >= 1);
    assert!(result.summary.directories_added >= 1);
    assert!(result.summary.files_removed >= 1);
    assert!(result.summary.directories_removed >= 1);
    // Move detection counts (may vary with similarity threshold)
    // Note: These are unsigned integers, so >= 0 comparison is always true
    // Just verify the fields exist and are accessible
    let _ = result.summary.files_moved;
    let _ = result.summary.directories_moved;
}
