// src/core/input/auto_detect.rs

//! Auto-detection logic for input formats.
//!
//! Attempts to determine the format of a tree file based on its content.

use crate::core::error::RustreeError;
use crate::core::input::InputFormat;

/// Detect the format of the given content
pub fn detect_format(content: &str) -> Result<InputFormat, RustreeError> {
    let trimmed = content.trim();

    // Check for JSON format
    if trimmed.starts_with('[')
        && trimmed.ends_with(']')
        && serde_json::from_str::<serde_json::Value>(content).is_ok()
    {
        return Ok(InputFormat::Json);
    }

    // Check for HTML format
    if content.contains("<html") || content.contains("<HTML") || content.contains("<pre>") {
        return Ok(InputFormat::Html);
    }

    // Check for Markdown format (look for list markers)
    let lines: Vec<&str> = content.lines().collect();
    let mut markdown_indicators = 0;
    let mut total_non_empty_lines = 0;

    for line in &lines {
        let trimmed_line = line.trim();
        if !trimmed_line.is_empty() {
            total_non_empty_lines += 1;
            if trimmed_line.starts_with("* ")
                || trimmed_line.starts_with("- ")
                || trimmed_line.starts_with("+ ")
                || trimmed_line.starts_with("# ")
            {
                markdown_indicators += 1;
            }
        }
    }

    // If more than 30% of lines look like markdown, assume it's markdown
    if total_non_empty_lines > 0 && (markdown_indicators * 100 / total_non_empty_lines) > 30 {
        return Ok(InputFormat::Markdown);
    }

    // Default to text format
    Ok(InputFormat::Text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_json() {
        let json_content = r#"[{"type": "directory", "name": "test"}]"#;
        assert_eq!(detect_format(json_content).unwrap(), InputFormat::Json);
    }

    #[test]
    fn test_detect_html() {
        let html_content = "<html><body><pre>tree content</pre></body></html>";
        assert_eq!(detect_format(html_content).unwrap(), InputFormat::Html);
    }

    #[test]
    fn test_detect_markdown() {
        let markdown_content = "# Project\n* file1\n* file2\n- dir1\n  - subfile";
        assert_eq!(
            detect_format(markdown_content).unwrap(),
            InputFormat::Markdown
        );
    }

    #[test]
    fn test_detect_text_default() {
        let text_content = ".\n├── file1\n└── file2";
        assert_eq!(detect_format(text_content).unwrap(), InputFormat::Text);
    }
}
