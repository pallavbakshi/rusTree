// src/core/diff/mod.rs

//! Diff functionality for comparing directory trees.
//!
//! This module provides algorithms and data structures for comparing two tree snapshots
//! and detecting various types of changes (additions, deletions, modifications, moves).

pub mod changes;
pub mod engine;
pub mod formatter;

// Re-export key types
pub use changes::{Change, ChangeType, DiffResult, DiffSummary};
// Additional frequently-used structures that are consumed directly by external
// callers (including integration tests) are re-exported here as well so that
// they can be imported via `rustree::core::diff::*` without having to know the
// internal sub-module layout.
pub use changes::{DiffMetadata, DiffOptions};
pub use engine::DiffEngine;
pub use formatter::{DiffFormatter, format_diff};
