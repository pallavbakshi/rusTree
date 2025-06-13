/// Enumerates the available output formats for the `rustree` library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    /// Plain text, tree-like structure.
    Text,
    /// Markdown list format.
    Markdown,

    /// JSON array of NodeInfo structs (pretty-printed).
    Json,

    /// HTML output wrapped in basic boilerplate, with the tree inside a <pre>
    /// block. Mimics GNU tree's -H output (without hyperlinks for now).
    Html,
}
