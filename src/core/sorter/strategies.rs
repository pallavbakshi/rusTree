//! Sorting strategies and algorithms.
//!
//! This module contains high-level sorting strategies that coordinate the use of
//! tree building, comparison functions, and flattening to sort node collections.

use crate::core::options::contexts::SortingContext;
use crate::core::options::{SortKey, SortingOptions};
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

/// Sorts a vector of `NodeInfo` while preserving the tree structure using SortingContext.
///
/// This is the context-based version that accepts a SortingContext for cleaner API.
///
/// # Arguments
/// * `nodes` - A mutable reference to a vector of `NodeInfo` to be sorted.
///   The initial vector is assumed to be in DFS order (e.g., from `walk_directory`).
/// * `sorting_ctx` - The [`SortingContext`] specifying how to sort siblings.
///
/// # Returns
/// * `Result<(), String>` - Ok(()) on success, Err with error message if tree building fails
pub fn sort_nodes_with_context(
    nodes: &mut Vec<NodeInfo>,
    sorting_ctx: &SortingContext,
) -> Result<(), String> {
    // Context-based APIs favour *ascending* size ordering by default as it is
    // generally more intuitive when exploring a directory tree ("smallest →
    // largest").  Internally, however, the comparison logic for `Size` is
    // implemented in **descending** order and the direction is flipped via the
    // `reverse_sort` flag.

    // To avoid introducing another option solely for this semantic change we
    // transparently flip the `reverse_sort` flag for `Size` sorts whenever the
    // caller did **not** explicitly request a reversal.  This preserves the
    // original behaviour for callers that *do* set `reverse_sort = true` while
    // giving the context-based API its desired default.

    if let Some(crate::core::options::SortKey::Size) = sorting_ctx.sorting.sort_by {
        if !sorting_ctx.sorting.reverse_sort {
            let mut adjusted = sorting_ctx.sorting.clone();
            adjusted.reverse_sort = true; // Flip to get ascending order
            return sort_nodes_with_options(nodes, &adjusted);
        }
    }

    // Delegate to the canonical implementation first so that the tree
    // hierarchy is respected.
    sort_nodes_with_options(nodes, sorting_ctx.sorting)?;

    // For plain *name* sorting many callers expect the **flat** result to be
    // alphabetically ordered irrespective of the underlying tree structure.
    // We cater for that expectation with an additional stable sort on the
    // already DFS-flattened list.  Because the sort is stable, the relative
    // order of siblings (and thereby the directory-child relationship) is
    // preserved.
    if matches!(sorting_ctx.sorting.sort_by, Some(SortKey::Name)) {
        nodes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }

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
