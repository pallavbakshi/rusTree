// src/core/diff/formatter/json.rs

//! JSON formatter for diff results, producing structured output for programmatic consumption.

use crate::config::RustreeLibConfig;
use crate::core::diff::formatter::DiffFormatter;
use crate::core::diff::{Change, ChangeType, DiffResult};
use crate::core::error::RustreeError;
use serde_json::{Value, json};

pub struct JsonDiffFormatter;

impl DiffFormatter for JsonDiffFormatter {
    fn format(
        &self,
        diff_result: &DiffResult,
        _config: &RustreeLibConfig,
    ) -> Result<String, RustreeError> {
        let json_value = json!({
            "diff_metadata": {
                "generated_at": diff_result.metadata.generated_at,
                "snapshot_file": diff_result.metadata.snapshot_file,
                "snapshot_date": diff_result.metadata.snapshot_date,
                "comparison_root": diff_result.metadata.comparison_root,
                "filters_applied": diff_result.metadata.filters_applied,
                "options": {
                    "max_depth": diff_result.metadata.options.max_depth,
                    "show_size": diff_result.metadata.options.show_size,
                    "sort_by": diff_result.metadata.options.sort_by,
                    "detect_moves": diff_result.metadata.options.detect_moves,
                    "move_threshold": diff_result.metadata.options.move_threshold,
                    "show_unchanged": diff_result.metadata.options.show_unchanged,
                }
            },
            "diff_summary": {
                "added": diff_result.summary.added,
                "removed": diff_result.summary.removed,
                "modified": diff_result.summary.modified,
                "moved": diff_result.summary.moved,
                "type_changed": diff_result.summary.type_changed,
                "unchanged": diff_result.summary.unchanged,
                "total_size_change": diff_result.summary.size_change,
                "detailed_breakdown": {
                    "directories_added": diff_result.summary.directories_added,
                    "files_added": diff_result.summary.files_added,
                    "directories_removed": diff_result.summary.directories_removed,
                    "files_removed": diff_result.summary.files_removed,
                    "directories_moved": diff_result.summary.directories_moved,
                    "files_moved": diff_result.summary.files_moved
                }
            },
            "changes": diff_result.changes.iter()
                .filter(|c| !matches!(c.change_type, ChangeType::Unchanged) ||
                           diff_result.metadata.options.show_unchanged)
                .map(format_change_json)
                .collect::<Vec<_>>(),
            "unchanged": if diff_result.metadata.options.show_unchanged {
                diff_result.changes.iter()
                    .filter(|c| matches!(c.change_type, ChangeType::Unchanged))
                    .map(format_unchanged_json)
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        });

        serde_json::to_string_pretty(&json_value).map_err(|_| RustreeError::Fmt(std::fmt::Error))
    }
}

fn format_change_json(change: &Change) -> Value {
    let mut obj = json!({
        "path": change.path(),
        "change_type": format_change_type(&change.change_type),
        "node_type": format_node_type(change),
    });

    // Add current node info if available
    if let Some(current) = &change.current {
        if let Some(size) = current.size {
            obj["size"] = json!(size);
        }
        if let Some(mtime) = current.mtime {
            if let Ok(duration) = mtime.duration_since(std::time::UNIX_EPOCH) {
                obj["modified"] = json!(duration.as_secs());
            }
        }
    }

    // Add previous node info for removed items
    if let Some(previous) = &change.previous {
        if change.current.is_none() {
            if let Some(size) = previous.size {
                obj["previous_size"] = json!(size);
            }
        }
    }

    // Add change-specific details
    match &change.change_type {
        ChangeType::Moved {
            from_path,
            similarity,
        } => {
            obj["previous_path"] = json!(from_path);
            obj["similarity_score"] = json!(similarity);
        }
        ChangeType::TypeChanged { from_type, to_type } => {
            obj["old_type"] = json!(format!("{:?}", from_type).to_lowercase());
            obj["new_type"] = json!(format!("{:?}", to_type).to_lowercase());
        }
        ChangeType::Modified => {
            if !change.children.is_empty() {
                // Summary of changes inside
                let mut changes_inside = json!({
                    "added": 0,
                    "removed": 0,
                    "moved": 0,
                    "modified": 0,
                });

                for child in &change.children {
                    match child.change_type {
                        ChangeType::Added => {
                            changes_inside["added"] =
                                json!(changes_inside["added"].as_i64().unwrap_or(0) + 1)
                        }
                        ChangeType::Removed => {
                            changes_inside["removed"] =
                                json!(changes_inside["removed"].as_i64().unwrap_or(0) + 1)
                        }
                        ChangeType::Moved { .. } => {
                            changes_inside["moved"] =
                                json!(changes_inside["moved"].as_i64().unwrap_or(0) + 1)
                        }
                        ChangeType::Modified => {
                            changes_inside["modified"] =
                                json!(changes_inside["modified"].as_i64().unwrap_or(0) + 1)
                        }
                        _ => {}
                    }
                }

                obj["changes_inside"] = changes_inside;
            }
        }
        ChangeType::Removed => {
            // For removed directories, count children
            if change.is_directory() && !change.children.is_empty() {
                obj["children_count"] = json!(count_all_children(change));
            }
        }
        _ => {}
    }

    // Add children for directories with changes
    if !change.children.is_empty()
        && matches!(change.change_type, ChangeType::Added | ChangeType::Modified)
    {
        obj["children"] = json!(
            change
                .children
                .iter()
                .map(format_change_json)
                .collect::<Vec<_>>()
        );
    }

    obj
}

fn format_unchanged_json(change: &Change) -> Value {
    let mut obj = json!({
        "path": change.path(),
        "node_type": format_node_type(change),
    });

    if let Some(current) = &change.current {
        if let Some(size) = current.size {
            obj["size"] = json!(size);
        }
        if let Some(mtime) = current.mtime {
            if let Ok(duration) = mtime.duration_since(std::time::UNIX_EPOCH) {
                obj["last_modified"] = json!(duration.as_secs());
            }
        }
    }

    obj
}

fn format_change_type(change_type: &ChangeType) -> &'static str {
    match change_type {
        ChangeType::Added => "added",
        ChangeType::Removed => "removed",
        ChangeType::Modified => "modified",
        ChangeType::Moved { .. } => "moved",
        ChangeType::TypeChanged { .. } => "type_changed",
        ChangeType::Unchanged => "unchanged",
    }
}

fn format_node_type(change: &Change) -> &'static str {
    if change.is_directory() {
        "directory"
    } else {
        "file"
    }
}

fn count_all_children(change: &Change) -> usize {
    let mut count = change.children.len();
    for child in &change.children {
        if child.is_directory() {
            count += count_all_children(child);
        }
    }
    count
}
