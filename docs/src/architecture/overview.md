## Architectural Overview

RusTree is designed with a modular approach, separating concerns into different components. The primary data flow for the library is as follows:

1.  **Configuration (`RustreeLibConfig` from `src/config/tree_options.rs`)**: The process starts with a configuration object that dictates how the tree traversal, analysis, and formatting should occur. `RustreeLibConfig` is composed of sub-structs like `ListingOptions`, `FilteringOptions` (including `prune_empty_directories`), `MetadataOptions` (controlling reporting of mtime, ctime, crtime, etc.), and `SortingOptions` (specifying sort keys like `Name`, `Version`, `MTime`, `ChangeTime`, `CreateTime`, `None`, and behaviors like `files_before_directories`).

2.  **Walking (`core::walker`)**:
    - The `walk_directory` function (in `core::walker::filesystem`) uses the `ignore` crate to traverse the file system.
    - It respects configuration settings from `RustreeLibConfig`:
        - `config.listing.max_depth`, `config.listing.show_hidden`.
        - `config.filtering.use_gitignore_rules` and `config.filtering.gitignore_file` for gitignore rules (handled by the `ignore` crate).
        - `config.filtering.ignore_patterns` (CLI `-I`) are compiled by `core::filter::pattern` and used by the `ignore` crate's `WalkBuilder::filter_entry()` to prune the walk.
        - `config.filtering.case_insensitive_filter` controls case sensitivity for all pattern matching.
    - After the `ignore` crate yields an entry, further filtering is applied by the walker using `core::filter::pattern::entry_matches_glob_patterns`:
        - `config.filtering.match_patterns` (CLI `-P`): Files and symlinks must match these patterns. Directories are generally kept if they might contain matching children.
    - For each qualifying entry, it gathers initial metadata (mtime, ctime, crtime based on `config.metadata` and platform capabilities). Symlinks are resolved. The `list_directories_only` filter is NOT applied at this stage.

3.  **Metadata Collection & Analysis (`core::metadata`)**:
    - As the walker processes entries, it invokes functions from `core::metadata` based on `config.metadata`.
    - `core::metadata::size_calculator`: Calculates line counts and word counts for files.
    - `core::metadata::file_info`: Applies built-in functions (from `config::metadata::BuiltInFunction`) to file content using `apply_builtin_to_file`.
    - The results are stored in `NodeInfo` objects (defined in `core::tree::node`). This step is skipped for directories for file-specific analyses.

4.  **Node Representation (`NodeInfo` from `core::tree::node`)**:
    - Each qualifying file system entry is represented by a `NodeInfo` struct. This struct holds its path, name, effective `node_type`, depth, metadata (size, mtime, change_time, create_time), and any analysis results.
    - The walker produces a `Vec<NodeInfo>`.

5.  **Pruning (`lib.rs`, using `core::tree::manipulator`)**:
    - If `config.filtering.prune_empty_directories` is `true`, the `Vec<NodeInfo>` is processed to remove empty directories.
    - This involves building a temporary tree, pruning nodes where a directory contains no files and no non-empty subdirectories, and then flattening the tree back.
    - This step occurs *after* all initial walking and filtering, but *before* the `list_directories_only` filter and sorting.

6.  **`list_directories_only` Filtering (`lib.rs`)**:
    - If `config.listing.list_directories_only` is `true`, the `Vec<NodeInfo>` (potentially already pruned) is further filtered to retain only entries with `NodeType::Directory`.
    - This ensures that pruning decisions are made based on the full content of directories before they are potentially removed by this filter.

7.  **Sorting (`core::sorter`)**:
    - If `config.sorting.sort_by` specifies a `SortKey`, the `sort_nodes_with_options` function (in `core::sorter::strategies`) sorts the (potentially pruned and filtered) `Vec<NodeInfo>`.
    - Sorting involves building a temporary tree, sorting sibling nodes at each level using comparison logic from `core::sorter::comparators` (which respects `config.sorting.reverse_sort` and `config.sorting.files_before_directories`), and then flattening the tree back.
    - Default size sorting is now largest first, with files/symlinks grouped before directories. Version sorting is more intelligent. If `SortKey::None` is used, directory traversal order is preserved.

8.  **Formatting (`core::formatter`)**:
    - The processed (sorted or unsorted) `Vec<NodeInfo>` is passed to a formatter.
    - The `TreeFormatter` trait defines the interface. Formatters use `core::metadata::file_info::format_node_metadata` for consistent metadata display.
    - `TextTreeFormatter`: Generates plain text, `tree`-like output.
    - `MarkdownFormatter`: Generates a Markdown list.
    - The formatter produces the final string output, considering configuration like `config.input_source.root_display_name`. The choice of formatter is determined by `LibOutputFormat` (from `config::output_format`).

### CLI Layer

The command-line interface (`src/cli/`) acts as a wrapper around the core library:

- **Argument Parsing (`cli::args`)**: Uses `clap` to parse arguments.
- **Mapping (`cli::mapping`)**: The `map_cli_to_lib_config` function translates `CliArgs` into `RustreeLibConfig` (e.g., setting `sorting.sort_by`, `sorting.files_before_directories`, `metadata.show_last_modified`).
- **Orchestration (`main.rs`)**:
  1. Parses CLI args.
  2. Maps CLI args to library config.
  3. Calls `rustree::get_tree_nodes()`.
  4. Calls `rustree::format_nodes()`.
  5. Prints the output.

This separation allows the core library to be used independently.