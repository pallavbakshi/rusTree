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

use crate::core::options::{ApplyFunction, FunctionOutputKind};
use crate::core::options::{BuiltInFunction, RustreeLibConfig};
use crate::core::tree::node::{NodeInfo, NodeType};
use crate::core::util::format_size;

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

    /// Generic numeric total aggregated from custom apply-functions that yield numbers.
    pub custom_number_total: Option<u64>,
    /// Generic bytes total aggregated from custom apply-functions that yield byte counts.
    pub custom_bytes_total: Option<u64>,
}

impl MetadataAggregator {
    /// Aggregates metadata from a collection of nodes based on the configuration.
    pub fn aggregate_from_nodes(nodes: &[NodeInfo], config: &RustreeLibConfig) -> Self {
        let mut aggregator = Self::default();

        // Track whether we should aggregate each type
        let should_aggregate_size = config.metadata.show_size_bytes;
        let should_aggregate_lines = config.metadata.calculate_line_count;
        let should_aggregate_words = config.metadata.calculate_word_count;

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
            if let Some(Ok(output)) = &node.custom_function_output {
                // Determine output kind based on configuration (built-in vs external)
                let kind = if let Some(apply_fn) = &config.metadata.apply_function {
                    apply_fn.output_kind()
                } else {
                    FunctionOutputKind::Text
                };

                let builtin_func = match &config.metadata.apply_function {
                    Some(ApplyFunction::BuiltIn(func)) => Some(func.clone()),
                    _ => None,
                };
                aggregator.aggregate_function_output(output, kind, &builtin_func);
            }
        }

        aggregator
    }

    /// Parses and aggregates output from apply functions.
    fn aggregate_function_output(
        &mut self,
        output: &str,
        kind: FunctionOutputKind,
        builtin: &Option<BuiltInFunction>,
    ) {
        // Aggregate generically only when the result originates from an external
        // function (i.e. no built-in function specified).
        if builtin.is_none() {
            match kind {
                FunctionOutputKind::Number => {
                    if let Ok(num) = output.parse::<u64>() {
                        *self.custom_number_total.get_or_insert(0) += num;
                    }
                }
                FunctionOutputKind::Bytes => {
                    if let Ok(bytes) = output.parse::<u64>() {
                        *self.custom_bytes_total.get_or_insert(0) += bytes;
                    }
                }
                FunctionOutputKind::Text => {}
            }
        }

        // Still keep legacy built-in aggregation for specific directory functions
        if let Some(function) = builtin {
            match function {
                BuiltInFunction::CountFiles => {
                    if let Ok(count) = output.parse::<usize>() {
                        *self.file_count_from_function.get_or_insert(0) += count;
                    }
                }
                BuiltInFunction::CountDirs => {
                    if let Ok(count) = output.parse::<usize>() {
                        *self.dir_count_from_function.get_or_insert(0) += count;
                    }
                }
                BuiltInFunction::SizeTotal => {
                    if let Ok(size) = output.parse::<u64>() {
                        *self.size_from_function.get_or_insert(0) += size;
                    }
                }
                BuiltInFunction::DirStats => {
                    let parts: Vec<&str> = output.split(',').collect();
                    if parts.len() == 3 {
                        if let Some(file_part) = parts[0].strip_suffix('f') {
                            if let Ok(count) = file_part.parse::<usize>() {
                                *self.file_count_from_function.get_or_insert(0) += count;
                            }
                        }
                        if let Some(dir_part) = parts[1].strip_suffix('d') {
                            if let Ok(count) = dir_part.parse::<usize>() {
                                *self.dir_count_from_function.get_or_insert(0) += count;
                            }
                        }
                        if let Some(size_part) = parts[2].strip_suffix('B') {
                            if let Ok(size) = size_part.parse::<u64>() {
                                *self.size_from_function.get_or_insert(0) += size;
                            }
                        }
                    }
                }
                _ => {}
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
            parts.push(format!("{} total", format_size(size)));
        }

        // Function-based totals (built-in directory functions & external)
        if let Some(size) = self.size_from_function {
            if self.size_total.is_none() {
                parts.push(format!("{} total (from function)", format_size(size)));
            }
        }

        if let Some(bytes) = self.custom_bytes_total.filter(|b| *b > 0) {
            // Avoid duplicate display if already counted
            if self.size_total.is_none() && self.size_from_function.is_none() {
                parts.push(format!("{} total (custom)", format_size(bytes)));
            }
        }

        if let Some(num) = self.custom_number_total.filter(|n| *n > 0) {
            parts.push(format!(
                "{} total (custom)",
                Self::format_number(num as usize)
            ));
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

    /// Formats a size in bytes to a human-readable string by delegating to the
    /// shared helper in `core::util`.  This wrapper is kept to avoid breaking
    /// existing public API and unit tests, while ensuring the formatting logic
    /// itself lives in a single place.
    pub fn format_size(bytes: u64) -> String {
        format_size(bytes)
    }
}
