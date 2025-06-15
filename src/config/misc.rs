/// Miscellaneous configuration options that don't fit into other categories.
#[derive(Debug, Clone, Default)]
pub struct MiscOptions {
    /// Whether to omit the summary report at the end of the tree listing.
    pub no_summary_report: bool,
    /// Whether to display output in human-friendly format (e.g., "1.2 MB" instead of "1234567 B").
    pub human_friendly: bool,
    /// Whether to disable colored output.
    pub no_color: bool,
    /// Whether to show verbose output with additional details.
    pub verbose: bool,
}
