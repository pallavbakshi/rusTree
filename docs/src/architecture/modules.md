## Core Library Modules

The `rustree` library is organized into several modules, each with a specific responsibility.

### `src/config/` - Configuration Module

This top-level module centralizes all configuration-related definitions for the library. It has been refactored to group related options into sub-modules and specific structs.

- **`tree_options.rs`**:
  - Defines `RustreeLibConfig`, the main configuration struct. It is now composed of several sub-structs, each handling a specific aspect of configuration:
    - `InputSourceOptions` (from `input_source.rs`): Options related to the root input, like display name and initial metadata.
    - `ListingOptions` (from `listing.rs`): Options controlling directory traversal, such as `max_depth`, `show_hidden`, and `list_directories_only`.
    - `FilteringOptions` (from `filtering.rs`): Options for including/excluding files/directories, such as `match_patterns`, `ignore_patterns`, `use_gitignore`, `git_ignore_files`, and `ignore_case_for_patterns`.
    - `SortingOptions` (from `sorting.rs`): Options for sorting, including `sort_by` (using `SortKey`) and `reverse_sort`.
    - `MetadataOptions` (from `metadata.rs`): Options for collecting and reporting metadata, like `report_sizes`, `report_mtime`, `calculate_line_count`, `calculate_word_count`, and `apply_function` (using `BuiltInFunction`).
    - `MiscOptions` (from `misc.rs`): For miscellaneous options.

- **`input_source.rs`**:
  - Defines `InputSourceOptions` struct for root path display and initial metadata.

- **`listing.rs`**:
  - Defines `ListingOptions` struct for directory traversal settings.

- **`filtering.rs`**:
  - Defines `FilteringOptions` struct for inclusion/exclusion patterns and gitignore settings.

- **`sorting.rs`**:
  - Defines the `SortKey` enum (e.g., `Name`, `Size`, `MTime`, `Version`, `ChangeTime`, `CreateTime`, `None`).
  - Defines `SortingOptions` struct, used in `RustreeLibConfig` to specify sorting criteria and order.

- **`metadata.rs`**:
  - Defines `MetadataOptions` struct for metadata collection and content analysis flags (e.g., `report_sizes`, `report_modification_time`, `report_change_time`, `report_creation_time`).
  - Defines `BuiltInFunction` enum for functions applicable to file content (formerly in `fileinfo.rs`).
  - Defines `ApplyFnError` for errors during custom function application (formerly in `fileinfo.rs`).

- **`output_format.rs`** (formerly `output.rs`):
  - Defines the `OutputFormat` enum (re-exported as `LibOutputFormat`), used to specify the desired output format (e.g., Text, Markdown).

- **`misc.rs`**:
  - Defines `MiscOptions` struct for any other configuration options.


### `src/core/` - Core Logic Modules

- **`node.rs`**:
  - Defines `NodeInfo`, the struct representing a single file system entry (file, directory, symlink) and its collected data (including `path`, `name`, `node_type`, `depth`, `size`, `mtime`, `change_time`, `create_time`, analysis results).
  - Defines `NodeType`, an enum for the type of file system entry.

- **`walker.rs`**:
  - Contains the `walk_directory` function responsible for traversing the file system.
  - Uses the `ignore` crate (`ignore::WalkBuilder`) for directory walking, which provides powerful filtering capabilities including gitignore processing.
  - Implements filtering logic based on `RustreeLibConfig` (from `src/config/tree_options.rs`):
    - **Hidden Files**: Controlled by `config.listing.show_hidden` via `WalkBuilder::hidden()`.
    - **Max Depth**: Controlled by `config.listing.max_depth` via `WalkBuilder::max_depth()`.
    - **Gitignore Processing**:
      - `config.filtering.use_gitignore`: Enables processing of standard `.gitignore` files, global gitignore, and repository-specific exclude files.
      - `config.filtering.git_ignore_files`: Allows specifying custom ignore files.
    - **Ignore Patterns (`-I` / `--filter-exclude`)**: `config.filtering.ignore_patterns` are compiled into glob patterns and used with `WalkBuilder::filter_entry()` to prune the traversal, excluding matching files and directories.
    - **Case Insensitivity**: `config.filtering.ignore_case_for_patterns` affects all pattern matching, including gitignore, `-I`, and `-P` patterns.
    - **Match Patterns (`-P` / `--filter-include`)**: `config.filtering.match_patterns` are compiled into glob patterns. After the `ignore` crate yields an entry (i.e., it wasn't filtered by gitignore or `-I` patterns), these `-P` patterns are applied. Files and symlinks must match one of these patterns to be included. Directories are generally included if they might contain matching children (they are not filtered out by `-P` at this stage if they passed previous filters).
    - `config.listing.list_directories_only`: If true, only entries that are effectively directories (including symlinks to directories) are processed further.
  - Handles symlink resolution to determine the effective type of an entry (file, directory, or symlink) for filtering and metadata.
  - Populates `NodeInfo` structs with metadata (path, name, type, depth, size, mtime, change_time, create_time) and triggers content analysis for files based on `RustreeLibConfig` (specifically fields within `config.metadata`). It attempts to fetch ctime and creation time based on platform capabilities.

- **`analyzer/`**: This sub-module handles file content analysis.
  - `file_stats.rs`: Provides functions like `count_lines_from_string` and `count_words_from_string`.
  - `apply_fn.rs`: Defines the logic for applying custom functions (defined by `BuiltInFunction` from `src/config/metadata.rs`) to file content, handling `ApplyFnError` (also from `src/config/metadata.rs`).

- **`sorter.rs`**:
  - Contains the `sort_nodes` function, which sorts a `Vec<NodeInfo>` based on the specified `SortKey` (e.g., `Name`, `Size`, `MTime`, `Version`, `ChangeTime`, `CreateTime`, `None` from `src/config/sorting.rs`) and order (from `config.sorting.reverse_sort`), primarily acting on sibling nodes.

- **`formatter/`**: This sub-module is responsible for generating the final output string.
  - `base.rs`: Defines the `TreeFormatter` trait, which all specific formatters implement.
  - `text_tree.rs`: Implements `TextTreeFormatter` for the classic `tree`-like text output. It handles the display of metadata (like sizes, modification time, change time, creation time, controlled by `config.metadata`), adapts the summary line based on `config.listing.list_directories_only`, and uses `config.input_source.root_node_size` and `config.input_source.root_is_directory` from `RustreeLibConfig` for accurate root display.
  - `markdown.rs`: Implements `MarkdownFormatter` for generating Markdown lists.
  - `mod.rs` (in `formatter`): Re-exports `OutputFormat` (as `LibOutputFormat`) from `src/config/output_format.rs`.

- **`error.rs`**:
  - Defines `RustreeError`, the common error type used throughout the library. This includes variants for I/O errors, glob pattern errors, and errors from the `ignore` crate (`IgnoreError`).

### Top-Level Library File (`src/lib.rs`)

- Re-exports key public types from the `config` and `core` modules to form the library's public API. This includes:
  - `RustreeLibConfig` and its constituent option structs: `InputSourceOptions`, `ListingOptions`, `FilteringOptions`, `SortingOptions`, `MetadataOptions` (including fields like `report_modification_time`, `report_change_time`, `report_creation_time`), `MiscOptions`.
  - Enums and related types: `SortKey` (e.g., `Name`, `Version`, `MTime`, `ChangeTime`, `CreateTime`, `None` from `config::sorting`), `BuiltInFunction`, `ApplyFnError` (both from `config::metadata`).
  - `LibOutputFormat` (an alias for `OutputFormat` from `config::output_format`).
  - Core types: `NodeInfo` (including fields like `mtime`, `change_time`, `create_time`), `NodeType`, and `RustreeError`.
- Provides the main entry-point functions:
  - `get_tree_nodes()`: Orchestrates walking, analysis, and sorting.
  - `format_nodes()`: Takes the processed nodes and applies the chosen formatter.

This modular structure aims to make the codebase maintainable and extensible. For example, adding a new output format would involve creating a new struct that implements the `TreeFormatter` trait and updating the `format_nodes` function and relevant enums (like `OutputFormat` in `src/config/output_format.rs`). Similarly, new analysis functions or sort keys can be added by extending their respective modules.