/// Miscellaneous configuration options that don't fit into other categories.
#[derive(Debug, Clone, Default)]
pub struct MiscOptions {
    /// Whether to omit the summary report at the end of the tree listing.
    pub no_summary_report: bool,
}
