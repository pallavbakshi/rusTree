/// Enumerates the available output formats for the `rustree` library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    /// Plain text, tree-like structure.
    Text,
    /// Markdown list format.
    Markdown,
} 