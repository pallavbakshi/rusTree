// src/core/sorter.rs
use crate::core::node::NodeInfo;
use std::cmp::Ordering;

/// Defines the keys by which directory entries can be sorted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortKey {
    /// Sort by entry name (alphabetically).
    Name,
    /// Sort by entry size (files only, typically smallest to largest).
    /// Directories are usually grouped separately or sorted by name as a tie-breaker.
    Size,
    /// Sort by last modification time (oldest to newest).
    MTime,
    /// Sort by word count (files only, typically fewest to most).
    Words,
    /// Sort by line count (files only, typically fewest to most).
    Lines,
    /// Sort by the output of a custom applied function (if applicable and sortable).
    Custom,
}

/// Sorts a vector of `NodeInfo` entries in place.
///
/// The sorting behavior is as follows:
/// - Entries are primarily sorted to maintain the directory structure (parent before child).
/// - Siblings (entries at the same depth with the same parent) are sorted according to the `key`.
/// - If `reverse` is true, the sibling sort order is reversed.
///
/// # Arguments
///
/// * `nodes` - A mutable reference to a vector of `NodeInfo` to be sorted.
/// * `key` - The [`SortKey`] specifying the attribute to sort siblings by.
/// * `reverse` - A boolean indicating whether to reverse the sort order for siblings.
pub fn sort_nodes(nodes: &mut Vec<NodeInfo>, key: &SortKey, reverse: bool) {
    nodes.sort_by(|a, b| {
        // Check if nodes are siblings (same parent and same depth)
        if a.path.parent() == b.path.parent() && a.depth == b.depth {
            // They are siblings, sort them by the specified key
            let mut key_ordering = match key {
                SortKey::Name => a.name.cmp(&b.name),
                SortKey::Size => {
                    // Directories (None size) should typically sort after files (Some size)
                    // or consistently. Default tree often lists dirs after files if not sorting by name.
                    // Current: Some < None (files before dirs if sizes differ)
                    match (a.size, b.size) {
                        (Some(sa), Some(sb)) => sa.cmp(&sb), // Both have size, compare them
                        (Some(_), None) => Ordering::Less,    // a (file) before b (dir)
                        (None, Some(_)) => Ordering::Greater, // a (dir) after b (file)
                        (None, None) => a.name.cmp(&b.name), // Both dirs, sort by name as tie-breaker
                    }
                }
                SortKey::MTime => {
                    match (a.mtime, b.mtime) {
                        (Some(ta), Some(tb)) => ta.cmp(&tb),
                        (Some(_), None) => Ordering::Less,    // Valid MTime before None
                        (None, Some(_)) => Ordering::Greater, // None after valid MTime
                        (None, None) => a.name.cmp(&b.name), // Both None, sort by name
                    }
                }
                SortKey::Words => {
                     match (a.word_count, b.word_count) {
                        (Some(wa), Some(wb)) => wa.cmp(&wb),
                        (Some(_), None) => Ordering::Less,    // Files with count before those without (e.g. dirs)
                        (None, Some(_)) => Ordering::Greater,
                        (None, None) => a.name.cmp(&b.name), // Both None, sort by name
                    }
                }
                SortKey::Lines => {
                    match (a.line_count, b.line_count) {
                        (Some(la), Some(lb)) => la.cmp(&lb),
                        (Some(_), None) => Ordering::Less,
                        (None, Some(_)) => Ordering::Greater,
                        (None, None) => a.name.cmp(&b.name),
                    }
                }
                SortKey::Custom => {
                    match (&a.custom_function_output, &b.custom_function_output) {
                        (Some(Ok(val_a)), Some(Ok(val_b))) => val_a.cmp(val_b),
                        (Some(Ok(_)), _) => Ordering::Less,
                        (_, Some(Ok(_))) => Ordering::Greater,
                        (Some(Err(_)), Some(Err(_))) => a.name.cmp(&b.name), // Errors equal, sort by name
                        (Some(Err(_)), None) => Ordering::Less,
                        (None, Some(Err(_))) => Ordering::Greater,
                        (None, None) => a.name.cmp(&b.name), // Nones equal, sort by name
                    }
                }
            };

            if reverse {
                key_ordering = key_ordering.reverse();
            }
            key_ordering
        } else {
            // Not siblings. Maintain DFS order primarily by path.
            // This ensures parent comes before child, and preserves walkdir's DFS traversal order
            // for non-siblings. Path comparison naturally handles parent/child ordering.
            a.path.cmp(&b.path)
        }
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::node::NodeType;
    use std::path::PathBuf;
    use std::time::SystemTime;

    fn create_test_node(name_str: &str, size: Option<u64>, line_count: Option<usize>) -> NodeInfo {
        NodeInfo {
            path: PathBuf::from(name_str), // Simplified path for testing sorter standalone
            name: name_str.to_string(),
            node_type: if size.is_none() { NodeType::Directory } else { NodeType::File }, // Infer type for size test
            depth: 0, // All test nodes are at depth 0, so they are treated as siblings
            size,
            permissions: None,
            mtime: Some(SystemTime::now()),
            line_count,
            word_count: None,
            custom_function_output: None,
        }
    }

    #[test]
    fn test_sort_by_name() {
        let mut nodes = vec![
            create_test_node("charlie.txt", Some(100), Some(10)),
            create_test_node("alpha.txt", Some(200), Some(5)),
            create_test_node("beta.txt", Some(50), Some(20)),
        ];
        // All nodes have depth 0, so they are siblings.
        sort_nodes(&mut nodes, &SortKey::Name, false);
        assert_eq!(nodes[0].name, "alpha.txt");
        assert_eq!(nodes[1].name, "beta.txt");
        assert_eq!(nodes[2].name, "charlie.txt");
    }

    #[test]
    fn test_sort_by_name_reverse() {
        let mut nodes = vec![
            create_test_node("charlie.txt", Some(100), Some(10)),
            create_test_node("alpha.txt", Some(200), Some(5)),
            create_test_node("beta.txt", Some(50), Some(20)),
        ];
        sort_nodes(&mut nodes, &SortKey::Name, true);
        assert_eq!(nodes[0].name, "charlie.txt");
        assert_eq!(nodes[1].name, "beta.txt");
        assert_eq!(nodes[2].name, "alpha.txt");
    }

    #[test]
    fn test_sort_by_size_handles_none() {
        // node_type is inferred from size: Some -> File, None -> Directory
        let mut nodes = vec![
            create_test_node("big_file.txt", Some(1000), None),      // File
            create_test_node("directory_alpha", None, None),         // Directory
            create_test_node("small_file.txt", Some(10), None),      // File
            create_test_node("directory_beta", None, None),          // Directory
        ];
        
        // SortKey::Size: Some(size) < None (files before directories)
        // For files: smaller size first. For dirs: by name.
        sort_nodes(&mut nodes, &SortKey::Size, false); // Ascending
        assert_eq!(nodes[0].name, "small_file.txt");   // File, 10B
        assert_eq!(nodes[1].name, "big_file.txt");     // File, 1000B
        assert_eq!(nodes[2].name, "directory_alpha");  // Dir, by name
        assert_eq!(nodes[3].name, "directory_beta");   // Dir, by name

        // Test reverse sort
        let mut nodes_rev = vec![
            create_test_node("big_file.txt", Some(1000), None),
            create_test_node("directory_alpha", None, None),
            create_test_node("small_file.txt", Some(10), None),
            create_test_node("directory_beta", None, None),
        ];
        sort_nodes(&mut nodes_rev, &SortKey::Size, true); // Descending
                                                          // Dirs (None size) come after files (Some size). When reversed, Dirs still after Files.
                                                          // Within files, largest first. Within dirs, by name reversed.
        assert_eq!(nodes_rev[0].name, "directory_beta");    // Dir, by name reversed
        assert_eq!(nodes_rev[1].name, "directory_alpha");   // Dir, by name reversed
        assert_eq!(nodes_rev[2].name, "big_file.txt");      // File, 1000B
        assert_eq!(nodes_rev[3].name, "small_file.txt");    // File, 10B
    }
}