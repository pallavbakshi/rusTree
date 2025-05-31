use thiserror::Error;

/// Errors that can occur when applying a function to file content.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ApplyFnError {
    /// Indicates that the function execution or calculation failed.
    #[error("Function calculation failed: {0}")]
    CalculationFailed(String),
    // Add other specific error types for apply functions if needed
}

/// Enumerates built-in functions that can be applied to file contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltInFunction {
    /// Counts the occurrences of the '+' character in the content.
    CountPluses,
    // Add other built-in functions here
}

/// Configuration for metadata collection and display.
#[derive(Debug, Clone, Default)]
pub struct MetadataOptions {
    /// Whether to report file and directory sizes.
    pub show_size_bytes: bool,
    /// Whether to report file permissions.
    pub report_permissions: bool,
    /// Whether to report last modification time.
    pub show_last_modified: bool,
    /// Whether to report last status change time (ctime).
    pub report_change_time: bool,
    /// Whether to report creation time (btime).
    pub report_creation_time: bool,
    /// Whether to calculate and report line counts for files.
    pub calculate_line_count: bool,
    /// Whether to calculate and report word counts for files.
    pub calculate_word_count: bool,
    /// Optional built-in function to apply to file contents.
    pub apply_function: Option<BuiltInFunction>,
}
