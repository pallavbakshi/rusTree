//! Tree manipulation utilities for modifying and transforming tree structures.
//!
//! This module provides functionality for manipulating tree structures after they have been
//! built. It includes operations for pruning, filtering, transforming, and reorganizing
//! tree nodes based on various criteria.

use crate::core::tree::builder::TempNode;
use crate::core::tree::node::NodeInfo;

/// A filter function type for tree manipulation operations.
///
/// This function takes a reference to a `NodeInfo` and returns `true` if the node
/// should be kept, `false` if it should be filtered out.
pub type NodeFilter = dyn Fn(&NodeInfo) -> bool;

/// A transformation function type for modifying nodes during tree manipulation.
///
/// This function takes a mutable reference to a `NodeInfo` and can modify it in place.
pub type NodeTransformer = dyn FnMut(&mut NodeInfo);

/// Provides utilities for manipulating tree structures.
///
/// This struct contains methods for common tree manipulation operations such as
/// pruning branches, filtering nodes, and applying transformations to tree content.
pub struct TreeManipulator;

impl TreeManipulator {
    /// Prunes a tree by removing nodes that don't satisfy the given filter predicate.
    ///
    /// This operation preserves the tree structure by keeping parent nodes if any of their
    /// descendants satisfy the filter, even if the parent itself doesn't. The tree is
    /// modified in place.
    ///
    /// # Arguments
    ///
    /// * `root` - A mutable reference to the root node of the tree to prune
    /// * `filter` - A predicate function that returns `true` for nodes to keep
    ///
    /// # Returns
    ///
    /// `true` if the node should be kept (either it passes the filter or has children
    /// after pruning), `false` if it should be removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rustree::core::tree::manipulator::TreeManipulator;
    /// # use rustree::core::tree::node::NodeType;
    /// // Keep only files (filter out directories)
    /// let filter = |node: &rustree::core::tree::node::NodeInfo| {
    ///     node.node_type == NodeType::File
    /// };
    /// // let should_keep = TreeManipulator::prune_tree(&mut root, &filter);
    /// ```
    pub fn prune_tree(root: &mut TempNode, filter: &NodeFilter) -> bool {
        // Recursively prune children first, keeping only those that should be retained
        root.children
            .retain_mut(|child| Self::prune_tree(child, filter));

        // Keep this node if it passes the filter OR if it has children after pruning
        filter(&root.node_info) || !root.children.is_empty()
    }

    /// Filters a flat list of nodes based on a predicate.
    ///
    /// Unlike `prune_tree`, this operates on a flat list and doesn't preserve
    /// hierarchical relationships. It's useful for simple filtering operations.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A mutable reference to the vector of nodes to filter
    /// * `filter` - A predicate function that returns `true` for nodes to keep
    pub fn filter_nodes(nodes: &mut Vec<NodeInfo>, filter: &NodeFilter) {
        nodes.retain(filter);
    }

    /// Applies a transformation function to all nodes in a tree.
    ///
    /// This performs an in-place transformation of the tree structure, applying
    /// the transformer function to each node in the tree.
    ///
    /// # Arguments
    ///
    /// * `root` - A mutable reference to the root node of the tree
    /// * `transformer` - A function that modifies nodes in place
    pub fn transform_tree(root: &mut TempNode, transformer: &mut NodeTransformer) {
        // Transform this node
        transformer(&mut root.node_info);

        // Recursively transform all children
        for child in &mut root.children {
            Self::transform_tree(child, transformer);
        }
    }

    /// Applies a transformation function to all nodes in a flat list.
    ///
    /// This performs an in-place transformation of all nodes in the provided vector.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A mutable reference to the vector of nodes to transform
    /// * `transformer` - A function that modifies nodes in place
    pub fn transform_nodes(nodes: &mut [NodeInfo], transformer: &mut NodeTransformer) {
        for node in nodes {
            transformer(node);
        }
    }

    /// Flattens a tree structure into a vector of nodes in depth-first order.
    ///
    /// This converts a hierarchical tree structure into a flat list while preserving
    /// the depth information in each node. Uses an iterative approach to prevent
    /// stack overflow on very deep trees.
    ///
    /// # Arguments
    ///
    /// * `root` - The root node of the tree to flatten
    ///
    /// # Returns
    ///
    /// A vector of `NodeInfo` objects in depth-first traversal order.
    pub fn flatten_tree(root: TempNode) -> Vec<NodeInfo> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        // Start with the root node
        stack.push(root);

        while let Some(current_node) = stack.pop() {
            // Add the current node to the result
            result.push(current_node.node_info);

            // Push children to the stack in reverse order to maintain
            // depth-first left-to-right traversal order
            for child in current_node.children.into_iter().rev() {
                stack.push(child);
            }
        }

        result
    }

    /// Limits the depth of a tree by pruning nodes beyond the specified depth.
    ///
    /// This operation preserves all nodes up to and including the specified maximum depth,
    /// but removes all deeper nodes. Uses a fully iterative approach to prevent stack overflow
    /// on very deep trees.
    ///
    /// # Arguments
    ///
    /// * `root` - A mutable reference to the root node of the tree
    /// * `max_depth` - The maximum depth to preserve (root is depth 0)
    pub fn limit_depth(root: &mut TempNode, max_depth: usize) {
        // Collect all node paths that need their children cleared
        // We'll do this in two phases to avoid borrowing conflicts:
        // 1. Collect paths to nodes at max_depth using iterative traversal
        // 2. Process each path to clear children
        let paths_to_clear = Self::collect_paths_at_max_depth_iterative(root, max_depth);

        // Now process each path and clear children
        for path in paths_to_clear {
            Self::clear_children_at_path(root, &path);
        }
    }

    /// Iteratively collects paths to nodes that are at the maximum depth.
    ///
    /// This function uses a queue-based approach to traverse the tree without recursion,
    /// identifying all nodes at the specified maximum depth that need their children cleared.
    fn collect_paths_at_max_depth_iterative(root: &TempNode, max_depth: usize) -> Vec<Vec<usize>> {
        let mut paths_to_clear = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        // Start with the root node (empty path, depth 0)
        queue.push_back((Vec::new(), 0));

        while let Some((current_path, current_depth)) = queue.pop_front() {
            if current_depth >= max_depth {
                // This node is at or beyond max depth, record its path for clearing
                paths_to_clear.push(current_path);
                continue;
            }

            // Navigate to the current node to check its children
            let mut current_node = root;
            for &index in &current_path {
                current_node = &current_node.children[index];
            }

            // Add all children to the queue for further processing
            for (child_index, _) in current_node.children.iter().enumerate() {
                let mut child_path = current_path.clone();
                child_path.push(child_index);
                queue.push_back((child_path, current_depth + 1));
            }
        }

        paths_to_clear
    }

    /// Clears the children of a node at the specified path.
    ///
    /// This function navigates to a node using the provided path and clears its children.
    fn clear_children_at_path(root: &mut TempNode, path: &[usize]) {
        let mut current = root;
        for &index in path {
            current = &mut current.children[index];
        }
        current.children.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tree::node::NodeType;
    use std::path::PathBuf;

    fn create_test_node(name: &str, node_type: NodeType, depth: usize) -> TempNode {
        TempNode {
            node_info: NodeInfo {
                name: name.to_string(),
                path: PathBuf::from(name),
                node_type,
                depth,
                size: None,
                permissions: None,
                line_count: None,
                word_count: None,
                mtime: None,
                change_time: None,
                create_time: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        }
    }

    #[test]
    fn test_filter_nodes() {
        let mut nodes = vec![
            NodeInfo {
                name: "file.txt".to_string(),
                path: PathBuf::from("file.txt"),
                node_type: NodeType::File,
                depth: 1,
                size: None,
                permissions: None,
                line_count: None,
                word_count: None,
                mtime: None,
                change_time: None,
                create_time: None,
                custom_function_output: None,
            },
            NodeInfo {
                name: "dir".to_string(),
                path: PathBuf::from("dir"),
                node_type: NodeType::Directory,
                depth: 1,
                size: None,
                permissions: None,
                line_count: None,
                word_count: None,
                mtime: None,
                change_time: None,
                create_time: None,
                custom_function_output: None,
            },
        ];

        // Filter to keep only files
        let filter = |node: &NodeInfo| node.node_type == NodeType::File;
        TreeManipulator::filter_nodes(&mut nodes, &filter);

        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "file.txt");
    }

    #[test]
    fn test_prune_tree_keep_files_only() {
        // Test pruning to keep only files
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let mut dir1 = create_test_node("dir1", NodeType::Directory, 1);
        let file1 = create_test_node("file1.txt", NodeType::File, 2);
        let file2 = create_test_node("file2.txt", NodeType::File, 1);

        dir1.children.push(file1);
        root.children.push(dir1);
        root.children.push(file2);

        // Filter to keep only files
        let filter = |node: &NodeInfo| node.node_type == NodeType::File;
        let should_keep_root = TreeManipulator::prune_tree(&mut root, &filter);

        // Root should be kept because it has descendants that match
        assert!(should_keep_root);
        assert_eq!(root.children.len(), 2); // dir1 and file2
        assert_eq!(root.children[0].node_info.name, "dir1");
        assert_eq!(root.children[1].node_info.name, "file2.txt");

        // dir1 should be kept because it has a file child
        assert_eq!(root.children[0].children.len(), 1);
        assert_eq!(root.children[0].children[0].node_info.name, "file1.txt");
    }

    #[test]
    fn test_prune_tree_keep_directories_only() {
        // Test pruning to keep only directories
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let dir1 = create_test_node("dir1", NodeType::Directory, 1);
        let file1 = create_test_node("file1.txt", NodeType::File, 1);

        root.children.push(dir1);
        root.children.push(file1);

        // Filter to keep only directories
        let filter = |node: &NodeInfo| node.node_type == NodeType::Directory;
        let should_keep_root = TreeManipulator::prune_tree(&mut root, &filter);

        // Root should be kept because it matches the filter
        assert!(should_keep_root);
        assert_eq!(root.children.len(), 1); // Only dir1
        assert_eq!(root.children[0].node_info.name, "dir1");
    }

    #[test]
    fn test_prune_tree_empty_after_pruning() {
        // Test case where nothing matches the filter
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let file1 = create_test_node("file1.txt", NodeType::File, 1);
        let file2 = create_test_node("file2.txt", NodeType::File, 1);

        root.children.push(file1);
        root.children.push(file2);

        // Filter that matches nothing
        let filter = |node: &NodeInfo| node.name.contains("nonexistent");
        let should_keep_root = TreeManipulator::prune_tree(&mut root, &filter);

        // Root should not be kept because nothing matches
        assert!(!should_keep_root);
        assert_eq!(root.children.len(), 0); // All children removed
    }

    #[test]
    fn test_prune_tree_complex_structure() {
        // Test pruning with a more complex tree structure
        let mut root = create_test_node("root", NodeType::Directory, 0);

        // Branch 1: dir1 -> file1.rs, file2.txt
        let mut dir1 = create_test_node("dir1", NodeType::Directory, 1);
        dir1.children
            .push(create_test_node("file1.rs", NodeType::File, 2));
        dir1.children
            .push(create_test_node("file2.txt", NodeType::File, 2));

        // Branch 2: dir2 -> dir3 -> file3.rs
        let mut dir2 = create_test_node("dir2", NodeType::Directory, 1);
        let mut dir3 = create_test_node("dir3", NodeType::Directory, 2);
        dir3.children
            .push(create_test_node("file3.rs", NodeType::File, 3));
        dir2.children.push(dir3);

        // Branch 3: file4.txt (direct child)
        root.children.push(dir1);
        root.children.push(dir2);
        root.children
            .push(create_test_node("file4.txt", NodeType::File, 1));

        // Filter to keep only .rs files
        let filter =
            |node: &NodeInfo| node.node_type == NodeType::File && node.name.ends_with(".rs");
        let should_keep_root = TreeManipulator::prune_tree(&mut root, &filter);

        // Root should be kept because it has descendants that match
        assert!(should_keep_root);
        assert_eq!(root.children.len(), 2); // dir1 and dir2 (file4.txt removed)

        // dir1 should have only file1.rs
        assert_eq!(root.children[0].children.len(), 1);
        assert_eq!(root.children[0].children[0].node_info.name, "file1.rs");

        // dir2 -> dir3 should have file3.rs
        assert_eq!(root.children[1].children.len(), 1);
        assert_eq!(root.children[1].children[0].children.len(), 1);
        assert_eq!(
            root.children[1].children[0].children[0].node_info.name,
            "file3.rs"
        );
    }

    #[test]
    fn test_prune_tree_preserves_parent_structure() {
        // Test that parent directories are preserved even if they don't match the filter
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let mut deep_dir1 = create_test_node("deep1", NodeType::Directory, 1);
        let mut deep_dir2 = create_test_node("deep2", NodeType::Directory, 2);
        let mut deep_dir3 = create_test_node("deep3", NodeType::Directory, 3);
        let target_file = create_test_node("target.txt", NodeType::File, 4);

        deep_dir3.children.push(target_file);
        deep_dir2.children.push(deep_dir3);
        deep_dir1.children.push(deep_dir2);
        root.children.push(deep_dir1);

        // Filter to keep only the target file
        let filter = |node: &NodeInfo| node.name == "target.txt";
        let should_keep_root = TreeManipulator::prune_tree(&mut root, &filter);

        // All parent directories should be preserved
        assert!(should_keep_root);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].node_info.name, "deep1");
        assert_eq!(root.children[0].children.len(), 1);
        assert_eq!(root.children[0].children[0].node_info.name, "deep2");
        assert_eq!(root.children[0].children[0].children.len(), 1);
        assert_eq!(
            root.children[0].children[0].children[0].node_info.name,
            "deep3"
        );
        assert_eq!(root.children[0].children[0].children[0].children.len(), 1);
        assert_eq!(
            root.children[0].children[0].children[0].children[0]
                .node_info
                .name,
            "target.txt"
        );
    }

    #[test]
    fn test_transform_nodes() {
        let mut nodes = vec![NodeInfo {
            name: "file.txt".to_string(),
            path: PathBuf::from("file.txt"),
            node_type: NodeType::File,
            depth: 1,
            size: None,
            permissions: None,
            line_count: None,
            word_count: None,
            mtime: None,
            change_time: None,
            create_time: None,
            custom_function_output: None,
        }];

        // Transform to uppercase names
        let mut transformer = |node: &mut NodeInfo| {
            node.name = node.name.to_uppercase();
        };
        TreeManipulator::transform_nodes(&mut nodes, &mut transformer);

        assert_eq!(nodes[0].name, "FILE.TXT");
    }

    #[test]
    fn test_transform_tree() {
        // Test transform_tree that applies a transformer to all nodes in a tree structure
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let mut dir1 = create_test_node("src", NodeType::Directory, 1);
        let file1 = create_test_node("main.rs", NodeType::File, 2);
        let file2 = create_test_node("lib.rs", NodeType::File, 1);

        dir1.children.push(file1);
        root.children.push(dir1);
        root.children.push(file2);

        // Transform to uppercase node names
        let mut transformer = |node: &mut NodeInfo| {
            node.name = node.name.to_uppercase();
        };
        TreeManipulator::transform_tree(&mut root, &mut transformer);

        // Verify all nodes in the tree were transformed
        assert_eq!(root.node_info.name, "ROOT");
        assert_eq!(root.children[0].node_info.name, "SRC");
        assert_eq!(root.children[0].children[0].node_info.name, "MAIN.RS");
        assert_eq!(root.children[1].node_info.name, "LIB.RS");
    }

    #[test]
    fn test_flatten_tree() {
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let child1 = create_test_node("child1", NodeType::File, 1);
        let child2 = create_test_node("child2", NodeType::Directory, 1);
        root.children.push(child1);
        root.children.push(child2);

        let flattened = TreeManipulator::flatten_tree(root);
        assert_eq!(flattened.len(), 3);
        assert_eq!(flattened[0].name, "root");
        assert_eq!(flattened[1].name, "child1");
        assert_eq!(flattened[2].name, "child2");
    }

    #[test]
    fn test_flatten_tree_deep_structure() {
        // Test that the iterative flatten_tree approach can handle very deep trees
        // without stack overflow and maintains correct depth-first traversal order
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let mut current = &mut root;

        // Create a chain of 1000 nested directories to test deep tree handling
        for i in 1..=1000 {
            let child = create_test_node(&format!("level_{}", i), NodeType::Directory, i);
            current.children.push(child);
            current = &mut current.children[0];
        }

        // Add a file at the deepest level
        let deep_file = create_test_node("deep_file", NodeType::File, 1001);
        current.children.push(deep_file);

        // Flatten the deep tree - this should not cause stack overflow
        let flattened = TreeManipulator::flatten_tree(root);

        // Verify that all nodes are present and in correct order
        assert_eq!(flattened.len(), 1002); // root + 1000 directories + 1 file
        assert_eq!(flattened[0].name, "root");
        assert_eq!(flattened[1].name, "level_1");
        assert_eq!(flattened[1000].name, "level_1000");
        assert_eq!(flattened[1001].name, "deep_file");

        // Verify depth information is preserved
        assert_eq!(flattened[0].depth, 0);
        assert_eq!(flattened[500].depth, 500);
        assert_eq!(flattened[1000].depth, 1000);
        assert_eq!(flattened[1001].depth, 1001);
    }

    #[test]
    fn test_flatten_tree_complex_structure() {
        // Test flattening a tree with multiple branches to ensure correct traversal order
        let mut root = create_test_node("root", NodeType::Directory, 0);

        // Create first branch
        let mut branch1 = create_test_node("branch1", NodeType::Directory, 1);
        branch1
            .children
            .push(create_test_node("file1", NodeType::File, 2));
        branch1
            .children
            .push(create_test_node("file2", NodeType::File, 2));

        // Create second branch
        let mut branch2 = create_test_node("branch2", NodeType::Directory, 1);
        branch2
            .children
            .push(create_test_node("file3", NodeType::File, 2));

        root.children.push(branch1);
        root.children.push(branch2);

        let flattened = TreeManipulator::flatten_tree(root);

        // Verify correct depth-first left-to-right traversal order
        assert_eq!(flattened.len(), 6);
        assert_eq!(flattened[0].name, "root");
        assert_eq!(flattened[1].name, "branch1");
        assert_eq!(flattened[2].name, "file1");
        assert_eq!(flattened[3].name, "file2");
        assert_eq!(flattened[4].name, "branch2");
        assert_eq!(flattened[5].name, "file3");
    }

    #[test]
    fn test_limit_depth() {
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let mut child = create_test_node("child", NodeType::Directory, 1);
        let grandchild = create_test_node("grandchild", NodeType::File, 2);
        child.children.push(grandchild);
        root.children.push(child);

        TreeManipulator::limit_depth(&mut root, 1);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].children.len(), 0); // Grandchild should be removed
    }

    #[test]
    fn test_limit_depth_deep_tree() {
        // Create a very deep tree to test that the iterative approach
        // doesn't cause stack overflow on deep structures
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let mut current = &mut root;

        // Create a chain of 1000 nested directories
        for i in 1..=1000 {
            let mut child = create_test_node(&format!("level_{}", i), NodeType::Directory, i);
            // Add a file at the deepest level to ensure it gets pruned
            if i == 1000 {
                let file = create_test_node("deep_file", NodeType::File, i + 1);
                child.children.push(file);
            }
            current.children.push(child);
            current = &mut current.children[0];
        }

        // Limit depth to 10, which should prune most of the deep structure
        TreeManipulator::limit_depth(&mut root, 10);

        // Verify that the tree was properly limited
        // We should be able to traverse to depth 10 but not beyond
        let mut current = &root;
        for i in 0..10 {
            assert!(
                !current.children.is_empty(),
                "Should have children at depth {}",
                i
            );
            current = &current.children[0];
        }

        // At depth 10, there should be no children (they were pruned)
        assert!(
            current.children.is_empty(),
            "Should have no children at max depth"
        );
    }
}
