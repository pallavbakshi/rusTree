// src/core/diff/formatter/html.rs

//! HTML formatter for diff results, producing interactive HTML output.

use crate::core::diff::formatter::{DiffFormatter, format_size_change};
use crate::core::diff::{Change, ChangeType, DiffResult};
use crate::core::error::RustreeError;
use crate::core::options::RustreeLibConfig;
use crate::core::tree::node::NodeType;
use std::fmt::Write;

pub struct HtmlDiffFormatter;

impl DiffFormatter for HtmlDiffFormatter {
    fn format(
        &self,
        diff_result: &DiffResult,
        config: &RustreeLibConfig,
    ) -> Result<String, RustreeError> {
        let mut output = String::new();

        // HTML head
        writeln!(&mut output, "<!DOCTYPE html>")?;
        writeln!(&mut output, "<html lang=\"en\">")?;
        writeln!(&mut output, "<head>")?;
        writeln!(&mut output, "    <meta charset=\"utf-8\">")?;
        writeln!(&mut output, "    <title>Directory Diff - RusTree</title>")?;

        // CSS styles
        write_css(&mut output)?;

        writeln!(&mut output, "</head>")?;
        writeln!(&mut output, "<body>")?;
        writeln!(&mut output, "    <div class=\"diff-container\">")?;

        // Header
        writeln!(&mut output, "        <h1>🌳 Directory Diff Report</h1>")?;
        writeln!(&mut output, "        <div class=\"metadata\">")?;
        writeln!(
            &mut output,
            "            <p><strong>Generated:</strong> {}</p>",
            diff_result.metadata.generated_at
        )?;
        writeln!(
            &mut output,
            "            <p><strong>Snapshot:</strong> {}</p>",
            diff_result.metadata.snapshot_file.display()
        )?;
        if let Some(snapshot_date) = &diff_result.metadata.snapshot_date {
            writeln!(
                &mut output,
                "            <p><strong>Snapshot Date:</strong> {}</p>",
                snapshot_date
            )?;
        }
        writeln!(
            &mut output,
            "            <p><strong>Root:</strong> {}</p>",
            diff_result.metadata.comparison_root.display()
        )?;
        writeln!(&mut output, "        </div>")?;

        // Summary
        if diff_result.summary.total_changes() > 0 {
            write_summary(&mut output, diff_result, config)?;
        } else {
            writeln!(&mut output, "        <div class=\"no-changes\">")?;
            writeln!(&mut output, "            <h2>No Changes Detected</h2>")?;
            writeln!(
                &mut output,
                "            <p>The directory structure is identical to the snapshot.</p>"
            )?;
            writeln!(&mut output, "        </div>")?;
        }

        // Tree diff
        if diff_result.summary.total_changes() > 0 {
            write_tree_diff(&mut output, diff_result, config)?;

            // Detailed changes (expandable)
            write_detailed_changes(&mut output, diff_result, config)?;
        }

        // Footer
        writeln!(&mut output, "        <footer>")?;
        writeln!(
            &mut output,
            "            <p>Generated by <strong>RusTree</strong> on {}</p>",
            diff_result.metadata.generated_at
        )?;
        writeln!(&mut output, "        </footer>")?;

        writeln!(&mut output, "    </div>")?;

        // JavaScript
        write_javascript(&mut output)?;

        writeln!(&mut output, "</body>")?;
        writeln!(&mut output, "</html>")?;

        Ok(output)
    }
}

fn write_css(output: &mut String) -> Result<(), RustreeError> {
    writeln!(output, "    <style>")?;
    writeln!(
        output,
        "        body {{ font-family: -apple-system, system-ui, sans-serif; margin: 2rem; background: #f8fafc; }}"
    )?;
    writeln!(
        output,
        "        .diff-container {{ max-width: 1200px; margin: 0 auto; background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }}"
    )?;
    writeln!(
        output,
        "        .metadata {{ background: #f1f5f9; padding: 1rem; border-radius: 8px; margin: 1rem 0; }}"
    )?;
    writeln!(output, "        .metadata p {{ margin: 0.25rem 0; }}")?;
    writeln!(
        output,
        "        .summary {{ background: #f8fafc; padding: 1.5rem; border-radius: 8px; margin: 2rem 0; }}"
    )?;
    writeln!(
        output,
        "        .stat-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-top: 1rem; }}"
    )?;
    writeln!(
        output,
        "        .stat-card {{ background: white; padding: 1rem; border-radius: 6px; border-left: 4px solid #3b82f6; }}"
    )?;
    writeln!(
        output,
        "        .stat-card.added {{ border-left-color: #22c55e; }}"
    )?;
    writeln!(
        output,
        "        .stat-card.removed {{ border-left-color: #ef4444; }}"
    )?;
    writeln!(
        output,
        "        .stat-card.moved {{ border-left-color: #a855f7; }}"
    )?;
    writeln!(
        output,
        "        .stat-card.modified {{ border-left-color: #f59e0b; }}"
    )?;
    writeln!(
        output,
        "        .tree-diff {{ font-family: 'Monaco', 'Courier New', monospace; background: #1f2937; color: #f9fafb; padding: 1.5rem; border-radius: 8px; overflow-x: auto; margin: 2rem 0; }}"
    )?;
    writeln!(
        output,
        "        .added {{ color: #34d399; font-weight: 600; }}"
    )?;
    writeln!(
        output,
        "        .removed {{ color: #f87171; text-decoration: line-through; }}"
    )?;
    writeln!(
        output,
        "        .modified {{ color: #fbbf24; font-weight: 600; }}"
    )?;
    writeln!(
        output,
        "        .moved {{ color: #a78bfa; font-style: italic; }}"
    )?;
    writeln!(
        output,
        "        .type-changed {{ color: #60a5fa; font-weight: 600; }}"
    )?;
    writeln!(output, "        .unchanged {{ color: #9ca3af; }}")?;
    writeln!(
        output,
        "        .expandable {{ cursor: pointer; user-select: none; margin: 1rem 0; padding: 1rem; background: #f3f4f6; border-radius: 8px; border: 1px solid #e5e7eb; }}"
    )?;
    writeln!(
        output,
        "        .expandable:hover {{ background: #f9fafb; }}"
    )?;
    writeln!(output, "        .expandable h3 {{ margin: 0; }}")?;
    writeln!(output, "        .hidden {{ display: none; }}")?;
    writeln!(output, "        .change-section {{ margin: 2rem 0; }}")?;
    writeln!(
        output,
        "        .change-list {{ list-style: none; padding: 0; }}"
    )?;
    writeln!(
        output,
        "        .change-item {{ padding: 0.5rem; margin: 0.25rem 0; border-radius: 4px; }}"
    )?;
    writeln!(
        output,
        "        .change-item.added {{ background: #dcfce7; border-left: 3px solid #22c55e; }}"
    )?;
    writeln!(
        output,
        "        .change-item.removed {{ background: #fee2e2; border-left: 3px solid #ef4444; }}"
    )?;
    writeln!(
        output,
        "        .change-item.moved {{ background: #f3e8ff; border-left: 3px solid #a855f7; }}"
    )?;
    writeln!(
        output,
        "        .change-item.type-changed {{ background: #e0f2fe; border-left: 3px solid #0284c7; }}"
    )?;
    writeln!(
        output,
        "        .no-changes {{ text-align: center; padding: 3rem; color: #6b7280; }}"
    )?;
    writeln!(
        output,
        "        footer {{ margin-top: 3rem; padding-top: 2rem; border-top: 1px solid #e5e7eb; color: #6b7280; text-align: center; }}"
    )?;
    writeln!(
        output,
        "        a {{ color: inherit; text-decoration: none; }}"
    )?;
    writeln!(output, "        a:hover {{ text-decoration: underline; }}")?;
    writeln!(
        output,
        "        code {{ background: #f1f5f9; padding: 0.25rem 0.5rem; border-radius: 4px; font-family: 'Monaco', 'Courier New', monospace; }}"
    )?;
    writeln!(output, "    </style>")?;
    Ok(())
}

fn write_summary(
    output: &mut String,
    diff_result: &DiffResult,
    config: &RustreeLibConfig,
) -> Result<(), RustreeError> {
    writeln!(output, "        <div class=\"summary\">")?;
    writeln!(output, "            <h2>📊 Changes Summary</h2>")?;
    writeln!(output, "            <div class=\"stat-grid\">")?;

    // Added items
    if diff_result.summary.added > 0 {
        writeln!(output, "                <div class=\"stat-card added\">")?;
        if diff_result.summary.directories_added > 0 && diff_result.summary.files_added > 0 {
            writeln!(
                output,
                "                    <div>+{} Directories, +{} Files Added</div>",
                diff_result.summary.directories_added, diff_result.summary.files_added
            )?;
        } else if diff_result.summary.directories_added > 0 {
            writeln!(
                output,
                "                    <div>+{} Directories Added</div>",
                diff_result.summary.directories_added
            )?;
        } else if diff_result.summary.files_added > 0 {
            writeln!(
                output,
                "                    <div>+{} Files Added</div>",
                diff_result.summary.files_added
            )?;
        }
        writeln!(output, "                </div>")?;
    }

    // Removed items
    if diff_result.summary.removed > 0 {
        writeln!(output, "                <div class=\"stat-card removed\">")?;
        if diff_result.summary.directories_removed > 0 && diff_result.summary.files_removed > 0 {
            writeln!(
                output,
                "                    <div>-{} Directories, -{} Files Removed</div>",
                diff_result.summary.directories_removed, diff_result.summary.files_removed
            )?;
        } else if diff_result.summary.directories_removed > 0 {
            writeln!(
                output,
                "                    <div>-{} Directories Removed</div>",
                diff_result.summary.directories_removed
            )?;
        } else if diff_result.summary.files_removed > 0 {
            writeln!(
                output,
                "                    <div>-{} Files Removed</div>",
                diff_result.summary.files_removed
            )?;
        }
        writeln!(output, "                </div>")?;
    }

    // Moved items
    if diff_result.summary.moved > 0 {
        writeln!(output, "                <div class=\"stat-card moved\">")?;
        if diff_result.summary.directories_moved > 0 && diff_result.summary.files_moved > 0 {
            writeln!(
                output,
                "                    <div>{} Directories, {} Files Moved</div>",
                diff_result.summary.directories_moved, diff_result.summary.files_moved
            )?;
        } else if diff_result.summary.directories_moved > 0 {
            writeln!(
                output,
                "                    <div>{} Directories Moved</div>",
                diff_result.summary.directories_moved
            )?;
        } else if diff_result.summary.files_moved > 0 {
            writeln!(
                output,
                "                    <div>{} Files Moved</div>",
                diff_result.summary.files_moved
            )?;
        }
        writeln!(
            output,
            "                    <small>Renamed/relocated</small>"
        )?;
        writeln!(output, "                </div>")?;
    }

    if diff_result.summary.type_changed > 0 {
        writeln!(output, "                <div class=\"stat-card\">")?;
        writeln!(
            output,
            "                    <div>{} Type Changes</div>",
            diff_result.summary.type_changed
        )?;
        writeln!(
            output,
            "                    <small>File ↔ Directory</small>"
        )?;
        writeln!(output, "                </div>")?;
    }

    if diff_result.summary.modified > 0 {
        writeln!(output, "                <div class=\"stat-card modified\">")?;
        writeln!(
            output,
            "                    <div>{} Directories Modified</div>",
            diff_result.summary.modified
        )?;
        writeln!(output, "                </div>")?;
    }

    // Size change
    if config.metadata.show_size_bytes && diff_result.summary.size_change != 0 {
        let size_str =
            format_size_change(diff_result.summary.size_change, config.misc.human_friendly);
        writeln!(output, "                <div class=\"stat-card\">")?;
        writeln!(output, "                    <div>Total Size Change</div>")?;
        writeln!(output, "                    <small>{}</small>", size_str)?;
        writeln!(output, "                </div>")?;
    }

    writeln!(output, "            </div>")?;
    writeln!(output, "        </div>")?;
    Ok(())
}

fn write_tree_diff(
    output: &mut String,
    diff_result: &DiffResult,
    config: &RustreeLibConfig,
) -> Result<(), RustreeError> {
    writeln!(output, "        <pre class=\"tree-diff\">./")?;

    // Get root-level changes
    let root_changes: Vec<_> = diff_result
        .changes
        .iter()
        .filter(|c| {
            let path = c.path();
            path.components().count() <= 2 // Adjust based on depth
        })
        .collect();

    for (i, change) in root_changes.iter().enumerate() {
        let is_last = i == root_changes.len() - 1;
        write_change_tree_html(output, change, "", is_last, config)?;
    }

    writeln!(output, "</pre>")?;
    Ok(())
}

fn write_change_tree_html(
    output: &mut String,
    change: &Change,
    prefix: &str,
    is_last: bool,
    _config: &RustreeLibConfig,
) -> Result<(), RustreeError> {
    let (connector, extension) = if is_last {
        ("└── ", "    ")
    } else {
        ("├── ", "│   ")
    };

    write!(output, "{}{}", prefix, connector)?;

    // Get CSS class and symbol
    let (css_class, symbol) = match &change.change_type {
        ChangeType::Added => ("added", "[+]"),
        ChangeType::Removed => ("removed", "[-]"),
        ChangeType::Modified => ("modified", "[M]"),
        ChangeType::Moved { .. } => ("moved", "[~]"),
        ChangeType::TypeChanged { .. } => ("type-changed", "[T]"),
        ChangeType::Unchanged => ("unchanged", ""),
    };

    let display_name = change
        .path()
        .file_name()
        .unwrap_or(change.path().as_os_str())
        .to_string_lossy();

    write!(output, "<span class=\"{}\">{} ", css_class, symbol)?;

    if change.is_directory() {
        write!(output, "{}/", display_name)?;
    } else {
        write!(output, "{}", display_name)?;
    }

    // Add extra info
    match &change.change_type {
        ChangeType::Moved { from_path, .. } => {
            let from_name = from_path
                .file_name()
                .unwrap_or(from_path.as_os_str())
                .to_string_lossy();
            write!(output, " ← {}", from_name)?;
        }
        ChangeType::TypeChanged { from_type, to_type } => {
            write!(
                output,
                " ({} → {})",
                format_node_type(from_type),
                format_node_type(to_type)
            )?;
        }
        _ => {}
    }

    writeln!(output, "</span>")?;

    // Children
    if !change.children.is_empty() {
        let new_prefix = format!("{}{}", prefix, extension);
        for (i, child) in change.children.iter().enumerate() {
            let child_is_last = i == change.children.len() - 1;
            write_change_tree_html(output, child, &new_prefix, child_is_last, _config)?;
        }
    }

    Ok(())
}

fn write_detailed_changes(
    output: &mut String,
    diff_result: &DiffResult,
    _config: &RustreeLibConfig,
) -> Result<(), RustreeError> {
    // Count changes for auto-expand decision
    let total_changes = diff_result.summary.total_changes();
    let auto_expand = total_changes < 10;

    writeln!(
        output,
        "        <div class=\"expandable\" onclick=\"toggleSection('details')\">"
    )?;
    writeln!(
        output,
        "            <h3>📋 Detailed Changes (click to {})</h3>",
        if auto_expand { "collapse" } else { "expand" }
    )?;
    writeln!(output, "        </div>")?;

    let hidden_class = if auto_expand { "" } else { " hidden" };
    writeln!(
        output,
        "        <div id=\"details\" class=\"change-section{}\">\n",
        hidden_class
    )?;

    // Group and display each type
    write_change_group(
        output,
        &diff_result.changes,
        "Added Entities",
        ChangeType::Added,
        "added",
    )?;
    write_change_group(
        output,
        &diff_result.changes,
        "Removed Entities",
        ChangeType::Removed,
        "removed",
    )?;
    write_change_group_moved(output, &diff_result.changes)?;
    write_change_group_type_changed(output, &diff_result.changes)?;

    writeln!(output, "        </div>")?;
    Ok(())
}

fn write_change_group(
    output: &mut String,
    changes: &[Change],
    title: &str,
    change_type: ChangeType,
    css_class: &str,
) -> Result<(), RustreeError> {
    let filtered: Vec<_> = changes
        .iter()
        .filter(|c| std::mem::discriminant(&c.change_type) == std::mem::discriminant(&change_type))
        .collect();

    if !filtered.is_empty() {
        writeln!(output, "            <h4>{}</h4>", title)?;
        writeln!(output, "            <ul class=\"change-list\">")?;

        for change in filtered {
            let path = change.path().to_string_lossy();
            writeln!(
                output,
                "                <li class=\"change-item {}\">{}</li>",
                css_class, path
            )?;
        }

        writeln!(output, "            </ul>")?;
    }

    Ok(())
}

fn write_change_group_moved(output: &mut String, changes: &[Change]) -> Result<(), RustreeError> {
    let moved: Vec<_> = changes
        .iter()
        .filter(|c| matches!(c.change_type, ChangeType::Moved { .. }))
        .collect();

    if !moved.is_empty() {
        writeln!(output, "            <h4>Moved/Renamed Entities</h4>")?;
        writeln!(output, "            <ul class=\"change-list\">")?;

        for change in moved {
            if let ChangeType::Moved {
                from_path,
                similarity,
            } = &change.change_type
            {
                let to_path = change.path().to_string_lossy();
                let from_path_str = from_path.to_string_lossy();
                writeln!(
                    output,
                    "                <li class=\"change-item moved\"><code>{}</code> ← <code>{}</code> ({:.0}% similarity)</li>",
                    to_path,
                    from_path_str,
                    similarity * 100.0
                )?;
            }
        }

        writeln!(output, "            </ul>")?;
    }

    Ok(())
}

fn write_change_group_type_changed(
    output: &mut String,
    changes: &[Change],
) -> Result<(), RustreeError> {
    let type_changed: Vec<_> = changes
        .iter()
        .filter(|c| matches!(c.change_type, ChangeType::TypeChanged { .. }))
        .collect();

    if !type_changed.is_empty() {
        writeln!(output, "            <h4>Type Changes</h4>")?;
        writeln!(output, "            <ul class=\"change-list\">")?;

        for change in type_changed {
            if let ChangeType::TypeChanged { from_type, to_type } = &change.change_type {
                let path = change.path().to_string_lossy();
                writeln!(
                    output,
                    "                <li class=\"change-item type-changed\"><code>{}</code> changed from {} to {}</li>",
                    path,
                    format_node_type(from_type),
                    format_node_type(to_type)
                )?;
            }
        }

        writeln!(output, "            </ul>")?;
    }

    Ok(())
}

fn write_javascript(output: &mut String) -> Result<(), RustreeError> {
    writeln!(output, "    <script>")?;
    writeln!(output, "        function toggleSection(id) {{")?;
    writeln!(
        output,
        "            const element = document.getElementById(id);"
    )?;
    writeln!(output, "            element.classList.toggle('hidden');")?;
    writeln!(output, "        }}")?;
    writeln!(output, "    </script>")?;
    Ok(())
}

fn format_node_type(node_type: &NodeType) -> &'static str {
    match node_type {
        NodeType::File => "file",
        NodeType::Directory => "directory",
        NodeType::Symlink => "symlink",
    }
}
