## Core Library Modules

The `rustree` library is organized into several modules, each with a specific responsibility.

### `src/config/` - Configuration Module

This top-level module centralizes all configuration-related definitions for the library.

- **`tree_options.rs`** (formerly `src/core/config.rs`):
  - Defines `RustreeLibConfig`, the struct holding all configuration options for the library's behavior. This includes settings for traversal depth, visibility of hidden files, metadata reporting (sizes, modification times), content analysis flags, sorting preferences. It also includes filtering options such as pattern matching (`match_patterns`), pattern ignoring (`ignore_patterns`), use of gitignore files (`use_gitignore`, `git_ignore_files`), case sensitivity for patterns (`ignore_case_for_patterns`), and options like `list_directories_only`, `root_node_size`, and `root_is_directory`.

- **`sorting.rs`**:
  - Defines the `SortKey` enum, used in `RustreeLibConfig` to specify sorting criteria.

- **`output.rs`**:
  - Defines the `OutputFormat` enum (re-exported as `LibOutputFormat`), used to specify the desired output format (e.g., Text, Markdown).

- **`fileinfo.rs`**:
  - Defines `BuiltInFunction` enum for functions applicable to file content.
  - Defines `ApplyFnError` for errors during custom function application.

### `src/core/` - Core Logic Modules

- **`node.rs`**:
  - Defines `NodeInfo`, the struct representing a single file system entry (file, directory, symlink) and its collected data.
  - Defines `NodeType`, an enum for the type of file system entry.

- **`walker.rs`**:
  - Contains the `walk_directory` function responsible for traversing the file system.
  - Uses the `ignore` crate (`ignore::WalkBuilder`) for directory walking, which provides powerful filtering capabilities including gitignore processing.
  - Implements filtering logic based on `RustreeLibConfig` (from `src/config/tree_options.rs`):
    - **Hidden Files**: Controlled by `config.show_hidden` via `WalkBuilder::hidden()`.
    - **Max Depth**: Controlled by `config.max_depth` via `WalkBuilder::max_depth()`.
    - **Gitignore Processing**:
      - `config.use_gitignore`: Enables processing of standard `.gitignore` files, global gitignore, and repository-specific exclude files.
      - `config.git_ignore_files`: Allows specifying custom ignore files.
    - **Ignore Patterns (`-I` / `--filter-exclude`)**: `config.ignore_patterns` are compiled into glob patterns and used with `WalkBuilder::filter_entry()` to prune the traversal, excluding matching files and directories.
    - **Case Insensitivity**: `config.ignore_case_for_patterns` affects all pattern matching, including gitignore, `-I`, and `-P` patterns.
    - **Match Patterns (`-P` / `--filter-include`)**: `config.match_patterns` are compiled into glob patterns. After the `ignore` crate yields an entry (i.e., it wasn't filtered by gitignore or `-I` patterns), these `-P` patterns are applied. Files and symlinks must match one of these patterns to be included. Directories are generally included if they might contain matching children (they are not filtered out by `-P` at this stage if they passed previous filters).
    - `list_directories_only`: If true, only entries that are effectively directories (including symlinks to directories) are processed further.
  - Handles symlink resolution to determine the effective type of an entry (file, directory, or symlink) for filtering and metadata.
  - Populates `NodeInfo` structs with metadata (path, name, type, depth, size, mtime) and triggers content analysis for files based on `RustreeLibConfig`.

- **`analyzer/`**: This sub-module handles file content analysis.
  - `file_stats.rs`: Provides functions like `count_lines_from_string` and `count_words_from_string`.
  - `apply_fn.rs`: Defines the logic for applying custom functions (defined by `BuiltInFunction` from `src/config/fileinfo.rs`) to file content, handling `ApplyFnError` (also from `src/config/fileinfo.rs`).

- **`sorter.rs`**:
  - Contains the `sort_nodes` function, which sorts a `Vec<NodeInfo>` based on the specified `SortKey` (from `src/config/sorting.rs`) and order, primarily acting on sibling nodes.

- **`formatter/`**: This sub-module is responsible for generating the final output string.
  - `base.rs`: Defines the `TreeFormatter` trait, which all specific formatters implement.
  - `text_tree.rs`: Implements `TextTreeFormatter` for the classic `tree`-like text output. It handles the display of metadata (like sizes for directories when `-d` and `-s` are used), adapts the summary line based on `list_directories_only`, and uses `root_node_size` and `root_is_directory` from `RustreeLibConfig` for accurate root display.
  - `markdown.rs`: Implements `MarkdownFormatter` for generating Markdown lists.
  - `mod.rs` (in `formatter`): Re-exports `OutputFormat` (as `LibOutputFormat`) from `src/config/output.rs`.

- **`error.rs`**:
  - Defines `RustreeError`, the common error type used throughout the library. This includes variants for I/O errors, glob pattern errors, and errors from the `ignore` crate (`IgnoreError`).

### Top-Level Library File (`src/lib.rs`)

- Re-exports key public types from the `config` and `core` modules to form the library's public API. This includes `RustreeLibConfig`, `SortKey`, `BuiltInFunction`, `ApplyFnError`, `LibOutputFormat`, `NodeInfo`, `NodeType`, and `RustreeError`.
- Provides the main entry-point functions:
  - `get_tree_nodes()`: Orchestrates walking, analysis, and sorting.
  - `format_nodes()`: Takes the processed nodes and applies the chosen formatter.

This modular structure aims to make the codebase maintainable and extensible. For example, adding a new output format would involve creating a new struct that implements the `TreeFormatter` trait and updating the `format_nodes` function and relevant enums (like `OutputFormat` in `src/config/output.rs`). Similarly, new analysis functions or sort keys can be added by extending their respective modules.