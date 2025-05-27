## Core Library Modules

The `rustree` library (`src/core/`) is organized into several modules, each with a specific responsibility:

*   **`config.rs`**:
    *   Defines `RustreeLibConfig`, the struct holding all configuration options for the library's behavior.

*   **`node.rs`**:
    *   Defines `NodeInfo`, the struct representing a single file system entry (file, directory, symlink) and its collected data.
    *   Defines `NodeType`, an enum for the type of file system entry.

*   **`walker.rs`**:
    *   Contains the `walk_directory` function responsible for traversing the file system.
    *   Uses the `walkdir` crate for efficient directory walking.
    *   Populates `NodeInfo` structs with basic metadata and triggers analysis based on `RustreeLibConfig`.

*   **`analyzer/`**: This sub-module handles file content analysis.
    *   `file_stats.rs`: Provides functions like `count_lines_from_string` and `count_words_from_string`.
    *   `apply_fn.rs`: Defines `BuiltInFunction` enum, `ApplyFnError`, and the `apply_function_to_content` logic for applying custom functions to file content.

*   **`sorter.rs`**:
    *   Defines the `SortKey` enum.
    *   Contains the `sort_nodes` function, which sorts a `Vec<NodeInfo>` based on the specified key and order, primarily acting on sibling nodes.

*   **`formatter/`**: This sub-module is responsible for generating the final output string.
    *   `base.rs`: Defines the `TreeFormatter` trait, which all specific formatters implement.
    *   `text_tree.rs`: Implements `TextTreeFormatter` for the classic `tree`-like text output.
    *   `markdown.rs`: Implements `MarkdownFormatter` for generating Markdown lists (currently a placeholder).
    *   `mod.rs` (in `formatter`): Defines the `OutputFormat` enum (re-exported as `LibOutputFormat`).

*   **`error.rs`**:
    *   Defines `RustreeError`, the common error type used throughout the library.

### Top-Level Library File (`src/lib.rs`)

*   Re-exports key public types from the core modules to form the library's public API.
*   Provides the main entry-point functions:
    *   `get_tree_nodes()`: Orchestrates walking, analysis, and sorting.
    *   `format_nodes()`: Takes the processed nodes and applies the chosen formatter.

This modular structure aims to make the codebase maintainable and extensible. For example, adding a new output format would involve creating a new struct that implements the `TreeFormatter` trait and updating the `format_nodes` function and relevant enums. Similarly, new analysis functions or sort keys can be added by extending their respective modules.