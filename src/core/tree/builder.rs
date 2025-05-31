//! Tree construction and manipulation utilities.
//!
//! This module contains functionality for building temporary tree structures
//! from flat lists of nodes, primarily used for sorting operations.

use crate::config::sorting::{SortKey, SortingOptions};
use crate::core::tree::node::NodeInfo;

/// Temporary tree node used for building and sorting tree structures.
#[derive(Debug)]
pub struct TempNode {
    pub node_info: NodeInfo,
    pub children: Vec<TempNode>,
}

impl TempNode {
    pub fn new(node_info: NodeInfo) -> Self {
        Self {
            node_info,
            children: Vec::new(),
        }
    }

    /// Recursively sorts children at each level of the tree.
    pub fn sort_children_recursive(&mut self, key: &SortKey, reverse: bool) {
        if !self.children.is_empty() {
            self.children
                .sort_by(|a, b| crate::core::sorter::comparators::compare_siblings(a, b, key, reverse));
            for child in &mut self.children {
                child.sort_children_recursive(key, reverse);
            }
        }
    }

    /// Recursively sorts children at each level of the tree using SortingOptions.
    pub fn sort_children_recursive_with_options(&mut self, options: &SortingOptions) {
        if !self.children.is_empty() {
            self.children
                .sort_by(|a, b| crate::core::sorter::comparators::compare_siblings_with_options(a, b, options));
            for child in &mut self.children {
                child.sort_children_recursive_with_options(options);
            }
        }
    }
}

/// Builds a tree structure from a flat list of NodeInfo objects.
///
/// The input nodes are assumed to be in DFS order with correct depth information.
/// Returns a vector of root nodes, each potentially having children.
/// Returns an error if the depth sequence is malformed or inconsistent.
pub fn build_tree(nodes_info: Vec<NodeInfo>) -> Result<Vec<TempNode>, String> {
    if nodes_info.is_empty() {
        return Ok(Vec::new());
    }

    let mut roots: Vec<TempNode> = Vec::new();
    // Stack stores indices representing the path to the current parent node
    // This avoids raw pointers while still allowing efficient parent tracking
    let mut path_stack: Vec<usize> = Vec::new();

    for node_info in nodes_info {
        let current_depth = node_info.depth;
        let new_temp_node = TempNode {
            node_info,
            children: Vec::new(),
        };

        // Pop from path until we find the correct parent depth
        while !path_stack.is_empty() {
            let parent_node = get_node_by_path(&roots, &path_stack)
                .ok_or_else(|| format!("Invalid path stack: unable to find parent node at depth"))?;
            let parent_depth = parent_node.node_info.depth;
            if current_depth <= parent_depth {
                path_stack.pop();
            } else {
                break;
            }
        }

        if path_stack.is_empty() {
            // This node is a root
            roots.push(new_temp_node);
            path_stack.push(roots.len() - 1);
        } else {
            // This node is a child of the node at path_stack
            let parent = get_node_mut_by_path(&mut roots, &path_stack)
                .ok_or_else(|| format!("Invalid path stack: unable to find parent node for insertion"))?;
            parent.children.push(new_temp_node);
            path_stack.push(parent.children.len() - 1);
        }
    }
    Ok(roots)
}

/// Gets a node by following a path of indices through the tree.
fn get_node_by_path<'a>(roots: &'a [TempNode], path: &[usize]) -> Option<&'a TempNode> {
    if path.is_empty() {
        return None;
    }
    
    let mut current = roots.get(path[0])?;
    
    for &idx in &path[1..] {
        current = current.children.get(idx)?;
    }
    
    Some(current)
}

/// Gets a mutable node by following a path of indices through the tree.
fn get_node_mut_by_path<'a>(roots: &'a mut [TempNode], path: &[usize]) -> Option<&'a mut TempNode> {
    if path.is_empty() {
        return None;
    }
    
    let mut current = roots.get_mut(path[0])?;
    
    for &idx in &path[1..] {
        current = current.children.get_mut(idx)?;
    }
    
    Some(current)
}

/// Flattens a tree structure back into a DFS-ordered vector of NodeInfo objects.
///
/// This function consumes the tree structure and produces a flat list suitable
/// for further processing or output.
pub fn flatten_tree_to_dfs_consuming(roots: Vec<TempNode>, result: &mut Vec<NodeInfo>) {
    for temp_node in roots {
        result.push(temp_node.node_info); // NodeInfo is Clone
        flatten_tree_to_dfs_consuming(temp_node.children, result);
    }
} 