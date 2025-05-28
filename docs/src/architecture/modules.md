## Core Library Modules

The `rustree` library (`src/core/`) is organized into several modules, each with a specific responsibility:

*   **`config.rs`**:
    *   Defines `RustreeLibConfig`, the struct holding all configuration options for the library's behavior. This includes settings for traversal depth, visibility of hidden files, metadata reporting (sizes, modification times), content analysis flags, sorting preferences, pattern matching (`match_patterns`), and options like `list_directories_only`, `root_node_size`, and `root_is_directory`.

*   **`node.rs`**:
    *   Defines `NodeInfo`, the struct representing a single file system entry (file, directory, symlink) and its collected data.
    *   Defines `NodeType`, an enum for the type of file system entry.

*   **`walker.rs`**:
    *   Contains the `walk_directory` function responsible for traversing the file system.
    *   Uses the `walkdir` crate for efficient directory walking.
    *   Implements filtering logic, including:
        *   `list_directories_only`: Only directories (including symlinks to directories) are processed.
        *   Pattern matching: Based on `config.match_patterns`, filters entries using glob patterns. It handles various wildcard operators and directory-specific matching (e.g., `pattern/`).
        *   Hidden file visibility based on `config.show_hidden` and its interaction with pattern matching.
    *   Handles symlink resolution to determine the effective type of an entry.
    *   Populates `NodeInfo` structs with basic metadata (including size for directories if `report_sizes` is enabled) and triggers analysis for files based on `RustreeLibConfig`.

*   **`analyzer/`**: This sub-module handles file content analysis.
    *   `file_stats.rs`: Provides functions like `count_lines_from_string` and `count_words_from_string`.
    *   `apply_fn.rs`: Defines `BuiltInFunction` enum, `ApplyFnError`, and the `apply_function_to_content` logic for applying custom functions to file content.

*   **`sorter.rs`**:
    *   Defines the `SortKey` enum.
    *   Contains the `sort_nodes` function, which sorts a `Vec<NodeInfo>` based on the specified key and order, primarily acting on sibling nodes.

*   **`formatter/`**: This sub-module is responsible for generating the final output string.
    *   `base.rs`: Defines the `TreeFormatter` trait, which all specific formatters implement.
    *   `text_tree.rs`: Implements `TextTreeFormatter` for the classic `tree`-like text output. It handles the display of metadata (like sizes for directories when `-d` and `-s` are used), adapts the summary line based on `list_directories_only`, and uses `root_node_size` and `root_is_directory` from `RustreeLibConfig` for accurate root display.
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