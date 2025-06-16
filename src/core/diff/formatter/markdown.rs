// src/core/diff/formatter/markdown.rs

//! Markdown formatter for diff results, producing clean markdown output.

use crate::core::diff::formatter::{DiffFormatter, format_size_change};
use crate::core::diff::{Change, ChangeType, DiffResult};
use crate::core::error::RustreeError;
use crate::core::options::RustreeLibConfig;
use crate::core::tree::node::NodeType;
use std::fmt::Write;

pub struct MarkdownDiffFormatter;

impl DiffFormatter for MarkdownDiffFormatter {
    fn format(
        &self,
        diff_result: &DiffResult,
        config: &RustreeLibConfig,
    ) -> Result<String, RustreeError> {
        let mut output = String::new();

        // Title
        writeln!(&mut output, "# Directory Changes")?;
        writeln!(&mut output)?;

        // Metadata
        writeln!(
            &mut output,
            "**Generated:** {}",
            diff_result.metadata.generated_at
        )?;
        writeln!(
            &mut output,
            "**Snapshot:** {}",
            diff_result.metadata.snapshot_file.display()
        )?;
        if let Some(snapshot_date) = &diff_result.metadata.snapshot_date {
            writeln!(&mut output, "**Snapshot Date:** {}", snapshot_date)?;
        }
        writeln!(
            &mut output,
            "**Root:** {}",
            diff_result.metadata.comparison_root.display()
        )?;
        writeln!(&mut output)?;

        // Summary
        writeln!(&mut output, "## Summary")?;
        writeln!(&mut output)?;

        if diff_result.summary.total_changes() == 0 {
            writeln!(&mut output, "No changes detected.")?;
            return Ok(output);
        }

        // Added items
        if diff_result.summary.added > 0 {
            if diff_result.summary.directories_added > 0 && diff_result.summary.files_added > 0 {
                writeln!(
                    &mut output,
                    "- **{}** directories added, **{}** files added (+)",
                    diff_result.summary.directories_added, diff_result.summary.files_added
                )?;
            } else if diff_result.summary.directories_added > 0 {
                writeln!(
                    &mut output,
                    "- **{}** directories added (+)",
                    diff_result.summary.directories_added
                )?;
            } else if diff_result.summary.files_added > 0 {
                writeln!(
                    &mut output,
                    "- **{}** files added (+)",
                    diff_result.summary.files_added
                )?;
            }
        }

        // Removed items
        if diff_result.summary.removed > 0 {
            if diff_result.summary.directories_removed > 0 && diff_result.summary.files_removed > 0
            {
                writeln!(
                    &mut output,
                    "- **{}** directories removed, **{}** files removed (-)",
                    diff_result.summary.directories_removed, diff_result.summary.files_removed
                )?;
            } else if diff_result.summary.directories_removed > 0 {
                writeln!(
                    &mut output,
                    "- **{}** directories removed (-)",
                    diff_result.summary.directories_removed
                )?;
            } else if diff_result.summary.files_removed > 0 {
                writeln!(
                    &mut output,
                    "- **{}** files removed (-)",
                    diff_result.summary.files_removed
                )?;
            }
        }

        // Moved items
        if diff_result.summary.moved > 0 {
            if diff_result.summary.directories_moved > 0 && diff_result.summary.files_moved > 0 {
                writeln!(
                    &mut output,
                    "- **{}** directories moved, **{}** files moved/renamed (~)",
                    diff_result.summary.directories_moved, diff_result.summary.files_moved
                )?;
            } else if diff_result.summary.directories_moved > 0 {
                writeln!(
                    &mut output,
                    "- **{}** directories moved/renamed (~)",
                    diff_result.summary.directories_moved
                )?;
            } else if diff_result.summary.files_moved > 0 {
                writeln!(
                    &mut output,
                    "- **{}** files moved/renamed (~)",
                    diff_result.summary.files_moved
                )?;
            }
        }
        if diff_result.summary.type_changed > 0 {
            writeln!(
                &mut output,
                "- **{}** type changes (T)",
                diff_result.summary.type_changed
            )?;
        }
        if diff_result.summary.modified > 0 {
            writeln!(
                &mut output,
                "- **{}** directories modified (M)",
                diff_result.summary.modified
            )?;
        }
        if diff_result.metadata.options.show_unchanged && diff_result.summary.unchanged > 0 {
            writeln!(
                &mut output,
                "- **{}** files unchanged",
                diff_result.summary.unchanged
            )?;
        }

        // Size change
        if config.metadata.show_size_bytes && diff_result.summary.size_change != 0 {
            let size_str =
                format_size_change(diff_result.summary.size_change, config.misc.human_friendly);
            writeln!(&mut output, "- **Total size change:** {}", size_str)?;
        }

        writeln!(&mut output)?;

        // Group changes by type
        let mut added_changes = Vec::new();
        let mut removed_changes = Vec::new();
        let mut moved_changes = Vec::new();
        let mut type_changed_changes = Vec::new();
        let mut modified_changes = Vec::new();

        for change in &diff_result.changes {
            match &change.change_type {
                ChangeType::Added => added_changes.push(change),
                ChangeType::Removed => removed_changes.push(change),
                ChangeType::Moved { .. } => moved_changes.push(change),
                ChangeType::TypeChanged { .. } => type_changed_changes.push(change),
                ChangeType::Modified => modified_changes.push(change),
                ChangeType::Unchanged => {} // Handled separately if needed
            }
        }

        // Added Files
        if !added_changes.is_empty() {
            writeln!(&mut output, "## Added Entities (+)")?;
            writeln!(&mut output)?;
            for change in added_changes {
                format_change_list_item(&mut output, change, config)?;
            }
            writeln!(&mut output)?;
        }

        // Removed Files
        if !removed_changes.is_empty() {
            writeln!(&mut output, "## Removed Entities (-)")?;
            writeln!(&mut output)?;
            for change in removed_changes {
                format_change_list_item(&mut output, change, config)?;
            }
            writeln!(&mut output)?;
        }

        // Moved/Renamed Files
        if !moved_changes.is_empty() {
            writeln!(&mut output, "## Moved/Renamed Entities (~)")?;
            writeln!(&mut output)?;
            for change in moved_changes {
                format_moved_change(&mut output, change, config)?;
            }
            writeln!(&mut output)?;
        }

        // Type Changes
        if !type_changed_changes.is_empty() {
            writeln!(&mut output, "## Type Changes (T)")?;
            writeln!(&mut output)?;
            for change in type_changed_changes {
                format_type_change(&mut output, change, config)?;
            }
            writeln!(&mut output)?;
        }

        // Modified Directories
        if !modified_changes.is_empty() {
            writeln!(&mut output, "## Modified Directories (M)")?;
            writeln!(&mut output)?;
            for change in modified_changes {
                format_modified_change(&mut output, change, config)?;
            }
            writeln!(&mut output)?;
        }

        // Unchanged files if requested
        if diff_result.metadata.options.show_unchanged && diff_result.summary.unchanged > 0 {
            writeln!(&mut output, "## Unchanged Entities")?;
            writeln!(&mut output)?;

            let unchanged_changes: Vec<_> = diff_result
                .changes
                .iter()
                .filter(|c| matches!(c.change_type, ChangeType::Unchanged))
                .collect();

            for change in unchanged_changes {
                format_change_list_item(&mut output, change, config)?;
            }
            writeln!(&mut output)?;
        }

        // Footer
        writeln!(&mut output, "---")?;
        writeln!(
            &mut output,
            "*Generated by RusTree on {}*",
            diff_result.metadata.generated_at
        )?;

        Ok(output)
    }
}

fn format_change_list_item(
    output: &mut String,
    change: &Change,
    config: &RustreeLibConfig,
) -> Result<(), RustreeError> {
    let path = change.path();
    let path_str = if config.listing.show_full_path {
        path.to_string_lossy().to_string()
    } else {
        path.file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string()
    };

    write!(output, "- `{}`", path_str)?;

    // Add directory indicator
    if change.is_directory() {
        write!(output, "/")?;
    }

    // Add size if available and requested
    if config.metadata.show_size_bytes && !change.is_directory() {
        if let Some(current) = &change.current {
            if let Some(size) = current.size {
                let size_str = if config.misc.human_friendly {
                    format_human_size(size)
                } else {
                    format!("{} B", size)
                };
                write!(output, " ({})", size_str)?;
            }
        }
    }

    writeln!(output)?;

    // Add nested children with indentation
    if !change.children.is_empty() {
        for child in &change.children {
            write!(output, "  ")?;
            format_change_list_item(output, child, config)?;
        }
    }

    Ok(())
}

fn format_moved_change(
    output: &mut String,
    change: &Change,
    config: &RustreeLibConfig,
) -> Result<(), RustreeError> {
    let path = change.path();
    let path_str = if config.listing.show_full_path {
        path.to_string_lossy().to_string()
    } else {
        path.file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string()
    };

    write!(output, "- `{}`", path_str)?;

    if change.is_directory() {
        write!(output, "/")?;
    }

    if let ChangeType::Moved {
        from_path,
        similarity,
    } = &change.change_type
    {
        let from_name = from_path
            .file_name()
            .unwrap_or(from_path.as_os_str())
            .to_string_lossy();
        write!(output, " ← was `{}`", from_name)?;

        if config.misc.verbose {
            write!(output, " ({:.0}% similarity)", similarity * 100.0)?;
        }
    }

    writeln!(output)?;
    Ok(())
}

fn format_type_change(
    output: &mut String,
    change: &Change,
    config: &RustreeLibConfig,
) -> Result<(), RustreeError> {
    let path = change.path();
    let path_str = if config.listing.show_full_path {
        path.to_string_lossy().to_string()
    } else {
        path.file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string()
    };

    write!(output, "- `{}`", path_str)?;

    if let ChangeType::TypeChanged { from_type, to_type } = &change.change_type {
        write!(
            output,
            " changed from {} to {}",
            format_node_type(from_type),
            format_node_type(to_type)
        )?;
    }

    writeln!(output)?;
    Ok(())
}

fn format_modified_change(
    output: &mut String,
    change: &Change,
    config: &RustreeLibConfig,
) -> Result<(), RustreeError> {
    let path = change.path();
    let path_str = if config.listing.show_full_path {
        path.to_string_lossy().to_string()
    } else {
        path.file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string()
    };

    write!(output, "- `{}/` (contents changed)", path_str)?;

    if !change.children.is_empty() {
        writeln!(output)?;
        for child in &change.children {
            write!(output, "  ")?;
            format_change_list_item(output, child, config)?;
        }
    } else {
        writeln!(output)?;
    }

    Ok(())
}

fn format_node_type(node_type: &NodeType) -> &'static str {
    match node_type {
        NodeType::File => "file",
        NodeType::Directory => "directory",
        NodeType::Symlink => "symlink",
    }
}

fn format_human_size(size: u64) -> String {
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
