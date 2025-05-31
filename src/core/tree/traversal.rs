//! Tree traversal utilities and iterators for different traversal strategies.
//!
//! This module provides various algorithms and iterator implementations for traversing
//! tree structures in different orders. It supports common traversal patterns like
//! depth-first, breadth-first, and various orderings within those strategies.
//!
//! # Directory Hooks Example
//!
//! The `TreeVisitor` trait provides directory-specific hooks that are called during traversal:
//!
//! ```rust
//! use rustree::core::tree::traversal::{TreeTraversal, TreeVisitor};
//! use rustree::core::tree::node::{NodeInfo, NodeType};
//!
//! struct DirectoryWatcher {
//!     current_depth: usize,
//! }
//!
//! impl TreeVisitor for DirectoryWatcher {
//!     fn visit(&mut self, node: &NodeInfo, depth: usize) -> bool {
//!         println!("{}File: {}", "  ".repeat(depth), node.name);
//!         true
//!     }
//!
//!     fn enter_directory(&mut self, node: &NodeInfo, depth: usize) -> bool {
//!         println!("{}Entering directory: {}", "  ".repeat(depth), node.name);
//!         true // Continue into directory
//!     }
//!
//!     fn exit_directory(&mut self, node: &NodeInfo, depth: usize) {
//!         println!("{}Exiting directory: {}", "  ".repeat(depth), node.name);
//!     }
//! }
//! ```

use crate::core::tree::builder::TempNode;
use crate::core::tree::node::{NodeInfo, NodeType};
use std::collections::VecDeque;

/// Represents different traversal orders for tree iteration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalOrder {
    /// Visit nodes in depth-first pre-order (parent before children).
    DepthFirstPreOrder,
    /// Visit nodes in depth-first post-order (children before parent).
    DepthFirstPostOrder,
    /// Visit nodes in breadth-first order (level by level).
    BreadthFirst,
}

/// A visitor trait for implementing custom tree traversal operations.
///
/// This trait allows for flexible tree processing by implementing the visitor pattern.
/// The visitor's methods are called during traversal to process nodes and control
/// traversal behavior.
///
/// # Directory Handling
///
/// For directory nodes, the traversal calls `enter_directory()` instead of `visit()`.
/// After processing a directory's children, `exit_directory()` is called.
/// For non-directory nodes (files, symlinks, etc.), only `visit()` is called.
pub trait TreeVisitor {
    /// Called when visiting a non-directory node during traversal.
    ///
    /// This method is called for files, symlinks, and other non-directory nodes.
    /// For directory nodes, use `enter_directory()` and `exit_directory()` instead.
    ///
    /// # Arguments
    ///
    /// * `node` - A reference to the node being visited
    /// * `depth` - The depth of the node in the tree (root is depth 0)
    ///
    /// # Returns
    ///
    /// `true` to continue traversal, `false` to skip further processing.
    /// Note: Non-directory nodes typically don't have children, so this mainly
    /// affects whether the traversal continues to sibling nodes.
    fn visit(&mut self, node: &NodeInfo, depth: usize) -> bool;

    /// Called when entering a directory node (before visiting its children).
    ///
    /// This method is called for directory nodes instead of `visit()`.
    /// It allows the visitor to decide whether to traverse the directory's contents.
    ///
    /// # Arguments
    ///
    /// * `node` - A reference to the directory node being entered
    /// * `depth` - The depth of the directory in the tree (root is depth 0)
    ///
    /// # Returns
    ///
    /// `true` to continue traversal into the directory's children,
    /// `false` to skip this directory's subtree.
    fn enter_directory(&mut self, node: &NodeInfo, depth: usize) -> bool {
        self.visit(node, depth)
    }

    /// Called when exiting a directory node (after visiting its children).
    ///
    /// This method is called after all of a directory's children have been processed.
    /// It's useful for cleanup operations or aggregating information from child nodes.
    ///
    /// # Arguments
    ///
    /// * `node` - A reference to the directory node being exited
    /// * `depth` - The depth of the directory in the tree (root is depth 0)
    fn exit_directory(&mut self, _node: &NodeInfo, _depth: usize) {}
}

/// Provides various tree traversal algorithms and utilities.
pub struct TreeTraversal;

impl TreeTraversal {
    /// Performs a depth-first pre-order traversal of the tree.
    ///
    /// In pre-order traversal, each node is visited before its children.
    /// This is useful for operations that need to process parent nodes before
    /// their descendants.
    ///
    /// # Arguments
    ///
    /// * `root` - The root node to start traversal from
    /// * `visitor` - A mutable visitor that will be called for each node
    pub fn depth_first_pre_order<V: TreeVisitor>(root: &TempNode, visitor: &mut V) {
        Self::depth_first_pre_order_recursive(root, visitor, 0);
    }

    /// Recursive helper for depth-first pre-order traversal.
    fn depth_first_pre_order_recursive<V: TreeVisitor>(
        node: &TempNode,
        visitor: &mut V,
        depth: usize,
    ) {
        // Determine if this is a directory
        let is_directory = matches!(node.node_info.node_type, NodeType::Directory);
        
        let should_continue = if is_directory {
            // For directories, call enter_directory hook
            visitor.enter_directory(&node.node_info, depth)
        } else {
            // For files and other nodes, call regular visit
            visitor.visit(&node.node_info, depth)
        };

        // If visitor wants to continue, traverse children
        if should_continue {
            for child in &node.children {
                Self::depth_first_pre_order_recursive(child, visitor, depth + 1);
            }
        }

        // For directories, call exit_directory hook after processing children
        if is_directory {
            visitor.exit_directory(&node.node_info, depth);
        }
    }

    /// Performs a depth-first post-order traversal of the tree.
    ///
    /// In post-order traversal, each node is visited after its children.
    /// This is useful for operations that need to process descendants before
    /// their ancestors (e.g., calculating directory sizes).
    ///
    /// # Arguments
    ///
    /// * `root` - The root node to start traversal from
    /// * `visitor` - A mutable visitor that will be called for each node
    pub fn depth_first_post_order<V: TreeVisitor>(root: &TempNode, visitor: &mut V) {
        Self::depth_first_post_order_recursive(root, visitor, 0);
    }

    /// Recursive helper for depth-first post-order traversal.
    fn depth_first_post_order_recursive<V: TreeVisitor>(
        node: &TempNode,
        visitor: &mut V,
        depth: usize,
    ) {
        // Determine if this is a directory
        let is_directory = matches!(node.node_info.node_type, NodeType::Directory);
        
        // For directories in post-order, we need to decide whether to traverse children first
        // We'll call enter_directory to check if we should process this directory's children
        let should_traverse_children = if is_directory {
            visitor.enter_directory(&node.node_info, depth)
        } else {
            // For non-directories, we don't have children to traverse, so we'll visit them after
            true
        };

        // Traverse children first (if allowed)
        if should_traverse_children {
            for child in &node.children {
                Self::depth_first_post_order_recursive(child, visitor, depth + 1);
            }
        }

        // Visit the current node after children (post-order)
        if is_directory {
            // For directories, call exit_directory after children are processed
            visitor.exit_directory(&node.node_info, depth);
        } else {
            // For files and other nodes, call regular visit
            visitor.visit(&node.node_info, depth);
        }
    }

    /// Performs breadth-first traversal of the tree.
    ///
    /// This method visits nodes level by level, from left to right. Directory nodes
    /// trigger both `enter_directory` and `exit_directory` calls on the visitor.
    ///
    /// # Arguments
    ///
    /// * `root` - The root node to start traversal from
    /// * `visitor` - A mutable visitor that will be called for each node
    pub fn breadth_first<V: TreeVisitor>(root: &TempNode, visitor: &mut V) {
        let mut queue = VecDeque::new();
        let mut exit_hooks: VecDeque<(NodeInfo, usize)> = VecDeque::new(); // Queue for exit_directory calls
        
        queue.push_back((root, 0));
        let mut current_depth = 0;

        while let Some((node, depth)) = queue.pop_front() {
            // If we've moved to a shallower or different level, process exit hooks from deeper levels
            if depth != current_depth {
                // Find the maximum depth in exit_hooks
                let max_exit_depth = exit_hooks.iter().map(|(_, d)| *d).max().unwrap_or(0);
                
                // Process exit hooks from deepest to current depth
                for target_depth in (depth..=max_exit_depth).rev() {
                    // Process all hooks at this depth in FIFO order
                    let mut remaining_hooks = VecDeque::new();
                    while let Some((exit_node, exit_depth)) = exit_hooks.pop_front() {
                        if exit_depth == target_depth {
                            visitor.exit_directory(&exit_node, exit_depth);
                        } else {
                            remaining_hooks.push_back((exit_node, exit_depth));
                        }
                    }
                    exit_hooks = remaining_hooks;
                }
                
                current_depth = depth;
            }

            let is_directory = matches!(node.node_info.node_type, NodeType::Directory);
            
            let should_continue = if is_directory {
                // For directories, call enter_directory hook
                let result = visitor.enter_directory(&node.node_info, depth);
                // Queue the exit_directory call for after children are processed
                exit_hooks.push_back((node.node_info.clone(), depth));
                result
            } else {
                // For files and other nodes, call regular visit
                visitor.visit(&node.node_info, depth)
            };

            if should_continue {
                for child in &node.children {
                    queue.push_back((child, depth + 1));
                }
            }
        }
        
        // Process all remaining exit hooks (deepest first, FIFO within each depth)
        let max_exit_depth = exit_hooks.iter().map(|(_, d)| *d).max().unwrap_or(0);
        for target_depth in (0..=max_exit_depth).rev() {
            let mut remaining_hooks = VecDeque::new();
            while let Some((exit_node, exit_depth)) = exit_hooks.pop_front() {
                if exit_depth == target_depth {
                    visitor.exit_directory(&exit_node, exit_depth);
                } else {
                    remaining_hooks.push_back((exit_node, exit_depth));
                }
            }
            exit_hooks = remaining_hooks;
        }
    }

    /// Collects all nodes from a tree into a vector using the specified traversal order.
    ///
    /// This is a convenience method that performs traversal and collects all nodes
    /// into a vector, which can be useful for converting trees to flat lists.
    ///
    /// # Arguments
    ///
    /// * `root` - The root node to start traversal from
    /// * `order` - The traversal order to use
    ///
    /// # Returns
    ///
    /// A vector of `NodeInfo` objects in the specified traversal order.
    pub fn collect_nodes(root: &TempNode, order: TraversalOrder) -> Vec<NodeInfo> {
        let mut collector = NodeCollector::new();

        match order {
            TraversalOrder::DepthFirstPreOrder => {
                Self::depth_first_pre_order(root, &mut collector);
            }
            TraversalOrder::DepthFirstPostOrder => {
                Self::depth_first_post_order(root, &mut collector);
            }
            TraversalOrder::BreadthFirst => {
                Self::breadth_first(root, &mut collector);
            }
        }

        collector.into_nodes()
    }

    /// Finds the first node that satisfies the given predicate using depth-first search.
    ///
    /// This performs a depth-first search through the tree and returns the first node
    /// that matches the predicate.
    ///
    /// # Arguments
    ///
    /// * `root` - The root node to start searching from
    /// * `predicate` - A function that returns `true` for the desired node
    ///
    /// # Returns
    ///
    /// `Some(NodeInfo)` if a matching node is found, `None` otherwise.
    pub fn find_node<P>(root: &TempNode, predicate: P) -> Option<NodeInfo>
    where
        P: Fn(&NodeInfo) -> bool,
    {
        let mut finder = NodeFinder::new(predicate);
        Self::depth_first_pre_order(root, &mut finder);
        finder.into_result()
    }
}

/// A visitor implementation that collects all visited nodes.
struct NodeCollector {
    nodes: Vec<NodeInfo>,
}

impl NodeCollector {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn into_nodes(self) -> Vec<NodeInfo> {
        self.nodes
    }
}

impl TreeVisitor for NodeCollector {
    fn visit(&mut self, node: &NodeInfo, _depth: usize) -> bool {
        self.nodes.push(node.clone());
        true // Always continue traversal
    }
}

/// A visitor implementation that searches for a specific node.
struct NodeFinder<P>
where
    P: Fn(&NodeInfo) -> bool,
{
    predicate: P,
    found: Option<NodeInfo>,
}

impl<P> NodeFinder<P>
where
    P: Fn(&NodeInfo) -> bool,
{
    fn new(predicate: P) -> Self {
        Self {
            predicate,
            found: None,
        }
    }

    fn into_result(self) -> Option<NodeInfo> {
        self.found
    }
}

impl<P> TreeVisitor for NodeFinder<P>
where
    P: Fn(&NodeInfo) -> bool,
{
    fn visit(&mut self, node: &NodeInfo, _depth: usize) -> bool {
        if (self.predicate)(node) {
            self.found = Some(node.clone());
            false // Stop traversal once found
        } else {
            true // Continue searching
        }
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

    fn create_test_tree() -> TempNode {
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let mut child1 = create_test_node("child1", NodeType::Directory, 1);
        let child2 = create_test_node("child2", NodeType::File, 1);
        let grandchild = create_test_node("grandchild", NodeType::File, 2);

        child1.children.push(grandchild);
        root.children.push(child1);
        root.children.push(child2);

        root
    }

    #[test]
    fn test_collect_nodes_depth_first_pre_order() {
        let tree = create_test_tree();
        let nodes = TreeTraversal::collect_nodes(&tree, TraversalOrder::DepthFirstPreOrder);

        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].name, "root");
        assert_eq!(nodes[1].name, "child1");
        assert_eq!(nodes[2].name, "grandchild");
        assert_eq!(nodes[3].name, "child2");
    }

    #[test]
    fn test_collect_nodes_breadth_first() {
        let tree = create_test_tree();
        let nodes = TreeTraversal::collect_nodes(&tree, TraversalOrder::BreadthFirst);

        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].name, "root");
        assert_eq!(nodes[1].name, "child1");
        assert_eq!(nodes[2].name, "child2");
        assert_eq!(nodes[3].name, "grandchild");
    }

    #[test]
    fn test_find_node() {
        let tree = create_test_tree();
        let found = TreeTraversal::find_node(&tree, |node| node.name == "grandchild");

        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "grandchild");

        let not_found = TreeTraversal::find_node(&tree, |node| node.name == "nonexistent");
        assert!(not_found.is_none());
    }

    /// Test visitor that tracks directory hook calls
    struct DirectoryHookTracker {
        entries: Vec<String>,
        exits: Vec<String>,
        visits: Vec<String>,
    }

    impl DirectoryHookTracker {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                exits: Vec::new(),
                visits: Vec::new(),
            }
        }
    }

    impl TreeVisitor for DirectoryHookTracker {
        fn visit(&mut self, node: &NodeInfo, _depth: usize) -> bool {
            self.visits.push(node.name.clone());
            true
        }

        fn enter_directory(&mut self, node: &NodeInfo, _depth: usize) -> bool {
            self.entries.push(node.name.clone());
            true
        }

        fn exit_directory(&mut self, node: &NodeInfo, _depth: usize) {
            self.exits.push(node.name.clone());
        }
    }

    #[test]
    fn test_directory_hooks_depth_first_pre_order() {
        let tree = create_test_tree();
        let mut tracker = DirectoryHookTracker::new();
        TreeTraversal::depth_first_pre_order(&tree, &mut tracker);

        // Should have entered directories
        assert_eq!(tracker.entries, vec!["root", "child1"]);
        // Should have exited directories in reverse order (because of recursion)
        assert_eq!(tracker.exits, vec!["child1", "root"]);
        // Should have visited non-directory nodes
        assert_eq!(tracker.visits, vec!["grandchild", "child2"]);
    }

    #[test]
    fn test_directory_hooks_depth_first_post_order() {
        let tree = create_test_tree();
        let mut tracker = DirectoryHookTracker::new();
        TreeTraversal::depth_first_post_order(&tree, &mut tracker);

        // Should have entered directories
        assert_eq!(tracker.entries, vec!["root", "child1"]);
        // Should have exited directories after processing children
        assert_eq!(tracker.exits, vec!["child1", "root"]);
        // Should have visited non-directory nodes
        assert_eq!(tracker.visits, vec!["grandchild", "child2"]);
    }

    #[test]
    fn test_directory_hooks_breadth_first() {
        let tree = create_test_tree();
        let mut tracker = DirectoryHookTracker::new();
        
        TreeTraversal::breadth_first(&tree, &mut tracker);
        
        // In breadth-first, we should enter directories as we encounter them
        // and exit them when we finish processing their level
        assert_eq!(tracker.entries, vec!["root", "child1"]);
        assert_eq!(tracker.exits, vec!["child1", "root"]); // child1 exits before root
        assert_eq!(tracker.visits, vec!["child2", "grandchild"]);
    }

    #[test]
    fn test_breadth_first_exit_hooks_complex_tree() {
        // Create a more complex tree to test exit hook edge cases
        let mut root = create_test_node("root", NodeType::Directory, 0);
        
        // Level 1: multiple directories and files
        let mut dir1 = create_test_node("dir1", NodeType::Directory, 1);
        let mut dir2 = create_test_node("dir2", NodeType::Directory, 1);
        let file1 = create_test_node("file1", NodeType::File, 1);
        
        // Level 2: nested directories and files
        let mut subdir1 = create_test_node("subdir1", NodeType::Directory, 2);
        let file2 = create_test_node("file2", NodeType::File, 2);
        let file3 = create_test_node("file3", NodeType::File, 2);
        
        // Level 3: deep nesting
        let file4 = create_test_node("file4", NodeType::File, 3);
        
        // Build the tree structure
        subdir1.children.push(file4);
        dir1.children.push(subdir1);
        dir1.children.push(file2);
        dir2.children.push(file3);
        
        root.children.push(dir1);
        root.children.push(dir2);
        root.children.push(file1);
        
        let mut tracker = DirectoryHookTracker::new();
        TreeTraversal::breadth_first(&root, &mut tracker);
        
        // Expected order of operations:
        // Level 0: enter root
        // Level 1: enter dir1, enter dir2, visit file1
        // Level 2: enter subdir1, visit file2, visit file3
        // Level 3: visit file4
        // Exit hooks: subdir1 (after level 3), dir1, dir2 (after level 2), root (after everything)
        
        assert_eq!(tracker.entries, vec!["root", "dir1", "dir2", "subdir1"]);
        assert_eq!(tracker.exits, vec!["subdir1", "dir1", "dir2", "root"]);
        assert_eq!(tracker.visits, vec!["file1", "file2", "file3", "file4"]);
    }

    #[test]
    fn test_breadth_first_exit_hooks_single_directory() {
        // Test edge case: single directory with no children
        let root = create_test_node("single_dir", NodeType::Directory, 0);
        
        let mut tracker = DirectoryHookTracker::new();
        TreeTraversal::breadth_first(&root, &mut tracker);
        
        assert_eq!(tracker.entries, vec!["single_dir"]);
        assert_eq!(tracker.exits, vec!["single_dir"]);
        assert_eq!(tracker.visits, Vec::<String>::new());
    }

    #[test]
    fn test_breadth_first_exit_hooks_single_file() {
        // Test edge case: single file (no directory hooks)
        let root = create_test_node("single_file", NodeType::File, 0);
        
        let mut tracker = DirectoryHookTracker::new();
        TreeTraversal::breadth_first(&root, &mut tracker);
        
        assert_eq!(tracker.entries, Vec::<String>::new());
        assert_eq!(tracker.exits, Vec::<String>::new());
        assert_eq!(tracker.visits, vec!["single_file"]);
    }

    #[test]
    fn test_breadth_first_exit_hooks_directory_with_early_termination() {
        // Test when a visitor returns false to skip subtrees
        struct EarlyTerminationVisitor {
            entries: Vec<String>,
            exits: Vec<String>,
            visits: Vec<String>,
            skip_dir: String,
        }
        
        impl EarlyTerminationVisitor {
            fn new(skip_dir: String) -> Self {
                Self {
                    entries: Vec::new(),
                    exits: Vec::new(),
                    visits: Vec::new(),
                    skip_dir,
                }
            }
        }
        
        impl TreeVisitor for EarlyTerminationVisitor {
            fn visit(&mut self, node: &NodeInfo, _depth: usize) -> bool {
                self.visits.push(node.name.clone());
                true
            }
            
            fn enter_directory(&mut self, node: &NodeInfo, _depth: usize) -> bool {
                self.entries.push(node.name.clone());
                node.name != self.skip_dir // Skip the specified directory
            }
            
            fn exit_directory(&mut self, node: &NodeInfo, _depth: usize) {
                self.exits.push(node.name.clone());
            }
        }
        
        let tree = create_test_tree();
        let mut visitor = EarlyTerminationVisitor::new("child1".to_string());
        
        TreeTraversal::breadth_first(&tree, &mut visitor);
        
        // child1 should be entered but its children (grandchild) should not be processed
        // However, child1 should still get its exit hook called
        assert_eq!(visitor.entries, vec!["root", "child1"]);
        assert_eq!(visitor.exits, vec!["child1", "root"]);
        assert_eq!(visitor.visits, vec!["child2"]); // grandchild is not visited due to early termination
    }

    #[test]
    fn test_breadth_first_exit_hooks_deep_nesting() {
        // Test a deeply nested structure to ensure exit hooks are processed correctly
        let mut root = create_test_node("root", NodeType::Directory, 0);
        let mut current = &mut root;
        
        // Create a chain of nested directories
        for i in 1..=5 {
            let child = create_test_node(&format!("level_{}", i), NodeType::Directory, i);
            current.children.push(child);
            current = &mut current.children[0];
        }
        
        // Add a file at the deepest level
        let deep_file = create_test_node("deep_file", NodeType::File, 6);
        current.children.push(deep_file);
        
        let mut tracker = DirectoryHookTracker::new();
        TreeTraversal::breadth_first(&root, &mut tracker);
        
        // All directories should be entered in order
        assert_eq!(tracker.entries, vec!["root", "level_1", "level_2", "level_3", "level_4", "level_5"]);
        // And exited in reverse order (deepest first)
        assert_eq!(tracker.exits, vec!["level_5", "level_4", "level_3", "level_2", "level_1", "root"]);
        // Only the file should be visited
        assert_eq!(tracker.visits, vec!["deep_file"]);
    }
} 