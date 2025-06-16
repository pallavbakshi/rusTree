//! # Context-Aware Error Messages
//!
//! This module provides enhanced error messages that reference specific context
//! fields and provide helpful guidance for fixing configuration issues. Instead
//! of generic error messages, these errors help users understand exactly what
//! went wrong and how to fix it.

use std::fmt;

/// A context-aware validation error that provides specific guidance
#[derive(Debug, Clone)]
pub struct ContextValidationError {
    /// The specific field or setting that caused the error
    pub field_path: String,
    /// The actual value that was invalid
    pub invalid_value: String,
    /// A description of what went wrong
    pub error_description: String,
    /// Suggested fix or valid alternatives
    pub suggestion: Option<String>,
    /// Context type where the error occurred
    pub context_type: ContextType,
}

/// Types of contexts where errors can occur
#[derive(Debug, Clone, PartialEq)]
pub enum ContextType {
    Walking,
    Formatting,
    Sorting,
    Processing,
    Async,
}

impl fmt::Display for ContextType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextType::Walking => write!(f, "WalkingContext"),
            ContextType::Formatting => write!(f, "FormattingContext"),
            ContextType::Sorting => write!(f, "SortingContext"),
            ContextType::Processing => write!(f, "ProcessingContext"),
            ContextType::Async => write!(f, "AsyncContext"),
        }
    }
}

impl ContextValidationError {
    /// Create a new context validation error
    pub fn new(
        field_path: impl Into<String>,
        invalid_value: impl Into<String>,
        error_description: impl Into<String>,
        context_type: ContextType,
    ) -> Self {
        Self {
            field_path: field_path.into(),
            invalid_value: invalid_value.into(),
            error_description: error_description.into(),
            suggestion: None,
            context_type,
        }
    }

    /// Add a suggestion for fixing the error
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Create an error for an invalid max_depth value
    pub fn invalid_max_depth(value: u32, context_type: ContextType) -> Self {
        Self::new(
            "listing.max_depth",
            value.to_string(),
            "max_depth cannot be 0",
            context_type,
        )
        .with_suggestion(
            "Use None for unlimited depth, or a positive integer like Some(1), Some(2), etc.",
        )
    }

    /// Create an error for invalid file size range
    pub fn invalid_file_size_range(min: u64, max: u64, context_type: ContextType) -> Self {
        Self::new(
            "filtering.min_file_size/max_file_size",
            format!("min={}, max={}", min, max),
            "min_file_size cannot be greater than max_file_size",
            context_type,
        )
        .with_suggestion(format!(
            "Set min_file_size to a value less than or equal to {} bytes, or increase max_file_size",
            max
        ))
    }

    /// Create an error for empty patterns
    pub fn empty_pattern(pattern_type: &str, context_type: ContextType) -> Self {
        Self::new(
            format!("filtering.{}", pattern_type),
            "\"\" (empty string)".to_string(),
            format!("{} cannot contain empty strings", pattern_type),
            context_type,
        )
        .with_suggestion("Remove empty strings from the pattern list, or use meaningful glob patterns like '*.tmp'")
    }

    /// Create an error for invalid URL
    pub fn invalid_url(field: &str, url: &str, context_type: ContextType) -> Self {
        Self::new(
            format!("html.{}", field),
            url.to_string(),
            "URL is not in a valid format",
            context_type,
        )
        .with_suggestion(
            "Use a valid URL starting with http://, https://, or a relative path like ./docs",
        )
    }

    /// Create an error for missing file
    pub fn missing_file(field: &str, file_path: &str, context_type: ContextType) -> Self {
        Self::new(
            format!("html.{}", field),
            file_path.to_string(),
            "File does not exist",
            context_type,
        )
        .with_suggestion(format!(
            "Create the file at '{}' or update the path to point to an existing file",
            file_path
        ))
    }

    /// Create an error for empty root display name
    pub fn empty_root_display_name(context_type: ContextType) -> Self {
        Self::new(
            "input_source.root_display_name",
            "\"\" (empty or whitespace)".to_string(),
            "root_display_name cannot be empty or contain only whitespace",
            context_type,
        )
        .with_suggestion(
            "Set root_display_name to a meaningful name like the directory name or project name",
        )
    }

    /// Create an error for inconsistent metadata requirements
    pub fn inconsistent_metadata(
        walking_field: &str,
        formatting_field: &str,
        feature: &str,
    ) -> Self {
        Self::new(
            format!("walking.{} vs formatting.{}", walking_field, formatting_field),
            "walking=false, formatting=true".to_string(),
            format!("Formatting context requires {} display but walking context doesn't collect it", feature),
            ContextType::Processing,
        )
        .with_suggestion(format!(
            "Either set walking.{} = true to collect the data, or set formatting.{} = false to not display it",
            walking_field, formatting_field
        ))
    }

    /// Create an error for inconsistent depth settings
    pub fn inconsistent_depth(walking_depth: u32, formatting_depth: u32) -> Self {
        Self::new(
            "walking.listing.max_depth vs formatting.listing.max_depth",
            format!("walking={}, formatting={}", walking_depth, formatting_depth),
            "Formatting context requests more depth than walking context provides",
            ContextType::Processing,
        )
        .with_suggestion(format!(
            "Either increase walking max_depth to at least {}, or reduce formatting max_depth to {}",
            formatting_depth, walking_depth
        ))
    }

    /// Create an error for pattern compilation failure
    pub fn pattern_compilation_failed(
        pattern: &str,
        error: &str,
        context_type: ContextType,
    ) -> Self {
        Self::new(
            "filtering patterns",
            pattern.to_string(),
            format!("Failed to compile glob pattern: {}", error),
            context_type,
        )
        .with_suggestion("Check that the pattern uses valid glob syntax. Examples: '*.txt', '**/target/**', '!*.rs'")
    }
}

impl fmt::Display for ContextValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} validation error in {}: {}",
            self.context_type, self.field_path, self.error_description
        )?;

        if !self.invalid_value.is_empty() {
            write!(f, " (value: {})", self.invalid_value)?;
        }

        if let Some(ref suggestion) = self.suggestion {
            write!(f, "\n  Suggestion: {}", suggestion)?;
        }

        Ok(())
    }
}

impl std::error::Error for ContextValidationError {}

impl From<ContextValidationError> for String {
    fn from(err: ContextValidationError) -> String {
        err.to_string()
    }
}

/// A collection of validation errors for a context
#[derive(Debug, Clone)]
pub struct ContextValidationErrors {
    pub errors: Vec<ContextValidationError>,
    pub context_type: ContextType,
}

impl ContextValidationErrors {
    /// Create a new error collection
    pub fn new(context_type: ContextType) -> Self {
        Self {
            errors: Vec::new(),
            context_type,
        }
    }

    /// Add an error to the collection
    pub fn add_error(&mut self, error: ContextValidationError) {
        self.errors.push(error);
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get the number of errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Convert to a Result, returning the first error if any exist
    pub fn into_result(self) -> Result<(), ContextValidationError> {
        if let Some(first_error) = self.errors.into_iter().next() {
            Err(first_error)
        } else {
            Ok(())
        }
    }

    /// Convert to a Result with a combined error message
    pub fn into_combined_result(self) -> Result<(), String> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            let combined_message = self
                .errors
                .iter()
                .enumerate()
                .map(|(i, error)| format!("{}. {}", i + 1, error))
                .collect::<Vec<_>>()
                .join("\n");

            Err(format!(
                "{} validation failed with {} error(s):\n{}",
                self.context_type,
                self.errors.len(),
                combined_message
            ))
        }
    }
}

impl fmt::Display for ContextValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            write!(f, "No validation errors")
        } else {
            write!(
                f,
                "{} validation errors in {}:",
                self.errors.len(),
                self.context_type
            )?;
            for (i, error) in self.errors.iter().enumerate() {
                write!(f, "\n  {}. {}", i + 1, error)?;
            }
            Ok(())
        }
    }
}

impl std::error::Error for ContextValidationErrors {}

/// Helper trait for context validation with enhanced error reporting
pub trait ContextValidation {
    /// The context type for error reporting
    const CONTEXT_TYPE: ContextType;

    /// Validate the context and collect all errors
    fn validate_with_errors(&self) -> ContextValidationErrors;

    /// Validate the context and return the first error
    fn validate(&self) -> Result<(), String> {
        self.validate_with_errors().into_combined_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_validation_error_creation() {
        let error = ContextValidationError::invalid_max_depth(0, ContextType::Walking);

        assert_eq!(error.field_path, "listing.max_depth");
        assert_eq!(error.invalid_value, "0");
        assert_eq!(error.context_type, ContextType::Walking);
        assert!(error.suggestion.is_some());
        assert!(error.suggestion.unwrap().contains("positive integer"));
    }

    #[test]
    fn test_context_validation_error_display() {
        let error = ContextValidationError::invalid_max_depth(0, ContextType::Walking);
        let error_string = error.to_string();

        assert!(error_string.contains("WalkingContext"));
        assert!(error_string.contains("listing.max_depth"));
        assert!(error_string.contains("max_depth cannot be 0"));
        assert!(error_string.contains("Suggestion:"));
    }

    #[test]
    fn test_context_validation_errors_collection() {
        let mut errors = ContextValidationErrors::new(ContextType::Formatting);

        assert!(!errors.has_errors());
        assert_eq!(errors.error_count(), 0);

        errors.add_error(ContextValidationError::empty_root_display_name(
            ContextType::Formatting,
        ));
        errors.add_error(ContextValidationError::invalid_url(
            "base_href",
            "invalid-url",
            ContextType::Formatting,
        ));

        assert!(errors.has_errors());
        assert_eq!(errors.error_count(), 2);

        let result = errors.into_combined_result();
        assert!(result.is_err());
        let error_message = result.unwrap_err();
        assert!(error_message.contains("FormattingContext validation failed"));
        assert!(error_message.contains("2 error(s)"));
    }

    #[test]
    fn test_context_types_display() {
        assert_eq!(ContextType::Walking.to_string(), "WalkingContext");
        assert_eq!(ContextType::Formatting.to_string(), "FormattingContext");
        assert_eq!(ContextType::Sorting.to_string(), "SortingContext");
        assert_eq!(ContextType::Processing.to_string(), "ProcessingContext");
        assert_eq!(ContextType::Async.to_string(), "AsyncContext");
    }

    #[test]
    fn test_specific_error_constructors() {
        let file_size_error =
            ContextValidationError::invalid_file_size_range(1000, 100, ContextType::Walking);
        assert!(
            file_size_error
                .suggestion
                .unwrap()
                .contains("Set min_file_size")
        );

        let empty_pattern_error =
            ContextValidationError::empty_pattern("ignore_patterns", ContextType::Walking);
        assert!(
            empty_pattern_error
                .suggestion
                .unwrap()
                .contains("Remove empty strings")
        );

        let invalid_url_error =
            ContextValidationError::invalid_url("base_href", "invalid", ContextType::Formatting);
        assert!(invalid_url_error.suggestion.unwrap().contains("valid URL"));

        let missing_file_error = ContextValidationError::missing_file(
            "custom_intro",
            "/nonexistent",
            ContextType::Formatting,
        );
        assert!(
            missing_file_error
                .suggestion
                .unwrap()
                .contains("Create the file")
        );

        let metadata_error = ContextValidationError::inconsistent_metadata(
            "show_size_bytes",
            "show_size_bytes",
            "size",
        );
        assert_eq!(metadata_error.context_type, ContextType::Processing);

        let depth_error = ContextValidationError::inconsistent_depth(2, 5);
        assert!(
            depth_error
                .suggestion
                .unwrap()
                .contains("increase walking max_depth")
        );

        let pattern_error = ContextValidationError::pattern_compilation_failed(
            "invalid[",
            "unclosed bracket",
            ContextType::Walking,
        );
        assert!(
            pattern_error
                .suggestion
                .unwrap()
                .contains("valid glob syntax")
        );
    }
}
