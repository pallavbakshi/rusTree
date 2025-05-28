// src/core/config.rs
use crate::core::sorter::SortKey;
use crate::core::analyzer::apply_fn::BuiltInFunction;
// Potentially other necessary imports, e.g., for ignore patterns

/// Configuration for the `rustree` library.
///
/// This struct holds all the options that control how `rustree` processes
/// and displays directory trees.
#[derive(Debug, Clone)]
pub struct RustreeLibConfig {
    /// The name to display for the root of the scanned directory.
    /// Typically the directory name itself or "." for the current directory.
    pub root_display_name: String,
    /// The maximum depth to traverse into the directory structure.
    /// `None` means no limit. `Some(0)` would effectively show only the root (if walker adapted).
    /// `Some(1)` shows root and its direct children.
    pub max_depth: Option<usize>,
    /// If `true`, hidden files and directories (those starting with a `.`) will be included.
    pub show_hidden: bool,
    // pub ignore_patterns: Vec<String>, // Future: For ignoring specific patterns
    /// If `true`, report the size of files.
    pub report_sizes: bool,
    /// If `true`, report file permissions (currently not implemented in output).
    pub report_permissions: bool,
    /// If `true`, report the last modification time of files and directories.
    pub report_mtime: bool,

    /// If `true`, calculate and report the number of lines for files.
    pub calculate_line_count: bool,
    /// If `true`, calculate and report the number of words for files.
    pub calculate_word_count: bool,
    /// Specifies a built-in function to apply to the content of each file.
    /// The result of this function can be displayed.
    pub apply_function: Option<BuiltInFunction>,

    /// The key by which to sort directory entries. `None` means default (usually OS-dependent or DFS order).
    pub sort_by: Option<SortKey>,
    /// If `true`, reverses the sort order specified by `sort_by`.
    pub reverse_sort: bool,
    /// If `true`, only directories will be listed.
    pub list_directories_only: bool,
    /// Optional size of the root node itself, used by formatters if `report_sizes` is true.
    pub root_node_size: Option<u64>,
    /// Indicates if the root path itself is a directory.
    pub root_is_directory: bool,
    // Add any other options the library logic needs
}

impl Default for RustreeLibConfig {
    fn default() -> Self {
        Self {
            root_display_name: String::new(), // Default to empty
            max_depth: None,
            show_hidden: false,
            report_sizes: false,
            report_permissions: false,
            report_mtime: false,
            calculate_line_count: false,
            calculate_word_count: false,
            apply_function: None,
            sort_by: None,
            reverse_sort: false,
            list_directories_only: false,
            root_node_size: None,
            root_is_directory: false, // Default to false, will be set by handler
        }
    }
}