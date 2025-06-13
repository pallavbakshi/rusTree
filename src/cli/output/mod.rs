pub mod format;

/// Defines the possible output formats selectable via the CLI.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum CliOutputFormat {
    /// Plain text, tree-like structure.
    Text,
    /// Markdown list format.
    Markdown,

    /// JSON format (pretty-printed array).
    Json,
}
