//! File system traversal and directory walking functionality.
//!
//! This module contains logic for traversing directory structures, handling different
//! input sources, managing symlinks, and controlling traversal depth.

pub mod depth_control;
pub mod filesystem;
pub mod input_source;
pub mod symlinks;

// Re-export the main walk_directory function for backward compatibility
pub use filesystem::walk_directory;
