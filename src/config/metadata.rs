use thiserror::Error;

/// Errors that can occur when applying a function to file content.
use serde::Serialize;

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ApplyFnError {
    /// Indicates that the function execution or calculation failed.
    #[error("Function calculation failed: {0}")]
    CalculationFailed(String),
    /// External command failed or exited non-zero
    #[error("External command failed: {0}")]
    Execution(String),
    /// External command exceeded the configured timeout
    #[error("External command timed out")]
    Timeout,
    // Add other specific error types for apply functions if needed
}

/// Describes the type of value produced by an apply-function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionOutputKind {
    /// Arbitrary string; aggregator will not attempt numeric processing.
    Text,
    /// An integer representing a count or other unit-less number.
    Number,
    /// An integer representing bytes. Aggregator will show human readable size.
    Bytes,
}

/// Configuration describing an external command-based function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalFunction {
    pub cmd_template: String,
    pub timeout_secs: u64,
    pub kind: FunctionOutputKind,
}

/// Enumerates built-in functions that can be applied to file and directory contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltInFunction {
    // File functions
    /// Counts the occurrences of the '+' character in the content.
    CountPluses,
    /// Displays the content of each file.
    Cat,

    // Directory functions
    /// Counts the number of files (non-directories) in the directory.
    CountFiles,
    /// Counts the number of subdirectories in the directory.
    CountDirs,
    /// Calculates the total size of all contents recursively.
    SizeTotal,
    /// Shows combined statistics for the directory (files, dirs, total size).
    DirStats,
}

impl BuiltInFunction {
    /// Returns the kind of output this built-in produces, used by the aggregator.
    pub fn output_kind(&self) -> FunctionOutputKind {
        match self {
            BuiltInFunction::CountPluses => FunctionOutputKind::Number,
            BuiltInFunction::Cat => FunctionOutputKind::Text,
            BuiltInFunction::CountFiles => FunctionOutputKind::Number,
            BuiltInFunction::CountDirs => FunctionOutputKind::Number,
            BuiltInFunction::SizeTotal => FunctionOutputKind::Bytes,
            BuiltInFunction::DirStats => FunctionOutputKind::Text,
        }
    }
}

/// Configuration for metadata collection and display.
#[derive(Debug, Clone, Default)]
pub struct MetadataOptions {
    /// Whether to report file and directory sizes.
    pub show_size_bytes: bool,
    /// Whether to format sizes in a human-readable form (e.g. "1.2 KB" instead of raw bytes).
    /// This flag has an effect only when `show_size_bytes` is `true`.
    pub human_readable_size: bool,
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

    /// Optional external command to run for each file. Mutually exclusive with
    /// `apply_function`.
    pub external_function: Option<ExternalFunction>,
}
