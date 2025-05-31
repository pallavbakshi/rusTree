## Core Library Modules

The `rustree` library is organized into several modules, each with a specific responsibility. The `src/core/` directory, in particular, has been significantly refactored into sub-modules for better organization and clarity.

### `src/config/` - Configuration Module

This top-level module centralizes all configuration-related definitions for the library. It groups related options into sub-modules and specific structs.

- **`tree_options.rs`**:
  - Defines `RustreeLibConfig`, the main configuration struct. It is composed of several sub-structs:
    - `InputSourceOptions` (from `input_source.rs`): Options related to the root input, like display name and initial metadata.
    - `ListingOptions` (from `listing.rs`): Options controlling directory traversal, such as `max_depth`, `show_hidden`, and `list_directories_only`.
    - `FilteringOptions` (from `filtering.rs`): Options for including/excluding files/directories, such as `match_patterns`, `ignore_patterns`, `use_gitignore`, `git_ignore_files`, and `ignore_case_for_patterns`.
    - `SortingOptions` (from `sorting.rs`): Options for sorting, including `sort_by` (using `SortKey`), `reverse_sort`, and `files_before_directories`.
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
  - Defines `SortingOptions` struct, used in `RustreeLibConfig` to specify sorting criteria, order, and whether files should appear before directories in size-based sorts.

- **`metadata.rs`**:
  - Defines `MetadataOptions` struct for metadata collection and content analysis flags (e.g., `report_sizes`, `report_modification_time`, `report_change_time`, `report_creation_time`).
  - Defines `BuiltInFunction` enum for functions applicable to file content.
  - Defines `ApplyFnError` for errors during custom function application.

- **`output_format.rs`** (formerly `output.rs`):
  - Defines the `OutputFormat` enum (re-exported as `LibOutputFormat`), used to specify the desired output format (e.g., Text, Markdown).

- **`misc.rs`**:
  - Defines `MiscOptions` struct for any other configuration options.


### `src/core/` - Core Logic Modules

The `src/core/` directory houses the main operational logic of `rustree`.

- **`tree/`**: This sub-module manages the tree data structure itself.
  - `node.rs`: Defines `NodeInfo`, the struct representing a single file system entry (file, directory, symlink) and its collected data (including `path`, `name`, `node_type`, `depth`, `size`, `mtime`, `change_time`, `create_time`, analysis results). Defines `NodeType`, an enum for the type of file system entry.
  - `builder.rs`: Contains logic for constructing a `Vec<TempNode>` (a temporary tree structure) from a flat `Vec<NodeInfo>` and for flattening it back. This is primarily used by the sorter.
  - `manipulator.rs`: Provides utilities for modifying tree structures, such as pruning, filtering, and transforming nodes.
  - `traversal.rs`: Implements various tree traversal algorithms (DFS pre-order, post-order, BFS) and a `TreeVisitor` trait for custom operations during traversal.

- **`walker/`**: This sub-module is responsible for traversing the file system.
  - `filesystem.rs`: Contains the `walk_directory` function. It uses the `ignore` crate (`ignore::WalkBuilder`) for directory walking. It implements initial filtering logic based on `RustreeLibConfig` (hidden files, max depth, gitignore, ignore patterns). After the `ignore` crate yields an entry, it applies further filtering (match patterns, list_directories_only). It handles symlink resolution and populates `NodeInfo` structs with basic metadata, triggering content analysis via the `metadata` module.
  - `depth_control.rs`: (Placeholder for future depth-specific control logic).
  - `input_source.rs`: (Placeholder for future advanced input source handling).
  - `symlinks.rs`: (Placeholder for future advanced symlink resolution strategies).

- **`filter/`**: This sub-module handles all filtering logic beyond what the `ignore` crate provides directly during its walk.
  - `pattern.rs`: Contains `compile_glob_patterns` to prepare glob patterns from `FilteringOptions` and `entry_matches_glob_patterns` to check if a `DirEntry` matches these compiled patterns. This is used by the `walker` after the `ignore` crate's initial pass.
  - `gitignore.rs`: (Placeholder for future gitignore-specific filtering utilities, complementing the `ignore` crate's built-in support).
  - `composite.rs`: (Placeholder for combining multiple filter criteria).
  - `matcher.rs`: (Placeholder for generic matching logic).
  - `size_filter.rs`: (Placeholder for size-based filtering).

- **`metadata/`**: This sub-module handles metadata collection, calculation, and content analysis.
  - `file_info.rs`:
    - Defines `format_node_metadata` for consistently formatting metadata strings for display (used by formatters).
    - Contains `apply_builtin_to_file` and `apply_builtin_function` for applying `BuiltInFunction`s to file content, handling `ApplyFnError`.
  - `size_calculator.rs`: Provides functions like `count_lines_from_string` and `count_words_from_string`.
  - `time_formatter.rs`: (Placeholder for advanced time formatting utilities).
  - `extended_attrs.rs`: (Placeholder for reading extended file attributes).

- **`sorter/`**: This sub-module is responsible for sorting nodes while preserving the tree hierarchy.
  - `strategies.rs`: Contains `sort_nodes_with_options` (and the older `sort_nodes`), which orchestrates tree building, sorting of sibling nodes, and tree flattening.
  - `comparators.rs`: Defines `compare_siblings_with_options` which implements the comparison logic for various `SortKey`s, considering `SortingOptions` like `reverse_sort` and `files_before_directories`. It includes improved version string comparison and new size sorting logic (defaulting to largest first, files before directories).
  - `composite.rs`: (Placeholder for defining and using composite sort keys).

- **`formatter/`**: This sub-module is responsible for generating the final output string.
  - `base.rs`: Defines the `TreeFormatter` trait, which all specific formatters implement.
  - `text_tree.rs`: Implements `TextTreeFormatter` for the classic `tree`-like text output. It uses `core::metadata::file_info::format_node_metadata` for consistent metadata display.
  - `markdown.rs`: Implements `MarkdownFormatter` for generating Markdown lists. It also uses `core::metadata::file_info::format_node_metadata`.
  - `mod.rs` (in `formatter`): Re-exports `OutputFormat` (as `LibOutputFormat`) from `src/config/output_format.rs`.

- **`util.rs`**: Contains general utility functions like `is_hidden`, `format_size`, `truncate_string`.

- **`error.rs`**:
  - Defines `RustreeError`, the common error type used throughout the library. This includes variants for I/O errors, glob pattern errors, and errors from the `ignore` crate (`IgnoreError`).

### Top-Level Library File (`src/lib.rs`)

- Re-exports key public types from the `config` and `core` modules to form the library's public API. This includes:
  - `RustreeLibConfig` and its constituent option structs: `InputSourceOptions`, `ListingOptions`, `FilteringOptions`, `SortingOptions` (including `files_before_directories`), `MetadataOptions`, `MiscOptions`.
  - Enums and related types: `SortKey`, `BuiltInFunction`, `ApplyFnError`.
  - `LibOutputFormat` (an alias for `OutputFormat`).
  - Core types: `NodeInfo` (from `core::tree::node`), `NodeType`, and `RustreeError`.
- Provides the main entry-point functions:
  - `get_tree_nodes()`: Orchestrates walking (via `core::walker`), analysis (via `core::metadata`), and sorting (via `core::sorter::strategies::sort_nodes_with_options`).
  - `format_nodes()`: Takes the processed nodes and applies the chosen formatter.

This modular structure aims to make the codebase maintainable and extensible.