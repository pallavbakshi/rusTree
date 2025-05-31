// src/core/formatter/base.rs
use crate::config::RustreeLibConfig;
use crate::core::error::RustreeError;
use crate::core::tree::node::NodeInfo;

/// A trait for formatting a list of `NodeInfo` objects into a string representation.
///
/// Implementors of this trait define specific output formats (e.g., text tree, Markdown).
pub trait TreeFormatter {
    /// Formats the given nodes according to the implementor's logic and configuration.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A slice of [`NodeInfo`] objects representing the directory tree to format.
    /// * `config` - The [`RustreeLibConfig`] providing context and options for formatting.
    ///
    /// # Returns
    ///
    /// A `Result` containing the formatted `String` on success, or a [`RustreeError`] on failure.
    fn format(&self, nodes: &[NodeInfo], config: &RustreeLibConfig)
    -> Result<String, RustreeError>;
}
