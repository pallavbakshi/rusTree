pub mod order;

/// Defines the possible keys for sorting directory entries via the CLI.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliSortKey {
    /// Sort by entry name.
    Name,
    /// Sort by file size.
    Size,
    /// Sort by last modification time.
    MTime,
    /// Sort by word count (for files).
    Words,
    /// Sort by line count (for files).
    Lines,
    /// Sort by the output of a custom applied function.
    Custom,
}
