/// Defines the keys by which directory entries can be sorted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortKey {
    /// Sort by entry name (alphabetically).
    Name,
    /// Sort by version string (e.g., `file_v1.0.txt` before `file_v2.0.txt`).
    Version,
    /// Sort by entry size.
    /// Files/symlinks are grouped before directories.
    /// Files/symlinks are sorted by size (then name). Directories by name.
    Size,
    /// Sort by last modification time (oldest to newest, then name).
    MTime,
    /// Sort by last status change time (oldest to newest, then name).
    ChangeTime,
    /// Sort by creation time (oldest to newest, then name).
    CreateTime,
    /// Sort by word count (files only, fewest to most, then name).
    Words,
    /// Sort by line count (files only, fewest to most, then name).
    Lines,
    /// Sort by the output of a custom applied function (then name).
    Custom,
    /// No sorting; preserve directory traversal order.
    None,
}

/// Configuration for sorting behavior.
#[derive(Debug, Clone)]
pub struct SortingOptions {
    /// The key to sort by. None means no sorting (preserve directory traversal order).
    pub sort_by: Option<SortKey>,
    /// Whether to reverse the sort order.
    pub reverse_sort: bool,
    /// Whether to sort files before directories when sorting by size.
    /// When true (default), files and symlinks appear before directories.
    /// When false, files and directories are intermixed based purely on size.
    pub files_before_directories: bool,
}

impl Default for SortingOptions {
    fn default() -> Self {
        Self {
            sort_by: Some(SortKey::Name),
            reverse_sort: false,
            files_before_directories: true, // Default to traditional behavior
        }
    }
}
