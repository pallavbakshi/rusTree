// src/core/config.rs
use crate::core::sorter::SortKey;
use crate::core::analyzer::apply_fn::BuiltInFunction;
// Potentially other necessary imports, e.g., for ignore patterns

#[derive(Debug, Clone, Default)] // Default can be useful
pub struct RustreeLibConfig {
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