## Architectural Overview

RusTree is designed with a modular approach, separating concerns into different components. The primary data flow for the library is as follows:

1.  **Configuration (`RustreeLibConfig` from `src/config/tree_options.rs`)**: The process starts with a configuration object that dictates how the tree traversal, analysis, and formatting should occur.

1.  **Walking (`core::walker`)**:

   - The `walk_directory` function, using the `ignore` crate, traverses the file system starting from a root path.
   - It respects configuration settings from `RustreeLibConfig`:
     - `max_depth`, `show_hidden`.
     - `use_gitignore` and `git_ignore_files` for respecting standard and custom gitignore rules.
     - `ignore_patterns` (CLI `-I` / `--filter-exclude`) to exclude entries matching specified glob patterns. These are applied early to prune the walk.
     - `ignore_case_for_patterns` to control case sensitivity for all pattern matching.
   - After initial filtering by the `ignore` crate (based on hidden status, gitignore rules, and `-I` patterns), further filtering is applied:
     - `match_patterns` (CLI `-P` / `--filter-include`): Files and symlinks must match these patterns. Directories are generally kept if they might contain matching children.
     - `list_directories_only`: If true, only effective directories are kept.
   - For each qualifying file system entry, it gathers initial metadata. Symlinks are resolved to determine their effective type for filtering and metadata collection.

1.  **Analysis (`core::analyzer`)**:

   - As the walker processes entries that are effectively files (i.e., not filtered out by `list_directories_only`), it can invoke analysis functions based on the configuration.
   - `file_stats`: Calculates line counts and word counts for files.
   - `apply_fn`: Applies a selected built-in function (from `src/config/fileinfo.rs`) to file content.
   - The results of these analyses are stored in `NodeInfo` objects. This step is skipped for directories or when `list_directories_only` is active.

1.  **Node Representation (`NodeInfo`)**:

   - Each qualifying file system entry is represented by a `NodeInfo` struct (from `src/core/node.rs`). This struct holds its path, name, effective `node_type` (e.g., a symlink to a directory might be stored as `NodeType::Directory` if `list_directories_only` is active), depth, metadata (size, mtime), and any analysis results.
   - The `size` field can be populated for directories if `report_sizes` is enabled.
   - The walker produces a `Vec<NodeInfo>`.

1.  **Sorting (`core::sorter`)**:

   - If a `SortKey` (from `src/config/sorting.rs`) is specified in the configuration, the `sort_nodes` function sorts the `Vec<NodeInfo>`.
   - Sorting primarily applies to sibling nodes (nodes at the same depth under the same parent) to maintain the overall tree structure.

1.  **Formatting (`core::formatter`)**:

   - The sorted (or unsorted) `Vec<NodeInfo>` is then passed to a formatter.
   - The `TreeFormatter` trait defines the interface for formatters.
   - `TextTreeFormatter`: Generates a plain text, `tree`-like output. It adapts its output based on `list_directories_only` (e.g., summary line, metadata shown).
   - `MarkdownFormatter`: Generates a Markdown list.
   - The formatter produces the final string output, considering configuration like `root_node_size` and `root_is_directory` for accurate root display. The choice of formatter is determined by `LibOutputFormat` (from `src/config/output.rs`).

### CLI Layer

The command-line interface (`src/cli/`) acts as a wrapper around the core library:

- **Argument Parsing (`cli::args`)**: Uses `clap` to parse command-line arguments. Arguments are organized into logical groups using flattened structs from submodules within `src/cli/` (e.g., `listing`, `filtering`, `metadata`).
- **Mapping (`cli::mapping`)**: The `map_cli_to_lib_config` function in this module translates the parsed `CliArgs` structure into the library's `RustreeLibConfig` and `LibOutputFormat`.
- **Orchestration (`main.rs`)**:
  1. Parses CLI args.
  1. Maps CLI args to library config using `cli::mapping`.
  1. Calls `rustree::get_tree_nodes()` to get processed nodes.
  1. Calls `rustree::format_nodes()` to get the output string.
  1. Prints the string to the console, potentially with special formatting for LLM piping.

This separation allows the core library to be used independently of the CLI.