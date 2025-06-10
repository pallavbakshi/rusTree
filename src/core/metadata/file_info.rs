//! File information processing and metadata extraction.
//!
//! This module provides utilities for extracting and processing file-specific
//! information and metadata, including content analysis and metadata formatting.

use crate::config::RustreeLibConfig;
use crate::config::metadata::{ApplyFnError, BuiltInFunction};
use crate::core::tree::node::{NodeInfo, NodeType};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents different styles for formatting metadata display.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetadataStyle {
    /// Text format with brackets: `[123B] [L:  45] [MTime: 1234567890s]`
    Text,
    /// Markdown format with backticks: ` `123B, 45L` `
    Markdown,
    /// Plain format without decorators: `123B 45L 1234567890s`
    Plain,
}

/// Formats metadata for a node according to the specified style and configuration.
///
/// This function consolidates all metadata formatting logic to ensure consistency
/// across different output formatters.
///
/// # Arguments
///
/// * `node` - The node whose metadata should be formatted
/// * `config` - Configuration specifying which metadata to include
/// * `style` - The formatting style to use
///
/// # Returns
///
/// A formatted string containing the requested metadata, or an empty string if
/// no metadata is configured to be displayed.
pub fn format_node_metadata(
    node: &NodeInfo,
    config: &RustreeLibConfig,
    style: MetadataStyle,
) -> String {
    let mut metadata_parts = Vec::new();

    // Size: applies to files and directories if config.show_size_bytes is true
    if config.metadata.show_size_bytes {
        if let Some(size) = node.size {
            match style {
                MetadataStyle::Text => metadata_parts.push(format!("[{:>7}B]", size)),
                MetadataStyle::Markdown | MetadataStyle::Plain => {
                    metadata_parts.push(format!("{}B", size))
                }
            }
        } else if style == MetadataStyle::Text {
            // Text format shows placeholders for missing data
            metadata_parts.push("[       B]".to_string());
        }
    }

    // Time metadata: applies to all node types if configured
    if config.metadata.show_last_modified {
        if let Some(formatted) = format_timestamp(node.mtime, "MTime", style) {
            metadata_parts.push(formatted);
        }
    }

    if config.metadata.report_change_time {
        if let Some(formatted) = format_timestamp(node.change_time, "CTime", style) {
            metadata_parts.push(formatted);
        }
    }

    if config.metadata.report_creation_time {
        if let Some(formatted) = format_timestamp(node.create_time, "BTime", style) {
            metadata_parts.push(formatted);
        }
    }

    // File-specific metadata: only show if the node is a file
    if node.node_type == NodeType::File {
        if config.metadata.calculate_line_count {
            if let Some(lc) = node.line_count {
                match style {
                    MetadataStyle::Text => metadata_parts.push(format!("[L:{:>4}]", lc)),
                    MetadataStyle::Markdown | MetadataStyle::Plain => {
                        metadata_parts.push(format!("{}L", lc))
                    }
                }
            } else if style == MetadataStyle::Text {
                metadata_parts.push("[L:    ]".to_string());
            }
        }

        if config.metadata.calculate_word_count {
            if let Some(wc) = node.word_count {
                match style {
                    MetadataStyle::Text => metadata_parts.push(format!("[W:{:>4}]", wc)),
                    MetadataStyle::Markdown | MetadataStyle::Plain => {
                        metadata_parts.push(format!("{}W", wc))
                    }
                }
            } else if style == MetadataStyle::Text {
                metadata_parts.push("[W:    ]".to_string());
            }
        }

        if config.metadata.apply_function.is_some() {
            // Don't display cat content in metadata since it's shown separately
            if let Some(BuiltInFunction::Cat) = &config.metadata.apply_function {
                // Skip displaying cat content in metadata
            } else {
                match &node.custom_function_output {
                    Some(Ok(val)) => match style {
                        MetadataStyle::Text => metadata_parts.push(format!("[F: \"{}\"]", val)),
                        MetadataStyle::Markdown | MetadataStyle::Plain => {
                            metadata_parts.push(format!("F:{}", val))
                        }
                    },
                    Some(Err(_)) => match style {
                        MetadataStyle::Text => metadata_parts.push("[F: error]".to_string()),
                        MetadataStyle::Markdown | MetadataStyle::Plain => {
                            metadata_parts.push("F:error".to_string())
                        }
                    },
                    None => {
                        if style == MetadataStyle::Text {
                            metadata_parts.push("[F: N/A]".to_string());
                        }
                    }
                }
            }
        }
    }

    // Format the final output based on style
    if metadata_parts.is_empty() {
        String::new()
    } else {
        match style {
            MetadataStyle::Text => {
                // Text style: parts separated by spaces, with a trailing space
                format!("{} ", metadata_parts.join(" "))
            }
            MetadataStyle::Markdown => {
                // Markdown style: parts in backticks, preceded by a space
                format!(" `{}`", metadata_parts.join(", "))
            }
            MetadataStyle::Plain => {
                // Plain style: parts separated by spaces
                metadata_parts.join(" ")
            }
        }
    }
}

/// Formats a timestamp for metadata display based on the specified style.
///
/// This helper function consolidates the logic for converting system time to timestamps
/// and formatting them according to different metadata styles.
///
/// # Arguments
///
/// * `time_opt` - Optional system time to format
/// * `label` - The label for this time type (e.g., "MTime", "CTime", "BTime")
/// * `style` - The formatting style to use
///
/// # Returns
///
/// A formatted string for the timestamp, or a placeholder string for Text style when time is None.
/// Returns None if no output should be generated (for non-Text styles when time is None).
fn format_timestamp(
    time_opt: Option<SystemTime>,
    label: &str,
    style: MetadataStyle,
) -> Option<String> {
    match time_opt {
        Some(time) => {
            let timestamp = time
                .duration_since(UNIX_EPOCH)
                .map_or_else(|_| 0, |d| d.as_secs());
            let formatted = match style {
                MetadataStyle::Text => format!("[{}: {:>10}s]", label, timestamp),
                MetadataStyle::Markdown | MetadataStyle::Plain => {
                    format!("{}:{}s", label, timestamp)
                }
            };
            Some(formatted)
        }
        None => {
            if style == MetadataStyle::Text {
                Some(format!("[{}:            ]", label))
            } else {
                None
            }
        }
    }
}

/// Applies a custom function to file content and returns the result.
///
/// This function reads the file content and applies the specified function,
/// handling errors appropriately.
///
/// # Arguments
///
/// * `file_path` - Path to the file to process
/// * `apply_fn` - The function to apply to the file content
///
/// # Returns
///
/// `Ok(String)` with the function result, or `Err(ApplyFnError)` if the file
/// cannot be read or the function fails.
pub fn apply_function_to_content<F>(
    file_path: &std::path::Path,
    apply_fn: F,
) -> Result<String, ApplyFnError>
where
    F: FnOnce(&str) -> Result<String, ApplyFnError>,
{
    let content = fs::read_to_string(file_path)
        .map_err(|e| ApplyFnError::CalculationFailed(format!("Failed to read file: {}", e)))?;
    apply_fn(&content)
}

/// Applies a specified built-in function to file content.
///
/// This is a convenience function that combines file reading and built-in function application.
///
/// # Arguments
///
/// * `file_path` - Path to the file to process
/// * `func` - The [`BuiltInFunction`] to apply
///
/// # Returns
///
/// A `Result` containing the string representation of the function's output on success,
/// or an [`ApplyFnError`] on failure.
pub fn apply_builtin_to_file(
    file_path: &std::path::Path,
    func: &BuiltInFunction,
) -> Result<String, ApplyFnError> {
    apply_function_to_content(file_path, |content| apply_builtin_function(content, func))
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
pub fn apply_builtin_function(
    content: &str,
    func: &BuiltInFunction,
) -> Result<String, ApplyFnError> {
    match func {
        BuiltInFunction::CountPluses => {
            let count = content.chars().filter(|&c| c == '+').count();
            Ok(count.to_string())
        }
        BuiltInFunction::Cat => {
            Ok(content.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MetadataOptions;
    use crate::config::metadata::BuiltInFunction;
    use std::path::PathBuf;
    use std::time::{Duration, UNIX_EPOCH};

    fn create_test_node() -> NodeInfo {
        NodeInfo {
            name: "test.txt".to_string(),
            path: PathBuf::from("test.txt"),
            node_type: NodeType::File,
            depth: 1,
            size: Some(1024),
            permissions: None,
            line_count: Some(42),
            word_count: Some(200),
            mtime: Some(UNIX_EPOCH + Duration::from_secs(1234567890)),
            change_time: None,
            create_time: None,
            custom_function_output: Some(Ok("test_result".to_string())),
        }
    }

    #[test]
    fn test_format_timestamp_with_time() {
        let test_time = Some(UNIX_EPOCH + Duration::from_secs(1234567890));

        // Test Text style
        let result = format_timestamp(test_time, "MTime", MetadataStyle::Text);
        assert_eq!(result, Some("[MTime: 1234567890s]".to_string()));

        // Test Markdown style
        let result = format_timestamp(test_time, "MTime", MetadataStyle::Markdown);
        assert_eq!(result, Some("MTime:1234567890s".to_string()));

        // Test Plain style
        let result = format_timestamp(test_time, "MTime", MetadataStyle::Plain);
        assert_eq!(result, Some("MTime:1234567890s".to_string()));
    }

    #[test]
    fn test_format_timestamp_with_none() {
        // Test Text style - should return placeholder
        let result = format_timestamp(None, "CTime", MetadataStyle::Text);
        assert_eq!(result, Some("[CTime:            ]".to_string()));

        // Test Markdown style - should return None
        let result = format_timestamp(None, "CTime", MetadataStyle::Markdown);
        assert_eq!(result, None);

        // Test Plain style - should return None
        let result = format_timestamp(None, "CTime", MetadataStyle::Plain);
        assert_eq!(result, None);
    }

    #[test]
    fn test_format_node_metadata_text_style() {
        let node = create_test_node();
        let config = RustreeLibConfig {
            metadata: MetadataOptions {
                show_size_bytes: true,
                calculate_line_count: true,
                calculate_word_count: true,
                show_last_modified: true,
                apply_function: Some(BuiltInFunction::CountPluses),
                ..Default::default()
            },
            ..Default::default()
        };

        let result = format_node_metadata(&node, &config, MetadataStyle::Text);

        assert!(result.contains("[   1024B]")); // 3 spaces + 1024 = 7 chars total
        assert!(result.contains("[MTime: 1234567890s]"));
        assert!(result.contains("[L:  42]"));
        assert!(result.contains("[W: 200]"));
        assert!(result.contains("[F: \"test_result\"]"));
    }

    #[test]
    fn test_format_node_metadata_markdown_style() {
        let node = create_test_node();
        let config = RustreeLibConfig {
            metadata: MetadataOptions {
                show_size_bytes: true,
                calculate_line_count: true,
                calculate_word_count: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let result = format_node_metadata(&node, &config, MetadataStyle::Markdown);

        assert_eq!(result, " `1024B, 42L, 200W`");
    }

    #[test]
    fn test_format_node_metadata_directory() {
        let mut node = create_test_node();
        node.node_type = NodeType::Directory;

        let config = RustreeLibConfig {
            metadata: MetadataOptions {
                show_size_bytes: true,
                calculate_line_count: true, // Should be ignored for directories
                calculate_word_count: true, // Should be ignored for directories
                ..Default::default()
            },
            ..Default::default()
        };

        let result = format_node_metadata(&node, &config, MetadataStyle::Markdown);

        // Only size should be shown for directories
        assert_eq!(result, " `1024B`");
    }

    #[test]
    fn test_format_node_metadata_no_metadata() {
        let node = create_test_node();
        let config = RustreeLibConfig::default(); // No metadata enabled

        let result = format_node_metadata(&node, &config, MetadataStyle::Text);

        assert_eq!(result, "");
    }

    #[test]
    fn test_apply_builtin_function_cat() {
        let test_content = "Hello, World!\nThis is a test file.";
        let result = apply_builtin_function(test_content, &BuiltInFunction::Cat);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[test]
    fn test_apply_builtin_function_cat_empty_content() {
        let test_content = "";
        let result = apply_builtin_function(test_content, &BuiltInFunction::Cat);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_apply_builtin_function_cat_multiline() {
        let test_content = "Line 1\nLine 2\nLine 3\n";
        let result = apply_builtin_function(test_content, &BuiltInFunction::Cat);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[test]
    fn test_format_node_metadata_with_cat_function() {
        let mut node = create_test_node();
        node.custom_function_output = Some(Ok("File content here".to_string()));
        
        let config = RustreeLibConfig {
            metadata: MetadataOptions {
                apply_function: Some(BuiltInFunction::Cat),
                show_size_bytes: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let result = format_node_metadata(&node, &config, MetadataStyle::Text);
        
        // Should show size but NOT show cat content in metadata (it's displayed separately)
        assert!(result.contains("[   1024B]"));
        assert!(!result.contains("File content here"));
        assert!(!result.contains("[F:"));
    }

    #[test]
    fn test_format_node_metadata_with_count_pluses_function() {
        let mut node = create_test_node();
        node.custom_function_output = Some(Ok("5".to_string()));
        
        let config = RustreeLibConfig {
            metadata: MetadataOptions {
                apply_function: Some(BuiltInFunction::CountPluses),
                show_size_bytes: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let result = format_node_metadata(&node, &config, MetadataStyle::Text);
        
        // Should show both size and function result for non-Cat functions
        assert!(result.contains("[   1024B]"));
        assert!(result.contains("[F: \"5\"]"));
    }
}
