//! Tests for the sorter module functionality.

#[cfg(test)]
mod tests {
    use crate::config::sorting::SortKey;
    use crate::core::sorter::strategies::sort_nodes;
    use crate::core::tree::node::{NodeInfo, NodeType};
    use std::path::PathBuf;
    use std::time::SystemTime;

    // Helper to create NodeInfo for testing.
    // Path is set to name for simplicity in these unit tests, as sorter primarily looks at NodeInfo fields.
    // Depth is crucial for build_tree.
    fn create_test_node_info(
        name_str: &str,
        depth: usize,
        node_type: NodeType,
        size: Option<u64>,
        line_count: Option<usize>,
    ) -> NodeInfo {
        NodeInfo {
            path: PathBuf::from(format!("{}/{}", "/".repeat(depth), name_str)), // Simplified unique path based on depth and name
            name: name_str.to_string(),
            node_type,
            depth,
            size,
            permissions: None,
            mtime: Some(SystemTime::now()), // Consistent MTime for tests not focusing on it
            change_time: None,
            create_time: None,
            line_count,
            word_count: None,
            custom_function_output: None,
        }
    }

    // Helper to get names from Vec<NodeInfo>
    fn get_names(nodes: &[NodeInfo]) -> Vec<String> {
        nodes.iter().map(|n| n.name.clone()).collect()
    }

    #[test]
    fn test_sort_by_name_flat() {
        let mut nodes_info = vec![
            create_test_node_info("charlie.txt", 1, NodeType::File, Some(100), Some(10)),
            create_test_node_info("alpha.txt", 1, NodeType::File, Some(200), Some(5)),
            create_test_node_info("beta.txt", 1, NodeType::File, Some(50), Some(20)),
        ];
        let _ = sort_nodes(&mut nodes_info, &SortKey::Name, false);
        assert_eq!(
            get_names(&nodes_info),
            vec!["alpha.txt", "beta.txt", "charlie.txt"]
        );
    }

    #[test]
    fn test_sort_by_name_reverse_flat() {
        let mut nodes_info = vec![
            create_test_node_info("charlie.txt", 1, NodeType::File, Some(100), Some(10)),
            create_test_node_info("alpha.txt", 1, NodeType::File, Some(200), Some(5)),
            create_test_node_info("beta.txt", 1, NodeType::File, Some(50), Some(20)),
        ];
        let _ = sort_nodes(&mut nodes_info, &SortKey::Name, true);
        assert_eq!(
            get_names(&nodes_info),
            vec!["charlie.txt", "beta.txt", "alpha.txt"]
        );
    }

    #[test]
    fn test_sort_by_size_files_and_dirs_flat() {
        // Files should come before directories. Files sorted by size (descending: largest first). Dirs by name.
        let mut nodes_info = vec![
            create_test_node_info("big_file.txt", 1, NodeType::File, Some(1000), None),
            create_test_node_info("dir_alpha", 1, NodeType::Directory, None, None),
            create_test_node_info("small_file.txt", 1, NodeType::File, Some(10), None),
            create_test_node_info("dir_beta", 1, NodeType::Directory, None, None),
            create_test_node_info("medium_file.txt", 1, NodeType::File, Some(100), None),
        ];
        let _ = sort_nodes(&mut nodes_info, &SortKey::Size, false); // Size sort (descending for files)
        assert_eq!(
            get_names(&nodes_info),
            vec![
                "big_file.txt",    // File, 1000B (largest first)
                "medium_file.txt", // File, 100B
                "small_file.txt",  // File, 10B (smallest last)
                "dir_alpha",       // Dir, by name
                "dir_beta",        // Dir, by name
            ]
        );
    }

    #[test]
    fn test_sort_by_size_reverse_files_and_dirs_flat() {
        // Reverse=true flips the whole order: dirs first, then files (both groups reversed)
        let mut nodes_info = vec![
            create_test_node_info("big_file.txt", 1, NodeType::File, Some(1000), None),
            create_test_node_info("dir_alpha", 1, NodeType::Directory, None, None),
            create_test_node_info("small_file.txt", 1, NodeType::File, Some(10), None),
            create_test_node_info("dir_beta", 1, NodeType::Directory, None, None),
        ];
        let _ = sort_nodes(&mut nodes_info, &SortKey::Size, true); // Descending (reverse=true)
        assert_eq!(
            get_names(&nodes_info),
            vec![
                "dir_beta",       // Dir, by name (reversed from alpha->beta to beta->alpha)
                "dir_alpha",      // Dir, by name (reversed)
                "small_file.txt", // File, 10B (reversed from big->small to small->big)
                "big_file.txt",   // File, 1000B (smallest first when reversed)
            ]
        );
    }

    #[test]
    fn test_tree_structure_preserved_and_sorted() {
        // Simulates a small directory tree:
        // root/
        //   ├── dir_b/
        //   │   └── file_c.txt (5B)
        //   └── dir_a/
        //       └── file_d.txt (10B)
        //   └── file_alpha.txt (100B)
        //   └── file_beta.txt (20B)
        // Expected sort by name:
        // root/
        //   ├── dir_a/
        //   │   └── file_d.txt
        //   ├── dir_b/
        //   │   └── file_c.txt
        //   ├── file_alpha.txt
        //   └── file_beta.txt
        let mut nodes_info = vec![
            // These must be in DFS order for build_tree to work correctly
            create_test_node_info("dir_b", 1, NodeType::Directory, None, None),
            create_test_node_info("file_c.txt", 2, NodeType::File, Some(5), None), // child of dir_b
            create_test_node_info("dir_a", 1, NodeType::Directory, None, None),
            create_test_node_info("file_d.txt", 2, NodeType::File, Some(10), None), // child of dir_a
            create_test_node_info("file_alpha.txt", 1, NodeType::File, Some(100), None),
            create_test_node_info("file_beta.txt", 1, NodeType::File, Some(20), None),
        ];
        // Sort by name, ascending
        let _ = sort_nodes(&mut nodes_info, &SortKey::Name, false);
        let expected_names_paths = vec![
            ("dir_a", "/dir_a"),
            ("file_d.txt", "//file_d.txt"), // Path depth 2
            ("dir_b", "/dir_b"),
            ("file_c.txt", "//file_c.txt"), // Path depth 2
            ("file_alpha.txt", "/file_alpha.txt"),
            ("file_beta.txt", "/file_beta.txt"),
        ];
        let actual_names_paths: Vec<(String, String)> = nodes_info
            .iter()
            .map(|n| (n.name.clone(), n.path.to_string_lossy().into_owned()))
            .collect();
        assert_eq!(
            actual_names_paths.len(),
            expected_names_paths.len(),
            "Different number of nodes after sort"
        );

        for (idx, (name, path_str)) in expected_names_paths.iter().enumerate() {
            assert_eq!(
                nodes_info[idx].name, *name,
                "Name mismatch at index {}",
                idx
            );
            // Path check is tricky due to simplified paths in test helper.
            // Check depth instead to verify structure.
            let expected_depth = if path_str.starts_with("//") { 2 } else { 1 };
            assert_eq!(
                nodes_info[idx].depth, expected_depth,
                "Depth mismatch for {} at index {}",
                name, idx
            );
        }

        // More direct check of names in order
        assert_eq!(
            get_names(&nodes_info),
            vec![
                "dir_a",
                "file_d.txt",
                "dir_b",
                "file_c.txt",
                "file_alpha.txt",
                "file_beta.txt"
            ]
        );

        // Now sort the same structure by Size
        // Expected sort by size (Files before Dirs; Files by size desc; Dirs by name asc):
        // With the new descending size sort, larger files come first:
        // root/
        //   ├── file_alpha.txt (100B) (largest file first)
        //   ├── file_beta.txt (20B)
        //   ├── dir_a/
        //   │   └── file_d.txt (10B) (children sorted by size too)
        //   ├── dir_b/
        //   │   └── file_c.txt (5B)
        // Expected final DFS order:
        // file_alpha.txt (100B)
        // file_beta.txt (20B)
        // dir_a
        // file_d.txt (10B)
        // dir_b
        // file_c.txt (5B)

        let mut nodes_info_size_sort = vec![
            create_test_node_info("dir_b", 1, NodeType::Directory, None, None),
            create_test_node_info("file_c.txt", 2, NodeType::File, Some(5), None),
            create_test_node_info("dir_a", 1, NodeType::Directory, None, None),
            create_test_node_info("file_d.txt", 2, NodeType::File, Some(10), None),
            create_test_node_info("file_alpha.txt", 1, NodeType::File, Some(100), None),
            create_test_node_info("file_beta.txt", 1, NodeType::File, Some(20), None),
        ];
        let _ = sort_nodes(&mut nodes_info_size_sort, &SortKey::Size, false);
        assert_eq!(
            get_names(&nodes_info_size_sort),
            vec![
                "file_alpha.txt", // 100B (largest first)
                "file_beta.txt",  // 20B
                "dir_a",          // Dir
                "file_d.txt",     // Child of dir_a, 10B
                "dir_b",          // Dir
                "file_c.txt",     // Child of dir_b, 5B
            ]
        );
    }
}
