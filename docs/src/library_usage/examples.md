## Library Usage Examples

Here are some examples of how to use `rustree` as a library in your Rust code.
Make sure to add `rustree` to your `Cargo.toml` dependencies.
All key types like `RustreeLibConfig`, `NodeInfo`, `SortKey`, `LibOutputFormat`, `RustreeError` are re-exported by `rustree`'s `lib.rs`.

### Example 1: Basic Tree Listing

This example shows how to get a simple text tree of a directory.

```rust
use rustree::{
    get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat, RustreeError,
    InputSourceOptions, ListingOptions,
};
use std::path::Path;

fn main() -> Result<(), RustreeError> {
    let target_path = "."; // Current directory
    let path_obj = Path::new(target_path);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: path_obj.file_name().unwrap_or_default().to_string_lossy().into_owned(),
            root_is_directory: path_obj.is_dir(), // Set based on actual path
            ..Default::default()
        },
        listing: ListingOptions {
            max_depth: Some(2), // Limit depth to 2 levels
            ..Default::default()
        },
        ..Default::default()
    };

    // 1. Get the tree nodes
    let nodes = get_tree_nodes(path_obj, &config)?;

    // 2. Format the nodes into a string
    let output_string = format_nodes(&nodes, LibOutputFormat::Text, &config)?;

    // 3. Print the output
    println!("{}", output_string);

    Ok(())
}
```

### Example 2: Reporting Sizes and Sorting

This example demonstrates reporting file sizes and sorting by size in descending order.

```rust
use rustree::{
    get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat, SortKey, RustreeError,
    InputSourceOptions, MetadataOptions, SortingOptions,
};
use std::path::Path;

fn main() -> Result<(), RustreeError> {
    let target_path = "./src";
    let path_obj = Path::new(target_path);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "Source Files".to_string(),
            root_is_directory: path_obj.is_dir(),
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            show_last_modified: true, // To see mtime in output if sorting by size
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Size),
            reverse_sort: true, // Largest files first
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(path_obj, &config)?;
    let output_string = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("{}", output_string);

    Ok(())
}

```

### Example 3: Using Markdown Output and Line Counts

This example shows how to calculate line counts and output in Markdown format.

```rust
use rustree::{
    get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat, SortKey, RustreeError,
    InputSourceOptions, MetadataOptions, SortingOptions,
};
use std::path::Path;

fn main() -> Result<(), RustreeError> {
    let target_path = "./src";
    let path_obj = Path::new(target_path);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "Project Source (Markdown)".to_string(),
            root_is_directory: path_obj.is_dir(),
            ..Default::default()
        },
        metadata: MetadataOptions {
            calculate_line_count: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Lines), // Sort by line count
            reverse_sort: true,            // Most lines first
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(path_obj, &config)?;
    
    // Output as Markdown
    let markdown_output = format_nodes(&nodes, LibOutputFormat::Markdown, &config)?;
    println!("\n--- Markdown Output ---");
    println!("{}", markdown_output);
    // You could write this string to a .md file

    Ok(())
}
```

These examples should give you a good starting point for integrating `rustree` into your applications. Remember to handle the `Result` types appropriately in production code.