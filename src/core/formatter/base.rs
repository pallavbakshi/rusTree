// src/core/formatter/base.rs
use crate::core::error::RustreeError;
use crate::core::options::RustreeLibConfig;
use crate::core::options::contexts::FormattingContext;
use crate::core::tree::node::NodeInfo;

/// A trait for formatting a list of `NodeInfo` objects into a string representation.
///
/// Implementors of this trait define specific output formats (e.g., text tree, Markdown).
pub trait TreeFormatter {
    /// Formats the given nodes using a context-based approach.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A slice of [`NodeInfo`] objects representing the directory tree to format.
    /// * `formatting_ctx` - The [`FormattingContext`] providing focused formatting options.
    ///
    /// # Returns
    ///
    /// A `Result` containing the formatted `String` on success, or a [`RustreeError`] on failure.
    fn format(
        &self,
        nodes: &[NodeInfo],
        formatting_ctx: &FormattingContext,
    ) -> Result<String, RustreeError>;
}

/// Extension trait that provides backward compatibility with the old config-based API
pub trait TreeFormatterCompat: TreeFormatter {
    /// Formats the given nodes using the full config (backward compatibility)
    fn format_compat(
        &self,
        nodes: &[NodeInfo],
        config: &RustreeLibConfig,
    ) -> Result<String, RustreeError> {
        let formatting_ctx = FormattingContext::new(
            &config.input_source,
            &config.listing,
            &config.metadata,
            &config.misc,
            &config.html,
        );
        self.format(nodes, &formatting_ctx)
    }
}
