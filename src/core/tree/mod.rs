//! Tree data structures and operations.
//!
//! This module contains the core tree node representation and tree building utilities.

pub mod builder;
pub mod manipulator;
pub mod node;
pub mod traversal;

// Re-export commonly used types
pub use node::{NodeInfo, NodeType};
