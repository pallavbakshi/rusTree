// src/core/diff/formatter.rs

//! Formatters for rendering diff results in various output formats.

use crate::core::diff::{ChangeType, DiffResult};
use crate::core::error::RustreeError;
use crate::core::formatter::OutputFormat;
use crate::core::options::RustreeLibConfig;

pub mod html;
pub mod json;
pub mod markdown;
pub mod text;

/// Trait for formatting diff results.
pub trait DiffFormatter {
    /// Formats a diff result into a string representation.
    fn format(
        &self,
        diff_result: &DiffResult,
        config: &RustreeLibConfig,
    ) -> Result<String, RustreeError>;
}

/// Formats a diff result according to the specified output format.
pub fn format_diff(
    diff_result: &DiffResult,
    format: OutputFormat,
    config: &RustreeLibConfig,
) -> Result<String, RustreeError> {
    match format {
        OutputFormat::Text => {
            let formatter = text::TextDiffFormatter;
            formatter.format(diff_result, config)
        }
        OutputFormat::Json => {
            let formatter = json::JsonDiffFormatter;
            formatter.format(diff_result, config)
        }
        OutputFormat::Markdown => {
            let formatter = markdown::MarkdownDiffFormatter;
            formatter.format(diff_result, config)
        }
        OutputFormat::Html => {
            let formatter = html::HtmlDiffFormatter;
            formatter.format(diff_result, config)
        }
    }
}

/// Helper function to get a display symbol for a change type.
pub fn change_type_symbol(change_type: &ChangeType) -> &'static str {
    match change_type {
        ChangeType::Added => "[+]",
        ChangeType::Removed => "[-]",
        ChangeType::Modified => "[M]",
        ChangeType::Moved { .. } => "[~]",
        ChangeType::TypeChanged { .. } => "[T]",
        ChangeType::Unchanged => "",
    }
}

/// Helper function to get a display color for a change type (for terminal output).
pub fn change_type_color(change_type: &ChangeType) -> &'static str {
    match change_type {
        ChangeType::Added => "\x1b[32m",              // Green
        ChangeType::Removed => "\x1b[31m",            // Red
        ChangeType::Modified => "\x1b[33m",           // Yellow
        ChangeType::Moved { .. } => "\x1b[35m",       // Magenta
        ChangeType::TypeChanged { .. } => "\x1b[36m", // Cyan
        ChangeType::Unchanged => "\x1b[90m",          // Gray
    }
}

/// Helper to format a size change.
pub fn format_size_change(size_change: i128, human_friendly: bool) -> String {
    if size_change == 0 {
        return String::new();
    }

    let sign = if size_change > 0 { "+" } else { "" };

    if human_friendly {
        let abs_size = size_change.unsigned_abs();
        let formatted = format_human_size(abs_size);
        format!("{}{}", sign, formatted)
    } else {
        format!("{}{} B", sign, size_change)
    }
}

/// Formats a size in bytes to human-readable format.
fn format_human_size(size: u128) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    let mut size_f = size as f64;
    let mut unit_index = 0;

    while size_f >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size_f /= THRESHOLD;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size_f, UNITS[unit_index])
    }
}
