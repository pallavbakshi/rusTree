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