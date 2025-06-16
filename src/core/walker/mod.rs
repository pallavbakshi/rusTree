//! File system traversal and directory walking functionality.
//!
//! This module contains logic for traversing directory structures, handling different
//! input sources, managing symlinks, and controlling traversal depth.

pub mod depth_control;
pub mod filesystem;
pub mod input_source;
pub mod symlinks;

// Re-export old, parameter-based, and context-based walker functions
pub use filesystem::{
    walk_directory, walk_directory_owned, walk_directory_with_context, walk_directory_with_options,
};
