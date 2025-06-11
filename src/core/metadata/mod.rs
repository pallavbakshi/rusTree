//! Metadata collection and processing.
//!
//! This module contains functionality for collecting, calculating, and processing
//! metadata about file system entries, including file sizes, timestamps, content
//! analysis, and custom function application.

pub mod file_info;
pub mod size_calculator;

// Stubs for future implementation
pub mod extended_attrs;
pub mod time_formatter;

use crate::config::{RustreeLibConfig, metadata::BuiltInFunction};
use crate::core::tree::node::{NodeInfo, NodeType};

/// Aggregates metadata values from a collection of nodes.
/// Used to calculate totals for the summary report.
#[derive(Debug, Default)]
pub struct MetadataAggregator {
    /// Total size in bytes across all files
    pub size_total: Option<u64>,
    /// Total number of lines across all files
    pub line_total: Option<usize>,
    /// Total number of words across all files
    pub word_total: Option<usize>,
    /// File count extracted from apply functions
    pub file_count_from_function: Option<usize>,
    /// Directory count extracted from apply functions
    pub dir_count_from_function: Option<usize>,
    /// Size total extracted from apply functions
    pub size_from_function: Option<u64>,
}

impl MetadataAggregator {
    /// Aggregates metadata from a collection of nodes based on the configuration.
    pub fn aggregate_from_nodes(nodes: &[NodeInfo], config: &RustreeLibConfig) -> Self {
        let mut aggregator = Self::default();

        // Track whether we should aggregate each type
        let should_aggregate_size = config.metadata.show_size_bytes;
        let should_aggregate_lines = config.metadata.calculate_line_count;
        let should_aggregate_words = config.metadata.calculate_word_count;
        let has_apply_function = config.metadata.apply_function.is_some();

        for node in nodes {
            // Aggregate built-in metadata for files
            if node.node_type == NodeType::File {
                if should_aggregate_size {
                    if let Some(size) = node.size {
                        *aggregator.size_total.get_or_insert(0) += size;
                    }
                }

                if should_aggregate_lines {
                    if let Some(lines) = node.line_count {
                        *aggregator.line_total.get_or_insert(0) += lines;
                    }
                }

                if should_aggregate_words {
                    if let Some(words) = node.word_count {
                        *aggregator.word_total.get_or_insert(0) += words;
                    }
                }
            }

            // Aggregate apply function outputs
            if has_apply_function {
                if let Some(Ok(output)) = &node.custom_function_output {
                    aggregator.aggregate_function_output(output, &config.metadata.apply_function);
                }
            }
        }

        aggregator
    }

    /// Parses and aggregates output from apply functions.
    fn aggregate_function_output(&mut self, output: &str, function: &Option<BuiltInFunction>) {
        match function {
            Some(BuiltInFunction::CountFiles) => {
                if let Ok(count) = output.parse::<usize>() {
                    *self.file_count_from_function.get_or_insert(0) += count;
                }
            }
            Some(BuiltInFunction::CountDirs) => {
                if let Ok(count) = output.parse::<usize>() {
                    *self.dir_count_from_function.get_or_insert(0) += count;
                }
            }
            Some(BuiltInFunction::SizeTotal) => {
                if let Ok(size) = output.parse::<u64>() {
                    *self.size_from_function.get_or_insert(0) += size;
                }
            }
            Some(BuiltInFunction::DirStats) => {
                // Parse "Xf,Yd,ZB" format
                let parts: Vec<&str> = output.split(',').collect();
                if parts.len() == 3 {
                    // Extract file count
                    if let Some(file_part) = parts[0].strip_suffix('f') {
                        if let Ok(count) = file_part.parse::<usize>() {
                            *self.file_count_from_function.get_or_insert(0) += count;
                        }
                    }
                    // Extract directory count
                    if let Some(dir_part) = parts[1].strip_suffix('d') {
                        if let Ok(count) = dir_part.parse::<usize>() {
                            *self.dir_count_from_function.get_or_insert(0) += count;
                        }
                    }
                    // Extract size
                    if let Some(size_part) = parts[2].strip_suffix('B') {
                        if let Ok(size) = size_part.parse::<u64>() {
                            *self.size_from_function.get_or_insert(0) += size;
                        }
                    }
                }
            }
            _ => {
                // For other functions, try to parse as a number
                if let Ok(_num) = output.parse::<usize>() {
                    // Store in a generic counter (could be extended in the future)
                }
            }
        }
    }

    /// Formats the aggregated metadata as additions to the summary line.
    pub fn format_summary_additions(&self) -> String {
        let mut parts = Vec::new();

        if let Some(lines) = self.line_total {
            parts.push(format!("{} total lines", Self::format_number(lines)));
        }

        if let Some(words) = self.word_total {
            parts.push(format!("{} total words", Self::format_number(words)));
        }

        if let Some(size) = self.size_total {
            parts.push(format!("{} total", Self::format_size(size)));
        }

        // If we have function-based totals, include them separately
        if let Some(size) = self.size_from_function {
            if self.size_total.is_none() {
                // Only show if not already showing size_total
                parts.push(format!("{} total (from function)", Self::format_size(size)));
            }
        }

        if parts.is_empty() {
            String::new()
        } else {
            format!(", {}", parts.join(", "))
        }
    }

    /// Formats a number with thousand separators.
    pub fn format_number(n: usize) -> String {
        let s = n.to_string();
        let mut result = String::new();
        let mut count = 0;

        for ch in s.chars().rev() {
            if count == 3 {
                result.push(',');
                count = 0;
            }
            result.push(ch);
            count += 1;
        }

        result.chars().rev().collect()
    }

    /// Formats a size in bytes to a human-readable format.
    pub fn format_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        const THRESHOLD: f64 = 1024.0;

        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
            size /= THRESHOLD;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
}
