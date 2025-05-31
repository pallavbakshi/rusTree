pub mod order;

/// Defines the possible keys for sorting directory entries via the CLI.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliSortKey {
    /// Sort by entry name.
    Name,
    /// Sort by version string (e.g., `file_v1.0.txt` before `file_v2.0.txt`).
    Version,
    /// Sort by file size.
    Size,
    /// Sort by last modification time.
    #[value(name = "mod_time", alias = "m")]
    MTime,
    /// Sort by last status change time.
    #[value(name = "change_time", alias = "c")]
    ChangeTime,
    /// Sort by creation time.
    #[value(name = "create_time", alias = "cr")]
    CreateTime,
    /// Sort by word count (for files).
    Words,
    /// Sort by line count (for files).
    Lines,
    /// Sort by the output of a custom applied function.
    Custom,
    /// No sorting; preserve directory order.
    #[value(name = "none", alias = "n")]
    None,
}
