// src/core/analyzer/apply_fn.rs
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ApplyFnError {
    #[error("Function calculation failed: {0}")]
    CalculationFailed(String),
    // Add other specific error types for apply functions if needed
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltInFunction {
    CountPluses,
    // Add other built-in functions here
}

pub fn apply_function_to_content(content: &str, func: &BuiltInFunction) -> Result<String, ApplyFnError> {
    match func {
        BuiltInFunction::CountPluses => {
            let count = content.chars().filter(|&c| c == '+').count();
            Ok(count.to_string())
        }
    }
}