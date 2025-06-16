//! # Context-based Core Operations
//!
//! This module provides focused context structures for core operations, enabling:
//! - Cleaner APIs with minimal dependencies
//! - Better testability of core functions
//! - Flexible ownership with owned contexts
//! - Performance optimizations through cached compilation
//!
//! ## Context Types
//!
//! ### Borrowed Contexts (CLI/Short-lived operations)
//! - [`WalkingContext`] - Directory traversal configuration
//! - [`FormattingContext`] - Output formatting configuration  
//! - [`SortingContext`] - Sorting configuration
//!
//! ### Owned Contexts (Long-lived/Advanced operations)
//! - [`OwnedWalkingContext`] - With pattern compilation caching
//! - [`OwnedFormattingContext`] - Independent formatting state
//! - [`OwnedSortingContext`] - Independent sorting state
//!
//! ### Composite Contexts
//! - [`ProcessingContext`] - Complete tree processing pipeline
//! - [`OwnedProcessingContext`] - Owned version for advanced use cases
//!
//! ## Usage Patterns
//!
//! ### For CLI Applications
//! ```rust
//! # use rustree::RustreeLibConfig;
//! # use std::path::Path;
//! let config = RustreeLibConfig::default();
//! let processing_ctx = config.processing_context();
//! // let nodes = rustree::get_tree_nodes_with_context(Path::new("."), &processing_ctx)?;
//! ```
//!
//! ### For Interactive Applications
//! ```rust
//! # use rustree::RustreeLibConfig;
//! # use std::path::Path;
//! // Create owned contexts that can be modified independently
//! let mut walking_ctx = RustreeLibConfig::default().to_owned_walking_context();
//!
//! // Change max depth dynamically
//! walking_ctx.listing.max_depth = Some(5);
//!
//! // Efficiently recompute with cached patterns
//! // let nodes = rustree::walk_path_owned(Path::new("."), &mut walking_ctx)?;
//! ```
//!
//! ### For Library Integration
//! ```rust
//! // Build contexts programmatically
//! // let processing_ctx = ProcessingContextBuilder::new()
//! //     .with_walking(my_walking_config)
//! //     .with_formatting(my_formatting_config)
//! //     .build()?;
//! ```

pub mod async_support;
pub mod diff;
pub mod errors;
pub mod formatting;
pub mod lazy;
pub mod processing;
pub mod sorting;
pub mod walking;

pub use async_support::*;
pub use diff::*;
pub use errors::*;
pub use formatting::*;
pub use lazy::*;
pub use processing::*;
pub use sorting::*;
pub use walking::*;
