// src/core/sorter.rs
use crate::core::node::{NodeInfo, NodeType};
use std::cmp::Ordering;
use crate::config::sorting::SortKey;
// HashMap is not needed for the stack-based build_tree
// use std::collections::HashMap;
// Path and PathBuf not strictly needed in this file after NodeInfo has PathBuf
// use std::path::{Path, PathBuf};

#[derive(Debug)]
struct TempNode {
    node_info: NodeInfo,
    children: Vec<Box<TempNode>>,
}

impl TempNode {
    fn sort_children_recursive(&mut self, key: &SortKey, reverse: bool) {
        if !self.children.is_empty() {
            self.children.sort_by(|a, b| {
                compare_siblings(a.as_ref(), b.as_ref(), key, reverse)
            });
            for child_box in &mut self.children {
                child_box.as_mut().sort_children_recursive(key, reverse);
            }
        }
    }
}

fn build_tree(nodes_info: Vec<NodeInfo>) -> Vec<Box<TempNode>> {
    if nodes_info.is_empty() {
        return Vec::new();
    }

    let mut roots: Vec<Box<TempNode>> = Vec::new();
    // parent_stack stores raw pointers to TempNodes already in the tree structure.
    // These TempNodes are owned by `roots` or `parent.children` via `Box`.
    // Using raw pointers requires `unsafe` but resolves complex borrow checker issues
    // in this specific tree-building scenario.
    let mut parent_stack: Vec<*mut TempNode> = Vec::new();

    for node_info in nodes_info { // Consumes node_info from the input Vec
        let current_depth = node_info.depth;
        // Create the Box, but get the raw pointer to the data *before* the Box is moved.
        let mut new_temp_node_boxed = Box::new(TempNode { node_info, children: Vec::new() });
        let new_node_ptr: *mut TempNode = new_temp_node_boxed.as_mut();

        // Pop parents from stack until stack top is the actual parent of new_temp_node_boxed
        loop {
            match parent_stack.last() {
                Some(&last_parent_ptr) => {
                    // Safety: last_parent_ptr was obtained from a Box'd TempNode that's part of the tree
                    // and whose lifetime covers this usage. The Box ensures the TempNode data is stable.
                    let last_parent_in_stack = unsafe { &*last_parent_ptr };
                    if current_depth <= last_parent_in_stack.node_info.depth {
                        parent_stack.pop();
                    } else {
                        break; // Found correct parent level for new_temp_node_boxed
                    }
                }
                None => break, // Stack is empty, new node is a root
            }
        }

        if parent_stack.is_empty() {
            // This node is a root
            roots.push(new_temp_node_boxed); // new_temp_node_boxed is moved here
            parent_stack.push(new_node_ptr); // Store the raw pointer to the node within the Box in roots
        } else {
            // This node is a child of the current top of parent_stack
            let current_parent_ptr = *parent_stack.last().unwrap();
            // Safety: current_parent_ptr is valid as per above.
            // We need a mutable reference to add a child.
            let current_parent_on_stack = unsafe { &mut *current_parent_ptr };
            current_parent_on_stack.children.push(new_temp_node_boxed); // new_temp_node_boxed is moved here
            parent_stack.push(new_node_ptr); // Store the raw pointer to the node within the Box in children
        }
    }
    roots
}

fn flatten_tree_to_dfs_consuming(roots: Vec<Box<TempNode>>, result: &mut Vec<NodeInfo>) {
    for root_node_boxed in roots { // root_node_boxed is Box<TempNode>
        // Move TempNode out of Box.
        let temp_node = *root_node_boxed;
        result.push(temp_node.node_info); // NodeInfo is Clone
        flatten_tree_to_dfs_consuming(temp_node.children, result); // temp_node.children is Vec<Box<TempNode>>
    }
}

fn compare_siblings(a: &TempNode, b: &TempNode, key: &SortKey, reverse: bool) -> Ordering {
    let ord = match key { // Removed `mut` as it's not needed
        SortKey::Name => a.node_info.name.cmp(&b.node_info.name),
        SortKey::Size => {
            let type_a = &a.node_info.node_type;
            let type_b = &b.node_info.node_type;

            // Primary sort: type (Files/Symlinks before Directories)
            let type_ord = match (type_a, type_b) {
                (NodeType::File | NodeType::Symlink, NodeType::Directory) => Ordering::Less,
                (NodeType::Directory, NodeType::File | NodeType::Symlink) => Ordering::Greater,
                _ => Ordering::Equal, // Same types, proceed to size/name comparison
            };

            if type_ord != Ordering::Equal {
                type_ord
            } else {
                // Types are the same (both File/Symlink or both Directory)
                match type_a {
                    NodeType::File | NodeType::Symlink => {
                        // Compare by size (Option<u64>), then by name.
                        // None (no size reported) is treated as smaller than Some(size).
                        // If size is critical and None should be largest or error, adjust here.
                        // For typical use, None size means it's effectively 0 or smallest.
                        a.node_info.size.cmp(&b.node_info.size)
                            .then_with(|| a.node_info.name.cmp(&b.node_info.name))
                    }
                    NodeType::Directory => {
                        // Compare directories by name
                        a.node_info.name.cmp(&b.node_info.name)
                    }
                }
            }
        }
        SortKey::MTime => {
            match (a.node_info.mtime, b.node_info.mtime) {
                (Some(ta), Some(tb)) => ta.cmp(&tb),
                (Some(_), None) => Ordering::Less,    // Valid MTime before None
                (None, Some(_)) => Ordering::Greater, // None after valid MTime
                (None, None) => Ordering::Equal,      // Both None, fall through to name
            }.then_with(|| a.node_info.name.cmp(&b.node_info.name))
        }
        SortKey::Words => {
             match (a.node_info.word_count, b.node_info.word_count) {
                (Some(wa), Some(wb)) => wa.cmp(&wb),
                (Some(_), None) => Ordering::Less,    // Files with count before those without (e.g. dirs)
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,      // Both None (e.g. two dirs), fall through to name
            }.then_with(|| a.node_info.name.cmp(&b.node_info.name))
        }
        SortKey::Lines => {
            match (a.node_info.line_count, b.node_info.line_count) {
                (Some(la), Some(lb)) => la.cmp(&lb),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }.then_with(|| a.node_info.name.cmp(&b.node_info.name))
        }
        SortKey::Custom => {
            match (&a.node_info.custom_function_output, &b.node_info.custom_function_output) {
                (Some(Ok(val_a)), Some(Ok(val_b))) => val_a.cmp(val_b),
                (Some(Ok(_)), _) => Ordering::Less, // Successful custom output first
                (_, Some(Ok(_))) => Ordering::Greater,
                // Error cases:
                (Some(Err(_)), Some(Err(_))) => Ordering::Equal, // Both errors, use name
                (Some(Err(_)), None) => Ordering::Less,          // Error before None (e.g. dir for which func not run)
                (None, Some(Err(_))) => Ordering::Greater,
                (None, None) => Ordering::Equal,                 // Both None, use name
            }.then_with(|| a.node_info.name.cmp(&b.node_info.name))
        }
    };

    if reverse {
        ord.reverse()
    } else {
        ord
    }
}


/// Sorts a vector of `NodeInfo` entries in place.
///
/// This function builds a temporary tree structure from the `nodes`,
/// sorts the children at each level of the tree according to the `key` and `reverse` flag,
/// and then flattens the tree back into the `nodes` vector in the correct DFS order.
///
/// # Arguments
///
/// * `nodes` - A mutable reference to a vector of `NodeInfo` to be sorted.
///             The initial vector is assumed to be in DFS order (e.g., from `walk_directory`).
/// * `key` - The [`SortKey`] specifying the attribute to sort siblings by.
/// * `reverse` - A boolean indicating whether to reverse the sort order for siblings.
pub fn sort_nodes(nodes: &mut Vec<NodeInfo>, key: &SortKey, reverse: bool) {
    if nodes.is_empty() {
        return;
    }

    // 1. Build the tree. `nodes` is moved and consumed.
    // std::mem::take is used to move out of &mut Vec<NodeInfo>
    let mut roots = build_tree(std::mem::take(nodes));

    // 2. Sort the root nodes themselves (they are siblings at the top level)
    roots.sort_by(|a, b| compare_siblings(a.as_ref(), b.as_ref(), key, reverse));

    // 3. Recursively sort children of each root node
    for root_node_box in &mut roots {
        root_node_box.as_mut().sort_children_recursive(key, reverse);
    }

    // 4. Flatten the tree back into a new list
    let mut sorted_nodes_list = Vec::with_capacity(nodes.capacity()); // Pre-allocate
    flatten_tree_to_dfs_consuming(roots, &mut sorted_nodes_list);

    // 5. Replace original nodes content with the new sorted list
    *nodes = sorted_nodes_list;
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::node::NodeType;
    use std::path::PathBuf;
    use std::time::SystemTime;

    // Helper to create NodeInfo for testing.
    // Path is set to name for simplicity in these unit tests, as sorter primarily looks at NodeInfo fields.
    // Depth is crucial for build_tree.
    fn create_test_node_info(name_str: &str, depth: usize, node_type: NodeType, size: Option<u64>, line_count: Option<usize>) -> NodeInfo {
        NodeInfo {
            path: PathBuf::from(format!("{}/{}", "/".repeat(depth), name_str)), // Simplified unique path based on depth and name
            name: name_str.to_string(),
            node_type,
            depth,
            size,
            permissions: None,
            mtime: Some(SystemTime::now()), // Consistent MTime for tests not focusing on it
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
        sort_nodes(&mut nodes_info, &SortKey::Name, false);
        assert_eq!(get_names(&nodes_info), vec!["alpha.txt", "beta.txt", "charlie.txt"]);
    }

    #[test]
    fn test_sort_by_name_reverse_flat() {
        let mut nodes_info = vec![
            create_test_node_info("charlie.txt", 1, NodeType::File, Some(100), Some(10)),
            create_test_node_info("alpha.txt", 1, NodeType::File, Some(200), Some(5)),
            create_test_node_info("beta.txt", 1, NodeType::File, Some(50), Some(20)),
        ];
        sort_nodes(&mut nodes_info, &SortKey::Name, true);
        assert_eq!(get_names(&nodes_info), vec!["charlie.txt", "beta.txt", "alpha.txt"]);
    }

    #[test]
    fn test_sort_by_size_files_and_dirs_flat() {
        // Files should come before directories. Files sorted by size. Dirs by name.
        let mut nodes_info = vec![
            create_test_node_info("big_file.txt", 1, NodeType::File, Some(1000), None),
            create_test_node_info("dir_alpha", 1, NodeType::Directory, None, None),
            create_test_node_info("small_file.txt", 1, NodeType::File, Some(10), None),
            create_test_node_info("dir_beta", 1, NodeType::Directory, None, None),
            create_test_node_info("medium_file.txt", 1, NodeType::File, Some(100), None),
        ];
        sort_nodes(&mut nodes_info, &SortKey::Size, false); // Ascending size for files
        assert_eq!(get_names(&nodes_info), vec![
            "small_file.txt",  // File, 10B
            "medium_file.txt", // File, 100B
            "big_file.txt",    // File, 1000B
            "dir_alpha",       // Dir, by name
            "dir_beta",        // Dir, by name
        ]);
    }

    #[test]
    fn test_sort_by_size_reverse_files_and_dirs_flat() {
        // Dirs should come before files (grouping reversed). Dirs by name (reversed). Files by size (reversed).
        let mut nodes_info = vec![
            create_test_node_info("big_file.txt", 1, NodeType::File, Some(1000), None),
            create_test_node_info("dir_alpha", 1, NodeType::Directory, None, None),
            create_test_node_info("small_file.txt", 1, NodeType::File, Some(10), None),
            create_test_node_info("dir_beta", 1, NodeType::Directory, None, None),
        ];
        sort_nodes(&mut nodes_info, &SortKey::Size, true); // Descending
        assert_eq!(get_names(&nodes_info), vec![
            "dir_beta",        // Dir, by name reversed
            "dir_alpha",       // Dir, by name reversed
            "big_file.txt",    // File, 1000B (largest)
            "small_file.txt",  // File, 10B (smallest)
        ]);
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
            create_test_node_info("file_d.txt", 2, NodeType::File, Some(10), None),// child of dir_a
            create_test_node_info("file_alpha.txt", 1, NodeType::File, Some(100), None),
            create_test_node_info("file_beta.txt", 1, NodeType::File, Some(20), None),
        ];
        // Sort by name, ascending
        sort_nodes(&mut nodes_info, &SortKey::Name, false);
        let expected_names_paths = vec![
            ("dir_a", "/dir_a"),
            ("file_d.txt", "//file_d.txt"), // Path depth 2
            ("dir_b", "/dir_b"),
            ("file_c.txt", "//file_c.txt"), // Path depth 2
            ("file_alpha.txt", "/file_alpha.txt"),
            ("file_beta.txt", "/file_beta.txt"),
        ];
        let actual_names_paths: Vec<(String, String)> = nodes_info.iter().map(|n| (n.name.clone(), n.path.to_string_lossy().into_owned())).collect();
        assert_eq!(actual_names_paths.len(), expected_names_paths.len(), "Different number of nodes after sort");

        for (idx, (name, path_str)) in expected_names_paths.iter().enumerate() {
            assert_eq!(nodes_info[idx].name, *name, "Name mismatch at index {}", idx);
            // Path check is tricky due to simplified paths in test helper.
            // Check depth instead to verify structure.
            let expected_depth = if path_str.starts_with("//") { 2 } else { 1 };
            assert_eq!(nodes_info[idx].depth, expected_depth, "Depth mismatch for {} at index {}", name, idx);
        }
        
        // More direct check of names in order
         assert_eq!(get_names(&nodes_info), vec![
            "dir_a", "file_d.txt", "dir_b", "file_c.txt", "file_alpha.txt", "file_beta.txt"
        ]);


        // Now sort the same structure by Size
        // Expected sort by size (Files before Dirs; Files by size asc; Dirs by name asc):
        // root/
        //   ├── file_beta.txt (20B)
        //   ├── file_alpha.txt (100B)
        //   ├── dir_a/
        //   │   └── file_d.txt (10B) (children sorted by size too)
        //   ├── dir_b/
        //   │   └── file_c.txt (5B)
        // Oh, wait. The files file_beta and file_alpha are siblings of dir_a and dir_b.
        // So, they should be sorted amongst themselves first by size.
        // Then dir_a and dir_b sorted by name.
        // Children of dir_a and dir_b are sorted by size.
        // Expected:
        // file_beta.txt (20B)
        // file_alpha.txt (100B)
        // dir_a/
        //   file_d.txt (10B)
        // dir_b/
        //   file_c.txt (5B)
        // This is if files are grouped before directories.
        // The example output from `tree` command shows files first, then directories.
        // My `compare_siblings` for Size does this.
        // So, for the root level siblings: file_beta (20), file_alpha (100), dir_a, dir_b
        // Sorted by size (files first): file_beta, file_alpha, then dir_a, dir_b (sorted by name)
        // Children of dir_a: file_d (10B)
        // Children of dir_b: file_c (5B)
        // Final order:
        // file_beta.txt
        // file_alpha.txt
        // dir_a
        // file_d.txt
        // dir_b
        // file_c.txt

        let mut nodes_info_size_sort = vec![
            create_test_node_info("dir_b", 1, NodeType::Directory, None, None),
            create_test_node_info("file_c.txt", 2, NodeType::File, Some(5), None),
            create_test_node_info("dir_a", 1, NodeType::Directory, None, None),
            create_test_node_info("file_d.txt", 2, NodeType::File, Some(10), None),
            create_test_node_info("file_alpha.txt", 1, NodeType::File, Some(100), None),
            create_test_node_info("file_beta.txt", 1, NodeType::File, Some(20), None),
        ];
        sort_nodes(&mut nodes_info_size_sort, &SortKey::Size, false);
        assert_eq!(get_names(&nodes_info_size_sort), vec![
            "file_beta.txt",    // 20B
            "file_alpha.txt",   // 100B
            "dir_a",            // Dir
            "file_d.txt",       // Child of dir_a, 10B
            "dir_b",            // Dir
            "file_c.txt",       // Child of dir_b, 5B
        ]);
    }
}