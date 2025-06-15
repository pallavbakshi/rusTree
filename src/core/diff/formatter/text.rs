// src/core/diff/formatter/text.rs

//! Text formatter for diff results, producing tree-style output with change markers.

use crate::core::diff::formatter::{
    DiffFormatter, change_type_color, change_type_symbol, format_size_change,
};
use crate::core::diff::{Change, ChangeType, DiffResult};
use crate::core::error::RustreeError;
use crate::core::options::RustreeLibConfig;
use crate::core::tree::node::NodeType;
use is_terminal::IsTerminal;
use std::fmt::Write;
use std::io;

pub struct TextDiffFormatter;

impl DiffFormatter for TextDiffFormatter {
    fn format(
        &self,
        diff_result: &DiffResult,
        config: &RustreeLibConfig,
    ) -> Result<String, RustreeError> {
        let mut output = String::new();

        // Format the tree
        writeln!(&mut output, "./")?;

        // Show only changes that are not "Unchanged"
        let mut changes_to_show: Vec<&Change> = diff_result
            .changes
            .iter()
            .filter(|c| !matches!(c.change_type, ChangeType::Unchanged))
            .collect();

        // Sort for consistent output
        changes_to_show.sort_by_key(|c| c.path());

        // Format each change recursively
        for (i, change) in changes_to_show.iter().enumerate() {
            let is_last = i == changes_to_show.len() - 1;
            format_change_tree(
                &mut output,
                change,
                "",
                is_last,
                config,
                &diff_result.metadata.comparison_root.to_string_lossy(),
            )?;
        }

        // Add summary if not disabled
        if !config.misc.no_summary_report {
            writeln!(&mut output)?;
            writeln!(&mut output, "Changes Summary:")?;

            // Added items
            if diff_result.summary.added > 0 {
                if diff_result.summary.directories_added > 0 && diff_result.summary.files_added > 0
                {
                    writeln!(
                        &mut output,
                        "  {} directories added, {} files added (+)",
                        diff_result.summary.directories_added, diff_result.summary.files_added
                    )?;
                } else if diff_result.summary.directories_added > 0 {
                    writeln!(
                        &mut output,
                        "  {} directories added (+)",
                        diff_result.summary.directories_added
                    )?;
                } else if diff_result.summary.files_added > 0 {
                    writeln!(
                        &mut output,
                        "  {} files added (+)",
                        diff_result.summary.files_added
                    )?;
                }
            }

            // Removed items
            if diff_result.summary.removed > 0 {
                if diff_result.summary.directories_removed > 0
                    && diff_result.summary.files_removed > 0
                {
                    writeln!(
                        &mut output,
                        "  {} directories removed, {} files removed (-)",
                        diff_result.summary.directories_removed, diff_result.summary.files_removed
                    )?;
                } else if diff_result.summary.directories_removed > 0 {
                    writeln!(
                        &mut output,
                        "  {} directories removed (-)",
                        diff_result.summary.directories_removed
                    )?;
                } else if diff_result.summary.files_removed > 0 {
                    writeln!(
                        &mut output,
                        "  {} files removed (-)",
                        diff_result.summary.files_removed
                    )?;
                }
            }

            // Moved items
            if diff_result.summary.moved > 0 {
                if diff_result.summary.directories_moved > 0 && diff_result.summary.files_moved > 0
                {
                    writeln!(
                        &mut output,
                        "  {} directories moved, {} files moved/renamed (~)",
                        diff_result.summary.directories_moved, diff_result.summary.files_moved
                    )?;
                } else if diff_result.summary.directories_moved > 0 {
                    writeln!(
                        &mut output,
                        "  {} directories moved/renamed (~)",
                        diff_result.summary.directories_moved
                    )?;
                } else if diff_result.summary.files_moved > 0 {
                    writeln!(
                        &mut output,
                        "  {} files moved/renamed (~)",
                        diff_result.summary.files_moved
                    )?;
                }
            }

            if diff_result.summary.type_changed > 0 {
                writeln!(
                    &mut output,
                    "  {} type changes (T)",
                    diff_result.summary.type_changed
                )?;
            }
            if diff_result.summary.modified > 0 {
                writeln!(
                    &mut output,
                    "  {} directories modified (M)",
                    diff_result.summary.modified
                )?;
            }
            if diff_result.metadata.options.show_unchanged && diff_result.summary.unchanged > 0 {
                writeln!(
                    &mut output,
                    "  {} items unchanged",
                    diff_result.summary.unchanged
                )?;
            }

            // Show total size change if requested
            if config.metadata.show_size_bytes && diff_result.summary.size_change != 0 {
                let size_str =
                    format_size_change(diff_result.summary.size_change, config.misc.human_friendly);
                writeln!(&mut output, "  Total size change: {}", size_str)?;
            }
        }

        Ok(output)
    }
}

fn format_change_tree(
    output: &mut String,
    change: &Change,
    prefix: &str,
    is_last: bool,
    config: &RustreeLibConfig,
    _root_path: &str,
) -> Result<(), RustreeError> {
    // Determine the tree characters
    let (connector, extension) = if is_last {
        ("└── ", "    ")
    } else {
        ("├── ", "│   ")
    };

    // Format the current node
    write!(output, "{}{}", prefix, connector)?;

    // Add change marker and color
    let use_color = !config.misc.no_color && io::stdout().is_terminal();
    if use_color {
        write!(output, "{}", change_type_color(&change.change_type))?;
    }

    write!(output, "{} ", change_type_symbol(&change.change_type))?;

    // Get the display name - paths are already relative due to normalization in diff engine
    let path = change.path();
    let display_name = if config.listing.show_full_path {
        path.to_string_lossy().to_string()
    } else {
        // Path is already relative from the diff engine normalization
        path.to_string_lossy().to_string()
    };

    // Add slash for directories
    let is_dir = change.is_directory();
    if is_dir {
        write!(output, "{}/", display_name)?;
    } else {
        write!(output, "{}", display_name)?;
    }

    // Add additional info based on change type
    match &change.change_type {
        ChangeType::Moved {
            from_path,
            similarity,
        } => {
            write!(
                output,
                " ← {}",
                from_path
                    .file_name()
                    .unwrap_or(from_path.as_os_str())
                    .to_string_lossy()
            )?;
            if config.misc.verbose {
                write!(output, " ({:.0}% similar)", similarity * 100.0)?;
            }
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

    // Add size info if requested
    if config.metadata.show_size_bytes && !is_dir {
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

        // Show size change for modified files
        let size_change = change.size_change();
        if size_change != 0 && matches!(change.change_type, ChangeType::Moved { .. }) {
            let change_str = format_size_change(size_change, config.misc.human_friendly);
            write!(output, " [{}]", change_str)?;
        }
    }

    if use_color {
        write!(output, "\x1b[0m")?; // Reset color
    }

    writeln!(output)?;

    // Format children for modified directories
    if !change.children.is_empty() {
        let new_prefix = format!("{}{}", prefix, extension);
        let child_count = change.children.len();

        for (i, child) in change.children.iter().enumerate() {
            let child_is_last = i == child_count - 1;
            format_change_tree(
                output,
                child,
                &new_prefix,
                child_is_last,
                config,
                _root_path,
            )?;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::diff::changes::{ChangeType, DiffMetadata, DiffOptions, DiffSummary};
    use crate::core::options::{MiscOptions, RustreeLibConfig};
    use crate::core::tree::node::{NodeInfo, NodeType};
    use std::path::PathBuf;
    use std::time::SystemTime;

    fn create_test_node(name: &str, node_type: NodeType, size: Option<u64>) -> NodeInfo {
        NodeInfo {
            name: name.to_string(),
            path: PathBuf::from(name),
            node_type,
            depth: 0,
            size,
            mtime: Some(SystemTime::UNIX_EPOCH),
            change_time: None,
            create_time: None,
            permissions: None,
            line_count: None,
            word_count: None,
            custom_function_output: None,
        }
    }

    fn create_test_config() -> RustreeLibConfig {
        RustreeLibConfig {
            misc: MiscOptions {
                human_friendly: false,
                no_color: true,
                verbose: false,
                no_summary_report: false,
            },
            ..Default::default()
        }
    }

    fn create_test_diff_result() -> DiffResult {
        let file_node = create_test_node("test.txt", NodeType::File, Some(100));
        let change = Change::new(ChangeType::Added, Some(file_node), None);
        let mut summary = DiffSummary::new();
        summary.add_change(&change);

        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions {
                max_depth: None,
                show_size: true,
                sort_by: None,
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        DiffResult {
            changes: vec![change],
            summary,
            metadata,
        }
    }

    #[test]
    fn test_text_formatter_basic() {
        let formatter = TextDiffFormatter;
        let diff_result = create_test_diff_result();
        let config = create_test_config();

        let result = formatter.format(&diff_result, &config).unwrap();

        assert!(result.contains("./"));
        assert!(result.contains("test.txt"));
        assert!(result.contains("[+]"));
        assert!(result.contains("Changes Summary:"));
        assert!(result.contains("1 files added (+)"));
    }

    #[test]
    fn test_text_formatter_added_file() {
        let formatter = TextDiffFormatter;
        let file_node = create_test_node("new_file.txt", NodeType::File, Some(256));
        let change = Change::new(ChangeType::Added, Some(file_node), None);
        let mut summary = DiffSummary::new();
        summary.add_change(&change);

        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions {
                max_depth: None,
                show_size: true,
                sort_by: None,
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        let diff_result = DiffResult {
            changes: vec![change],
            summary,
            metadata,
        };
        let config = create_test_config();

        let result = formatter.format(&diff_result, &config).unwrap();

        assert!(result.contains("[+] new_file.txt"));
        assert!(result.contains("1 files added (+)"));
    }

    #[test]
    fn test_text_formatter_removed_file() {
        let formatter = TextDiffFormatter;
        let file_node = create_test_node("deleted.txt", NodeType::File, Some(128));
        let change = Change::new(ChangeType::Removed, None, Some(file_node));
        let mut summary = DiffSummary::new();
        summary.add_change(&change);

        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions {
                max_depth: None,
                show_size: true,
                sort_by: None,
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        let diff_result = DiffResult {
            changes: vec![change],
            summary,
            metadata,
        };
        let config = create_test_config();

        let result = formatter.format(&diff_result, &config).unwrap();

        assert!(result.contains("[-] deleted.txt"));
        assert!(result.contains("1 files removed (-)"));
    }

    #[test]
    fn test_text_formatter_moved_file() {
        let formatter = TextDiffFormatter;
        let old_node = create_test_node("old_name.txt", NodeType::File, Some(100));
        let new_node = create_test_node("new_name.txt", NodeType::File, Some(100));
        let change = Change::new(
            ChangeType::Moved {
                from_path: PathBuf::from("old_name.txt"),
                similarity: 0.95,
            },
            Some(new_node),
            Some(old_node),
        );
        let mut summary = DiffSummary::new();
        summary.add_change(&change);

        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions {
                max_depth: None,
                show_size: true,
                sort_by: None,
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        let diff_result = DiffResult {
            changes: vec![change],
            summary,
            metadata,
        };
        let config = create_test_config();

        let result = formatter.format(&diff_result, &config).unwrap();

        assert!(result.contains("[~] new_name.txt ← old_name.txt"));
        assert!(result.contains("1 files moved/renamed (~)"));
    }

    #[test]
    fn test_text_formatter_type_changed() {
        let formatter = TextDiffFormatter;
        let old_node = create_test_node("item", NodeType::File, Some(100));
        let new_node = create_test_node("item", NodeType::Directory, None);
        let change = Change::new(
            ChangeType::TypeChanged {
                from_type: NodeType::File,
                to_type: NodeType::Directory,
            },
            Some(new_node),
            Some(old_node),
        );
        let mut summary = DiffSummary::new();
        summary.add_change(&change);

        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions {
                max_depth: None,
                show_size: true,
                sort_by: None,
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        let diff_result = DiffResult {
            changes: vec![change],
            summary,
            metadata,
        };
        let config = create_test_config();

        let result = formatter.format(&diff_result, &config).unwrap();

        assert!(result.contains("[T] item/ (file → directory)"));
        assert!(result.contains("1 type changes (T)"));
    }

    #[test]
    fn test_text_formatter_with_size_info() {
        let formatter = TextDiffFormatter;
        let file_node = create_test_node("large_file.txt", NodeType::File, Some(2048));
        let change = Change::new(ChangeType::Added, Some(file_node), None);
        let mut summary = DiffSummary::new();
        summary.add_change(&change);

        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions {
                max_depth: None,
                show_size: true,
                sort_by: None,
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        let diff_result = DiffResult {
            changes: vec![change],
            summary,
            metadata,
        };
        let mut config = create_test_config();
        config.metadata.show_size_bytes = true;

        let result = formatter.format(&diff_result, &config).unwrap();

        assert!(result.contains("[+] large_file.txt (2048 B)"));
        assert!(result.contains("Total size change: +2048 B"));
    }

    #[test]
    fn test_text_formatter_human_readable_size() {
        let formatter = TextDiffFormatter;
        let file_node = create_test_node("big_file.txt", NodeType::File, Some(1536)); // 1.5 KB
        let change = Change::new(ChangeType::Added, Some(file_node), None);
        let mut summary = DiffSummary::new();
        summary.add_change(&change);

        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions {
                max_depth: None,
                show_size: true,
                sort_by: None,
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        let diff_result = DiffResult {
            changes: vec![change],
            summary,
            metadata,
        };
        let mut config = create_test_config();
        config.metadata.show_size_bytes = true;
        config.misc.human_friendly = true;

        let result = formatter.format(&diff_result, &config).unwrap();

        assert!(result.contains("[+] big_file.txt (1.5 KB)"));
    }

    #[test]
    fn test_text_formatter_no_summary() {
        let formatter = TextDiffFormatter;
        let diff_result = create_test_diff_result();
        let mut config = create_test_config();
        config.misc.no_summary_report = true;

        let result = formatter.format(&diff_result, &config).unwrap();

        assert!(result.contains("./"));
        assert!(result.contains("test.txt"));
        assert!(!result.contains("Changes Summary:"));
    }

    #[test]
    fn test_text_formatter_mixed_changes() {
        let formatter = TextDiffFormatter;

        // Create multiple changes
        let added_file = create_test_node("added.txt", NodeType::File, Some(100));
        let removed_file = create_test_node("removed.txt", NodeType::File, Some(200));
        let added_dir = create_test_node("new_dir", NodeType::Directory, None);

        let changes = vec![
            Change::new(ChangeType::Added, Some(added_file), None),
            Change::new(ChangeType::Removed, None, Some(removed_file)),
            Change::new(ChangeType::Added, Some(added_dir), None),
        ];

        let mut summary = DiffSummary::new();
        for change in &changes {
            summary.add_change(change);
        }

        let metadata = DiffMetadata {
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            snapshot_file: PathBuf::from("test.json"),
            snapshot_date: None,
            comparison_root: PathBuf::from("."),
            filters_applied: vec![],
            options: DiffOptions {
                max_depth: None,
                show_size: true,
                sort_by: None,
                detect_moves: true,
                move_threshold: 0.8,
                show_unchanged: false,
                ignore_moves: false,
            },
        };

        let diff_result = DiffResult {
            changes,
            summary,
            metadata,
        };
        let config = create_test_config();

        let result = formatter.format(&diff_result, &config).unwrap();

        assert!(result.contains("[+] added.txt"));
        assert!(result.contains("[-] removed.txt"));
        assert!(result.contains("[+] new_dir/"));
        assert!(result.contains("1 directories added, 1 files added (+)"));
        assert!(result.contains("1 files removed (-)"));
    }

    #[test]
    fn test_format_human_size() {
        assert_eq!(format_human_size(512), "512 B");
        assert_eq!(format_human_size(1024), "1.0 KB");
        assert_eq!(format_human_size(1536), "1.5 KB");
        assert_eq!(format_human_size(1048576), "1.0 MB");
        assert_eq!(format_human_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_format_node_type() {
        assert_eq!(format_node_type(&NodeType::File), "file");
        assert_eq!(format_node_type(&NodeType::Directory), "directory");
        assert_eq!(format_node_type(&NodeType::Symlink), "symlink");
    }
}
