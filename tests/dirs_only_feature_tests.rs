// tests/dirs_only_feature_tests.rs

mod common;
use common::common_test_utils;

use anyhow::Result;
use rustree::{
    BuiltInFunction, InputSourceOptions, LibOutputFormat, ListingOptions, MetadataOptions,
    NodeType, RustreeLibConfig, SortKey, SortingOptions, format_nodes, get_tree_nodes,
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// Helper to get the root display name from a TempDir
fn get_root_name(temp_dir: &TempDir) -> String {
    temp_dir
        .path()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

// Helper for creating directory structures
fn create_dir_structure_for_d_tests(base_path: &Path) -> Result<()> {
    fs::create_dir_all(base_path.join("dir_a/sub_dir_a1"))?;
    fs::create_dir_all(base_path.join("dir_b"))?;
    fs::create_dir_all(base_path.join(".hidden_dir/sub_hidden"))?;
    common_test_utils::create_file_with_content(
        &base_path.join("dir_a"),
        "file_in_a.txt",
        "content",
    )?;
    common_test_utils::create_file_with_content(base_path, "root_file.txt", "content")?;
    Ok(())
}

// --- Basic -d Functionality ---

#[test]
fn test_d_filters_files_shows_only_dirs() -> Result<()> {
    let temp_dir = TempDir::new()?;
    create_dir_structure_for_d_tests(temp_dir.path())?;
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;

    assert!(
        nodes.iter().all(|n| n.node_type == NodeType::Directory),
        "Not all nodes are directories"
    );
    assert!(
        !nodes.iter().any(|n| n.name == "root_file.txt"),
        "root_file.txt should be filtered"
    );
    assert!(
        !nodes.iter().any(|n| n.name == "file_in_a.txt"),
        "file_in_a.txt should be filtered"
    );

    let node_names: Vec<_> = nodes.iter().map(|n| n.name.as_str()).collect();
    assert!(node_names.contains(&"dir_a"));
    assert!(node_names.contains(&"sub_dir_a1"));
    assert!(node_names.contains(&"dir_b"));
    assert_eq!(
        nodes.len(),
        3,
        "Expected 3 directories. Found: {:?}",
        node_names
    );

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!(
        "[test_d_filters_files_shows_only_dirs]\nOutput:\n{}",
        output
    );
    assert!(!output.contains("root_file.txt"));
    assert!(!output.contains("file_in_a.txt"));
    assert!(output.contains("dir_a/"));
    assert!(output.contains("sub_dir_a1/"));
    assert!(output.contains("dir_b/"));
    assert!(
        output.starts_with(&format!("{}/", root_name)),
        "Root directory name missing trailing slash or incorrect. Output: {}",
        output
    );
    assert!(
        output.trim_end().ends_with("4 directories, 0 files"),
        "Summary line mismatch. Output: {}",
        output
    );

    Ok(())
}

#[test]
fn test_d_on_empty_directory() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    assert!(
        nodes.is_empty(),
        "Nodes should be empty for an empty directory"
    );

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("[test_d_on_empty_directory]\nOutput:\n{}", output);
    // Root is a directory, so it's counted.
    let expected_output = format!("{}/\n\n1 directory, 0 files", root_name);
    assert_eq!(output.trim(), expected_output.trim());
    Ok(())
}

#[test]
fn test_d_on_directory_with_only_files_no_subdirs() -> Result<()> {
    let temp_dir = TempDir::new()?;
    common_test_utils::create_file_with_content(temp_dir.path(), "file1.txt", "a")?;
    common_test_utils::create_file_with_content(temp_dir.path(), "file2.txt", "b")?;
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    assert!(
        nodes.is_empty(),
        "Nodes should be empty if only files exist and -d is active"
    );

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!(
        "[test_d_on_directory_with_only_files_no_subdirs]\nOutput:\n{}",
        output
    );
    // Root is a directory, so it's counted.
    let expected_output = format!("{}/\n\n1 directory, 0 files", root_name);
    assert_eq!(output.trim(), expected_output.trim());
    Ok(())
}

// --- -d Interaction with Other Flags ---

#[test]
fn test_d_with_max_depth_l() -> Result<()> {
    let temp_dir = TempDir::new()?;
    create_dir_structure_for_d_tests(temp_dir.path())?; // dir_a/sub_dir_a1
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config_depth_1 = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            max_depth: Some(1),
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes_d1 = get_tree_nodes(root_path, &config_depth_1)?;
    let node_names_d1: Vec<_> = nodes_d1.iter().map(|n| n.name.as_str()).collect();
    assert!(
        node_names_d1.contains(&"dir_a"),
        "dir_a missing at depth 1. Found: {:?}",
        node_names_d1
    );
    assert!(
        node_names_d1.contains(&"dir_b"),
        "dir_b missing at depth 1. Found: {:?}",
        node_names_d1
    );
    assert!(
        !node_names_d1.contains(&"sub_dir_a1"),
        "sub_dir_a1 should not be present at depth 1. Found: {:?}",
        node_names_d1
    );
    assert_eq!(
        nodes_d1.len(),
        2,
        "Expected 2 directories at depth 1. Found: {:?}",
        node_names_d1
    );
    for node in &nodes_d1 {
        assert_eq!(node.depth, 1, "Node {} has incorrect depth", node.name);
    }

    let output_d1 = format_nodes(&nodes_d1, LibOutputFormat::Text, &config_depth_1)?;
    println!("[test_d_with_max_depth_l] Depth 1 Output:\n{}", output_d1);
    // nodes_d1.len() is 2 (children), root is 1. Total 3.
    assert!(output_d1.trim_end().ends_with("3 directories, 0 files"));

    let config_depth_2 = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            max_depth: Some(2),
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };
    let nodes_d2 = get_tree_nodes(root_path, &config_depth_2)?;
    let node_names_d2: Vec<_> = nodes_d2.iter().map(|n| n.name.as_str()).collect();
    assert!(node_names_d2.contains(&"dir_a"));
    assert!(node_names_d2.contains(&"sub_dir_a1"));
    assert!(node_names_d2.contains(&"dir_b"));
    assert_eq!(
        nodes_d2.len(),
        3,
        "Expected 3 directories at depth 2. Found: {:?}",
        node_names_d2
    );

    let output_d2 = format_nodes(&nodes_d2, LibOutputFormat::Text, &config_depth_2)?;
    println!("[test_d_with_max_depth_l] Depth 2 Output:\n{}", output_d2);
    // nodes_d2.len() is 3 (children), root is 1. Total 4.
    assert!(output_d2.trim_end().ends_with("4 directories, 0 files"));
    Ok(())
}

#[test]
fn test_d_with_show_hidden_a() -> Result<()> {
    let temp_dir = TempDir::new()?;
    create_dir_structure_for_d_tests(temp_dir.path())?; // .hidden_dir/sub_hidden
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            show_hidden: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let node_names: Vec<_> = nodes.iter().map(|n| n.name.as_str()).collect();
    assert!(
        node_names.contains(&".hidden_dir"),
        "Missing .hidden_dir. Found: {:?}",
        node_names
    );
    assert!(
        node_names.contains(&"sub_hidden"),
        "Missing sub_hidden. Found: {:?}",
        node_names
    );
    assert!(node_names.contains(&"dir_a"));
    assert!(node_names.contains(&"dir_b"));
    assert!(
        !nodes.iter().any(|n| n.node_type == NodeType::File),
        "Files should not be present"
    );
    assert_eq!(
        nodes.len(),
        5,
        "Expected 5 directories with hidden. Found: {:?}",
        node_names
    );

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("[test_d_with_show_hidden_a]\nOutput:\n{}", output);
    assert!(output.contains(".hidden_dir/"));
    assert!(output.contains("sub_hidden/"));
    // nodes.len() is 5 (children), root is 1. Total 6.
    assert!(output.trim_end().ends_with("6 directories, 0 files"));
    Ok(())
}

#[test]
fn test_d_with_report_sizes_s_for_dirs() -> Result<()> {
    let temp_dir = TempDir::new()?;
    create_dir_structure_for_d_tests(temp_dir.path())?;
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    for node in &nodes {
        assert_eq!(node.node_type, NodeType::Directory);
        assert!(
            node.size.is_some(),
            "Directory {} should have size reported",
            node.name
        );
    }

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("[test_d_with_report_sizes_s_for_dirs]\nOutput:\n{}", output);
    for line in output.lines() {
        if (line.contains("├──") || line.contains("└──")) && !line.contains("B]") {
            panic!("Directory line missing size prefix: {}", line);
        }
    }
    assert!(output.contains("B] dir_a/")); // Example check
    Ok(())
}

#[test]
fn test_d_with_report_mtime_big_d_for_dirs() -> Result<()> {
    let temp_dir = TempDir::new()?;
    create_dir_structure_for_d_tests(temp_dir.path())?;
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_mtime: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::MTime),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    for node in &nodes {
        assert_eq!(node.node_type, NodeType::Directory);
        assert!(
            node.mtime.is_some(),
            "Directory {} should have mtime reported",
            node.name
        );
    }

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!(
        "[test_d_with_report_mtime_big_d_for_dirs]\nOutput:\n{}",
        output
    );
    for line in output.lines() {
        if (line.contains("├──") || line.contains("└──")) && !line.contains("MTime:") {
            panic!("Directory line missing MTime prefix: {}", line);
        }
    }
    assert!(output.contains("MTime:")); // Example check
    Ok(())
}

#[test]
fn test_d_ignores_file_specific_stats_options() -> Result<()> {
    let temp_dir = TempDir::new()?;
    create_dir_structure_for_d_tests(temp_dir.path())?;
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            calculate_line_count: true,
            calculate_word_count: true,
            apply_function: Some(BuiltInFunction::CountPluses),
            report_sizes: true, // Keep one dir-compatible flag
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    for node in &nodes {
        assert_eq!(node.node_type, NodeType::Directory);
        assert!(
            node.line_count.is_none(),
            "Line count should be None for dir {}",
            node.name
        );
        assert!(
            node.word_count.is_none(),
            "Word count should be None for dir {}",
            node.name
        );
        assert!(
            node.custom_function_output.is_none(),
            "Custom func output should be None for dir {}",
            node.name
        );
        assert!(
            node.size.is_some(),
            "Size should be Some for dir {}",
            node.name
        );
    }

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!(
        "[test_d_ignores_file_specific_stats_options]\nOutput:\n{}",
        output
    );
    for line in output.lines() {
        if line.contains("├──") || line.contains("└──") {
            assert!(
                !line.contains("[L:"),
                "Line count prefix found for dir: {}",
                line
            );
            assert!(
                !line.contains("[W:"),
                "Word count prefix found for dir: {}",
                line
            );
            assert!(
                !line.contains("[F:"),
                "Function prefix found for dir: {}",
                line
            );
            assert!(line.contains("B]"), "Size prefix missing for dir: {}", line);
        }
    }
    Ok(())
}

// --- -d Interaction with Sorting ---

#[test]
fn test_d_with_sort_by_name_default() -> Result<()> {
    let temp_dir = TempDir::new()?;
    fs::create_dir(temp_dir.path().join("ccc_dir"))?;
    fs::create_dir(temp_dir.path().join("aaa_dir"))?;
    fs::create_dir(temp_dir.path().join("bbb_dir"))?;
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0].name, "aaa_dir");
    assert_eq!(nodes[1].name, "bbb_dir");
    assert_eq!(nodes[2].name, "ccc_dir");

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("[test_d_with_sort_by_name_default]\nOutput:\n{}", output);
    assert!(output.contains("aaa_dir/\n├── bbb_dir/\n└── ccc_dir/"));
    Ok(())
}

#[test]
fn test_d_with_sort_by_mtime_t() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();

    fs::create_dir(p.join("dir_oldest"))?;
    std::thread::sleep(std::time::Duration::from_millis(50));
    fs::create_dir(p.join("dir_newest"))?;
    std::thread::sleep(std::time::Duration::from_millis(50));
    fs::create_dir(p.join("dir_middle"))?;
    // To make dir_middle newest for default sort (mtime ascending)
    // We need to touch dir_middle last, or dir_newest second to last.
    // Let's reset and create with controlled timing for mtime sort (oldest first)
    // dir_oldest (created first)
    // dir_middle (created second)
    // dir_newest (created third)
    // So order should be oldest, middle, newest
    // The test setup above is: oldest, newest, middle. So order: oldest, newest, middle.
    // Let's fix the setup for clearer expectation:
    fs::remove_dir_all(p.join("dir_newest"))?;
    fs::remove_dir_all(p.join("dir_middle"))?;
    std::thread::sleep(std::time::Duration::from_millis(50));
    fs::create_dir(p.join("dir_middle"))?; // Middle mtime
    std::thread::sleep(std::time::Duration::from_millis(50));
    fs::create_dir(p.join("dir_newest"))?; // Newest mtime

    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_mtime: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::MTime),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0].name, "dir_oldest");
    assert_eq!(nodes[1].name, "dir_middle");
    assert_eq!(nodes[2].name, "dir_newest");

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("[test_d_with_sort_by_mtime_t]\nOutput:\n{}", output);
    // Visual order check is tricky with MTime values, but programmatic sort is key.
    Ok(())
}

#[test]
fn test_d_with_reverse_sort_r() -> Result<()> {
    let temp_dir = TempDir::new()?;
    fs::create_dir(temp_dir.path().join("ccc_dir"))?;
    fs::create_dir(temp_dir.path().join("aaa_dir"))?;
    fs::create_dir(temp_dir.path().join("bbb_dir"))?;
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            reverse_sort: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0].name, "ccc_dir");
    assert_eq!(nodes[1].name, "bbb_dir");
    assert_eq!(nodes[2].name, "aaa_dir");

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("[test_d_with_reverse_sort_r]\nOutput:\n{}", output);
    assert!(output.contains("ccc_dir/\n├── bbb_dir/\n└── aaa_dir/"));
    Ok(())
}

#[test]
fn test_d_with_unsorted_big_u() -> Result<()> {
    let temp_dir = TempDir::new()?;
    // Create in a specific, non-alpha order
    fs::create_dir(temp_dir.path().join("order_2_zeta"))?;
    fs::create_dir(temp_dir.path().join("order_1_alpha"))?;
    fs::create_dir(temp_dir.path().join("order_3_gamma"))?;
    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: None,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    // OS-dependent, but check it's not sorted alphabetically as a basic check
    if nodes.len() == 3 {
        let names: Vec<_> = nodes.iter().map(|n| n.name.as_str()).collect();
        let sorted_names = {
            let mut temp = names.clone();
            temp.sort();
            temp
        };
        if names != sorted_names {
            println!(
                "Unsorted order is different from sorted, as expected for -U (actual: {:?})",
                names
            );
        } else {
            // This might happen on some filesystems, not a strict failure for -U
            println!(
                "Warning: Unsorted order happened to be alphabetical for -U (actual: {:?})",
                names
            );
        }
    }

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("[test_d_with_unsorted_big_u]\nOutput:\n{}", output);
    // Check summary. nodes.len() is 3 (children), root is 1. Total 4.
    assert!(output.trim_end().ends_with("4 directories, 0 files"));
    Ok(())
}

// --- -d with Symlinks ---

#[test]
#[cfg(unix)]
fn test_d_with_symlinks_to_dirs_and_files() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let p = temp_dir.path();
    fs::create_dir(p.join("actual_dir"))?;
    common_test_utils::create_file_with_content(p, "actual_file.txt", "content")?;
    fs::create_dir(p.join("another_actual_dir"))?;

    std::os::unix::fs::symlink(p.join("actual_dir"), p.join("link_to_actual_dir"))?;
    std::os::unix::fs::symlink(p.join("actual_file.txt"), p.join("link_to_actual_file"))?;
    std::os::unix::fs::symlink("non_existent_target", p.join("link_to_nothing"))?;

    let root_path = temp_dir.path();
    let root_name = get_root_name(&temp_dir);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: root_name.clone(),
            root_is_directory: true,
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Name),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(root_path, &config)?;
    let node_names: Vec<String> = nodes.iter().map(|n| n.name.clone()).collect();

    assert!(
        node_names.contains(&"actual_dir".to_string()),
        "actual_dir missing. Found: {:?}",
        node_names
    );
    assert!(
        node_names.contains(&"another_actual_dir".to_string()),
        "another_actual_dir missing. Found: {:?}",
        node_names
    );

    let link_to_dir_node = nodes.iter().find(|n| n.name == "link_to_actual_dir");
    assert!(
        link_to_dir_node.is_some(),
        "link_to_actual_dir missing. Found: {:?}",
        node_names
    );
    assert_eq!(
        link_to_dir_node.unwrap().node_type,
        NodeType::Directory,
        "link_to_actual_dir should be NodeType::Directory"
    );

    assert!(
        !node_names.contains(&"actual_file.txt".to_string()),
        "actual_file.txt should be filtered. Found: {:?}",
        node_names
    );
    assert!(
        !node_names.contains(&"link_to_actual_file".to_string()),
        "link_to_actual_file should be filtered. Found: {:?}",
        node_names
    );
    assert!(
        !node_names.contains(&"link_to_nothing".to_string()),
        "link_to_nothing should be filtered. Found: {:?}",
        node_names
    );

    // Expected: actual_dir, another_actual_dir, link_to_actual_dir
    assert_eq!(
        nodes.len(),
        3,
        "Expected 3 directory entries. Found: {:?}",
        node_names
    );

    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!(
        "[test_d_with_symlinks_to_dirs_and_files]\nOutput:\n{}",
        output
    );
    // nodes.len() is 3 (children), root is 1. Total 4.
    assert!(output.trim_end().ends_with("4 directories, 0 files"));
    Ok(())
}
