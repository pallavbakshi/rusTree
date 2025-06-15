// src/core/diff/changes.rs

//! Data structures representing changes between tree snapshots.

use crate::core::tree::node::{NodeInfo, NodeType};
use serde::Serialize;
use std::path::PathBuf;

/// Represents a single change detected between two tree snapshots.
#[derive(Debug, Clone, Serialize)]
pub struct Change {
    /// The type of change detected
    pub change_type: ChangeType,
    /// The current node information (None for removed items)
    pub current: Option<NodeInfo>,
    /// The previous node information (None for added items)
    pub previous: Option<NodeInfo>,
    /// Child changes for modified directories
    pub children: Vec<Change>,
}

/// Types of changes that can be detected between snapshots.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ChangeType {
    /// File or directory added (exists in current but not in snapshot)
    Added,
    /// File or directory removed (exists in snapshot but not in current)
    Removed,
    /// Directory with changed contents
    Modified,
    /// File moved to a different location
    Moved {
        /// The path where the file was previously located
        from_path: PathBuf,
        /// Similarity score between 0.0 and 1.0
        similarity: f64,
    },
    /// Node type changed (e.g., file became directory)
    TypeChanged {
        /// The previous node type
        from_type: NodeType,
        /// The new node type
        to_type: NodeType,
    },
    /// File or directory unchanged
    Unchanged,
}

/// Summary statistics of all changes in a diff operation.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DiffSummary {
    /// Number of files/directories added
    pub added: usize,
    /// Number of files/directories removed
    pub removed: usize,
    /// Number of directories with modified contents
    pub modified: usize,
    /// Number of files moved/renamed
    pub moved: usize,
    /// Number of type changes
    pub type_changed: usize,
    /// Number of unchanged items
    pub unchanged: usize,
    /// Total size change in bytes (positive for growth, negative for shrinkage)
    pub size_change: i128,

    // Detailed breakdown by type
    /// Number of directories added
    pub directories_added: usize,
    /// Number of files added
    pub files_added: usize,
    /// Number of directories removed
    pub directories_removed: usize,
    /// Number of files removed
    pub files_removed: usize,
    /// Number of directories moved/renamed
    pub directories_moved: usize,
    /// Number of files moved/renamed
    pub files_moved: usize,
}

/// Metadata about the diff operation itself.
#[derive(Debug, Clone, Serialize)]
pub struct DiffMetadata {
    /// When the diff was generated
    pub generated_at: String,
    /// Path to the snapshot file used for comparison
    pub snapshot_file: PathBuf,
    /// When the snapshot was created (if available)
    pub snapshot_date: Option<String>,
    /// Root path used for comparison
    pub comparison_root: PathBuf,
    /// Filters that were applied
    pub filters_applied: Vec<String>,
    /// Other options used during diff
    pub options: DiffOptions,
}

/// Options that affect diff behavior.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DiffOptions {
    /// Maximum depth for comparison
    pub max_depth: Option<usize>,
    /// Whether to show file sizes
    pub show_size: bool,
    /// Sort key used
    pub sort_by: Option<String>,
    /// Whether to detect moves
    pub detect_moves: bool,
    /// Similarity threshold for move detection (0.0 to 1.0)
    pub move_threshold: f64,
    /// Whether to include unchanged files in output
    pub show_unchanged: bool,
    /// Whether to ignore moves
    pub ignore_moves: bool,
}

/// Complete result of a diff operation.
#[derive(Debug, Clone, Serialize)]
pub struct DiffResult {
    /// All detected changes
    pub changes: Vec<Change>,
    /// Summary statistics
    pub summary: DiffSummary,
    /// Metadata about the diff operation
    pub metadata: DiffMetadata,
}

impl Change {
    /// Creates a new Change with the given type.
    pub fn new(
        change_type: ChangeType,
        current: Option<NodeInfo>,
        previous: Option<NodeInfo>,
    ) -> Self {
        Self {
            change_type,
            current,
            previous,
            children: Vec::new(),
        }
    }

    /// Adds a child change (for modified directories).
    pub fn add_child(&mut self, child: Change) {
        self.children.push(child);
    }

    /// Gets the path of the change (from either current or previous).
    pub fn path(&self) -> &PathBuf {
        if let Some(ref current) = self.current {
            &current.path
        } else if let Some(ref previous) = self.previous {
            &previous.path
        } else {
            panic!("Change must have either current or previous NodeInfo");
        }
    }

    /// Gets the node type of the change.
    pub fn node_type(&self) -> NodeType {
        if let Some(ref current) = self.current {
            current.node_type.clone()
        } else if let Some(ref previous) = self.previous {
            previous.node_type.clone()
        } else {
            panic!("Change must have either current or previous NodeInfo");
        }
    }

    /// Checks if this represents a directory change.
    pub fn is_directory(&self) -> bool {
        self.node_type() == NodeType::Directory
    }

    /// Gets the size change for this item.
    pub fn size_change(&self) -> i128 {
        let current_size = self.current.as_ref().and_then(|n| n.size).unwrap_or(0) as i128;
        let previous_size = self.previous.as_ref().and_then(|n| n.size).unwrap_or(0) as i128;
        current_size - previous_size
    }
}

impl DiffSummary {
    /// Creates a new empty summary.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a change to the summary statistics.
    pub fn add_change(&mut self, change: &Change) {
        let is_directory = change.is_directory();

        match &change.change_type {
            ChangeType::Added => {
                self.added += 1;
                if is_directory {
                    self.directories_added += 1;
                } else {
                    self.files_added += 1;
                }
            }
            ChangeType::Removed => {
                self.removed += 1;
                if is_directory {
                    self.directories_removed += 1;
                } else {
                    self.files_removed += 1;
                }
            }
            ChangeType::Modified => self.modified += 1,
            ChangeType::Moved { .. } => {
                self.moved += 1;
                if is_directory {
                    self.directories_moved += 1;
                } else {
                    self.files_moved += 1;
                }
            }
            ChangeType::TypeChanged { .. } => self.type_changed += 1,
            ChangeType::Unchanged => self.unchanged += 1,
        }

        // Update size change
        self.size_change += change.size_change();

        // Recursively process children for modified directories
        if change.change_type == ChangeType::Modified {
            for child in &change.children {
                self.add_change(child);
            }
        }
    }

    /// Gets the total number of changes (excluding unchanged items).
    pub fn total_changes(&self) -> usize {
        self.added + self.removed + self.modified + self.moved + self.type_changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    fn create_test_node(name: &str, node_type: NodeType, size: Option<u64>) -> NodeInfo {
        NodeInfo {
            name: name.to_string(),
            path: PathBuf::from(name),
            node_type,
            depth: 0,
            size,
            mtime: Some(SystemTime::UNIX_EPOCH),
            change_time: None,
            create_time: None,
            permissions: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        }
    }

    #[test]
    fn test_change_new() {
        let node = create_test_node("test.txt", NodeType::File, Some(100));
        let change = Change::new(ChangeType::Added, Some(node.clone()), None);

        assert!(matches!(change.change_type, ChangeType::Added));
        assert!(change.current.is_some());
        assert!(change.previous.is_none());
        assert!(change.children.is_empty());
    }

    #[test]
    fn test_change_add_child() {
        let parent = create_test_node("parent", NodeType::Directory, None);
        let child = create_test_node("child.txt", NodeType::File, Some(50));

        let mut parent_change = Change::new(ChangeType::Modified, Some(parent), None);
        let child_change = Change::new(ChangeType::Added, Some(child), None);

        parent_change.add_child(child_change);
        assert_eq!(parent_change.children.len(), 1);
        assert!(matches!(
            parent_change.children[0].change_type,
            ChangeType::Added
        ));
    }

    #[test]
    fn test_change_path() {
        let node = create_test_node("test.txt", NodeType::File, Some(100));
        let change = Change::new(ChangeType::Added, Some(node), None);

        assert_eq!(change.path(), &PathBuf::from("test.txt"));
    }

    #[test]
    fn test_change_path_from_previous() {
        let node = create_test_node("deleted.txt", NodeType::File, Some(100));
        let change = Change::new(ChangeType::Removed, None, Some(node));

        assert_eq!(change.path(), &PathBuf::from("deleted.txt"));
    }

    #[test]
    fn test_change_node_type() {
        let node = create_test_node("test", NodeType::Directory, None);
        let change = Change::new(ChangeType::Added, Some(node), None);

        assert_eq!(change.node_type(), NodeType::Directory);
    }

    #[test]
    fn test_change_is_directory() {
        let file_node = create_test_node("file.txt", NodeType::File, Some(100));
        let dir_node = create_test_node("dir", NodeType::Directory, None);

        let file_change = Change::new(ChangeType::Added, Some(file_node), None);
        let dir_change = Change::new(ChangeType::Added, Some(dir_node), None);

        assert!(!file_change.is_directory());
        assert!(dir_change.is_directory());
    }

    #[test]
    fn test_change_size_change_added() {
        let node = create_test_node("new.txt", NodeType::File, Some(100));
        let change = Change::new(ChangeType::Added, Some(node), None);

        assert_eq!(change.size_change(), 100);
    }

    #[test]
    fn test_change_size_change_removed() {
        let node = create_test_node("deleted.txt", NodeType::File, Some(200));
        let change = Change::new(ChangeType::Removed, None, Some(node));

        assert_eq!(change.size_change(), -200);
    }

    #[test]
    fn test_change_size_change_modified() {
        let old_node = create_test_node("file.txt", NodeType::File, Some(100));
        let new_node = create_test_node("file.txt", NodeType::File, Some(150));
        let change = Change::new(ChangeType::Modified, Some(new_node), Some(old_node));

        assert_eq!(change.size_change(), 50);
    }

    #[test]
    fn test_change_type_moved() {
        let moved = ChangeType::Moved {
            from_path: PathBuf::from("old_location.txt"),
            similarity: 0.95,
        };

        assert!(matches!(moved, ChangeType::Moved { .. }));
        if let ChangeType::Moved {
            from_path,
            similarity,
        } = moved
        {
            assert_eq!(from_path, PathBuf::from("old_location.txt"));
            assert_eq!(similarity, 0.95);
        }
    }

    #[test]
    fn test_change_type_type_changed() {
        let type_changed = ChangeType::TypeChanged {
            from_type: NodeType::File,
            to_type: NodeType::Directory,
        };

        assert!(matches!(type_changed, ChangeType::TypeChanged { .. }));
        if let ChangeType::TypeChanged { from_type, to_type } = type_changed {
            assert_eq!(from_type, NodeType::File);
            assert_eq!(to_type, NodeType::Directory);
        }
    }

    #[test]
    fn test_diff_summary_new() {
        let summary = DiffSummary::new();

        assert_eq!(summary.added, 0);
        assert_eq!(summary.removed, 0);
        assert_eq!(summary.modified, 0);
        assert_eq!(summary.moved, 0);
        assert_eq!(summary.type_changed, 0);
        assert_eq!(summary.unchanged, 0);
        assert_eq!(summary.size_change, 0);
        assert_eq!(summary.total_changes(), 0);
    }

    #[test]
    fn test_diff_summary_add_file() {
        let mut summary = DiffSummary::new();
        let file_node = create_test_node("new.txt", NodeType::File, Some(100));
        let change = Change::new(ChangeType::Added, Some(file_node), None);

        summary.add_change(&change);

        assert_eq!(summary.added, 1);
        assert_eq!(summary.files_added, 1);
        assert_eq!(summary.directories_added, 0);
        assert_eq!(summary.size_change, 100);
        assert_eq!(summary.total_changes(), 1);
    }

    #[test]
    fn test_diff_summary_add_directory() {
        let mut summary = DiffSummary::new();
        let dir_node = create_test_node("new_dir", NodeType::Directory, None);
        let change = Change::new(ChangeType::Added, Some(dir_node), None);

        summary.add_change(&change);

        assert_eq!(summary.added, 1);
        assert_eq!(summary.files_added, 0);
        assert_eq!(summary.directories_added, 1);
        assert_eq!(summary.size_change, 0);
    }

    #[test]
    fn test_diff_summary_remove_file() {
        let mut summary = DiffSummary::new();
        let file_node = create_test_node("deleted.txt", NodeType::File, Some(200));
        let change = Change::new(ChangeType::Removed, None, Some(file_node));

        summary.add_change(&change);

        assert_eq!(summary.removed, 1);
        assert_eq!(summary.files_removed, 1);
        assert_eq!(summary.directories_removed, 0);
        assert_eq!(summary.size_change, -200);
    }

    #[test]
    fn test_diff_summary_move_file() {
        let mut summary = DiffSummary::new();
        let old_node = create_test_node("old.txt", NodeType::File, Some(100));
        let new_node = create_test_node("new.txt", NodeType::File, Some(100));
        let change = Change::new(
            ChangeType::Moved {
                from_path: PathBuf::from("old.txt"),
                similarity: 0.9,
            },
            Some(new_node),
            Some(old_node),
        );

        summary.add_change(&change);

        assert_eq!(summary.moved, 1);
        assert_eq!(summary.files_moved, 1);
        assert_eq!(summary.directories_moved, 0);
        assert_eq!(summary.size_change, 0);
    }

    #[test]
    fn test_diff_summary_type_changed() {
        let mut summary = DiffSummary::new();
        let old_node = create_test_node("item", NodeType::File, Some(100));
        let new_node = create_test_node("item", NodeType::Directory, None);
        let change = Change::new(
            ChangeType::TypeChanged {
                from_type: NodeType::File,
                to_type: NodeType::Directory,
            },
            Some(new_node),
            Some(old_node),
        );

        summary.add_change(&change);

        assert_eq!(summary.type_changed, 1);
        assert_eq!(summary.size_change, -100);
    }

    #[test]
    fn test_diff_summary_unchanged() {
        let mut summary = DiffSummary::new();
        let node = create_test_node("same.txt", NodeType::File, Some(100));
        let change = Change::new(ChangeType::Unchanged, Some(node.clone()), Some(node));

        summary.add_change(&change);

        assert_eq!(summary.unchanged, 1);
        assert_eq!(summary.size_change, 0);
        assert_eq!(summary.total_changes(), 0); // Unchanged doesn't count as a change
    }

    #[test]
    fn test_diff_summary_modified_with_children() {
        let mut summary = DiffSummary::new();
        let dir_node = create_test_node("modified_dir", NodeType::Directory, None);
        let mut dir_change = Change::new(ChangeType::Modified, Some(dir_node), None);

        // Add children to the modified directory
        let child1 = create_test_node("added.txt", NodeType::File, Some(50));
        let child2 = create_test_node("removed.txt", NodeType::File, Some(75));
        dir_change.add_child(Change::new(ChangeType::Added, Some(child1), None));
        dir_change.add_child(Change::new(ChangeType::Removed, None, Some(child2)));

        summary.add_change(&dir_change);

        assert_eq!(summary.modified, 1);
        assert_eq!(summary.added, 1);
        assert_eq!(summary.removed, 1);
        assert_eq!(summary.files_added, 1);
        assert_eq!(summary.files_removed, 1);
        assert_eq!(summary.size_change, -25); // +50 -75 = -25
        assert_eq!(summary.total_changes(), 3); // modified + added + removed
    }

    #[test]
    fn test_diff_options_default() {
        let options = DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        };

        assert!(options.detect_moves);
        assert_eq!(options.move_threshold, 0.8);
        assert!(!options.show_unchanged);
    }

    #[test]
    fn test_diff_metadata_creation() {
        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: Some("2024-01-01T00:00:00Z".to_string()),
            comparison_root: PathBuf::from("."),
            filters_applied: vec!["*.rs".to_string()],
            options: DiffOptions {
                max_depth: Some(3),
                show_size: true,
                sort_by: Some("name".to_string()),
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        assert_eq!(metadata.snapshot_file, PathBuf::from("test.json"));
        assert_eq!(metadata.filters_applied.len(), 1);
        assert_eq!(metadata.options.max_depth, Some(3));
    }

    #[test]
    fn test_diff_result_creation() {
        let file_node = create_test_node("test.txt", NodeType::File, Some(100));
        let change = Change::new(ChangeType::Added, Some(file_node), None);
        let mut summary = DiffSummary::new();
        summary.add_change(&change);

        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions {
                max_depth: None,
                show_size: true,
                sort_by: None,
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        let result = DiffResult {
            changes: vec![change],
            summary,
            metadata,
        };

        assert_eq!(result.changes.len(), 1);
        assert_eq!(result.summary.added, 1);
        assert_eq!(result.metadata.snapshot_file, PathBuf::from("test.json"));
    }
}
