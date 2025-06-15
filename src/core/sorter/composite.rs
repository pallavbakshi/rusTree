//! Composite sorting functionality for combining multiple sort criteria.
//!
//! This module provides utilities for combining multiple sorting keys and criteria
//! to create complex sorting behaviors. This is useful for implementing hierarchical
//! sorting where files might be sorted by type first, then by name, then by size, etc.

use crate::core::options::SortKey;
use crate::core::sorter::comparators::compare_siblings;
use crate::core::tree::builder::TempNode;
use std::cmp::Ordering;

/// A composite sort key that combines multiple sorting criteria.
///
/// This allows for hierarchical sorting where the first key takes precedence,
/// but if two items are equal according to the first key, the second key is used,
/// and so on.
///
/// # Examples
///
/// ```rust
/// # use rustree::core::sorter::composite::CompositeSortKey;
/// # use rustree::config::sorting::SortKey;
///
/// // Sort by type first (files before directories), then by name
/// let composite = CompositeSortKey::new(vec![
///     (SortKey::Size, false),  // Size sorting (which includes type sorting)
///     (SortKey::Name, false),  // Then by name
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct CompositeSortKey {
    /// The sorting criteria in order of precedence
    criteria: Vec<(SortKey, bool)>, // (key, reverse)
}

impl CompositeSortKey {
    /// Creates a new composite sort key with the given criteria.
    ///
    /// # Arguments
    ///
    /// * `criteria` - A vector of (SortKey, reverse) tuples in order of precedence
    ///
    /// # Returns
    ///
    /// A new `CompositeSortKey` instance.
    pub fn new(criteria: Vec<(SortKey, bool)>) -> Self {
        Self { criteria }
    }

    /// Compares two nodes using the composite sorting criteria.
    ///
    /// This method applies each sort key in order until a non-equal comparison
    /// is found, or all keys have been exhausted.
    ///
    /// # Arguments
    ///
    /// * `a` - First node to compare
    /// * `b` - Second node to compare
    ///
    /// # Returns
    ///
    /// `Ordering` indicating the relationship between the nodes.
    pub fn compare(&self, a: &TempNode, b: &TempNode) -> Ordering {
        for (key, reverse) in &self.criteria {
            let ord = compare_siblings(a, b, key, *reverse);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    }

    /// Returns true if this composite key is empty (no criteria).
    pub fn is_empty(&self) -> bool {
        self.criteria.is_empty()
    }

    /// Returns the number of sort criteria in this composite key.
    pub fn len(&self) -> usize {
        self.criteria.len()
    }
}

impl Default for CompositeSortKey {
    /// Creates a default composite sort key that sorts by name only.
    fn default() -> Self {
        Self {
            criteria: vec![(SortKey::Name, false)],
        }
    }
}

/// Creates a common composite sort key for "natural" file system sorting.
///
/// This creates a sort key that:
/// 1. Sorts directories before files
/// 2. Then sorts by name using version-aware comparison
///
/// This is similar to how many file managers sort by default.
pub fn natural_sort() -> CompositeSortKey {
    CompositeSortKey::new(vec![
        (SortKey::Size, false), // This puts files before directories and sorts by size within type
        (SortKey::Version, false), // Then by version-aware name comparison
    ])
}

/// Creates a composite sort key for detailed file information sorting.
///
/// This creates a sort key that:
/// 1. Sorts by modification time (newest first)
/// 2. Then by size (largest first)
/// 3. Finally by name
///
/// This is useful for finding recently modified large files.
pub fn detailed_sort() -> CompositeSortKey {
    CompositeSortKey::new(vec![
        (SortKey::MTime, true), // Newest first
        (SortKey::Size, true),  // Largest first (within same mtime)
        (SortKey::Name, false), // Then by name
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tree::node::{NodeInfo, NodeType};
    use std::path::PathBuf;

    fn create_test_node(name: &str, node_type: NodeType, size: Option<u64>) -> TempNode {
        TempNode {
            node_info: NodeInfo {
                name: name.to_string(),
                path: PathBuf::from(name),
                node_type,
                depth: 1,
                size,
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
    fn test_composite_sort_key_creation() {
        let composite = CompositeSortKey::new(vec![(SortKey::Name, false), (SortKey::Size, true)]);

        assert_eq!(composite.len(), 2);
        assert!(!composite.is_empty());
    }

    #[test]
    fn test_composite_sort_comparison() {
        let composite = CompositeSortKey::new(vec![
            (SortKey::Size, false), // Files before directories
            (SortKey::Name, false), // Then by name
        ]);

        let file_a = create_test_node("a.txt", NodeType::File, Some(100));
        let file_b = create_test_node("b.txt", NodeType::File, Some(200));
        let dir = create_test_node("z_dir", NodeType::Directory, None);

        // Files should come before directories
        assert_eq!(composite.compare(&file_a, &dir), Ordering::Less);
        assert_eq!(composite.compare(&dir, &file_a), Ordering::Greater);

        // Among files, larger sizes come first (descending order)
        assert_eq!(composite.compare(&file_a, &file_b), Ordering::Greater);
    }

    #[test]
    fn test_natural_sort() {
        let natural = natural_sort();
        assert!(!natural.is_empty());
        assert_eq!(natural.len(), 2);
    }

    #[test]
    fn test_detailed_sort() {
        let detailed = detailed_sort();
        assert!(!detailed.is_empty());
        assert_eq!(detailed.len(), 3);
    }
}
