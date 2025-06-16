// src/core/formatter/base.rs
use crate::core::error::RustreeError;
use crate::core::options::contexts::FormattingContext;
use crate::core::options::{FormatterOptions, RustreeLibConfig};
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

    /// Formats the given nodes using FormatterOptions (transitional method).
    ///
    /// This provides backward compatibility during the transition to context-based APIs.
    /// It converts FormatterOptions to FormattingContext internally.
    fn format_with_options(
        &self,
        nodes: &[NodeInfo],
        formatter_opts: &FormatterOptions,
    ) -> Result<String, RustreeError> {
        // Convert FormatterOptions to FormattingContext for compatibility
        let formatting_ctx = FormattingContext {
            input_source: formatter_opts.input_source,
            listing: formatter_opts.listing,
            metadata: formatter_opts.metadata,
            misc: formatter_opts.misc,
            html: formatter_opts.html,
        };
        self.format(nodes, &formatting_ctx)
    }
}

/// Extension trait that provides backward compatibility with the old config-based API
pub trait TreeFormatterCompat: TreeFormatter {
    /// Formats the given nodes using the full config (backward compatibility)
    fn format_compat(
        &self,
        nodes: &[NodeInfo],
        config: &RustreeLibConfig,
    ) -> Result<String, RustreeError> {
        let formatter_opts = FormatterOptions::from_config(config);
        self.format_with_options(nodes, &formatter_opts)
    }
}
