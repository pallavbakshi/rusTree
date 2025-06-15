/// Configuration for directory listing behaviour.
///
/// This is a verbatim copy of the original `src/config/listing.rs` file,
/// moved into the *core* layer so that it can be used without depending on
/// the higher-level configuration module.

#[derive(Debug, Clone, Default)]
pub struct ListingOptions {
    /// Maximum depth to recurse into sub-directories. `None` means unlimited.
    pub max_depth: Option<usize>,
    /// Whether to show hidden files and directories (those starting with '.').
    pub show_hidden: bool,
    /// Whether to list only directories, excluding files.
    pub list_directories_only: bool,
    /// Whether to show the full relative path for each file/directory.
    pub show_full_path: bool,
}
