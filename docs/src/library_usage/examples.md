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

### Example 4: Using the Cat Function to Display File Contents

This example demonstrates using the `Cat` built-in function to display file contents after the tree structure.

```rust
use rustree::{
    get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat, BuiltInFunction, RustreeError,
    InputSourceOptions, MetadataOptions, FilteringOptions,
};
use std::path::Path;

fn main() -> Result<(), RustreeError> {
    let target_path = "./config"; // Directory with configuration files
    let path_obj = Path::new(target_path);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "Configuration Files".to_string(),
            root_is_directory: path_obj.is_dir(),
            ..Default::default()
        },
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::Cat), // Display file contents
            show_size_bytes: true, // Also show file sizes
            ..Default::default()
        },
        filtering: FilteringOptions {
            // Only show text-based config files
            match_patterns: Some(vec!["*.toml".to_string(), "*.json".to_string(), "*.yaml".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(path_obj, &config)?;
    
    // format_nodes will automatically display the tree first, then file contents
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("{}", output);
    
    // The output will show:
    // 1. Directory tree structure with file sizes
    // 2. "--- File Contents ---" section
    // 3. Each file's content with clear headers

    Ok(())
}
```

### Example 5: Combining Apply Functions with Custom Sorting

This example shows how to use built-in functions and sort by their results.

```rust
use rustree::{
    get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat, BuiltInFunction, SortKey, RustreeError,
    InputSourceOptions, MetadataOptions, SortingOptions,
};
use std::path::Path;

fn main() -> Result<(), RustreeError> {
    let target_path = "./text_files";
    let path_obj = Path::new(target_path);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "Text Analysis".to_string(),
            root_is_directory: path_obj.is_dir(),
            ..Default::default()
        },
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountPluses), // Count '+' characters
            calculate_line_count: true,
            calculate_word_count: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Custom), // Sort by the apply_function result
            reverse_sort: true, // Files with most '+' characters first
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(path_obj, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("{}", output);

    Ok(())
}
```

### Example 6: Directory Analysis with Built-in Functions

This example demonstrates using directory functions to analyze project structure.

```rust
use rustree::{
    get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat, BuiltInFunction, SortKey, RustreeError,
    InputSourceOptions, MetadataOptions, SortingOptions, ListingOptions,
};
use std::path::Path;

fn main() -> Result<(), RustreeError> {
    let target_path = "./my_project";
    let path_obj = Path::new(target_path);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "Project Analysis".to_string(),
            root_is_directory: path_obj.is_dir(),
            ..Default::default()
        },
        listing: ListingOptions {
            list_directories_only: true, // Only show directories
            ..Default::default()
        },
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::DirStats), // Get comprehensive directory stats
            show_size_bytes: true, // Required for size calculations
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Custom), // Sort by directory stats (complexity)
            reverse_sort: true, // Most complex directories first
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(path_obj, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("{}", output);
    
    // Output will show directories with stats like [F: "15f,3d,52KB"]
    // meaning 15 files, 3 subdirectories, 52KB total size

    Ok(())
}
```

### Example 7: Selective Function Application with Filtering

This example shows how to apply functions only to specific files or directories using patterns.

```rust
use rustree::{
    get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat, BuiltInFunction, RustreeError,
    InputSourceOptions, MetadataOptions, FilteringOptions,
};
use std::path::Path;

fn main() -> Result<(), RustreeError> {
    let target_path = "./workspace";
    let path_obj = Path::new(target_path);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "Workspace Analysis".to_string(),
            root_is_directory: path_obj.is_dir(),
            ..Default::default()
        },
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountFiles), // Count files in directories
            show_size_bytes: true,
            ..Default::default()
        },
        filtering: FilteringOptions {
            // Apply function only to source directories, exclude build artifacts
            apply_include_patterns: Some(vec!["src*".to_string(), "lib*".to_string()]),
            apply_exclude_patterns: Some(vec!["*target*".to_string(), "*build*".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(path_obj, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("{}", output);

    Ok(())
}
```

### Example 8: File Content Analysis with Filtering

This example demonstrates using the cat function with selective application for code review.

```rust
use rustree::{
    get_tree_nodes, format_nodes, RustreeLibConfig, LibOutputFormat, BuiltInFunction, RustreeError,
    InputSourceOptions, MetadataOptions, FilteringOptions,
};
use std::path::Path;

fn main() -> Result<(), RustreeError> {
    let target_path = "./src";
    let path_obj = Path::new(target_path);

    let config = RustreeLibConfig {
        input_source: InputSourceOptions {
            root_display_name: "Code Review".to_string(),
            root_is_directory: path_obj.is_dir(),
            ..Default::default()
        },
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::Cat), // Show file contents
            calculate_line_count: true,
            ..Default::default()
        },
        filtering: FilteringOptions {
            // Only show Rust files and exclude test files
            match_patterns: Some(vec!["*.rs".to_string()]),
            apply_exclude_patterns: Some(vec!["*test*".to_string(), "*tests*".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(path_obj, &config)?;
    let output = format_nodes(&nodes, LibOutputFormat::Text, &config)?;
    println!("{}", output);
    
    // This will show:
    // 1. Tree structure of all .rs files
    // 2. File contents for non-test .rs files only

    Ok(())
}
```

### Example 9: Working with NodeInfo Directly

This example shows how to work with the raw `NodeInfo` data for custom processing.

```rust
use rustree::{
    get_tree_nodes, RustreeLibConfig, BuiltInFunction, RustreeError,
    InputSourceOptions, MetadataOptions, NodeType,
};
use std::path::Path;

fn main() -> Result<(), RustreeError> {
    let target_path = "./project";
    let path_obj = Path::new(target_path);

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            apply_function: Some(BuiltInFunction::CountFiles),
            show_size_bytes: true,
            calculate_line_count: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let nodes = get_tree_nodes(path_obj, &config)?;

    // Custom analysis of the nodes
    for node in &nodes {
        match node.node_type {
            NodeType::Directory => {
                if let Some(Ok(file_count)) = &node.custom_function_output {
                    println!("Directory '{}' contains {} files", node.name, file_count);
                }
            }
            NodeType::File => {
                if let Some(lines) = node.line_count {
                    println!("File '{}' has {} lines", node.name, lines);
                }
                if let Some(size) = node.size {
                    println!("File '{}' is {} bytes", node.name, size);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
```

These examples should give you a good starting point for integrating `rustree` into your applications. Remember to handle the `Result` types appropriately in production code.