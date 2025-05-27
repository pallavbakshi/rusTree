// tests/walker_integration_tests.rs

use rustree::{get_tree_nodes, RustreeLibConfig, NodeType};
use anyhow::Result;

mod common;
use common::common_test_utils;

#[test]
fn test_walker_basic_depth_one() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        max_depth: Some(1),
        show_hidden: false,
        ..Default::default()
    };

    let mut nodes = get_tree_nodes(root_path, &config)?;
    nodes.sort_by_key(|n| n.path.clone()); // For stable assertions

    // Expected at depth 1 (excluding .hidden_file): file1.txt, file2.log, sub_dir
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0].name, "file1.txt");
    assert_eq!(nodes[1].name, "file2.log");
    assert_eq!(nodes[2].name, "sub_dir");
    assert_eq!(nodes[2].node_type, NodeType::Directory);

    Ok(())
}

#[test]
fn test_walker_show_hidden_at_depth_two() -> Result<()> {
    let temp_dir = common_test_utils::setup_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        max_depth: Some(2), // Need depth 2 to reach .hidden_file
        show_hidden: true,
        ..Default::default()
    };

    let mut nodes = get_tree_nodes(root_path, &config)?;
    nodes.sort_by_key(|n| n.path.clone()); // For stable assertions

    // Expected: file1.txt, file2.log, sub_dir, sub_dir/.hidden_file, sub_dir/file3.dat
    assert_eq!(nodes.len(), 5);

    let hidden_node = nodes.iter().find(|n| n.name == ".hidden_file").expect(".hidden_file not found");
    assert_eq!(hidden_node.depth, 2);
    assert_eq!(hidden_node.node_type, NodeType::File);
    
    let file3_node = nodes.iter().find(|n| n.name == "file3.dat").expect("file3.dat not found");
    assert_eq!(file3_node.depth, 2);

    Ok(())
}

// Add more walker-specific tests here, e.g.:
// - Test behavior with non-existent root path (should return Err)
// - Test symlink handling (if implemented and configured)
// - Test ignore patterns (if implemented)
// - Test specific edge cases for depth and hidden files logic in walker.rs