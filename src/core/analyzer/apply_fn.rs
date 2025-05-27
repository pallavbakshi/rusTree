// src/core/analyzer/apply_fn.rs
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

/// Applies a specified built-in function to the given string content.
///
/// # Arguments
///
/// * `content` - The string content to process.
/// * `func` - The [`BuiltInFunction`] to apply.
///
/// # Returns
///
/// A `Result` containing the string representation of the function's output on success,
/// or an [`ApplyFnError`] on failure.
pub fn apply_function_to_content(content: &str, func: &BuiltInFunction) -> Result<String, ApplyFnError> {
    match func {
        BuiltInFunction::CountPluses => {
            let count = content.chars().filter(|&c| c == '+').count();
            Ok(count.to_string())
        }
    }
}