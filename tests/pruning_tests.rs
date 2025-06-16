// tests/pruning_tests.rs
mod common;
use common::common_test_utils;

use anyhow::Result;
use rustree::{
    FilteringOptions, InputSourceOptions, LibOutputFormat, ListingOptions, NodeInfo,
    RustreeLibConfig, SortKey, SortingOptions, format_nodes, get_tree_nodes,
};
use std::collections::HashSet;
use std::fs;
// std::path::Path is not directly used, common_test_utils::get_root_name_from_path takes &Path but it's not used here.
// TempDir is used via its return type.
use tempfile::TempDir;

// Helper to get node names for assertion
fn get_node_names_set(nodes: &[NodeInfo]) -> HashSet<String> {
    nodes.iter().map(|n| n.name.clone()).collect()
}

// Helper to get (name, depth) for assertion, useful for checking structure
fn get_node_details_vec(nodes: &[NodeInfo]) -> Vec<(String, usize)> {
    nodes.iter().map(|n| (n.name.clone(), n.depth)).collect()
}

// Helper to create a basic config for pruning tests
fn create_test_config(
    root_name: String,
    prune: bool,
    max_depth: Option<usize>,
) -> RustreeLibConfig {
    RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name,
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth,
            show_hidden: true, // Show hidden to ensure they are pruned if empty, or kept if not
            ..Default::default()
        },
        filtering: FilteringOptions {
            prune_empty_directories: prune,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name), // For predictable test output
            ..Default::default()
        },
        // metadata: MetadataOptions is not configured here, so ..Default::default() covers it.
        ..Default::default()
    }
}

#[test]
fn test_prune_when_disabled() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    fs::create_dir(p.join("empty_dir"))?;
    common_test_utils::create_file_with_content(p, "file.txt", "content")?;

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), false, None);
    let nodes = get_tree_nodes(p, &config)?;
    let names = get_node_names_set(&nodes);

    assert!(
        names.contains("empty_dir"),
        "empty_dir should be present when pruning is off"
    );
    assert!(names.contains("file.txt"), "file.txt should be present");
    assert_eq!(names.len(), 2);
    Ok(())
}

#[test]
fn test_prune_simple_empty_dir() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    fs::create_dir(p.join("empty_dir_to_prune"))?;
    common_test_utils::create_file_with_content(p, "file_to_keep.txt", "content")?;

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    let nodes = get_tree_nodes(p, &config)?;
    let names = get_node_names_set(&nodes);

    assert!(
        !names.contains("empty_dir_to_prune"),
        "empty_dir_to_prune should be pruned"
    );
    assert!(
        names.contains("file_to_keep.txt"),
        "file_to_keep.txt should remain"
    );
    assert_eq!(names.len(), 1);
    Ok(())
}

#[test]
fn test_prune_dir_with_only_empty_subdir() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    fs::create_dir_all(p.join("parent_dir/empty_child_dir"))?;
    common_test_utils::create_file_with_content(p, "root_file.txt", "content")?;

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    let nodes = get_tree_nodes(p, &config)?;
    let names = get_node_names_set(&nodes);

    assert!(!names.contains("parent_dir"), "parent_dir should be pruned");
    assert!(
        !names.contains("empty_child_dir"),
        "empty_child_dir should be pruned"
    );
    assert!(
        names.contains("root_file.txt"),
        "root_file.txt should remain"
    );
    assert_eq!(names.len(), 1);
    Ok(())
}

#[test]
fn test_prune_dir_with_file_not_pruned() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    fs::create_dir(p.join("dir_with_file"))?;
    common_test_utils::create_file_with_content(&p.join("dir_with_file"), "file.txt", "content")?;

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    let nodes = get_tree_nodes(p, &config)?;
    let node_details = get_node_details_vec(&nodes);

    let expected_details = vec![
        ("dir_with_file".to_string(), 1),
        ("file.txt".to_string(), 2),
    ];
    assert_eq!(
        node_details, expected_details,
        "Directory with file was incorrectly pruned or structured"
    );
    Ok(())
}

#[test]
fn test_prune_dir_with_non_empty_subdir_not_pruned() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    fs::create_dir_all(p.join("parent/child_with_file"))?;
    common_test_utils::create_file_with_content(
        &p.join("parent/child_with_file"),
        "file.txt",
        "content",
    )?;

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    let nodes = get_tree_nodes(p, &config)?;
    let node_details = get_node_details_vec(&nodes);

    let expected_details = vec![
        ("parent".to_string(), 1),
        ("child_with_file".to_string(), 2),
        ("file.txt".to_string(), 3),
    ];
    assert_eq!(
        node_details, expected_details,
        "Directory with non-empty subdir was incorrectly pruned or structured"
    );
    Ok(())
}

#[test]
fn test_prune_nested_empty_dirs() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    fs::create_dir_all(p.join("empty1/empty2/empty3"))?;
    common_test_utils::create_file_with_content(p, "root_file.txt", "content")?;

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    let nodes = get_tree_nodes(p, &config)?;
    let names = get_node_names_set(&nodes);

    assert!(!names.contains("empty1"));
    assert!(!names.contains("empty2"));
    assert!(!names.contains("empty3"));
    assert!(names.contains("root_file.txt"));
    assert_eq!(names.len(), 1);
    Ok(())
}

#[test]
fn test_prune_mixed_content_scenario() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();

    // dir_A (kept)
    fs::create_dir(p.join("dir_A"))?;
    common_test_utils::create_file_with_content(&p.join("dir_A"), "file_A1.txt", "content")?;

    // dir_B (pruned)
    fs::create_dir_all(p.join("dir_B/empty_B1/empty_B2"))?;

    // dir_C (kept)
    fs::create_dir_all(p.join("dir_C/non_empty_C1"))?;
    common_test_utils::create_file_with_content(&p.join("dir_C"), "file_C1.txt", "content")?;
    common_test_utils::create_file_with_content(
        &p.join("dir_C/non_empty_C1"),
        "file_C2.txt",
        "content",
    )?;

    // empty_top_dir (pruned)
    fs::create_dir(p.join("empty_top_dir"))?;

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    let nodes = get_tree_nodes(p, &config)?;
    let node_details = get_node_details_vec(&nodes);

    // Expected structure after pruning (sorted by name)
    let expected_details = vec![
        ("dir_A".to_string(), 1),
        ("file_A1.txt".to_string(), 2),
        ("dir_C".to_string(), 1),
        ("file_C1.txt".to_string(), 2), // Sorted before non_empty_C1
        ("non_empty_C1".to_string(), 2),
        ("file_C2.txt".to_string(), 3),
    ];

    assert_eq!(
        node_details, expected_details,
        "Mixed content pruning failed. Actual: {:?}",
        node_details
    );

    let names = get_node_names_set(&nodes);
    assert!(!names.contains("dir_B"), "dir_B should be pruned");
    assert!(!names.contains("empty_B1"), "empty_B1 should be pruned");
    assert!(!names.contains("empty_B2"), "empty_B2 should be pruned");
    assert!(
        !names.contains("empty_top_dir"),
        "empty_top_dir should be pruned"
    );

    Ok(())
}

#[test]
fn test_prune_root_becomes_empty() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    fs::create_dir_all(p.join("root_empty_dir/another_empty"))?;

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    let nodes = get_tree_nodes(p, &config)?;

    assert!(
        nodes.is_empty(),
        "Node list should be empty if root effectively becomes empty after pruning. Found: {:?}",
        get_node_details_vec(&nodes)
    );
    Ok(())
}

#[test]
fn test_prune_does_not_affect_files_at_root_level() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    common_test_utils::create_file_with_content(p, "file1.txt", "content")?;
    common_test_utils::create_file_with_content(p, "file2.txt", "content")?;
    fs::create_dir(p.join("empty_dir"))?; // This dir should be pruned

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    let nodes = get_tree_nodes(p, &config)?;
    let names = get_node_names_set(&nodes);

    assert!(names.contains("file1.txt"));
    assert!(names.contains("file2.txt"));
    assert!(!names.contains("empty_dir"));
    assert_eq!(
        names.len(),
        2,
        "Expected 2 files, empty_dir pruned. Found: {:?}",
        names
    );
    Ok(())
}

#[test]
fn test_prune_with_hidden_files_and_dirs() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();

    // Kept structure
    fs::create_dir(p.join("kept_dir"))?;
    common_test_utils::create_file_with_content(
        &p.join("kept_dir"),
        ".hidden_file_in_kept.txt",
        "content",
    )?;

    // Pruned structure
    fs::create_dir(p.join(".hidden_empty_dir"))?;
    fs::create_dir_all(p.join("visible_parent_of_empty_hidden/.empty_child"))?;

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    // config.listing.show_hidden is true by default in create_test_config

    let nodes = get_tree_nodes(p, &config)?;
    let node_details = get_node_details_vec(&nodes);

    let expected_details = vec![
        ("kept_dir".to_string(), 1),
        (".hidden_file_in_kept.txt".to_string(), 2),
    ];

    assert_eq!(
        node_details, expected_details,
        "Pruning with hidden files/dirs failed. Actual: {:?}",
        node_details
    );

    let names = get_node_names_set(&nodes);
    assert!(
        !names.contains(".hidden_empty_dir"),
        ".hidden_empty_dir should be pruned"
    );
    assert!(
        !names.contains("visible_parent_of_empty_hidden"),
        "visible_parent_of_empty_hidden should be pruned"
    );
    assert!(
        !names.contains(".empty_child"),
        ".empty_child should be pruned"
    );

    Ok(())
}

#[test]
fn test_prune_output_format_consistency() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    fs::create_dir_all(p.join("dir1/sub1"))?; // sub1 is empty, dir1 will be pruned
    common_test_utils::create_file_with_content(&p.join("dir1"), "file1.txt", "content")?; // Makes dir1 not empty
    fs::create_dir(p.join("dir2"))?; // dir2 is empty, will be pruned
    common_test_utils::create_file_with_content(p, "root_file.txt", "content")?;

    let root_name = common_test_utils::get_root_name_from_path(p);
    let config_prune_on = create_test_config(root_name.clone(), true, None);
    let config_prune_off = create_test_config(root_name.clone(), false, None);

    // Get nodes with pruning OFF
    let nodes_off = get_tree_nodes(p, &config_prune_off)?;
    let output_off = format_nodes(&nodes_off, LibOutputFormat::Text, &config_prune_off)?;

    // Get nodes with pruning ON
    let nodes_on = get_tree_nodes(p, &config_prune_on)?;
    let output_on = format_nodes(&nodes_on, LibOutputFormat::Text, &config_prune_on)?;

    println!("--- Output Pruning OFF ---");
    println!("{}", output_off);
    println!("--- Output Pruning ON ---");
    println!("{}", output_on);

    // Assertions based on expected content
    // With pruning OFF: dir1, sub1, file1.txt, dir2, root_file.txt
    assert!(output_off.contains("dir1/"));
    assert!(output_off.contains("sub1/")); // sub1 is child of dir1
    assert!(output_off.contains("file1.txt"));
    assert!(output_off.contains("dir2/"));
    assert!(output_off.contains("root_file.txt"));
    // Summary for OFF: dir1, sub1, dir2 (3 dirs) + root_file.txt, file1.txt (2 files)
    // Total 3 child dirs, 2 child files. Root is 1 dir.
    // Formatter counts children for summary.
    // Nodes off: dir1 (d1), sub1 (d2), file1.txt (d2), dir2 (d1), root_file.txt (d1)
    // Dirs in nodes_off: dir1, sub1, dir2. Files: file1.txt, root_file.txt
    // Summary: 4 directories, 2 files (these are children counts)
    assert!(output_off.trim_end().ends_with("4 directories, 2 files"));

    // With pruning ON: dir1, file1.txt, root_file.txt (sub1 and dir2 are pruned)
    assert!(output_on.contains("dir1/"));
    assert!(
        !output_on.contains("sub1/"),
        "sub1 should be pruned from output_on"
    );
    assert!(output_on.contains("file1.txt"));
    assert!(
        !output_on.contains("dir2/"),
        "dir2 should be pruned from output_on"
    );
    assert!(output_on.contains("root_file.txt"));
    // Summary for ON: dir1 (1 dir) + root_file.txt, file1.txt (2 files)
    // Nodes on: dir1 (d1), file1.txt (d2), root_file.txt (d1)
    // Dirs in nodes_on: dir1. Files: file1.txt, root_file.txt
    // Summary: 1 directory, 2 files
    assert!(output_on.trim_end().ends_with("2 directories, 2 files"));

    Ok(())
}

#[test]
fn test_prune_empty_root_directory_scenario() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    // Root directory is 'p'. It's initially empty.

    let config = create_test_config(common_test_utils::get_root_name_from_path(p), true, None);
    let nodes = get_tree_nodes(p, &config)?;
    assert!(
        nodes.is_empty(),
        "Nodes should be empty for an empty root directory with pruning enabled"
    );

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    let root_name = common_test_utils::get_root_name_from_path(p);
    // The root itself is displayed, then the summary.
    // If nodes is empty, the summary should reflect 0 child directories and 0 child files.
    // The root itself is counted as 1 directory by the formatter if root_is_directory is true.
    // However, the PRD implies that if the root itself becomes empty, the output might be minimal.
    // The current `format_nodes` logic for summary counts nodes passed to it.
    // If `nodes` is empty, it will report "1 directory, 0 files" for children.
    // The root name is always printed.
    // With the new changes, empty directories output "0 directories, 0 files" first
    let expected_output = format!(
        r#"{}/
0 directories, 0 files

1 directory, 0 files"#, // This reflects 0 child directories and 0 child files plus the root.
        root_name
    );
    assert_eq!(
        output.trim(),
        expected_output.trim(),
        "Output for empty pruned root is incorrect"
    );
    Ok(())
}
