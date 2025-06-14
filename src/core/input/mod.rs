// src/core/input/mod.rs

//! Input parsing module for reading tree structures from various formats.
//!
//! This module provides functionality to parse previously generated tree output files
//! and reconstruct them into the internal NodeInfo structure for further processing.

use crate::core::error::RustreeError;
use crate::core::tree::node::NodeInfo;
use std::path::Path;

pub mod auto_detect;
pub mod html;
pub mod json;
pub mod markdown;
pub mod text;

/// Supported input formats for tree files
#[derive(Debug, Clone, PartialEq)]
pub enum InputFormat {
    /// Plain text tree format (ASCII art)
    Text,
    /// Markdown list format
    Markdown,
    /// JSON format (pretty-printed array)
    Json,
    /// HTML format (tree wrapped in <pre> inside HTML page)
    Html,
    /// Auto-detect format based on file content
    Auto,
}

impl std::str::FromStr for InputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(InputFormat::Text),
            "markdown" | "md" => Ok(InputFormat::Markdown),
            "json" => Ok(InputFormat::Json),
            "html" => Ok(InputFormat::Html),
            "auto" => Ok(InputFormat::Auto),
            _ => Err(format!("Invalid input format: {}", s)),
        }
    }
}

/// Trait for parsing tree files in different formats
pub trait TreeParser {
    /// Parse a tree file and return a vector of NodeInfo structures
    fn parse(&self, content: &str) -> Result<Vec<NodeInfo>, RustreeError>;
}

/// Main interface for parsing tree files
pub struct TreeFileParser;

impl TreeFileParser {
    /// Parse a tree file with the specified format
    pub fn parse_file<P: AsRef<Path>>(
        file_path: P,
        format: InputFormat,
    ) -> Result<Vec<NodeInfo>, RustreeError> {
        let content = std::fs::read_to_string(file_path.as_ref()).map_err(RustreeError::Io)?;

        Self::parse_content(&content, format)
    }

    /// Parse tree content with the specified format
    pub fn parse_content(
        content: &str,
        format: InputFormat,
    ) -> Result<Vec<NodeInfo>, RustreeError> {
        let actual_format = match format {
            InputFormat::Auto => auto_detect::detect_format(content)?,
            _ => format,
        };

        let parser: Box<dyn TreeParser> = match actual_format {
            InputFormat::Json => Box::new(json::JsonTreeParser),
            InputFormat::Text => Box::new(text::TextTreeParser),
            InputFormat::Markdown => Box::new(markdown::MarkdownTreeParser),
            InputFormat::Html => Box::new(html::HtmlTreeParser),
            InputFormat::Auto => unreachable!("Auto format should be resolved by now"),
        };

        parser.parse(content)
    }
}
