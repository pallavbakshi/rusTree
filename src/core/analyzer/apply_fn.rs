// src/core/analyzer/apply_fn.rs
use crate::config::metadata::{ApplyFnError, BuiltInFunction};

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
pub fn apply_function_to_content(
    content: &str,
    func: &BuiltInFunction,
) -> Result<String, ApplyFnError> {
    match func {
        BuiltInFunction::CountPluses => {
            let count = content.chars().filter(|&c| c == '+').count();
            Ok(count.to_string())
        }
    }
}
