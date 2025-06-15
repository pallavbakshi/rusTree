// src/core/diff/engine.rs

//! Core diff engine for comparing tree structures.

use crate::core::diff::changes::{
    Change, ChangeType, DiffMetadata, DiffOptions, DiffResult, DiffSummary,
};
use crate::core::error::RustreeError;
use crate::core::tree::node::{NodeInfo, NodeType};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// The main diff engine that compares two sets of nodes.
pub struct DiffEngine {
    options: DiffOptions,
}

/// Context for directory modification checking
struct DirCheckContext<'a> {
    previous_map: &'a HashMap<PathBuf, NodeInfo>,
    current_map: &'a HashMap<PathBuf, NodeInfo>,
    moves: &'a HashMap<PathBuf, (PathBuf, f64)>,
    processed_previous: &'a mut HashMap<PathBuf, bool>,
    processed_current: &'a mut HashMap<PathBuf, bool>,
    comparison_root: &'a Path,
    _options: &'a DiffOptions,
    // Add children caches for performance
    previous_children_cache: &'a HashMap<PathBuf, Vec<PathBuf>>,
    current_children_cache: &'a HashMap<PathBuf, Vec<PathBuf>>,
    // Add cycle detection
    processing_stack: &'a mut HashSet<PathBuf>,
}

impl DiffEngine {
    /// Creates a new diff engine with the given options.
    pub fn new(options: DiffOptions) -> Self {
        Self { options }
    }

    /// Compares two sets of nodes and produces a diff result.
    pub fn compare(
        &self,
        previous_nodes: &[NodeInfo],
        current_nodes: &[NodeInfo],
        metadata: DiffMetadata,
    ) -> Result<DiffResult, RustreeError> {
        // Build path maps for efficient lookup, normalizing paths relative to the comparison root
        let previous_map = build_path_map(previous_nodes, &metadata.comparison_root);
        let current_map = build_path_map(current_nodes, &metadata.comparison_root);

        // Build children caches for performance
        let previous_children_cache = build_children_cache(&previous_map);
        let current_children_cache = build_children_cache(&current_map);

        // Track which nodes we've already processed
        let mut processed_previous = HashMap::new();
        let mut processed_current = HashMap::new();

        // Cycle detection for recursive directory processing
        let mut processing_stack = HashSet::new();

        // Detect moves if enabled (with performance optimization)
        let moves = if self.options.detect_moves && !self.options.ignore_moves {
            detect_moves_optimized(&previous_map, &current_map, self.options.move_threshold)
        } else {
            HashMap::new()
        };

        let mut changes = Vec::new();
        let mut summary = DiffSummary::default();

        // Find all current nodes and classify them
        for (path, current_node) in &current_map {
            let change = if let Some(previous_node) = previous_map.get(path) {
                // Node exists in both - check if it's different
                processed_previous.insert(path.clone(), true);
                processed_current.insert(path.clone(), true);

                if current_node.node_type != previous_node.node_type {
                    // Type changed
                    Change::new(
                        ChangeType::TypeChanged {
                            from_type: previous_node.node_type.clone(),
                            to_type: current_node.node_type.clone(),
                        },
                        Some(normalize_node_info(current_node, &metadata.comparison_root)),
                        Some(normalize_node_info(
                            previous_node,
                            &metadata.comparison_root,
                        )),
                    )
                } else if current_node.node_type == NodeType::Directory {
                    // Directory - check if contents changed
                    let mut dir_change = Change::new(
                        ChangeType::Unchanged,
                        Some(normalize_node_info(current_node, &metadata.comparison_root)),
                        Some(normalize_node_info(
                            previous_node,
                            &metadata.comparison_root,
                        )),
                    );

                    let mut context = DirCheckContext {
                        previous_map: &previous_map,
                        current_map: &current_map,
                        moves: &moves,
                        processed_previous: &mut processed_previous,
                        processed_current: &mut processed_current,
                        comparison_root: &metadata.comparison_root,
                        _options: &self.options,
                        previous_children_cache: &previous_children_cache,
                        current_children_cache: &current_children_cache,
                        processing_stack: &mut processing_stack,
                    };

                    Self::check_directory_modified(&mut dir_change, &mut context);
                    dir_change
                } else {
                    // File - check if content changed (for now, assume unchanged)
                    Change::new(
                        ChangeType::Unchanged,
                        Some(normalize_node_info(current_node, &metadata.comparison_root)),
                        Some(normalize_node_info(
                            previous_node,
                            &metadata.comparison_root,
                        )),
                    )
                }
            } else if let Some((from_path, similarity)) = moves.get(path) {
                // This is a moved file
                processed_current.insert(path.clone(), true);
                if let Some(previous_node) = previous_map.get(from_path) {
                    processed_previous.insert(from_path.clone(), true);
                    Change::new(
                        ChangeType::Moved {
                            from_path: from_path.clone(),
                            similarity: *similarity,
                        },
                        Some(normalize_node_info(current_node, &metadata.comparison_root)),
                        Some(normalize_node_info(
                            previous_node,
                            &metadata.comparison_root,
                        )),
                    )
                } else {
                    // Shouldn't happen, but treat as added
                    Change::new(
                        ChangeType::Added,
                        Some(normalize_node_info(current_node, &metadata.comparison_root)),
                        None,
                    )
                }
            } else {
                // Added
                processed_current.insert(path.clone(), true);
                Change::new(
                    ChangeType::Added,
                    Some(normalize_node_info(current_node, &metadata.comparison_root)),
                    None,
                )
            };

            summary.add_change(&change);
            changes.push(change);
        }

        // Find all removed nodes (in previous but not in current)
        for (path, previous_node) in &previous_map {
            if !processed_previous.contains_key(path) {
                let change = Change::new(
                    ChangeType::Removed,
                    None,
                    Some(normalize_node_info(
                        previous_node,
                        &metadata.comparison_root,
                    )),
                );
                summary.add_change(&change);
                changes.push(change);
            }
        }

        // Sort changes by path for consistent output
        changes.sort_by(|a, b| a.path().cmp(b.path()));

        Ok(DiffResult {
            changes,
            summary,
            metadata,
        })
    }

    /// Checks if a directory has been modified by examining its children.
    #[allow(clippy::only_used_in_recursion)]
    fn check_directory_modified(dir_change: &mut Change, context: &mut DirCheckContext) {
        let dir_path = dir_change.path().clone();

        // Cycle detection: if we're already processing this directory, skip it
        if context.processing_stack.contains(&dir_path) {
            return;
        }

        // Add to processing stack
        context.processing_stack.insert(dir_path.clone());

        // Use cached children lookup for performance
        let previous_children = context
            .previous_children_cache
            .get(&dir_path)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        let current_children = context
            .current_children_cache
            .get(&dir_path)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        let mut has_changes = false;

        // Check current children
        for child_path in current_children {
            if !context.processed_current.contains_key(child_path) {
                context.processed_current.insert(child_path.clone(), true);
                has_changes = true;

                let current_child = context.current_map.get(child_path).unwrap();
                let child_change =
                    if let Some(previous_child) = context.previous_map.get(child_path) {
                        context.processed_previous.insert(child_path.clone(), true);

                        if current_child.node_type != previous_child.node_type {
                            Change::new(
                                ChangeType::TypeChanged {
                                    from_type: previous_child.node_type.clone(),
                                    to_type: current_child.node_type.clone(),
                                },
                                Some(normalize_node_info(current_child, context.comparison_root)),
                                Some(normalize_node_info(previous_child, context.comparison_root)),
                            )
                        } else if current_child.node_type == NodeType::Directory {
                            let mut nested_change = Change::new(
                                ChangeType::Unchanged,
                                Some(normalize_node_info(current_child, context.comparison_root)),
                                Some(normalize_node_info(previous_child, context.comparison_root)),
                            );
                            Self::check_directory_modified(&mut nested_change, context);
                            nested_change
                        } else {
                            Change::new(
                                ChangeType::Unchanged,
                                Some(normalize_node_info(current_child, context.comparison_root)),
                                Some(normalize_node_info(previous_child, context.comparison_root)),
                            )
                        }
                    } else if let Some((from_path, _similarity)) = context.moves.get(child_path) {
                        if let Some(previous_child) = context.previous_map.get(from_path) {
                            context.processed_previous.insert(from_path.clone(), true);
                            let similarity = calculate_similarity(previous_child, current_child);
                            Change::new(
                                ChangeType::Moved {
                                    from_path: from_path.clone(),
                                    similarity,
                                },
                                Some(normalize_node_info(current_child, context.comparison_root)),
                                Some(normalize_node_info(previous_child, context.comparison_root)),
                            )
                        } else {
                            Change::new(
                                ChangeType::Added,
                                Some(normalize_node_info(current_child, context.comparison_root)),
                                None,
                            )
                        }
                    } else {
                        Change::new(
                            ChangeType::Added,
                            Some(normalize_node_info(current_child, context.comparison_root)),
                            None,
                        )
                    };

                dir_change.add_child(child_change);
            }
        }

        // Check for removed children
        for child_path in previous_children {
            if !context.processed_previous.contains_key(child_path) {
                has_changes = true;
                if let Some(previous_child) = context.previous_map.get(child_path) {
                    let child_change = Change::new(
                        ChangeType::Removed,
                        None,
                        Some(normalize_node_info(previous_child, context.comparison_root)),
                    );
                    dir_change.add_child(child_change);
                    context.processed_previous.insert(child_path.clone(), true);
                }
            }
        }

        // Update the directory change type if children changed
        if has_changes {
            dir_change.change_type = ChangeType::Modified;
        }

        // Remove from processing stack when done
        context.processing_stack.remove(&dir_path);
    }
}

/// Builds a map from paths to NodeInfo for efficient lookup.
/// Normalizes paths to be relative for consistent comparison between JSON and filesystem sources.
fn build_path_map(nodes: &[NodeInfo], comparison_root: &Path) -> HashMap<PathBuf, NodeInfo> {
    nodes
        .iter()
        .map(|node| {
            let normalized_path = normalize_path_for_diff(&node.path, comparison_root);
            (normalized_path, node.clone())
        })
        .collect()
}

/// Normalizes a path for diff comparison by making it relative to the comparison root.
/// This ensures filesystem paths (which are absolute) can be compared with JSON paths (which are relative).
fn normalize_path_for_diff(path: &Path, comparison_root: &Path) -> PathBuf {
    // If the path is already relative, normalise it by removing any leading
    // "./" segments that are semantically irrelevant but would otherwise make
    // two logically identical paths (`./src/foo.rs` vs `src/foo.rs`) compare
    // unequal.
    if path.is_relative() {
        use std::path::Component;
        let cleaned: PathBuf = path
            .components()
            .filter(|c| !matches!(c, Component::CurDir))
            .collect();
        return cleaned;
    }

    // Try to make the path relative to the comparison root
    if let Ok(relative_path) = path.strip_prefix(comparison_root) {
        return relative_path.to_path_buf();
    }

    // If strip_prefix fails, try with canonicalized paths
    if let (Ok(canonical_path), Ok(canonical_root)) = (
        std::fs::canonicalize(path),
        std::fs::canonicalize(comparison_root),
    ) {
        if let Ok(relative_path) = canonical_path.strip_prefix(canonical_root) {
            return relative_path.to_path_buf();
        }
    }

    // Fallback: return the original path
    path.to_path_buf()
}

/// Normalizes the paths in a NodeInfo to be relative to the comparison root.
fn normalize_node_info(node: &NodeInfo, comparison_root: &Path) -> NodeInfo {
    let mut normalized = node.clone();
    normalized.path = normalize_path_for_diff(&node.path, comparison_root);
    normalized
}

/// Builds a cache mapping each directory to its direct children for O(1) lookup.
fn build_children_cache(node_map: &HashMap<PathBuf, NodeInfo>) -> HashMap<PathBuf, Vec<PathBuf>> {
    let mut children_cache: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();

    for path in node_map.keys() {
        if let Some(parent) = path.parent() {
            if parent != path {
                // Avoid self-references
                children_cache
                    .entry(parent.to_path_buf())
                    .or_default()
                    .push(path.clone());
            }
        }
    }

    children_cache
}

/// Detects moves between two sets of nodes (optimized version).
fn detect_moves_optimized(
    previous_map: &HashMap<PathBuf, NodeInfo>,
    current_map: &HashMap<PathBuf, NodeInfo>,
    threshold: f64,
) -> HashMap<PathBuf, (PathBuf, f64)> {
    let mut moves = HashMap::new();

    // Pre-filter candidates for better performance
    let unmatched_previous: Vec<&PathBuf> = previous_map
        .keys()
        .filter(|path| !current_map.contains_key(*path))
        .collect();

    let unmatched_current: Vec<&PathBuf> = current_map
        .keys()
        .filter(|path| !previous_map.contains_key(*path))
        .collect();

    // Early exit if one list is much larger (avoid O(n²) when impractical)
    if unmatched_previous.len() > 1000 || unmatched_current.len() > 1000 {
        let max_comparisons = 10000; // Limit total comparisons
        if unmatched_previous.len() * unmatched_current.len() > max_comparisons {
            return moves; // Skip move detection for very large sets
        }
    }

    // Group by node type and size for faster matching
    let mut previous_by_type_size: HashMap<(NodeType, Option<u64>), Vec<&PathBuf>> = HashMap::new();
    for path in &unmatched_previous {
        if let Some(node) = previous_map.get(*path) {
            previous_by_type_size
                .entry((node.node_type.clone(), node.size))
                .or_default()
                .push(path);
        }
    }

    // Find matches using the grouped structure
    for current_path in &unmatched_current {
        if let Some(current_node) = current_map.get(*current_path) {
            let key = (current_node.node_type.clone(), current_node.size);

            if let Some(candidates) = previous_by_type_size.get(&key) {
                let mut best_match: Option<&PathBuf> = None;
                let mut best_similarity = 0.0;

                for &previous_path in candidates {
                    if let Some(previous_node) = previous_map.get(previous_path) {
                        let similarity = calculate_similarity(previous_node, current_node);
                        if similarity >= threshold && similarity > best_similarity {
                            best_similarity = similarity;
                            best_match = Some(previous_path);
                        }
                    }
                }

                if let Some(from_path) = best_match {
                    moves.insert(
                        (*current_path).clone(),
                        (from_path.clone(), best_similarity),
                    );

                    // Remove the matched path from candidates to avoid duplicate matches
                    if let Some(candidates) = previous_by_type_size.get_mut(&key) {
                        candidates.retain(|&path| path != from_path);
                    }
                }
            }
        }
    }

    moves
}

/// Gets all direct children of a directory path (legacy function for tests).
#[allow(dead_code)]
fn get_children(parent: &Path, node_map: &HashMap<PathBuf, NodeInfo>) -> Vec<PathBuf> {
    node_map
        .keys()
        .filter(|path| {
            if let Some(path_parent) = path.parent() {
                path_parent == parent && *path != parent
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

/// Detects moves between two sets of nodes (legacy function for tests).
#[allow(dead_code)]
fn detect_moves(
    previous_map: &HashMap<PathBuf, NodeInfo>,
    current_map: &HashMap<PathBuf, NodeInfo>,
    threshold: f64,
) -> HashMap<PathBuf, (PathBuf, f64)> {
    detect_moves_optimized(previous_map, current_map, threshold)
}

/// Calculates similarity between two nodes for move detection.
/// Returns a value between 0.0 (completely different) and 1.0 (identical).
fn calculate_similarity(previous: &NodeInfo, current: &NodeInfo) -> f64 {
    let mut score = 0.0;
    let mut factors = 0.0;

    // Compare file names
    if previous.name == current.name {
        score += 0.4;
    } else {
        // Calculate name similarity using simple heuristics
        let name_sim = calculate_name_similarity(&previous.name, &current.name);
        score += 0.4 * name_sim;
    }
    factors += 0.4;

    // Compare file sizes (if available)
    if let (Some(prev_size), Some(curr_size)) = (previous.size, current.size) {
        if prev_size == curr_size {
            score += 0.4;
        } else {
            // Size similarity based on ratio
            let size_ratio = if prev_size > curr_size {
                curr_size as f64 / prev_size as f64
            } else {
                prev_size as f64 / curr_size as f64
            };
            score += 0.4 * size_ratio;
        }
        factors += 0.4;
    }

    // Compare modification times (if available)
    if let (Some(prev_mtime), Some(curr_mtime)) = (previous.mtime, current.mtime) {
        if prev_mtime == curr_mtime {
            score += 0.2;
        } else {
            // Time similarity (closer times = higher similarity)
            let time_diff = if prev_mtime > curr_mtime {
                prev_mtime.duration_since(curr_mtime)
            } else {
                curr_mtime.duration_since(prev_mtime)
            };

            if let Ok(duration) = time_diff {
                let seconds_diff = duration.as_secs();
                // Consider files modified within an hour as potentially similar
                let time_sim = if seconds_diff == 0 {
                    1.0
                } else if seconds_diff < 3600 {
                    1.0 - (seconds_diff as f64 / 3600.0)
                } else {
                    0.0
                };
                score += 0.2 * time_sim;
            }
        }
        factors += 0.2;
    }

    if factors > 0.0 { score / factors } else { 0.0 }
}

/// Calculates name similarity using basic string comparison.
fn calculate_name_similarity(name1: &str, name2: &str) -> f64 {
    if name1 == name2 {
        return 1.0;
    }

    // Simple edit distance approximation
    let len1 = name1.len();
    let len2 = name2.len();
    let max_len = len1.max(len2);

    if max_len == 0 {
        return 1.0;
    }

    // Count matching characters at the same positions
    let mut matches = 0;
    let min_len = len1.min(len2);

    for (c1, c2) in name1.chars().take(min_len).zip(name2.chars().take(min_len)) {
        if c1 == c2 {
            matches += 1;
        }
    }

    matches as f64 / max_len as f64
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn create_test_metadata() -> DiffMetadata {
        DiffMetadata {
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
        }
    }

    #[test]
    fn test_diff_engine_new() {
        let options = DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        };
        let engine = DiffEngine::new(options.clone());
        assert_eq!(engine.options.detect_moves, options.detect_moves);
    }

    #[test]
    fn test_added_files() {
        let engine = DiffEngine::new(DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        });
        let previous = vec![];
        let current = vec![
            create_test_node("file1.txt", NodeType::File, Some(100)),
            create_test_node("dir1", NodeType::Directory, None),
        ];

        let result = engine
            .compare(&previous, &current, create_test_metadata())
            .unwrap();

        assert_eq!(result.changes.len(), 2);
        assert_eq!(result.summary.added, 2);
        assert_eq!(result.summary.files_added, 1);
        assert_eq!(result.summary.directories_added, 1);
        assert_eq!(result.summary.removed, 0);
        assert_eq!(result.summary.moved, 0);

        for change in &result.changes {
            assert!(matches!(change.change_type, ChangeType::Added));
            assert!(change.current.is_some());
            assert!(change.previous.is_none());
        }
    }

    #[test]
    fn test_removed_files() {
        let engine = DiffEngine::new(DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        });
        let previous = vec![
            create_test_node("file1.txt", NodeType::File, Some(100)),
            create_test_node("dir1", NodeType::Directory, None),
        ];
        let current = vec![];

        let result = engine
            .compare(&previous, &current, create_test_metadata())
            .unwrap();

        assert_eq!(result.changes.len(), 2);
        assert_eq!(result.summary.added, 0);
        assert_eq!(result.summary.removed, 2);
        assert_eq!(result.summary.files_removed, 1);
        assert_eq!(result.summary.directories_removed, 1);
        assert_eq!(result.summary.moved, 0);

        for change in &result.changes {
            assert!(matches!(change.change_type, ChangeType::Removed));
            assert!(change.current.is_none());
            assert!(change.previous.is_some());
        }
    }

    #[test]
    fn test_unchanged_files() {
        let engine = DiffEngine::new(DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        });
        let nodes = vec![
            create_test_node("file1.txt", NodeType::File, Some(100)),
            create_test_node("dir1", NodeType::Directory, None),
        ];

        let result = engine
            .compare(&nodes, &nodes, create_test_metadata())
            .unwrap();

        assert_eq!(result.changes.len(), 2);
        assert_eq!(result.summary.added, 0);
        assert_eq!(result.summary.removed, 0);
        assert_eq!(result.summary.moved, 0);
        assert_eq!(result.summary.unchanged, 2);

        for change in &result.changes {
            assert!(matches!(change.change_type, ChangeType::Unchanged));
            assert!(change.current.is_some());
            assert!(change.previous.is_some());
        }
    }

    #[test]
    fn test_type_changed() {
        let engine = DiffEngine::new(DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        });
        let previous = vec![create_test_node("item", NodeType::File, Some(100))];
        let current = vec![create_test_node("item", NodeType::Directory, None)];

        let result = engine
            .compare(&previous, &current, create_test_metadata())
            .unwrap();

        assert_eq!(result.changes.len(), 1);
        assert_eq!(result.summary.type_changed, 1);

        let change = &result.changes[0];
        match &change.change_type {
            ChangeType::TypeChanged { from_type, to_type } => {
                assert_eq!(*from_type, NodeType::File);
                assert_eq!(*to_type, NodeType::Directory);
            }
            _ => panic!("Expected TypeChanged"),
        }
    }

    #[test]
    fn test_move_detection_disabled() {
        let mut options = DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        };
        options.ignore_moves = true;
        let engine = DiffEngine::new(options);

        let previous = vec![create_test_node("old_name.txt", NodeType::File, Some(100))];
        let current = vec![create_test_node("new_name.txt", NodeType::File, Some(100))];

        let result = engine
            .compare(&previous, &current, create_test_metadata())
            .unwrap();

        assert_eq!(result.changes.len(), 2);
        assert_eq!(result.summary.added, 1);
        assert_eq!(result.summary.removed, 1);
        assert_eq!(result.summary.moved, 0);
    }

    #[test]
    fn test_move_detection_enabled() {
        let mut options = DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        };
        options.detect_moves = true;
        options.move_threshold = 0.5;
        let engine = DiffEngine::new(options);

        let previous = vec![create_test_node("old_name.txt", NodeType::File, Some(100))];
        let current = vec![create_test_node("new_name.txt", NodeType::File, Some(100))];

        let result = engine
            .compare(&previous, &current, create_test_metadata())
            .unwrap();

        // Should detect as move due to same size and similar name
        assert_eq!(result.changes.len(), 1);
        assert_eq!(result.summary.moved, 1);
        assert_eq!(result.summary.files_moved, 1);

        let change = &result.changes[0];
        match &change.change_type {
            ChangeType::Moved {
                from_path,
                similarity,
            } => {
                assert_eq!(from_path, &PathBuf::from("old_name.txt"));
                assert!(*similarity > 0.5);
            }
            _ => panic!("Expected Moved, got {:?}", change.change_type),
        }
    }

    #[test]
    fn test_calculate_similarity_identical() {
        let node1 = create_test_node("test.txt", NodeType::File, Some(100));
        let node2 = create_test_node("test.txt", NodeType::File, Some(100));

        let similarity = calculate_similarity(&node1, &node2);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_calculate_similarity_different() {
        let node1 = create_test_node("file1.txt", NodeType::File, Some(100));
        let node2 = create_test_node("file2.txt", NodeType::File, Some(200));

        let similarity = calculate_similarity(&node1, &node2);
        assert!(similarity > 0.0);
        assert!(similarity < 1.0);
    }

    #[test]
    fn test_calculate_name_similarity() {
        assert_eq!(calculate_name_similarity("test", "test"), 1.0);
        assert_eq!(calculate_name_similarity("test", "best"), 0.75);
        assert_eq!(calculate_name_similarity("abc", "xyz"), 0.0);
        assert!(calculate_name_similarity("test.txt", "test.rs") > 0.5);
    }

    #[test]
    fn test_normalize_path_for_diff_relative() {
        let path = PathBuf::from("src/main.rs");
        let root = PathBuf::from(".");
        let normalized = normalize_path_for_diff(&path, &root);
        assert_eq!(normalized, path);
    }

    #[test]
    fn test_build_path_map() {
        let nodes = vec![
            create_test_node("file1.txt", NodeType::File, Some(100)),
            create_test_node("dir1", NodeType::Directory, None),
        ];
        let root = PathBuf::from(".");

        let map = build_path_map(&nodes, &root);
        assert_eq!(map.len(), 2);
        assert!(map.contains_key(&PathBuf::from("file1.txt")));
        assert!(map.contains_key(&PathBuf::from("dir1")));
    }

    #[test]
    fn test_get_children() {
        let mut nodes = HashMap::new();
        nodes.insert(
            PathBuf::from("parent"),
            create_test_node("parent", NodeType::Directory, None),
        );
        nodes.insert(
            PathBuf::from("parent/child1"),
            create_test_node("child1", NodeType::File, Some(100)),
        );
        nodes.insert(
            PathBuf::from("parent/child2"),
            create_test_node("child2", NodeType::File, Some(200)),
        );
        nodes.insert(
            PathBuf::from("other"),
            create_test_node("other", NodeType::File, Some(50)),
        );

        let children = get_children(&PathBuf::from("parent"), &nodes);
        assert_eq!(children.len(), 2);
        assert!(children.contains(&PathBuf::from("parent/child1")));
        assert!(children.contains(&PathBuf::from("parent/child2")));
    }

    #[test]
    fn test_detect_moves() {
        let mut previous = HashMap::new();
        previous.insert(
            PathBuf::from("old.txt"),
            create_test_node("old.txt", NodeType::File, Some(100)),
        );

        let mut current = HashMap::new();
        current.insert(
            PathBuf::from("new.txt"),
            create_test_node("new.txt", NodeType::File, Some(100)),
        );

        let moves = detect_moves(&previous, &current, 0.5);
        assert_eq!(moves.len(), 1);
        assert!(moves.contains_key(&PathBuf::from("new.txt")));

        let (from_path, similarity) = &moves[&PathBuf::from("new.txt")];
        assert_eq!(*from_path, PathBuf::from("old.txt"));
        assert!(*similarity >= 0.5);
    }

    #[test]
    fn test_mixed_changes() {
        let engine = DiffEngine::new(DiffOptions {
            max_depth: None,
            show_size: true,
            sort_by: None,
            detect_moves: true,
            move_threshold: 0.8,
            show_unchanged: false,
            ignore_moves: false,
        });
        let previous = vec![
            create_test_node("keep.txt", NodeType::File, Some(100)),
            create_test_node("remove.txt", NodeType::File, Some(200)),
            create_test_node("change_type", NodeType::File, Some(50)),
        ];
        let current = vec![
            create_test_node("keep.txt", NodeType::File, Some(100)),
            create_test_node("add.txt", NodeType::File, Some(300)),
            create_test_node("change_type", NodeType::Directory, None),
        ];

        let result = engine
            .compare(&previous, &current, create_test_metadata())
            .unwrap();

        assert_eq!(result.summary.unchanged, 1);
        assert_eq!(result.summary.added, 1);
        assert_eq!(result.summary.removed, 1);
        assert_eq!(result.summary.type_changed, 1);

        // Total changes should account for all items processed
        let total_items = result.summary.unchanged
            + result.summary.added
            + result.summary.removed
            + result.summary.type_changed;
        assert_eq!(total_items, 4); // keep, add, remove, change_type
    }
}
