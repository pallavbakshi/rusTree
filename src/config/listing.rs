/// Configuration for directory listing behavior.
#[derive(Debug, Clone, Default)]
pub struct ListingOptions {
    /// Maximum depth to recurse into subdirectories. None means unlimited.
    pub max_depth: Option<usize>,
    /// Whether to show hidden files and directories (those starting with '.').
    pub show_hidden: bool,
    /// Whether to list only directories, excluding files.
    pub list_directories_only: bool,
}
