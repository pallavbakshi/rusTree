## Architectural Overview

RusTree is designed with a modular approach, separating concerns into different components. The primary data flow for the library is as follows:

1.  **Configuration (`RustreeLibConfig`)**: The process starts with a configuration object that dictates how the tree traversal, analysis, and formatting should occur.

2.  **Walking (`core::walker`)**:
    *   The `walk_directory` function traverses the file system starting from a root path.
    *   It respects configuration settings like `max_depth`, `show_hidden`, and `list_directories_only`. If `list_directories_only` is true, files are filtered out, and symlinks pointing to directories are treated as directories.
    *   For each qualifying file system entry, it gathers initial metadata. Symlinks are resolved to determine their effective type for filtering and metadata collection.

3.  **Analysis (`core::analyzer`)**:
    *   As the walker processes entries that are effectively files (i.e., not filtered out by `list_directories_only`), it can invoke analysis functions based on the configuration.
    *   `file_stats`: Calculates line counts and word counts for files.
    *   `apply_fn`: Applies a selected built-in function to file content.
    *   The results of these analyses are stored in `NodeInfo` objects. This step is skipped for directories or when `list_directories_only` is active.

4.  **Node Representation (`NodeInfo`)**:
    *   Each qualifying file system entry is represented by a `NodeInfo` struct. This struct holds its path, name, effective `node_type` (e.g., a symlink to a directory might be stored as `NodeType::Directory` if `list_directories_only` is active), depth, metadata (size, mtime), and any analysis results.
    *   The `size` field can be populated for directories if `report_sizes` is enabled.
    *   The walker produces a `Vec<NodeInfo>`.

5.  **Sorting (`core::sorter`)**:
    *   If a sort key is specified in the configuration, the `sort_nodes` function sorts the `Vec<NodeInfo>`.
    *   Sorting primarily applies to sibling nodes (nodes at the same depth under the same parent) to maintain the overall tree structure.

6.  **Formatting (`core::formatter`)**:
    *   The sorted (or unsorted) `Vec<NodeInfo>` is then passed to a formatter.
    *   The `TreeFormatter` trait defines the interface for formatters.
    *   `TextTreeFormatter`: Generates a plain text, `tree`-like output. It adapts its output based on `list_directories_only` (e.g., summary line, metadata shown).
    *   `MarkdownFormatter`: Generates a Markdown list.
    *   The formatter produces the final string output, considering configuration like `root_node_size` and `root_is_directory` for accurate root display.

### CLI Layer

The command-line interface (`src/cli/`) acts as a wrapper around the core library:

*   **Argument Parsing (`cli::args`)**: Uses `clap` to parse command-line arguments.
*   **Handler (`cli::handler`)**: Maps parsed CLI arguments (`CliArgs`) to the library's `RustreeLibConfig` and `LibOutputFormat`.
*   **Orchestration (`main.rs`)**:
    1.  Parses CLI args.
    2.  Maps CLI args to library config.
    3.  Calls `rustree::get_tree_nodes()` to get processed nodes.
    4.  Calls `rustree::format_nodes()` to get the output string.
    5.  Prints the string to the console, potentially with special formatting for LLM piping.

This separation allows the core library to be used independently of the CLI.