//! Sorting strategies and algorithms.
//!
//! This module contains high-level sorting strategies that coordinate the use of
//! tree building, comparison functions, and flattening to sort node collections.

use crate::config::sorting::{SortKey, SortingOptions};
use crate::core::sorter::comparators::{compare_siblings, compare_siblings_with_options};
use crate::core::tree::builder::{build_tree, flatten_tree_to_dfs_consuming};
use crate::core::tree::node::NodeInfo;

/// Sorts a vector of `NodeInfo` while preserving the tree structure.
///
/// This function takes a flat vector of nodes in DFS order, builds a tree structure,
/// sorts siblings at each level according to the specified criteria, and then
/// flattens the tree back to a DFS-ordered vector.
///
/// # Arguments
/// * `nodes` - A mutable reference to a vector of `NodeInfo` to be sorted.
///   The initial vector is assumed to be in DFS order (e.g., from `walk_directory`).
/// * `key` - The [`SortKey`] specifying the attribute to sort siblings by.
/// * `reverse` - A boolean indicating whether to reverse the sort order for siblings.
///
/// # Returns
/// * `Result<(), String>` - Ok(()) on success, Err with error message if tree building fails
pub fn sort_nodes(nodes: &mut Vec<NodeInfo>, key: &SortKey, reverse: bool) -> Result<(), String> {
    if nodes.is_empty() {
        return Ok(());
    }

    // Handle the error from build_tree by propagating it to caller
    let mut roots = build_tree(std::mem::take(nodes))?;

    // 2. Sort the root nodes themselves (they are siblings at the top level)
    roots.sort_by(|a, b| compare_siblings(a, b, key, reverse));

    // 3. Sort the children within each part of the tree
    for root in &mut roots {
        root.sort_children_recursive(key, reverse);
    }

    // 4. Flatten the sorted tree back into the `nodes` vector
    flatten_tree_to_dfs_consuming(roots, nodes);
    Ok(())
}

/// Sorts a vector of `NodeInfo` while preserving the tree structure using SortingOptions.
///
/// This is the newer version that accepts a SortingOptions struct for more flexible configuration.
///
/// # Arguments
/// * `nodes` - A mutable reference to a vector of `NodeInfo` to be sorted.
///   The initial vector is assumed to be in DFS order (e.g., from `walk_directory`).
/// * `options` - The [`SortingOptions`] specifying how to sort siblings.
///
/// # Returns
/// * `Result<(), String>` - Ok(()) on success, Err with error message if tree building fails
pub fn sort_nodes_with_options(
    nodes: &mut Vec<NodeInfo>,
    options: &SortingOptions,
) -> Result<(), String> {
    if nodes.is_empty() {
        return Ok(());
    }

    // If no sorting is requested, return early
    if options.sort_by.is_none() {
        return Ok(());
    }

    // 1. Build the tree. `nodes` is moved and consumed.
    let mut roots = build_tree(std::mem::take(nodes))?;

    // 2. Sort the root nodes themselves (they are siblings at the top level)
    roots.sort_by(|a, b| compare_siblings_with_options(a, b, options));

    // 3. Sort the children within each part of the tree
    for root in &mut roots {
        root.sort_children_recursive_with_options(options);
    }

    // 4. Flatten the sorted tree back into the `nodes` vector
    flatten_tree_to_dfs_consuming(roots, nodes);
    Ok(())
}
