# Apply Functions

Apply functions are a powerful feature in rustree that allow you to execute custom operations on files during directory traversal. This guide provides comprehensive documentation on how to use each apply function type and their associated filtering patterns.

## Overview

Apply functions work by processing file contents or executing external commands on selected files. The results are then displayed in a dedicated "File Contents" section of the output. You can control which files are processed using include and exclude patterns.

### Basic Usage

```rust
use rustree::{RustreeLibConfig, MetadataOptions, FilteringOptions, ApplyFunction, BuiltInFunction};

let config = RustreeLibConfig {
    metadata: MetadataOptions {
        apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
        ..Default::default()
    },
    filtering: FilteringOptions {
        apply_include_patterns: Some(vec!["*.rs".to_string()]),
        ..Default::default()
    },
    ..Default::default()
};
```

## Built-in Functions

### Cat Function

The `Cat` function displays the complete contents of text files, similar to the Unix `cat` command.

#### Usage Example

```rust
use rustree::config::metadata::ApplyFunction;
use rustree::{BuiltInFunction, MetadataOptions};

let metadata_opts = MetadataOptions {
    apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
    ..Default::default()
};
```

#### Command Line Usage

```bash
# Display contents of all Rust files
rustree --apply-function cat --apply-include "*.rs"

# Display contents of files in src directory
rustree --apply-function cat --apply-include "src/**"

# Display contents of specific file
rustree --apply-function cat --apply-include "src/main.rs"
```

#### Output Format

```
├── src/
│   ├── main.rs
│   └── lib.rs

--- File Contents ---

=== src/main.rs ===
fn main() {
    println!("Hello, world!");
}

=== src/lib.rs ===
pub mod utils;
pub use utils::*;
```

### CountPluses Function

The `CountPluses` function counts the number of '+' characters in text files. This is useful for analyzing diffs or counting specific symbols.

#### Usage Example

```rust
use rustree::config::metadata::ApplyFunction;
use rustree::{BuiltInFunction, MetadataOptions};

let metadata_opts = MetadataOptions {
    apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::CountPluses)),
    ..Default::default()
};
```

#### Command Line Usage

```bash
# Count plus signs in all text files
rustree --apply-function count-pluses --apply-include "*.txt"

# Count plus signs in specific files
rustree --apply-function count-pluses --apply-include "diff.patch|*.diff"
```

#### Output Format

```
=== changes.diff ===
Plus count: 42
```

## External Functions

External functions allow you to execute custom shell commands on files and capture their output.

### Usage Example

```rust
use rustree::config::metadata::ApplyFunction;
use rustree::{ExternalFunction, MetadataOptions};

let external_fn = ExternalFunction {
    command: "wc".to_string(),
    args: vec!["-l".to_string()], // Count lines
};

let metadata_opts = MetadataOptions {
    apply_function: Some(ApplyFunction::External(external_fn)),
    ..Default::default()
};
```

### Command Line Usage

```bash
# Count lines in all source files using wc -l
rustree --apply-function "wc -l" --apply-include "src/**/*.rs"

# Get file info using the file command
rustree --apply-function "file" --apply-include "**/*"

# Custom analysis with grep
rustree --apply-function "grep -c TODO" --apply-include "**/*.rs"
```

### Security Considerations

- External functions execute shell commands with the permissions of the current user
- Always validate and sanitize external command inputs
- Be cautious when processing untrusted files or directories
- Consider using built-in functions when possible for better security

## Pattern Matching

Apply functions use sophisticated pattern matching to determine which files to process. This system supports both include and exclude patterns with glob syntax.

### Include Patterns (`apply_include_patterns`)

Include patterns specify which files should have the apply function executed on them. If include patterns are specified, **only** files matching these patterns will be processed.

#### Examples

```rust
// Single pattern
apply_include_patterns: Some(vec!["*.rs".to_string()])

// Multiple patterns
apply_include_patterns: Some(vec![
    "src/**/*.rs".to_string(),
    "tests/**/*.rs".to_string(),
])

// Pattern with pipe separator (OR logic)
apply_include_patterns: Some(vec!["*.rs|*.toml|*.md".to_string()])
```

### Exclude Patterns (`apply_exclude_patterns`)

Exclude patterns specify files that should be skipped, even if they match include patterns. Exclude patterns take precedence over include patterns.

#### Examples

```rust
// Exclude test files
apply_exclude_patterns: Some(vec!["**/test*.rs".to_string()])

// Exclude multiple file types
apply_exclude_patterns: Some(vec![
    "**/*.tmp".to_string(),
    "**/*.bak".to_string(),
])

// Complex exclusion with pipe separator
apply_exclude_patterns: Some(vec!["*.tmp|*.bak|**/target/**".to_string()])
```

### Pattern Evaluation Order

1. **Include Check**: If include patterns are specified, the file must match at least one include pattern
2. **Exclude Check**: If the file matches any exclude pattern, it is skipped
3. **Apply Function**: If both checks pass, the apply function is executed on the file

### Pattern Types

#### Exact Match

```rust
// Matches exactly "src/main.rs"
"src/main.rs"
```

#### Wildcard Patterns

```rust
// Matches all .rs files in src directory
"src/*.rs"

// Matches all files with any extension in src
"src/*.*"
```

#### Recursive Patterns

```rust
// Matches all .rs files anywhere under src (recursive)
"src/**/*.rs"

// Matches all files anywhere in the tree
"**/*"
```

#### Directory Patterns

```rust
// Matches files directly in cli directory
"src/cli/*"

// Matches all files recursively under cli
"src/cli/**"
```

#### Basename Patterns

```rust
// Matches any file named "Cargo.toml" anywhere
"Cargo.toml"

// Matches any .md file anywhere
"*.md"
```

### Advanced Pattern Examples

#### Complex Project Structure

```rust
let config = RustreeLibConfig {
    metadata: MetadataOptions {
        apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
        ..Default::default()
    },
    filtering: FilteringOptions {
        // Include all source files
        apply_include_patterns: Some(vec![
            "src/**/*.rs".to_string(),
            "tests/**/*.rs".to_string(),
            "examples/**/*.rs".to_string(),
        ]),
        // Exclude generated and temporary files
        apply_exclude_patterns: Some(vec![
            "**/target/**".to_string(),
            "**/*.tmp".to_string(),
            "**/.*".to_string(), // Hidden files
        ]),
        ..Default::default()
    },
    ..Default::default()
};
```

#### Conditional Processing

```rust
// Process only CLI-related files
FilteringOptions {
    apply_include_patterns: Some(vec!["src/cli/**/*.rs".to_string()]),
    ..Default::default()
}

// Process all files except tests and documentation
FilteringOptions {
    apply_include_patterns: Some(vec!["src/**/*.rs".to_string()]),
    apply_exclude_patterns: Some(vec![
        "**/test*.rs".to_string(),
        "**/*_test.rs".to_string(),
        "**/tests/**".to_string(),
    ]),
    ..Default::default()
}
```

### Case Sensitivity

Pattern matching respects the `case_insensitive_filter` option:

```rust
FilteringOptions {
    apply_include_patterns: Some(vec!["SRC/**/*.RS".to_string()]),
    case_insensitive_filter: true, // Will match "src/**/*.rs"
    ..Default::default()
}
```

### Working Directory Considerations

Pattern matching is always relative to the walk root directory, not the current working directory. This ensures consistent behavior regardless of where the command is executed from.

```rust
// When scanning /home/user/project, these patterns are equivalent:
// Pattern: "src/*.rs"
// Matches: /home/user/project/src/main.rs, /home/user/project/src/lib.rs

// Absolute patterns are also supported:
// Pattern: "/home/user/project/src/*.rs"
// Matches: /home/user/project/src/main.rs, /home/user/project/src/lib.rs
```

## Error Handling

Apply functions handle various error conditions gracefully:

### File Access Errors

- **Unreadable files**: Skipped with a note in the output
- **Permission denied**: Logged as an error, processing continues
- **Binary files**: Built-in functions skip binary files automatically

### External Command Errors

- **Command not found**: Error message displayed in output
- **Command failure**: Exit code and stderr captured and displayed
- **Timeout**: Commands that run too long are terminated

### Pattern Errors

- **Invalid glob patterns**: Configuration error returned immediately
- **Empty pattern lists**: Treated as "match nothing" for includes, "exclude nothing" for excludes

## Performance Considerations

### Pattern Optimization

- Use specific patterns rather than overly broad ones
- Prefer directory-specific patterns over global recursive patterns when possible
- Consider using exclude patterns to filter out large directories early

### Built-in vs External Functions

- **Built-in functions** are faster and more secure
- **External functions** provide more flexibility but have higher overhead
- For simple operations, prefer built-in functions

### Memory Usage

- Large files are processed one at a time to minimize memory usage
- External command output is captured and buffered
- Binary files are automatically skipped to prevent memory issues

## Complete Examples

### Development Workflow

```rust
// Show all source code for code review
let review_config = RustreeLibConfig {
    metadata: MetadataOptions {
        apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
        ..Default::default()
    },
    filtering: FilteringOptions {
        apply_include_patterns: Some(vec![
            "src/**/*.rs".to_string(),
            "*.toml".to_string(),
            "*.md".to_string(),
        ]),
        apply_exclude_patterns: Some(vec![
            "**/target/**".to_string(),
            "**/*.lock".to_string(),
        ]),
        ..Default::default()
    },
    ..Default::default()
};
```

### Documentation Generation

```rust
// Extract all TODO comments
let todo_config = RustreeLibConfig {
    metadata: MetadataOptions {
        apply_function: Some(ApplyFunction::External(ExternalFunction {
            command: "grep".to_string(),
            args: vec!["-n".to_string(), "TODO".to_string()],
        })),
        ..Default::default()
    },
    filtering: FilteringOptions {
        apply_include_patterns: Some(vec!["**/*.rs".to_string()]),
        apply_exclude_patterns: Some(vec!["**/target/**".to_string()]),
        ..Default::default()
    },
    ..Default::default()
};
```

### Quality Analysis

```rust
// Count lines of code
let loc_config = RustreeLibConfig {
    metadata: MetadataOptions {
        apply_function: Some(ApplyFunction::External(ExternalFunction {
            command: "wc".to_string(),
            args: vec!["-l".to_string()],
        })),
        ..Default::default()
    },
    filtering: FilteringOptions {
        apply_include_patterns: Some(vec![
            "src/**/*.rs".to_string(),
            "tests/**/*.rs".to_string(),
        ]),
        ..Default::default()
    },
    ..Default::default()
};
```

## API Reference

### Core Types

```rust
// Apply function types
pub enum ApplyFunction {
    BuiltIn(BuiltInFunction),
    External(ExternalFunction),
}

// Built-in function types
pub enum BuiltInFunction {
    Cat,
    CountPluses,
}

// External function configuration
pub struct ExternalFunction {
    pub command: String,
    pub args: Vec<String>,
}
```

### Configuration Options

```rust
// Metadata options for apply functions
pub struct MetadataOptions {
    pub apply_function: Option<ApplyFunction>,
    // ... other metadata options
}

// Filtering options for pattern matching
pub struct FilteringOptions {
    pub apply_include_patterns: Option<Vec<String>>,
    pub apply_exclude_patterns: Option<Vec<String>>,
    pub case_insensitive_filter: bool,
    // ... other filtering options
}
```

This comprehensive guide covers all aspects of using apply functions in rustree. For more advanced usage and integration examples, see the [Examples](examples.md) section.