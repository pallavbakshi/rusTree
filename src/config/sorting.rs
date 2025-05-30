/// Defines the keys by which directory entries can be sorted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortKey {
    /// Sort by entry name (alphabetically).
    Name,
    /// Sort by entry size.
    /// Files/symlinks are grouped before directories.
    /// Files/symlinks are sorted by size (then name). Directories by name.
    Size,
    /// Sort by last modification time (oldest to newest, then name).
    MTime,
    /// Sort by word count (files only, fewest to most, then name).
    Words,
    /// Sort by line count (files only, fewest to most, then name).
    Lines,
    /// Sort by the output of a custom applied function (then name).
    Custom,
} 