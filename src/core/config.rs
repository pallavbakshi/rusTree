// src/core/config.rs
use crate::core::sorter::SortKey;
use crate::core::analyzer::apply_fn::BuiltInFunction;
// Potentially other necessary imports, e.g., for ignore patterns

#[derive(Debug, Clone)]
pub struct RustreeLibConfig {
    pub root_display_name: String, // Added for FR1
    pub max_depth: Option<usize>,
    pub show_hidden: bool,
    // pub ignore_patterns: Vec<String>, // Or compiled regex/glob patterns
    pub report_sizes: bool,
    pub report_permissions: bool, // New consideration
    pub report_mtime: bool,       // New consideration

    pub calculate_line_count: bool,
    pub calculate_word_count: bool,
    pub apply_function: Option<BuiltInFunction>, // Enum for specific built-in fns

    pub sort_by: Option<SortKey>,
    pub reverse_sort: bool,
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
        }
    }
}