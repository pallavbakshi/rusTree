## Core Library Concepts

Understanding these core components will help you effectively use the `rustree` library.

### `RustreeLibConfig`

This struct (defined in `src/config/tree_options.rs`) is central to controlling how `rustree` behaves. It has been refactored into a hierarchical structure, grouping related options into sub-structs for better organization. You create an instance of `RustreeLibConfig` and set fields within these sub-structs:

- **`input_source: InputSourceOptions`** (from `src/config/input_source.rs`):
  - `root_display_name`: How the root directory is named in the output.
  - `root_node_size`: Optional size of the root node itself, used by formatters if `metadata.show_size_bytes` is true.
  - `root_is_directory`: Indicates if the root path itself is a directory, used by formatters.
- **`listing: ListingOptions`** (from `src/config/listing.rs`):
  - `max_depth`: The maximum depth of traversal.
  - `show_hidden`: Whether to include hidden files/directories.
  - `list_directories_only`: If `true`, only directories (including symlinks to directories) are included in the results.
- **`filtering: FilteringOptions`** (from `src/config/filtering.rs`):
  - `match_patterns`: `Option<Vec<String>>` containing patterns to filter entries. Only entries matching any pattern will be included. Corresponds to the CLI `-P`/`--filter-include` options.
  - `ignore_patterns`: `Option<Vec<String>>` containing patterns to ignore entries. Entries matching any pattern will be excluded. Corresponds to the CLI `-I`/`--filter-exclude` options.
  - `use_gitignore_rules`: If `true`, standard gitignore files (`.gitignore`, global gitignore, etc.) will be used for filtering.
  - `gitignore_file`: `Option<Vec<PathBuf>>` specifying paths to custom files to be used as additional gitignore files.
  - `case_insensitive_filter`: If `true`, all pattern matching (`match_patterns`, `ignore_patterns`, and gitignore processing) will be case-insensitive.
  - `prune_empty_directories`: If `true`, empty directories are removed from the results after initial walking and filtering, but before sorting. An empty directory is one that contains no files and no non-empty subdirectories after other filters have been applied.
- **`sorting: SortingOptions`** (from `src/config/sorting.rs`):
  - `sort_by`: An optional `SortKey` (from `src/config/sorting.rs`) to sort sibling entries.
  - `reverse_sort`: Whether to reverse the sort order.
  - `files_before_directories`: A `bool` (default `true`) that, when sorting by size, determines if files and symlinks are grouped before directories. If `false`, types are intermingled based purely on size.
- **`metadata: MetadataOptions`** (from `src/config/metadata.rs`):
  - `show_size_bytes`: Whether to collect and report file sizes in bytes. Applies to directories as well.
  - `show_last_modified`: Whether to collect and report last modification times (mtime).
  - `report_change_time`: Whether to collect and report last status change times (ctime).
  - `report_creation_time`: Whether to collect and report creation times (btime/crtime).
  - `calculate_line_count`, `calculate_word_count`: Whether to perform these analyses on files.
  - `apply_function`: An optional `BuiltInFunction` (from `src/config/metadata.rs`) to apply to file contents.
  - `report_permissions`: (Currently not exposed via CLI, defaults to false).
- **`misc: MiscOptions`** (from `src/config/misc.rs`):
  - Currently no fields, reserved for future use.

**Example:**

```rust
use rustree::{
    RustreeLibConfig, SortKey, BuiltInFunction,
    InputSourceOptions, ListingOptions, FilteringOptions, SortingOptions, MetadataOptions,
};
use std::path::PathBuf;

let config = RustreeLibConfig {
    input_source: InputSourceOptions {
        root_display_name: "MyProject".to_string(),
        root_node_size: None, // Typically set by the CLI handler or by checking metadata
        root_is_directory: true, // Typically set by the CLI handler or by checking metadata
        ..Default::default()
    },
    listing: ListingOptions {
        max_depth: Some(3),
        show_hidden: false,
        list_directories_only: false,
        ..Default::default()
    },
    filtering: FilteringOptions {
        match_patterns: Some(vec!["*.rs".to_string(), "src/".to_string()]), // Example -P patterns
        ignore_patterns: Some(vec!["*.log".to_string(), "target/".to_string()]), // Example -I patterns
        use_gitignore_rules: true,
        gitignore_file: Some(vec![PathBuf::from(".customignore")]),
        case_insensitive_filter: false,
        ..Default::default()
    },
    sorting: SortingOptions {
        sort_by: Some(SortKey::Size),
        reverse_sort: false, // Size sort is descending by default, so false means largest first.
        files_before_directories: true, // Default behavior
        ..Default::default()
    },
    metadata: MetadataOptions {
        show_size_bytes: true,
        show_last_modified: true,
        report_change_time: false,
        report_creation_time: false,
        calculate_line_count: false, // Example: not calculating line count
        apply_function: Some(BuiltInFunction::CountPluses), // Example: applying a function
        ..Default::default()
    },
    ..Default::default() // Use defaults for misc and other fields if not specified
};
```

### `NodeInfo`

Each file or directory encountered during the scan is represented by a `NodeInfo` struct (defined in `src/core/tree/node.rs`). It contains:

- `path`: The full `PathBuf` to the entry.
- `name`: The file or directory name as a `String`.
- `node_type`: A `NodeType` enum (`File`, `Directory`, `Symlink`). When `listing.list_directories_only` is active, symlinks pointing to directories will have `NodeType::Directory`.
- `depth`: The entry's depth in the tree.
- `size`: `Option<u64>` for file or directory size (if `metadata.report_sizes` is enabled).
- `mtime`: `Option<SystemTime>` for last modification time.
- `change_time`: `Option<SystemTime>` for last status change time (ctime).
- `create_time`: `Option<SystemTime>` for creation time (btime/crtime).
- `line_count`, `word_count`: `Option<usize>` for analysis results (applicable to files only).
- `custom_function_output`: `Option<Result<String, ApplyFnError>>` (where `ApplyFnError` is from `src/config/metadata.rs`) for results of `metadata.apply_function`.

You typically receive a `Vec<NodeInfo>` from `get_tree_nodes()`.

### `get_tree_nodes()`

This is the primary function for generating the tree data.

```rust
use rustree::{get_tree_nodes, RustreeLibConfig, NodeInfo, RustreeError};
use std::path::Path;

fn list_directory_contents(path_str: &str, config: &RustreeLibConfig) -> Result<Vec<NodeInfo>, RustreeError> {
    let root_path = Path::new(path_str);
    get_tree_nodes(root_path, config)
}
```

It takes the root path and a `RustreeLibConfig` and returns a `Result<Vec<NodeInfo>, RustreeError>`.
The processing order is:
1. Walk the directory structure, applying initial filters (`match_patterns`, `ignore_patterns`, gitignore rules, etc.) and collecting metadata.
2. If `config.filtering.prune_empty_directories` is `true`, empty directories are pruned from the collected nodes.
3. If `config.listing.list_directories_only` is `true`, the node list is filtered to retain only directories. This happens *after* pruning, so pruning decisions are based on the full content before this filter.
4. If sorting is specified (`config.sorting.sort_by`), the remaining nodes are sorted.
The final `Vec<NodeInfo>` reflects these processing steps.

### `format_nodes()`

Once you have the `Vec<NodeInfo>`, you can format it into a string.

```rust
use rustree::{format_nodes, NodeInfo, LibOutputFormat, RustreeLibConfig, RustreeError};

fn display_tree(nodes: &[NodeInfo], format: LibOutputFormat, config: &RustreeLibConfig) -> Result<String, RustreeError> {
    format_nodes(nodes, format, config)
}
```

This function takes the nodes, a `LibOutputFormat` enum (`Text` or `Markdown`, from `src/config/output_format.rs` and re-exported), and the `RustreeLibConfig` (as some config options affect formatting).

### Key Enums

- **`SortKey`**: `Name`, `Version`, `Size`, `MTime`, `ChangeTime`, `CreateTime`, `Words`, `Lines`, `Custom`, `None`. Defined in `src/config/sorting.rs`. Used in `RustreeLibConfig.sorting.sort_by`.
- **`LibOutputFormat`**: `Text`, `Markdown`. Defined in `src/config/output_format.rs` (as `OutputFormat`). Used with `format_nodes()`.
- **`BuiltInFunction`**: e.g., `CountPluses`. Defined in `src/config/metadata.rs`. Used in `RustreeLibConfig.metadata.apply_function`.
- **`ApplyFnError`**: Error type for `BuiltInFunction` application. Defined in `src/config/metadata.rs`.
- **`NodeType`**: `File`, `Directory`, `Symlink`. Defined in `src/core/tree/node.rs`. Found in `NodeInfo`.
- **`RustreeError`**: The error type returned by library functions. Defined in `src/core/error.rs`. Includes variants like `Io`, `GlobPattern`, `IgnoreError`, and `TreeBuildError` (for errors during internal tree construction or sorting).

Refer to the API documentation (generated by `cargo doc`) for the full details of these types and their variants/fields.