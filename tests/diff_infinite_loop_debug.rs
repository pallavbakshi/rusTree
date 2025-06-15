// tests/diff_infinite_loop_debug.rs

//! Test to reproduce and debug infinite loop issues in diff engine

use rustree::{DiffEngine, DiffMetadata, DiffOptions, NodeInfo, NodeType};
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};

/// Helper function to create test nodes with specific paths
fn create_node_with_path(
    name: &str,
    path: &str,
    node_type: NodeType,
    size: Option<u64>,
) -> NodeInfo {
    NodeInfo {
        name: name.to_string(),
        path: PathBuf::from(path),
        node_type,
        depth: path.matches('/').count(),
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

/// Create a deep nested directory structure that might cause infinite loops
fn create_deep_structure(depth: usize) -> Vec<NodeInfo> {
    let mut nodes = Vec::new();

    // Create root
    nodes.push(create_node_with_path(
        "root",
        "root",
        NodeType::Directory,
        None,
    ));

    // Create nested directories
    let mut current_path = String::from("root");
    for i in 0..depth {
        current_path = format!("{}/level_{}", current_path, i);
        nodes.push(create_node_with_path(
            &format!("level_{}", i),
            &current_path,
            NodeType::Directory,
            None,
        ));

        // Add a file at each level
        let file_path = format!("{}/file_{}.txt", current_path, i);
        nodes.push(create_node_with_path(
            &format!("file_{}.txt", i),
            &file_path,
            NodeType::File,
            Some(100),
        ));
    }

    nodes
}

/// Create a structure with potential circular references through symbolic links
fn create_circular_structure() -> Vec<NodeInfo> {
    vec![
        create_node_with_path("root", "root", NodeType::Directory, None),
        create_node_with_path("subdir", "root/subdir", NodeType::Directory, None),
        create_node_with_path(
            "link_to_root",
            "root/subdir/link_to_root",
            NodeType::Symlink,
            None,
        ),
        create_node_with_path("file.txt", "root/file.txt", NodeType::File, Some(100)),
        create_node_with_path(
            "nested_file.txt",
            "root/subdir/nested_file.txt",
            NodeType::File,
            Some(200),
        ),
    ]
}

/// Create a structure with many siblings that might cause exponential processing
fn create_wide_structure(width: usize) -> Vec<NodeInfo> {
    let mut nodes = Vec::new();

    // Create root
    nodes.push(create_node_with_path(
        "root",
        "root",
        NodeType::Directory,
        None,
    ));

    // Create many siblings
    for i in 0..width {
        nodes.push(create_node_with_path(
            &format!("file_{}.txt", i),
            &format!("root/file_{}.txt", i),
            NodeType::File,
            Some(100 + i as u64),
        ));

        // Create subdirectories with files
        let subdir_path = format!("root/subdir_{}", i);
        nodes.push(create_node_with_path(
            &format!("subdir_{}", i),
            &subdir_path,
            NodeType::Directory,
            None,
        ));

        let subfile_path = format!("{}/subfile.txt", subdir_path);
        nodes.push(create_node_with_path(
            "subfile.txt",
            &subfile_path,
            NodeType::File,
            Some(50),
        ));
    }

    nodes
}

fn create_test_metadata() -> DiffMetadata {
    DiffMetadata {
        generated_at: "2024-01-01T00:00:00Z".to_string(),
        snapshot_file: PathBuf::from("test.json"),
        snapshot_date: None,
        comparison_root: PathBuf::from("."),
        filters_applied: vec![],
        options: DiffOptions::default(),
    }
}

/// Test with timeout to detect infinite loops
fn test_with_timeout<F>(test_name: &str, test_fn: F, timeout_secs: u64) -> bool
where
    F: FnOnce() + Send + 'static,
{
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        test_fn();
        let _ = tx.send(());
    });

    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(_) => {
            println!(
                "{}: PASSED (completed within {} seconds)",
                test_name, timeout_secs
            );
            true
        }
        Err(_) => {
            println!(
                "{}: FAILED (timed out after {} seconds - likely infinite loop)",
                test_name, timeout_secs
            );
            false
        }
    }
}

#[test]
fn test_deep_nested_structure_performance() {
    let test_passed = test_with_timeout(
        "Deep nested structure",
        || {
            let engine = DiffEngine::new(DiffOptions::default());
            let deep_structure = create_deep_structure(100); // 100 levels deep
            let metadata = create_test_metadata();

            let start = Instant::now();
            let result = engine.compare(&deep_structure, &deep_structure, metadata);
            let duration = start.elapsed();

            assert!(result.is_ok(), "Diff should succeed for deep structure");
            println!("Deep structure diff took: {:?}", duration);

            // Should complete in reasonable time (less than 5 seconds)
            assert!(
                duration < Duration::from_secs(5),
                "Deep structure diff should complete quickly"
            );
        },
        10,
    );

    assert!(test_passed, "Deep nested structure test should not hang");
}

#[test]
fn test_wide_structure_performance() {
    let test_passed = test_with_timeout(
        "Wide structure",
        || {
            let engine = DiffEngine::new(DiffOptions::default());
            let wide_structure = create_wide_structure(1000); // 1000 siblings
            let metadata = create_test_metadata();

            let start = Instant::now();
            let result = engine.compare(&wide_structure, &wide_structure, metadata);
            let duration = start.elapsed();

            assert!(result.is_ok(), "Diff should succeed for wide structure");
            println!("Wide structure diff took: {:?}", duration);

            // Should complete in reasonable time
            assert!(
                duration < Duration::from_secs(10),
                "Wide structure diff should complete quickly"
            );
        },
        15,
    );

    assert!(test_passed, "Wide structure test should not hang");
}

#[test]
fn test_circular_symlink_handling() {
    let test_passed = test_with_timeout(
        "Circular symlinks",
        || {
            let engine = DiffEngine::new(DiffOptions::default());
            let circular_structure = create_circular_structure();
            let metadata = create_test_metadata();

            let start = Instant::now();
            let result = engine.compare(&circular_structure, &circular_structure, metadata);
            let duration = start.elapsed();

            assert!(result.is_ok(), "Diff should handle circular symlinks");
            println!("Circular structure diff took: {:?}", duration);

            // Should not hang on circular references
            assert!(
                duration < Duration::from_secs(3),
                "Circular structure should not cause infinite loops"
            );
        },
        10,
    );

    assert!(test_passed, "Circular symlink test should not hang");
}

#[test]
fn test_mixed_changes_with_complex_structure() {
    let test_passed = test_with_timeout(
        "Mixed changes complex",
        || {
            let engine = DiffEngine::new(DiffOptions::default());
            let previous = create_deep_structure(50);
            let mut current = create_deep_structure(50);

            // Add some changes to trigger complex diff logic
            current.push(create_node_with_path(
                "new_file.txt",
                "root/level_0/new_file.txt",
                NodeType::File,
                Some(500),
            ));

            // Remove a middle level directory (by not including it)
            current.retain(|node| !node.path.to_string_lossy().contains("level_25"));

            let metadata = create_test_metadata();

            let start = Instant::now();
            let result = engine.compare(&previous, &current, metadata);
            let duration = start.elapsed();

            assert!(result.is_ok(), "Complex diff should succeed");
            println!("Complex mixed changes diff took: {:?}", duration);

            let result = result.unwrap();
            assert!(
                result.summary.added > 0 || result.summary.removed > 0,
                "Should detect changes"
            );

            // Should complete reasonably quickly
            assert!(
                duration < Duration::from_secs(5),
                "Complex diff should not hang"
            );
        },
        15,
    );

    assert!(test_passed, "Mixed changes complex test should not hang");
}

#[test]
fn test_move_detection_performance() {
    let test_passed = test_with_timeout(
        "Move detection performance",
        || {
            let options = DiffOptions {
                detect_moves: true,
                move_threshold: 0.5,
                ..Default::default()
            };

            let engine = DiffEngine::new(options);

            let previous = create_wide_structure(100);
            let mut current = create_wide_structure(100);

            // Rename many files to trigger move detection
            for node in &mut current {
                if node.node_type == NodeType::File && node.name.starts_with("file_") {
                    let new_name = format!("moved_{}", node.name);
                    let new_path = node.path.parent().unwrap().join(&new_name);
                    node.name = new_name;
                    node.path = new_path;
                }
            }

            let metadata = create_test_metadata();

            let start = Instant::now();
            let result = engine.compare(&previous, &current, metadata);
            let duration = start.elapsed();

            assert!(result.is_ok(), "Move detection diff should succeed");
            println!("Move detection diff took: {:?}", duration);

            let result = result.unwrap();
            assert!(result.summary.moved > 0, "Should detect moves");

            // Move detection can be expensive but should still complete
            assert!(
                duration < Duration::from_secs(30),
                "Move detection should not hang"
            );
        },
        45,
    );

    assert!(
        test_passed,
        "Move detection performance test should not hang"
    );
}

#[test]
fn test_pathological_case_many_similar_files() {
    let test_passed = test_with_timeout(
        "Pathological similar files",
        || {
            let options = DiffOptions {
                detect_moves: true,
                move_threshold: 0.8,
                ..Default::default()
            };

            let engine = DiffEngine::new(options);

            // Create many files with very similar names (worst case for move detection)
            let mut previous = Vec::new();
            let mut current = Vec::new();

            for i in 0..100 {
                previous.push(create_node_with_path(
                    &format!("similar_file_{:03}.txt", i),
                    &format!("root/similar_file_{:03}.txt", i),
                    NodeType::File,
                    Some(100),
                ));

                // Create slightly different files in current
                current.push(create_node_with_path(
                    &format!("similar_file_{:03}_new.txt", i),
                    &format!("root/similar_file_{:03}_new.txt", i),
                    NodeType::File,
                    Some(100),
                ));
            }

            let metadata = create_test_metadata();

            let start = Instant::now();
            let result = engine.compare(&previous, &current, metadata);
            let duration = start.elapsed();

            assert!(result.is_ok(), "Pathological case should succeed");
            println!("Pathological case diff took: {:?}", duration);

            // This is an expensive case but should still complete
            assert!(
                duration < Duration::from_secs(60),
                "Even pathological cases should complete"
            );
        },
        90,
    );

    assert!(test_passed, "Pathological case test should not hang");
}

#[test]
fn test_extreme_stress_no_infinite_loop() {
    let test_passed = test_with_timeout(
        "Extreme stress test",
        || {
            let engine = DiffEngine::new(DiffOptions::default());

            // Create extremely large structure to stress test
            let mut previous = Vec::new();
            let mut current = Vec::new();

            // Add 5000 files in 100 directories (50 files per directory)
            for dir_i in 0..100 {
                let dir_path = format!("root/dir_{}", dir_i);
                previous.push(create_node_with_path(
                    &format!("dir_{}", dir_i),
                    &dir_path,
                    NodeType::Directory,
                    None,
                ));
                current.push(create_node_with_path(
                    &format!("dir_{}", dir_i),
                    &dir_path,
                    NodeType::Directory,
                    None,
                ));

                for file_i in 0..50 {
                    let file_path = format!("{}/file_{}.txt", dir_path, file_i);
                    previous.push(create_node_with_path(
                        &format!("file_{}.txt", file_i),
                        &file_path,
                        NodeType::File,
                        Some(100 + file_i as u64),
                    ));

                    // Make 50% of files different to trigger heavy diff processing
                    if file_i % 2 == 0 {
                        current.push(create_node_with_path(
                            &format!("file_{}.txt", file_i),
                            &file_path,
                            NodeType::File,
                            Some(100 + file_i as u64),
                        ));
                    } else {
                        // Add modified file
                        current.push(create_node_with_path(
                            &format!("modified_file_{}.txt", file_i),
                            &format!("{}/modified_file_{}.txt", dir_path, file_i),
                            NodeType::File,
                            Some(200 + file_i as u64),
                        ));
                    }
                }
            }

            let metadata = create_test_metadata();

            let start = Instant::now();
            let result = engine.compare(&previous, &current, metadata);
            let duration = start.elapsed();

            assert!(result.is_ok(), "Extreme stress test should succeed");
            println!("Extreme stress test (5000+ files) took: {:?}", duration);

            let result = result.unwrap();
            assert!(
                result.summary.added > 0 || result.summary.removed > 0,
                "Should detect many changes"
            );

            // Even with 5000+ files, should complete in reasonable time
            assert!(
                duration < Duration::from_secs(30),
                "Extreme stress test should not hang"
            );
        },
        60,
    );

    assert!(
        test_passed,
        "Extreme stress test should not hang or loop infinitely"
    );
}
