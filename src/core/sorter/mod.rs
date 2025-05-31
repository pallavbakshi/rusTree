//! Sorting functionality for filesystem nodes.
//!
//! This module provides sorting capabilities for `NodeInfo` structures while
//! preserving tree hierarchy. It includes various sorting strategies and
//! comparison functions.

pub mod comparators;
pub mod composite;
pub mod strategies;

#[cfg(test)]
mod tests;

// Re-export the main sorting functions
pub use strategies::{sort_nodes, sort_nodes_with_options}; 