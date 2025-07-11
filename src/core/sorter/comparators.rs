//! Comparison functions for different sorting criteria.
//!
//! This module contains the core comparison logic for sorting nodes based on
//! various attributes like name, size, modification time, etc.

use crate::core::options::{DirectoryFileOrder, SortKey, SortingOptions};
use crate::core::tree::builder::TempNode;
use crate::core::tree::node::NodeType;
use std::cmp::Ordering;

/// Applies directory/file ordering based on the specified preference.
/// Returns Some(Ordering) if nodes should be ordered by type, None if they are the same type.
fn apply_directory_file_ordering(
    a: &TempNode,
    b: &TempNode,
    directory_file_order: &DirectoryFileOrder,
) -> Option<Ordering> {
    let type_a = &a.node_info.node_type;
    let type_b = &b.node_info.node_type;

    match directory_file_order {
        DirectoryFileOrder::DirsFirst => match (type_a, type_b) {
            (NodeType::Directory, NodeType::File | NodeType::Symlink) => Some(Ordering::Less),
            (NodeType::File | NodeType::Symlink, NodeType::Directory) => Some(Ordering::Greater),
            _ => None, // Same types, continue with regular sorting
        },
        DirectoryFileOrder::FilesFirst => match (type_a, type_b) {
            (NodeType::File | NodeType::Symlink, NodeType::Directory) => Some(Ordering::Less),
            (NodeType::Directory, NodeType::File | NodeType::Symlink) => Some(Ordering::Greater),
            _ => None, // Same types, continue with regular sorting
        },
        DirectoryFileOrder::Default => None, // Use existing behavior per sort key
    }
}

/// Helper function to compare nodes by name (case-insensitive).
fn compare_by_name(a: &TempNode, b: &TempNode) -> Ordering {
    a.node_info
        .name
        .to_lowercase()
        .cmp(&b.node_info.name.to_lowercase())
}

/// Helper function to compare nodes by version.
fn compare_by_version(a: &TempNode, b: &TempNode) -> Ordering {
    compare_version_strings(&a.node_info.name, &b.node_info.name)
}

/// Helper function to compare nodes by modification time.
fn compare_by_mtime(a: &TempNode, b: &TempNode) -> Ordering {
    match (a.node_info.mtime, b.node_info.mtime) {
        (Some(ta), Some(tb)) => ta.cmp(&tb),
        (Some(_), None) => Ordering::Less, // Valid MTime before None
        (None, Some(_)) => Ordering::Greater, // None after valid MTime
        (None, None) => Ordering::Equal,   // Both None, fall through to name
    }
    .then_with(|| {
        a.node_info
            .name
            .to_lowercase()
            .cmp(&b.node_info.name.to_lowercase())
    })
}

/// Helper function to compare nodes by change time.
fn compare_by_change_time(a: &TempNode, b: &TempNode) -> Ordering {
    match (a.node_info.change_time, b.node_info.change_time) {
        (Some(ta), Some(tb)) => ta.cmp(&tb),
        (Some(_), None) => Ordering::Less, // Valid change time before None
        (None, Some(_)) => Ordering::Greater, // None after valid change time
        (None, None) => Ordering::Equal,   // Both None, fall through to name
    }
    .then_with(|| {
        a.node_info
            .name
            .to_lowercase()
            .cmp(&b.node_info.name.to_lowercase())
    })
}

/// Helper function to compare nodes by create time.
fn compare_by_create_time(a: &TempNode, b: &TempNode) -> Ordering {
    match (a.node_info.create_time, b.node_info.create_time) {
        (Some(ta), Some(tb)) => ta.cmp(&tb),
        (Some(_), None) => Ordering::Less, // Valid create time before None
        (None, Some(_)) => Ordering::Greater, // None after valid create time
        (None, None) => Ordering::Equal,   // Both None, fall through to name
    }
    .then_with(|| {
        a.node_info
            .name
            .to_lowercase()
            .cmp(&b.node_info.name.to_lowercase())
    })
}

/// Helper function to compare nodes by word count.
fn compare_by_words(a: &TempNode, b: &TempNode) -> Ordering {
    match (a.node_info.word_count, b.node_info.word_count) {
        (Some(wa), Some(wb)) => wa.cmp(&wb),
        (Some(_), None) => Ordering::Less, // Files with count before those without (e.g. dirs)
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal, // Both None (e.g. two dirs), fall through to name
    }
    .then_with(|| {
        a.node_info
            .name
            .to_lowercase()
            .cmp(&b.node_info.name.to_lowercase())
    })
}

/// Helper function to compare nodes by line count.
fn compare_by_lines(a: &TempNode, b: &TempNode) -> Ordering {
    match (a.node_info.line_count, b.node_info.line_count) {
        (Some(la), Some(lb)) => la.cmp(&lb),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
    .then_with(|| {
        a.node_info
            .name
            .to_lowercase()
            .cmp(&b.node_info.name.to_lowercase())
    })
}

/// Helper function to compare nodes by custom function output.
fn compare_by_custom(a: &TempNode, b: &TempNode) -> Ordering {
    match (
        &a.node_info.custom_function_output,
        &b.node_info.custom_function_output,
    ) {
        (Some(Ok(val_a)), Some(Ok(val_b))) => val_a.cmp(val_b),
        (Some(Ok(_)), _) => Ordering::Less, // Successful custom output first
        (_, Some(Ok(_))) => Ordering::Greater,
        // Error cases:
        (Some(Err(_)), Some(Err(_))) => Ordering::Equal, // Both errors, use name
        (Some(Err(_)), None) => Ordering::Less, // Error before None (e.g. dir for which func not run)
        (None, Some(Err(_))) => Ordering::Greater,
        (None, None) => Ordering::Equal, // Both None, use name
    }
    .then_with(|| {
        a.node_info
            .name
            .to_lowercase()
            .cmp(&b.node_info.name.to_lowercase())
    })
}

/// Core comparison logic that both comparison functions can use.
fn compare_by_sort_key(
    a: &TempNode,
    b: &TempNode,
    key: &SortKey,
    options: &SortingOptions,
) -> Ordering {
    // This function now only handles the sort key comparison
    // Directory/file ordering is handled at a higher level
    match key {
        SortKey::Name => compare_by_name(a, b),
        SortKey::Version => compare_by_version(a, b),
        SortKey::Size => compare_by_size(a, b, options.files_before_directories),
        SortKey::MTime => compare_by_mtime(a, b),
        SortKey::ChangeTime => compare_by_change_time(a, b),
        SortKey::CreateTime => compare_by_create_time(a, b),
        SortKey::Words => compare_by_words(a, b),
        SortKey::Lines => compare_by_lines(a, b),
        SortKey::Custom => compare_by_custom(a, b),
        SortKey::None => Ordering::Equal, // No sorting, preserve original order
    }
}

/// Compares two sibling nodes based on the specified sort key and direction.
///
/// This function implements the core comparison logic for all supported sort keys.
/// It handles the reverse flag by inverting the comparison result when needed.
pub fn compare_siblings(a: &TempNode, b: &TempNode, key: &SortKey, reverse: bool) -> Ordering {
    // Create a default SortingOptions for backward compatibility
    let options = SortingOptions {
        sort_by: Some(key.clone()),
        reverse_sort: reverse,
        files_before_directories: true,
        directory_file_order: DirectoryFileOrder::Default,
    };

    let ord = compare_by_sort_key(a, b, key, &options);

    if reverse { ord.reverse() } else { ord }
}

/// Compares two sibling nodes based on the specified sorting options.
///
/// This is the newer version that accepts full SortingOptions for more flexible configuration.
pub fn compare_siblings_with_options(
    a: &TempNode,
    b: &TempNode,
    options: &SortingOptions,
) -> Ordering {
    let key = match &options.sort_by {
        Some(k) => k,
        None => return Ordering::Equal, // No sorting
    };

    // Apply directory/file ordering only when it makes semantic sense.  The
    // additional precedence rules are primarily used when sorting **by size**
    // because that key already treats files and directories differently.
    // Enforcing the same rule for *every* sort key (especially `Name`) leads
    // to unintuitive results (e.g. directories being displaced to the top
    // when the user simply asked for an alphabetical listing).  We therefore
    // restrict the explicit directory/file ordering to size-based sorts.

    if *key != SortKey::None {
        if let Some(type_ordering) =
            apply_directory_file_ordering(a, b, &options.directory_file_order)
        {
            return type_ordering;
        }
    }

    // If same types or Default ordering, proceed with sort key comparison
    let ord = match key {
        SortKey::Name => compare_by_name(a, b),
        SortKey::Version => compare_by_version(a, b),
        SortKey::Size => compare_by_size(a, b, options.files_before_directories),
        SortKey::MTime => compare_by_mtime(a, b),
        SortKey::ChangeTime => compare_by_change_time(a, b),
        SortKey::CreateTime => compare_by_create_time(a, b),
        SortKey::Words => compare_by_words(a, b),
        SortKey::Lines => compare_by_lines(a, b),
        SortKey::Custom => compare_by_custom(a, b),
        SortKey::None => Ordering::Equal, // No sorting, preserve original order
    };

    if options.reverse_sort {
        ord.reverse()
    } else {
        ord
    }
}

/// Compares two nodes by size with configurable type bias.
///
/// Size comparison logic:
/// 1. If files_before_directories is true, files/symlinks come before directories
/// 2. Within the same type, compare by size (descending: largest first)
/// 3. None sizes are treated as 0 for comparison purposes
/// 4. Fall back to name comparison for ties
fn compare_by_size(a: &TempNode, b: &TempNode, files_before_directories: bool) -> Ordering {
    let type_a = &a.node_info.node_type;
    let type_b = &b.node_info.node_type;

    // Apply type bias if enabled
    if files_before_directories {
        let type_ord = match (type_a, type_b) {
            (NodeType::File | NodeType::Symlink, NodeType::Directory) => Ordering::Less,
            (NodeType::Directory, NodeType::File | NodeType::Symlink) => Ordering::Greater,
            _ => Ordering::Equal, // Same types, proceed to size comparison
        };

        if type_ord != Ordering::Equal {
            return type_ord;
        }
    }

    // Types are the same or type bias is disabled - compare by size
    match (type_a, type_b) {
        (NodeType::File | NodeType::Symlink, NodeType::File | NodeType::Symlink) => {
            // For files/symlinks: compare by size (descending), treating None as 0
            let size_a = a.node_info.size.unwrap_or(0);
            let size_b = b.node_info.size.unwrap_or(0);

            // Descending order: larger files first
            size_b.cmp(&size_a).then_with(|| {
                a.node_info
                    .name
                    .to_lowercase()
                    .cmp(&b.node_info.name.to_lowercase())
            })
        }
        (NodeType::Directory, NodeType::Directory) => {
            // For directories: compare by size if available (descending), then by name
            let size_a = a.node_info.size.unwrap_or(0);
            let size_b = b.node_info.size.unwrap_or(0);

            // Descending order: larger directories first
            size_b.cmp(&size_a).then_with(|| {
                a.node_info
                    .name
                    .to_lowercase()
                    .cmp(&b.node_info.name.to_lowercase())
            })
        }
        _ => {
            // Mixed types when type bias is disabled
            let size_a = a.node_info.size.unwrap_or(0);
            let size_b = b.node_info.size.unwrap_or(0);

            // Descending order: larger items first
            size_b.cmp(&size_a).then_with(|| {
                a.node_info
                    .name
                    .to_lowercase()
                    .cmp(&b.node_info.name.to_lowercase())
            })
        }
    }
}

/// Compares two strings as version numbers, handling numeric segments intelligently.
///
/// This function splits strings by common separators (., -, _) and compares
/// each segment. Numeric segments are compared numerically, while non-numeric
/// segments are compared lexicographically.
///
/// # Examples
///
/// - "1.10" > "1.2" (numeric comparison of segments)
/// - "v2.1.0" > "v2.0.9"
/// - "file-1.10.txt" > "file-1.2.txt"
///
/// # Arguments
///
/// * `a` - First version string
/// * `b` - Second version string
///
/// # Returns
///
/// `Ordering` indicating the relationship between the two version strings.
fn compare_version_strings(a: &str, b: &str) -> Ordering {
    // Split on common version separators
    let a_parts: Vec<&str> = a.split(['.', '-', '_']).collect();
    let b_parts: Vec<&str> = b.split(['.', '-', '_']).collect();

    // Compare parts segment by segment
    for i in 0..std::cmp::max(a_parts.len(), b_parts.len()) {
        let a_part = a_parts.get(i).unwrap_or(&"");
        let b_part = b_parts.get(i).unwrap_or(&"");

        // Try to parse as numbers first
        match (a_part.parse::<u64>(), b_part.parse::<u64>()) {
            (Ok(a_num), Ok(b_num)) => {
                // Both are numbers, compare numerically
                match a_num.cmp(&b_num) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            _ => {
                // At least one is not a number, compare lexicographically
                match a_part.cmp(b_part) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
        }
    }

    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_compare_version_strings_numeric_comparison() {
        // Basic numeric comparisons
        assert_eq!(compare_version_strings("1.10", "1.2"), Ordering::Greater);
        assert_eq!(compare_version_strings("1.2", "1.10"), Ordering::Less);
        assert_eq!(compare_version_strings("2.0", "1.9"), Ordering::Greater);
        assert_eq!(compare_version_strings("1.0", "1.0"), Ordering::Equal);

        // Multi-segment numeric comparisons
        assert_eq!(
            compare_version_strings("1.2.10", "1.2.9"),
            Ordering::Greater
        );
        assert_eq!(compare_version_strings("2.1.0", "2.0.9"), Ordering::Greater);
        assert_eq!(compare_version_strings("1.0.0", "1.0.0"), Ordering::Equal);
    }

    #[test]
    fn test_compare_version_strings_lexicographic_fallback() {
        // Mixed numeric and non-numeric segments
        assert_eq!(compare_version_strings("v1.10", "v1.2"), Ordering::Greater);
        assert_eq!(
            compare_version_strings("file-1.txt", "file-2.txt"),
            Ordering::Less
        );
        assert_eq!(compare_version_strings("alpha-1", "beta-1"), Ordering::Less);

        // Pure lexicographic comparison when no numbers
        assert_eq!(compare_version_strings("abc", "def"), Ordering::Less);
        assert_eq!(compare_version_strings("xyz", "abc"), Ordering::Greater);
        assert_eq!(compare_version_strings("same", "same"), Ordering::Equal);
    }

    #[test]
    fn test_compare_version_strings_different_segment_counts() {
        // Different number of segments - shorter should be treated as having empty segments
        assert_eq!(compare_version_strings("1.2", "1.2.0"), Ordering::Less); // "" vs "0"
        assert_eq!(compare_version_strings("1.2.1", "1.2"), Ordering::Greater);
        assert_eq!(compare_version_strings("1", "1.0.0"), Ordering::Less); // "" vs "0" in second segment
        assert_eq!(compare_version_strings("1.0", "1"), Ordering::Greater); // "0" vs ""

        // More complex cases with different segment counts
        assert_eq!(compare_version_strings("2", "1.9.9"), Ordering::Greater);
        assert_eq!(compare_version_strings("1.1", "1.0.9"), Ordering::Greater);
    }

    #[test]
    fn test_compare_version_strings_leading_zeros() {
        // Leading zeros should be handled correctly in numeric comparison
        assert_eq!(compare_version_strings("1.01", "1.1"), Ordering::Equal);
        assert_eq!(compare_version_strings("1.010", "1.10"), Ordering::Equal);
        assert_eq!(compare_version_strings("01.2", "1.2"), Ordering::Equal);
        assert_eq!(compare_version_strings("1.09", "1.10"), Ordering::Less);

        // Leading zeros in non-numeric context should be lexicographic
        assert_eq!(
            compare_version_strings("file-01", "file-1"),
            Ordering::Equal
        ); // Both parse as "file" + "-" + "01"/"1" -> numeric 1
    }

    #[test]
    fn test_compare_version_strings_empty_segments() {
        // Empty segments should be treated as empty strings
        assert_eq!(compare_version_strings("1..2", "1.0.2"), Ordering::Less); // empty string < "0"
        assert_eq!(compare_version_strings("1.", "1.0"), Ordering::Less);
        assert_eq!(compare_version_strings(".1", "0.1"), Ordering::Less); // empty string < "0" lexicographically
        assert_eq!(compare_version_strings("", "0"), Ordering::Less);
        assert_eq!(compare_version_strings("", ""), Ordering::Equal);
    }

    #[test]
    fn test_compare_version_strings_mixed_separators() {
        // Test different separators (., -, _)
        assert_eq!(compare_version_strings("1.2-3", "1.2.3"), Ordering::Equal);
        assert_eq!(compare_version_strings("1_2_3", "1.2.3"), Ordering::Equal);
        assert_eq!(
            compare_version_strings("v1-2_3.4", "v1.2.3.4"),
            Ordering::Equal
        );
        assert_eq!(
            compare_version_strings("file-1.10.txt", "file-1.2.txt"),
            Ordering::Greater
        );
    }

    #[test]
    fn test_compare_version_strings_complex_real_world_cases() {
        // Real-world version string scenarios
        assert_eq!(
            compare_version_strings("v2.1.0", "v2.0.9"),
            Ordering::Greater
        );
        assert_eq!(
            compare_version_strings("release-1.10.5", "release-1.9.20"),
            Ordering::Greater
        );
        assert_eq!(
            compare_version_strings("build_2023.12.01", "build_2023.11.30"),
            Ordering::Greater
        );
        assert_eq!(
            compare_version_strings("snapshot-1.0", "release-1.0"),
            Ordering::Greater
        ); // "s" > "r"

        // File names with version-like patterns
        assert_eq!(
            compare_version_strings("document_v1.10.pdf", "document_v1.2.pdf"),
            Ordering::Greater
        );
        assert_eq!(
            compare_version_strings("backup-2023.12.01.tar.gz", "backup-2023.11.30.tar.gz"),
            Ordering::Greater
        );
    }

    #[test]
    fn test_compare_version_strings_edge_cases() {
        // Very large numbers (testing u64 parsing)
        assert_eq!(
            compare_version_strings("1.999999999999999999", "1.1000000000000000000"),
            Ordering::Less
        );

        // Numbers that would overflow - should fall back to lexicographic
        assert_eq!(
            compare_version_strings("1.99999999999999999999999999999", "1.2"),
            Ordering::Greater
        ); // Lexicographic

        // Mixed valid and invalid numbers in same string
        assert_eq!(
            compare_version_strings("1.abc.10", "1.abc.2"),
            Ordering::Greater
        );
        assert_eq!(
            compare_version_strings("1.2.abc", "1.10.abc"),
            Ordering::Less
        );
    }

    #[test]
    fn test_compare_version_strings_consistency() {
        // Test transitivity: if a > b and b > c, then a > c
        let versions = ["1.2", "1.10", "2.0"];
        for i in 0..versions.len() {
            for j in 0..versions.len() {
                for k in 0..versions.len() {
                    let ord_ij = compare_version_strings(versions[i], versions[j]);
                    let ord_jk = compare_version_strings(versions[j], versions[k]);
                    let ord_ik = compare_version_strings(versions[i], versions[k]);

                    // If i > j and j > k, then i > k
                    if ord_ij == Ordering::Greater && ord_jk == Ordering::Greater {
                        assert_eq!(
                            ord_ik,
                            Ordering::Greater,
                            "Transitivity failed: {} > {} > {} but {} not > {}",
                            versions[i],
                            versions[j],
                            versions[k],
                            versions[i],
                            versions[k]
                        );
                    }
                }
            }
        }

        // Test reflexivity: a == a
        for version in &versions {
            assert_eq!(compare_version_strings(version, version), Ordering::Equal);
        }

        // Test antisymmetry: if a <= b and b <= a, then a == b
        for version_a in &versions {
            for version_b in &versions {
                let ord_ab = compare_version_strings(version_a, version_b);
                let ord_ba = compare_version_strings(version_b, version_a);

                if ord_ab != Ordering::Greater && ord_ba != Ordering::Greater {
                    assert_eq!(ord_ab, Ordering::Equal);
                    assert_eq!(ord_ba, Ordering::Equal);
                }
            }
        }
    }

    #[test]
    fn test_apply_directory_file_ordering_dirs_first() {
        use crate::core::tree::builder::TempNode;
        use crate::core::tree::node::{NodeInfo, NodeType};
        use std::path::PathBuf;

        // Create test nodes
        let file_node = TempNode {
            node_info: NodeInfo {
                name: "file.txt".to_string(),
                path: PathBuf::from("file.txt"),
                node_type: NodeType::File,
                depth: 1,
                size: Some(100),
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        let dir_node = TempNode {
            node_info: NodeInfo {
                name: "dir".to_string(),
                path: PathBuf::from("dir"),
                node_type: NodeType::Directory,
                depth: 1,
                size: None,
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        let symlink_node = TempNode {
            node_info: NodeInfo {
                name: "symlink".to_string(),
                path: PathBuf::from("symlink"),
                node_type: NodeType::Symlink,
                depth: 1,
                size: Some(50),
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        // Test DirsFirst ordering
        assert_eq!(
            apply_directory_file_ordering(&dir_node, &file_node, &DirectoryFileOrder::DirsFirst),
            Some(Ordering::Less)
        );
        assert_eq!(
            apply_directory_file_ordering(&file_node, &dir_node, &DirectoryFileOrder::DirsFirst),
            Some(Ordering::Greater)
        );
        assert_eq!(
            apply_directory_file_ordering(&dir_node, &symlink_node, &DirectoryFileOrder::DirsFirst),
            Some(Ordering::Less)
        );
        assert_eq!(
            apply_directory_file_ordering(&symlink_node, &dir_node, &DirectoryFileOrder::DirsFirst),
            Some(Ordering::Greater)
        );

        // Same types should return None
        assert_eq!(
            apply_directory_file_ordering(
                &file_node,
                &symlink_node,
                &DirectoryFileOrder::DirsFirst
            ),
            None
        );
        assert_eq!(
            apply_directory_file_ordering(&dir_node, &dir_node, &DirectoryFileOrder::DirsFirst),
            None
        );
    }

    #[test]
    fn test_apply_directory_file_ordering_files_first() {
        use crate::core::tree::builder::TempNode;
        use crate::core::tree::node::{NodeInfo, NodeType};
        use std::path::PathBuf;

        let file_node = TempNode {
            node_info: NodeInfo {
                name: "file.txt".to_string(),
                path: PathBuf::from("file.txt"),
                node_type: NodeType::File,
                depth: 1,
                size: Some(100),
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        let dir_node = TempNode {
            node_info: NodeInfo {
                name: "dir".to_string(),
                path: PathBuf::from("dir"),
                node_type: NodeType::Directory,
                depth: 1,
                size: None,
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        let symlink_node = TempNode {
            node_info: NodeInfo {
                name: "symlink".to_string(),
                path: PathBuf::from("symlink"),
                node_type: NodeType::Symlink,
                depth: 1,
                size: Some(50),
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        // Test FilesFirst ordering
        assert_eq!(
            apply_directory_file_ordering(&file_node, &dir_node, &DirectoryFileOrder::FilesFirst),
            Some(Ordering::Less)
        );
        assert_eq!(
            apply_directory_file_ordering(&dir_node, &file_node, &DirectoryFileOrder::FilesFirst),
            Some(Ordering::Greater)
        );
        assert_eq!(
            apply_directory_file_ordering(
                &symlink_node,
                &dir_node,
                &DirectoryFileOrder::FilesFirst
            ),
            Some(Ordering::Less)
        );
        assert_eq!(
            apply_directory_file_ordering(
                &dir_node,
                &symlink_node,
                &DirectoryFileOrder::FilesFirst
            ),
            Some(Ordering::Greater)
        );

        // Same types should return None
        assert_eq!(
            apply_directory_file_ordering(
                &file_node,
                &symlink_node,
                &DirectoryFileOrder::FilesFirst
            ),
            None
        );
        assert_eq!(
            apply_directory_file_ordering(&dir_node, &dir_node, &DirectoryFileOrder::FilesFirst),
            None
        );
    }

    #[test]
    fn test_apply_directory_file_ordering_default() {
        use crate::core::tree::builder::TempNode;
        use crate::core::tree::node::{NodeInfo, NodeType};
        use std::path::PathBuf;

        let file_node = TempNode {
            node_info: NodeInfo {
                name: "file.txt".to_string(),
                path: PathBuf::from("file.txt"),
                node_type: NodeType::File,
                depth: 1,
                size: Some(100),
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        let dir_node = TempNode {
            node_info: NodeInfo {
                name: "dir".to_string(),
                path: PathBuf::from("dir"),
                node_type: NodeType::Directory,
                depth: 1,
                size: None,
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        // Default ordering should always return None (no preference)
        assert_eq!(
            apply_directory_file_ordering(&file_node, &dir_node, &DirectoryFileOrder::Default),
            None
        );
        assert_eq!(
            apply_directory_file_ordering(&dir_node, &file_node, &DirectoryFileOrder::Default),
            None
        );
    }

    #[test]
    fn test_compare_siblings_with_options_directory_ordering() {
        use crate::core::tree::builder::TempNode;
        use crate::core::tree::node::{NodeInfo, NodeType};
        use std::path::PathBuf;

        let file_node = TempNode {
            node_info: NodeInfo {
                name: "zfile.txt".to_string(), // Name that would come after directory alphabetically
                path: PathBuf::from("zfile.txt"),
                node_type: NodeType::File,
                depth: 1,
                size: Some(100),
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        let dir_node = TempNode {
            node_info: NodeInfo {
                name: "adir".to_string(), // Name that would come before file alphabetically
                path: PathBuf::from("adir"),
                node_type: NodeType::Directory,
                depth: 1,
                size: None,
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        // Test that DirsFirst overrides alphabetical ordering
        let dirs_first_options = SortingOptions {
            sort_by: Some(SortKey::Name),
            reverse_sort: false,
            files_before_directories: true,
            directory_file_order: DirectoryFileOrder::DirsFirst,
        };

        assert_eq!(
            compare_siblings_with_options(&dir_node, &file_node, &dirs_first_options),
            Ordering::Less
        );
        assert_eq!(
            compare_siblings_with_options(&file_node, &dir_node, &dirs_first_options),
            Ordering::Greater
        );

        // Test that FilesFirst overrides alphabetical ordering
        let files_first_options = SortingOptions {
            sort_by: Some(SortKey::Name),
            reverse_sort: false,
            files_before_directories: true,
            directory_file_order: DirectoryFileOrder::FilesFirst,
        };

        assert_eq!(
            compare_siblings_with_options(&file_node, &dir_node, &files_first_options),
            Ordering::Less
        );
        assert_eq!(
            compare_siblings_with_options(&dir_node, &file_node, &files_first_options),
            Ordering::Greater
        );

        // Test that Default uses alphabetical ordering
        let default_options = SortingOptions {
            sort_by: Some(SortKey::Name),
            reverse_sort: false,
            files_before_directories: true,
            directory_file_order: DirectoryFileOrder::Default,
        };

        assert_eq!(
            compare_siblings_with_options(&dir_node, &file_node, &default_options),
            Ordering::Less // "adir" < "zfile.txt"
        );
        assert_eq!(
            compare_siblings_with_options(&file_node, &dir_node, &default_options),
            Ordering::Greater // "zfile.txt" > "adir"
        );
    }

    #[test]
    fn test_directory_ordering_with_reverse_sort() {
        use crate::core::tree::builder::TempNode;
        use crate::core::tree::node::{NodeInfo, NodeType};
        use std::path::PathBuf;

        let file_node = TempNode {
            node_info: NodeInfo {
                name: "file.txt".to_string(),
                path: PathBuf::from("file.txt"),
                node_type: NodeType::File,
                depth: 1,
                size: Some(100),
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        let dir_node = TempNode {
            node_info: NodeInfo {
                name: "dir".to_string(),
                path: PathBuf::from("dir"),
                node_type: NodeType::Directory,
                depth: 1,
                size: None,
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        // Test DirsFirst with reverse sort
        let dirs_first_reverse_options = SortingOptions {
            sort_by: Some(SortKey::Name),
            reverse_sort: true,
            files_before_directories: true,
            directory_file_order: DirectoryFileOrder::DirsFirst,
        };

        // With reverse sort, directory/file ordering is NOT reversed, only the sort key comparison
        assert_eq!(
            compare_siblings_with_options(&dir_node, &file_node, &dirs_first_reverse_options),
            Ordering::Less
        );
        assert_eq!(
            compare_siblings_with_options(&file_node, &dir_node, &dirs_first_reverse_options),
            Ordering::Greater
        );
    }

    #[test]
    fn test_directory_ordering_skipped_for_none_sort() {
        use crate::core::tree::builder::TempNode;
        use crate::core::tree::node::{NodeInfo, NodeType};
        use std::path::PathBuf;

        let file_node = TempNode {
            node_info: NodeInfo {
                name: "file.txt".to_string(),
                path: PathBuf::from("file.txt"),
                node_type: NodeType::File,
                depth: 1,
                size: Some(100),
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        let dir_node = TempNode {
            node_info: NodeInfo {
                name: "dir".to_string(),
                path: PathBuf::from("dir"),
                node_type: NodeType::Directory,
                depth: 1,
                size: None,
                permissions: None,
                mtime: None,
                change_time: None,
                create_time: None,
                word_count: None,
                line_count: None,
                custom_function_output: None,
            },
            children: Vec::new(),
        };

        // When sort_by is None, directory ordering should be skipped
        let none_sort_options = SortingOptions {
            sort_by: None,
            reverse_sort: false,
            files_before_directories: true,
            directory_file_order: DirectoryFileOrder::DirsFirst,
        };

        assert_eq!(
            compare_siblings_with_options(&dir_node, &file_node, &none_sort_options),
            Ordering::Equal
        );
        assert_eq!(
            compare_siblings_with_options(&file_node, &dir_node, &none_sort_options),
            Ordering::Equal
        );
    }
}
