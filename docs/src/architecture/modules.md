## Core Library Modules

The `rustree` library is organized into several modules, each with a specific responsibility. The `src/core/` directory, in particular, has been significantly refactored into sub-modules for better organization and clarity.

### `src/config/` - Configuration Bridge Module

This module now serves as a bridge layer that re-exports configuration types from the core layer and provides helper functions for CLI integration. After the refactoring (commit 333f1c7), all configuration structs have been moved to `src/core/options/` for better modularity.

- **All configuration files now simply re-export from core**:
  - `filtering.rs`: Re-exports `FilteringOptions` from `core::options::filtering`
  - `html.rs`: Re-exports `HtmlOptions` from `core::options::html`
  - `input_source.rs`: Re-exports `InputSourceOptions` from `core::options::input_source`
  - `listing.rs`: Re-exports `ListingOptions` from `core::options::listing`
  - `metadata.rs`: Re-exports all metadata types from `core::options::metadata`
  - `misc.rs`: Re-exports `MiscOptions` from `core::options::misc`
  - `output_format.rs`: Re-exports `OutputFormat` from `core::options::output_format`
  - `sorting.rs`: Re-exports all sorting types from `core::options::sorting`
  - `tree_options.rs`: Re-exports `RustreeLibConfig` from `core::options::tree_options`

- **`llm.rs`** (special case):
  - Still contains CLI-specific helper functions (`from_cli_args`, `to_core_config`, `generate_sample_env_file`)
  - Re-exports core LLM types: `LlmConfigError`, `LlmOptions`, `LlmProvider`
  - Provides the bridge between CLI arguments and core LLM configuration

### `src/core/options/` - Core Configuration Module

This new module (added in the refactoring) contains all the actual configuration struct definitions that were previously in `src/config/`. This change improves modularity by making the core layer self-contained.

- **`tree_options.rs`**:
  - Defines `RustreeLibConfig`, the main configuration struct
  - Composed of all the sub-configuration structs listed below

- **`input_source.rs`**:
  - Defines `InputSourceOptions` struct for root path display and initial metadata

- **`listing.rs`**:
  - Defines `ListingOptions` struct for directory traversal settings
  - Includes `show_full_path` option to control whether formatters display full relative paths or just filenames

- **`filtering.rs`**:
  - Defines `FilteringOptions` struct for inclusion/exclusion patterns, gitignore settings, and the `prune_empty_directories` flag

- **`sorting.rs`**:
  - Defines the `SortKey` enum (e.g., `Name`, `Size`, `MTime`, `Version`, `ChangeTime`, `CreateTime`, `None`)
  - Defines `DirectoryFileOrder` enum to control directory vs. file ordering (`Default`, `DirsFirst`, `FilesFirst`)
  - Defines `SortingOptions` struct for sorting criteria and preferences

- **`metadata.rs`**:
  - Defines `MetadataOptions` struct for metadata collection and content analysis flags
  - Defines `BuiltInFunction` enum for functions applicable to file content
  - Defines `ApplyFnError` for errors during custom function application
  - Defines `FunctionOutputKind` and `ExternalFunction` for external command support

- **`output_format.rs`**:
  - Defines the `OutputFormat` enum used to specify output format (Text, Markdown, Json, Html)

- **`misc.rs`**:
  - Defines `MiscOptions` struct for miscellaneous configuration options

- **`html.rs`**:
  - Defines `HtmlOptions` struct for HTML-specific output configuration

- **`llm.rs`**:
  - Defines core LLM types: `LlmProvider`, `LlmOptions`, `LlmConfigError`
  - Contains provider-specific logic and validation


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
  - `text_tree.rs`: Implements `TextTreeFormatter` for the classic `tree`-like text output. It uses `core::metadata::file_info::format_node_metadata` for consistent metadata display. Supports full path display when `config.listing.show_full_path` is enabled.
  - `markdown.rs`: Implements `MarkdownFormatter` for generating Markdown lists. It also uses `core::metadata::file_info::format_node_metadata`. Supports full path display when `config.listing.show_full_path` is enabled.
  - `mod.rs` (in `formatter`): Re-exports `OutputFormat` (as `LibOutputFormat`) from `src/config/output_format.rs`.

- **`util.rs`**: Contains general utility functions like `is_hidden`, `format_size`, `truncate_string`.

- **`error.rs`**:
  - Defines `RustreeError`, the common error type used throughout the library. This includes variants for I/O errors, glob pattern errors, errors from the `ignore` crate (`IgnoreError`), and `TreeBuildError` (for errors during internal tree construction for sorting or pruning).

### Top-Level Library File (`src/lib.rs`)

- Re-exports key public types to form the library's public API:
  - From `config` module (which now re-exports from `core::options`):
    - `RustreeLibConfig` and its constituent option structs: `InputSourceOptions`, `ListingOptions`, `FilteringOptions`, `SortingOptions`, `MetadataOptions`, `MiscOptions`, `HtmlOptions`
    - Enums and related types: `SortKey`, `DirectoryFileOrder`, `BuiltInFunction`, `ApplyFnError`, `FunctionOutputKind`, `ExternalFunction`
    - `LibOutputFormat` (an alias for `OutputFormat`)
    - LLM types: `LlmProvider`, `LlmOptions`, `LlmConfigError`

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

## Known Architectural Issues

### Incomplete Core Module Independence

While the recent refactoring (commit 333f1c7) moved configuration structs from `src/config/` to `src/core/options/` to improve modularity, the core module is not yet fully independent. There are still some dependencies that need to be addressed:

- The core module has dependencies on the config layer through re-exports
- Some types and functions still reference higher-level modules
- Complete independence would require further refactoring of the module boundaries

This is a known limitation that may be addressed in future versions. For now, the library functions correctly despite these architectural imperfections.