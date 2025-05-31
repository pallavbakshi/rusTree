## Architectural Overview

RusTree is designed with a modular approach, separating concerns into different components. The primary data flow for the library is as follows:

1.  **Configuration (`RustreeLibConfig` from `src/config/tree_options.rs`)**: The process starts with a configuration object that dictates how the tree traversal, analysis, and formatting should occur. `RustreeLibConfig` is composed of sub-structs like `ListingOptions`, `FilteringOptions`, `MetadataOptions` (controlling reporting of mtime, ctime, crtime, etc.), `SortingOptions` (specifying sort keys like `Name`, `Version`, `MTime`, `ChangeTime`, `CreateTime`, `None`, etc.), to organize settings.

1.  **Walking (`core::walker`)**:
===
    </search>
  

   - The `walk_directory` function, using the `ignore` crate, traverses the file system starting from a root path.
   - It respects configuration settings from `RustreeLibConfig`:
     - `config.listing.max_depth`, `config.listing.show_hidden`.
     - `config.filtering.use_gitignore` and `config.filtering.git_ignore_files` for respecting standard and custom gitignore rules.
     - `config.filtering.ignore_patterns` (CLI `-I` / `--filter-exclude`) to exclude entries matching specified glob patterns. These are applied early to prune the walk.
     - `config.filtering.ignore_case_for_patterns` to control case sensitivity for all pattern matching.
   - After initial filtering by the `ignore` crate (based on hidden status, gitignore rules, and `-I` patterns), further filtering is applied:
     - `config.filtering.match_patterns` (CLI `-P` / `--filter-include`): Files and symlinks must match these patterns. Directories are generally kept if they might contain matching children.
     - `config.listing.list_directories_only`: If true, only effective directories are kept.
   - For each qualifying file system entry, it gathers initial metadata (including mtime, and attempting to fetch ctime and creation time based on `config.metadata` settings and platform capabilities). Symlinks are resolved to determine their effective type for filtering and metadata collection.

1.  **Analysis (`core::analyzer`)**:

   - As the walker processes entries that are effectively files (i.e., not filtered out by `config.listing.list_directories_only`), it can invoke analysis functions based on the configuration (e.g., `config.metadata.calculate_line_count`, `config.metadata.apply_function`).
   - `file_stats`: Calculates line counts and word counts for files.
   - `apply_fn`: Applies a selected built-in function (from `src/config/metadata.rs`, specified via `config.metadata.apply_function`) to file content.
   - The results of these analyses are stored in `NodeInfo` objects. This step is skipped for directories or when `config.listing.list_directories_only` is active.

1.  **Node Representation (`NodeInfo`)**:

   - Each qualifying file system entry is represented by a `NodeInfo` struct (from `src/core/node.rs`). This struct holds its path, name, effective `node_type` (e.g., a symlink to a directory might be stored as `NodeType::Directory` if `config.listing.list_directories_only` is active), depth, metadata (size, mtime, change_time, create_time), and any analysis results.
   - The `size` field can be populated for directories if `config.metadata.report_sizes` is enabled.
   - The walker produces a `Vec<NodeInfo>`.

1.  **Sorting (`core::sorter`)**:

   - If a `SortKey` (e.g., `Name`, `Version`, `Size`, `MTime`, `ChangeTime`, `CreateTime`, `None`, from `src/config/sorting.rs`) is specified in `config.sorting.sort_by`, the `sort_nodes` function sorts the `Vec<NodeInfo>`.
   - Sorting primarily applies to sibling nodes (nodes at the same depth under the same parent) to maintain the overall tree structure. If `SortKey::None` is used, the directory traversal order is preserved.

1.  **Formatting (`core::formatter`)**:
===
    </search>
  

   - The sorted (or unsorted) `Vec<NodeInfo>` is then passed to a formatter.
   - The `TreeFormatter` trait defines the interface for formatters.
   - `TextTreeFormatter`: Generates a plain text, `tree`-like output. It adapts its output based on `config.listing.list_directories_only` (e.g., summary line, metadata shown).
   - `MarkdownFormatter`: Generates a Markdown list.
   - The formatter produces the final string output, considering configuration like `config.input_source.root_node_size` and `config.input_source.root_is_directory` for accurate root display. The choice of formatter is determined by `LibOutputFormat` (from `src/config/output_format.rs`).

### CLI Layer

The command-line interface (`src/cli/`) acts as a wrapper around the core library:

- **Argument Parsing (`cli::args`)**: Uses `clap` to parse command-line arguments. Arguments are organized into logical groups using flattened structs from submodules within `src/cli/` (e.g., `listing`, `filtering`, `metadata`, `sorting`). This includes flags like `-v`, `-t`, `-c`, `-U`, `--sort-by`, and `-D`.
- **Mapping (`cli::mapping`)**: The `map_cli_to_lib_config` function in this module translates the parsed `CliArgs` structure into the library's `RustreeLibConfig` (setting fields like `sorting.sort_by` to `LibSortKey::Version`, `LibSortKey::MTime`, `LibSortKey::ChangeTime`, `LibSortKey::None`, etc., and `metadata.report_modification_time`, `metadata.report_change_time` based on CLI flags like `-D` and `-c`) and `LibOutputFormat`.
- **Orchestration (`main.rs`)**:
  1. Parses CLI args.
  1. Maps CLI args to library config using `cli::mapping`.
  1. Calls `rustree::get_tree_nodes()` to get processed nodes.
  1. Calls `rustree::format_nodes()` to get the output string.
  1. Prints the string to the console, potentially with special formatting for LLM piping.

This separation allows the core library to be used independently of the CLI.