## Core Library Modules

The `rustree` library is organized into several modules, each with a specific responsibility. The `src/core/` directory, in particular, has been significantly refactored into sub-modules for better organization and clarity.

### `src/config/` - Configuration Module

This top-level module centralizes all configuration-related definitions for the library. It groups related options into sub-modules and specific structs.

- **`tree_options.rs`**:
  - Defines `RustreeLibConfig`, the main configuration struct. It is composed of several sub-structs:
    - `InputSourceOptions` (from `input_source.rs`): Options related to the root input, like display name and initial metadata.
    - `ListingOptions` (from `listing.rs`): Options controlling directory traversal, such as `max_depth`, `show_hidden`, and `list_directories_only`.
    - `FilteringOptions` (from `filtering.rs`): Options for including/excluding files/directories, such as `match_patterns`, `ignore_patterns`, `use_gitignore_rules`, `gitignore_file`, and `case_insensitive_filter`.
    - `SortingOptions` (from `sorting.rs`): Options for sorting, including `sort_by` (using `SortKey`), `reverse_sort`, and `files_before_directories`.
    - `MetadataOptions` (from `metadata.rs`): Options for collecting and reporting metadata, like `show_size_bytes`, `show_last_modified`, `calculate_line_count`, `calculate_word_count`, and `apply_function` (using `BuiltInFunction`).
    - `MiscOptions` (from `misc.rs`): For miscellaneous options.

- **`input_source.rs`**:
  - Defines `InputSourceOptions` struct for root path display and initial metadata.

- **`listing.rs`**:
  - Defines `ListingOptions` struct for directory traversal settings.

- **`filtering.rs`**:
  - Defines `FilteringOptions` struct for inclusion/exclusion patterns, gitignore settings, and the `prune_empty_directories` flag.

- **`sorting.rs`**:
  - Defines the `SortKey` enum (e.g., `Name`, `Size`, `MTime`, `Version`, `ChangeTime`, `CreateTime`, `None`).
  - Defines `DirectoryFileOrder` enum to control directory vs. file ordering (`Default`, `DirsFirst`, `FilesFirst`).
  - Defines `SortingOptions` struct, used in `RustreeLibConfig` to specify sorting criteria, order, directory/file ordering preference, and backward compatibility options.

- **`metadata.rs`**:
  - Defines `MetadataOptions` struct for metadata collection and content analysis flags (e.g., `show_size_bytes`, `show_last_modified`, `report_change_time`, `report_creation_time`).
  - Defines `BuiltInFunction` enum for functions applicable to file content, including `CountPluses` (counts '+' characters) and `Cat` (returns full file content).
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
  - `filesystem.rs`: Contains the `walk_directory` function. It uses the `ignore` crate (`ignore::WalkBuilder`) for directory walking. It implements initial filtering logic based on `RustreeLibConfig` (hidden files, max depth, gitignore rules, ignore patterns). After the `ignore` crate yields an entry, it applies further filtering (match patterns). The `list_directories_only` filter is applied later in `lib.rs` after potential pruning. It handles symlink resolution and populates `NodeInfo` structs with basic metadata, triggering content analysis via the `metadata` module.
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
    - Defines `format_node_metadata` for consistently formatting metadata strings for display (used by formatters). For the `Cat` function, metadata display is suppressed since content is shown separately.
    - Contains `apply_builtin_to_file` and `apply_builtin_function` for applying `BuiltInFunction`s to file content, handling `ApplyFnError`. The `Cat` function simply returns the full file content.
  - `size_calculator.rs`: Provides functions like `count_lines_from_string` and `count_words_from_string`.
  - `time_formatter.rs`: (Placeholder for advanced time formatting utilities).
  - `extended_attrs.rs`: (Placeholder for reading extended file attributes).

- **`sorter/`**: This sub-module is responsible for sorting nodes while preserving the tree hierarchy.
  - `strategies.rs`: Contains `sort_nodes_with_options` (and the older `sort_nodes`), which orchestrates tree building, sorting of sibling nodes, and tree flattening.
  - `comparators.rs`: Defines `compare_siblings_with_options` which implements the comparison logic for various `SortKey`s, considering `SortingOptions` like `reverse_sort`, `files_before_directories` (legacy), and the new `directory_file_order` enum. It includes universal directory/file ordering that applies to all sort keys, improved version string comparison, and enhanced size sorting logic. The `apply_directory_file_ordering` function provides consistent directory vs. file ordering across all sorting modes.
  - `composite.rs`: (Placeholder for defining and using composite sort keys).

- **`formatter/`**: This sub-module is responsible for generating the final output string.
  - `base.rs`: Defines the `TreeFormatter` trait, which all specific formatters implement.
  - `text_tree.rs`: Implements `TextTreeFormatter` for the classic `tree`-like text output. It uses `core::metadata::file_info::format_node_metadata` for consistent metadata display.
  - `markdown.rs`: Implements `MarkdownFormatter` for generating Markdown lists. It also uses `core::metadata::file_info::format_node_metadata`.
  - `mod.rs` (in `formatter`): Re-exports `OutputFormat` (as `LibOutputFormat`) from `src/config/output_format.rs`.

- **`util.rs`**: Contains general utility functions like `is_hidden`, `format_size`, `truncate_string`.

- **`error.rs`**:
  - Defines `RustreeError`, the common error type used throughout the library. This includes variants for I/O errors, glob pattern errors, errors from the `ignore` crate (`IgnoreError`), and `TreeBuildError` (for errors during internal tree construction for sorting or pruning).

### Top-Level Library File (`src/lib.rs`)

- Re-exports key public types from the `config` and `core` modules to form the library's public API. This includes:
  - `RustreeLibConfig` and its constituent option structs: `InputSourceOptions`, `ListingOptions`, `FilteringOptions`, `SortingOptions` (including `directory_file_order` and legacy `files_before_directories`), `MetadataOptions`, `MiscOptions`.
  - Enums and related types: `SortKey`, `DirectoryFileOrder`, `BuiltInFunction`, `ApplyFnError`.
  - `LibOutputFormat` (an alias for `OutputFormat`).

- Core types: `NodeInfo` (from `core::tree::node`), `NodeType`, and `RustreeError`.
- The `cli` module, while part of the crate, is marked `#[doc(hidden)]` and is not part of the stable public API.
- Provides the main entry-point functions:
  - `get_tree_nodes()`: Orchestrates the main logic:
    1. Walking the file system (via `core::walker`), applying initial filters and collecting metadata.
    2. If `config.filtering.prune_empty_directories` is true, prunes empty directories from the results (using `core::tree::manipulator` and `core::tree::builder`).
    3. If `config.listing.list_directories_only` is true, filters the results to include only directories. This occurs *after* pruning.
    4. If sorting is requested, sorts the nodes (via `core::sorter::strategies::sort_nodes_with_options`). Errors during sorting or tree building for pruning now map to `RustreeError::TreeBuildError`.
  - `format_nodes()`: Takes the processed nodes and applies the chosen formatter. For the `Cat` function, it first generates the normal tree output, then appends a "--- File Contents ---" section with the content of each file.

This modular structure aims to make the codebase maintainable and extensible.