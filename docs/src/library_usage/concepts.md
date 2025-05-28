## Core Library Concepts

Understanding these core components will help you effectively use the `rustree` library.

### `RustreeLibConfig`

This struct is central to controlling how `rustree` behaves. You create an instance of `RustreeLibConfig` and set its fields to specify:

- `root_display_name`: How the root directory is named in the output.
- `max_depth`: The maximum depth of traversal.
- `show_hidden`: Whether to include hidden files/directories.
- `report_sizes`, `report_mtime`: Whether to collect and report file sizes and modification times. `report_sizes` also applies to directories.
- `calculate_line_count`, `calculate_word_count`: Whether to perform these analyses on files.
- `apply_function`: An optional `BuiltInFunction` to apply to file contents.
- `sort_by`: An optional `SortKey` to sort sibling entries.
- `reverse_sort`: Whether to reverse the sort order.
- `list_directories_only`: If `true`, only directories (including symlinks to directories) are included in the results.
- `match_patterns`: `Option<Vec<String>>` containing patterns to filter entries. Only entries matching any pattern will be included. Corresponds to the CLI `-P`/`--match-pattern` options.
- `ignore_patterns`: `Option<Vec<String>>` containing patterns to ignore entries. Entries matching any pattern will be excluded. Corresponds to the CLI `-I`/`--ignore-path` options.
- `use_gitignore`: If `true`, standard gitignore files (`.gitignore`, global gitignore, etc.) will be used for filtering.
- `git_ignore_files`: `Option<Vec<PathBuf>>` specifying paths to custom files to be used as additional gitignore files.
- `ignore_case_for_patterns`: If `true`, all pattern matching (`match_patterns`, `ignore_patterns`, and gitignore processing) will be case-insensitive.
- `root_node_size`: Optional size of the root node itself, used by formatters if `report_sizes` is true.
- `root_is_directory`: Indicates if the root path itself is a directory, used by formatters.

**Example:**

```rust
use rustree::{RustreeLibConfig, SortKey};
use std::path::PathBuf;

let config = RustreeLibConfig {
    root_display_name: "MyProject".to_string(),
    max_depth: Some(3),
    show_hidden: false,
    report_sizes: true,
    sort_by: Some(SortKey::Size),
    reverse_sort: true,
    list_directories_only: false,
    match_patterns: Some(vec!["*.rs".to_string(), "src/".to_string()]), // Example -P patterns
    ignore_patterns: Some(vec!["*.log".to_string(), "target/".to_string()]), // Example -I patterns
    use_gitignore: true,
    git_ignore_files: Some(vec![PathBuf::from(".customignore")]),
    ignore_case_for_patterns: false,
    root_node_size: None, // Typically set by the CLI handler or by checking metadata
    root_is_directory: true, // Typically set by the CLI handler or by checking metadata
    ..Default::default() // Use defaults for other fields
};
```

### `NodeInfo`

Each file or directory encountered during the scan is represented by a `NodeInfo` struct. It contains:

- `path`: The full `PathBuf` to the entry.
- `name`: The file or directory name as a `String`.
- `node_type`: A `NodeType` enum (`File`, `Directory`, `Symlink`). When `list_directories_only` is active, symlinks pointing to directories will have `NodeType::Directory`.
- `depth`: The entry's depth in the tree.
- `size`: `Option<u64>` for file or directory size (if `report_sizes` is enabled).
- `mtime`: `Option<SystemTime>` for modification time.
- `line_count`, `word_count`: `Option<usize>` for analysis results (applicable to files only).
- `custom_function_output`: `Option<Result<String, ApplyFnError>>` for results of `apply_function`.

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
The returned `Vec<NodeInfo>` will only contain directories if `config.list_directories_only` is true.

### `format_nodes()`

Once you have the `Vec<NodeInfo>`, you can format it into a string.

```rust
use rustree::{format_nodes, NodeInfo, LibOutputFormat, RustreeLibConfig, RustreeError};

fn display_tree(nodes: &[NodeInfo], format: LibOutputFormat, config: &RustreeLibConfig) -> Result<String, RustreeError> {
    format_nodes(nodes, format, config)
}
```

This function takes the nodes, a `LibOutputFormat` enum (`Text` or `Markdown`), and the `RustreeLibConfig` (as some config options affect formatting).

### Key Enums

- **`SortKey`**: `Name`, `Size`, `MTime`, `Words`, `Lines`, `Custom`. Used in `RustreeLibConfig` to specify sorting.
- **`LibOutputFormat`**: `Text`, `Markdown`. Used with `format_nodes()`.
- **`BuiltInFunction`**: e.g., `CountPluses`. Used in `RustreeLibConfig` for `apply_function`.
- **`NodeType`**: `File`, `Directory`, `Symlink`. Found in `NodeInfo`.
- **`RustreeError`**: The error type returned by library functions.

Refer to the API documentation (generated by `cargo doc`) for the full details of these types and their variants/fields.
